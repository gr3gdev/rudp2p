use std::collections::HashMap;

use openssl::{pkey::Private, rsa::Rsa};

use crate::{
    dao::part::RequestPart,
    network::{request::Type, *},
    utils::{decoder::Decoder, encoder::Encoder, generate_uid},
};

fn calculate_size(public_key: &Vec<u8>) -> usize {
    let size = if !public_key.is_empty() {
        Rsa::public_key_from_pem(public_key).unwrap().size() as usize / 2
    } else {
        256 as usize
    };
    size
}

pub(crate) struct Multipart;

impl Multipart {
    pub(crate) fn split(request: &Request, public_key: &Vec<u8>) -> Vec<RequestPart> {
        let mut parts = Vec::new();
        let uid = generate_uid("R");
        let size = calculate_size(public_key);
        let total = request.content.len();
        for i in (0..total).step_by(size) {
            let mut max = i + size;
            if max > total {
                max = total;
            }
            let content_size = max - i;
            let data = request.content[i..max].to_vec();
            let content = if request.request_type == Type::Message {
                Encoder::encrypt(public_key, &data)
            } else {
                data.clone()
            };
            parts.push(RequestPart {
                uid: uid.clone(),
                request_type: request.request_type.clone(),
                start: i,
                total,
                content_size,
                content,
            })
        }
        parts
    }

    pub(crate) fn merge(
        parts: Vec<RequestPart>,
        request_type: Type,
        size: usize,
        private_key: &Rsa<Private>,
    ) -> Option<Request> {
        let mut seen = HashMap::new();
        let mut parts = parts.clone();
        parts.retain(|p| seen.insert(p.start, true).is_none());
        parts.sort();
        let data = parts
            .iter()
            .flat_map(|p| {
                if request_type == Type::Message {
                    Decoder::decrypt(private_key, &p.content, p.content_size)
                } else {
                    p.content.clone()
                }
            })
            .collect::<Vec<u8>>();
        if data.len() == size {
            Some(Request {
                request_type,
                content: data,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use openssl::{pkey::Private, rsa::Rsa};

    use crate::{
        dao::part::RequestPart,
        network::{request::Type, *},
    };

    use super::Multipart;

    fn create_test_request() -> (Request, usize) {
        let content = "
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
        Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
        Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
        Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
        ".as_bytes().to_vec();
        (
            Request {
                request_type: Type::Message,
                content: content.clone(),
            },
            content.len(),
        )
    }

    fn check_part(part: &RequestPart, start: usize, total: usize) {
        assert!(!part.content.is_empty());
        assert_eq!(start, part.start);
        assert_eq!(total, part.total);
    }

    fn generate_ssl() -> (Rsa<Private>, Vec<u8>) {
        let rsa = Rsa::generate(2048).unwrap();
        let pk = rsa.public_key_to_pem().unwrap();
        (rsa, pk)
    }

    #[test]
    fn split() {
        let (request, total) = create_test_request();
        let (_, pk) = generate_ssl();
        let parts = Multipart::split(&request, &pk);
        assert_eq!(4, parts.len());
        check_part(parts.get(0).unwrap(), 0, total);
        check_part(parts.get(1).unwrap(), 128, total);
        check_part(parts.get(2).unwrap(), 256, total);
        check_part(parts.get(3).unwrap(), 384, total);
    }

    #[test]
    fn merge() {
        let (request, total) = create_test_request();
        let (rsa, pk) = generate_ssl();
        let parts = Multipart::split(&request, &pk);
        let merge = Multipart::merge(parts, request.request_type.clone(), total, &rsa);
        assert!(merge.is_some());
        if let Some(merge) = merge {
            assert_eq!(request, merge);
        }
    }

    #[test]
    fn merge_incomplete() {
        let (request, total) = create_test_request();
        let (rsa, pk) = generate_ssl();
        let mut parts = Multipart::split(&request, &pk);
        parts.remove(parts.len() - 1);
        let merge = Multipart::merge(parts, request.request_type.clone(), total, &rsa);
        assert!(merge.is_none());
    }

    #[test]
    fn merge_with_duplicate_parts() {
        let (request, total) = create_test_request();
        let (rsa, pk) = generate_ssl();
        let mut parts = Multipart::split(&request, &pk);
        parts.push(parts.get(1).unwrap().clone());
        let merge = Multipart::merge(parts, request.request_type.clone(), total, &rsa);
        assert!(merge.is_some());
        if let Some(merge) = merge {
            assert_eq!(request, merge);
        }
    }
}
