use std::net::SocketAddr;

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
    log::trace!("calculate_size({}) => {size}", public_key.len());
    size
}

pub(crate) struct Multipart;

impl Multipart {
    pub(crate) fn split(
        request: &Request,
        public_key: &Vec<u8>,
        addr: &SocketAddr,
    ) -> Vec<RequestPart> {
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
                sender: addr.clone(),
            })
        }
        log::trace!(
            "Multipart::split({:?}, {}, {addr}) => {:?}",
            request,
            public_key.len(),
            parts
        );
        parts
    }

    pub(crate) fn merge(
        parts: &Vec<RequestPart>,
        private_key: &Rsa<Private>,
    ) -> (Request, SocketAddr) {
        let first = parts.get(0).unwrap();
        let request_type = first.request_type.clone();
        let addr = first.sender;
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
        let res = (
            Request {
                request_type,
                content: data,
            },
            addr,
        );
        log::trace!(
            "Multipart::merge({:?}, {}) => {:?}",
            parts,
            private_key.size(),
            res
        );
        res
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
        let address = "127.0.0.1:9999".parse().unwrap();
        let parts = Multipart::split(&request, &pk, &address);
        assert_eq!(4, parts.len());
        check_part(parts.get(0).unwrap(), 0, total);
        check_part(parts.get(1).unwrap(), 128, total);
        check_part(parts.get(2).unwrap(), 256, total);
        check_part(parts.get(3).unwrap(), 384, total);
    }

    #[test]
    fn merge() {
        let (request, _) = create_test_request();
        let (rsa, pk) = generate_ssl();
        let address = "127.0.0.1:9999".parse().unwrap();
        let parts = Multipart::split(&request, &pk, &address);
        let merge = Multipart::merge(&parts, &rsa);
        assert_eq!(request, merge.0);
        assert_eq!(address, merge.1);
    }
}
