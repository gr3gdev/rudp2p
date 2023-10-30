use crate::{
    configuration::Configuration,
    network::{request::*, *},
    utils::{generate_uid, unwrap::unwrap_option},
};
use std::net::SocketAddr;

#[cfg(not(feature = "ssl"))]
fn get_public_key_size(_public_key: &Vec<u8>) -> usize {
    1024
}

#[cfg(feature = "ssl")]
fn get_public_key_size(public_key: &Vec<u8>) -> usize {
    use super::unwrap::unwrap_result;

    unwrap_result(
        openssl::rsa::Rsa::public_key_from_pem(public_key),
        "Unable to get the public key size !",
    )
    .size() as usize
        / 2
}

fn calculate_size(public_key: &Vec<u8>) -> usize {
    let size = if !public_key.is_empty() {
        get_public_key_size(public_key)
    } else {
        1024 as usize
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
            let content = get_content(request, public_key, data);
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
        configuration: &Configuration,
    ) -> (Request, SocketAddr) {
        let first = unwrap_option(parts.first(), "Unable to get the first request part");
        let request_type = first.request_type.clone();
        let addr = first.sender;
        let data = parts
            .iter()
            .flat_map(|p: &RequestPart| parse_content(&request_type, configuration, p))
            .collect::<Vec<u8>>();
        let res = (
            Request {
                request_type,
                content: data,
            },
            addr,
        );
        log::trace!(
            "Multipart::merge({:?}, {:?}) => {:?}",
            parts,
            configuration,
            res
        );
        res
    }
}

#[cfg(not(feature = "ssl"))]
fn parse_content(_request_type: &Type, _configuration: &Configuration, p: &RequestPart) -> Vec<u8> {
    p.content.clone()
}

#[cfg(feature = "ssl")]
fn parse_content(request_type: &Type, configuration: &Configuration, p: &RequestPart) -> Vec<u8> {
    use crate::utils::decoder::Decoder;

    if request_type.clone() == Type::Message {
        Decoder::decrypt(&configuration.ssl.private_key, &p.content, p.content_size)
    } else {
        p.content.clone()
    }
}

#[cfg(not(feature = "ssl"))]
fn get_content(_request: &Request, _public_key: &Vec<u8>, data: Vec<u8>) -> Vec<u8> {
    data.clone()
}

#[cfg(feature = "ssl")]
fn get_content(request: &Request, public_key: &Vec<u8>, data: Vec<u8>) -> Vec<u8> {
    use crate::utils::encoder::Encoder;

    let content = if request.request_type == Type::Message {
        Encoder::encrypt(public_key, &data)
    } else {
        data.clone()
    };
    content
}

#[cfg(test)]
mod tests {
    use crate::network::{request::*, *};

    use super::Multipart;

    fn create_test_request() -> (Request, usize) {
        let content = "
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
        Elementum pulvinar etiam non quam. Pellentesque nec nam aliquam sem. Fermentum leo vel orci porta. 
        Velit ut tortor pretium viverra suspendisse potenti nullam. Quisque id diam vel quam. 
        Ac tortor vitae purus faucibus ornare suspendisse sed nisi. Amet purus gravida quis blandit turpis cursus in hac habitasse. 
        Potenti nullam ac tortor vitae purus. Nisl vel pretium lectus quam id leo. Nulla at volutpat diam ut venenatis tellus in metus.
        Vel orci porta non pulvinar neque laoreet suspendisse interdum consectetur. Massa tempor nec feugiat nisl pretium fusce. 
        Praesent semper feugiat nibh sed pulvinar proin gravida hendrerit lectus. Porttitor rhoncus dolor purus non enim. 
        Cras sed felis eget velit aliquet. Rhoncus dolor purus non enim praesent elementum. 
        Condimentum vitae sapien pellentesque habitant morbi tristique senectus. 
        Gravida quis blandit turpis cursus in hac habitasse platea.

        Lacinia at quis risus sed vulputate odio ut enim. Pellentesque eu tincidunt tortor aliquam nulla facilisi. 
        Venenatis cras sed felis eget velit aliquet sagittis. Proin libero nunc consequat interdum. Quis risus sed vulputate odio. 
        Nisl condimentum id venenatis a condimentum vitae sapien pellentesque. 
        Vulputate mi sit amet mauris commodo quis imperdiet massa tincidunt. Mi sit amet mauris commodo quis imperdiet massa. 
        Dignissim diam quis enim lobortis. Quam vulputate dignissim suspendisse in est ante in. Etiam non quam lacus suspendisse. 
        Vulputate enim nulla aliquet porttitor lacus luctus. Mi tempus imperdiet nulla malesuada pellentesque elit eget gravida cum. 
        Eget dolor morbi non arcu. Viverra aliquet eget sit amet tellus cras adipiscing enim.

        Eu mi bibendum neque egestas congue. Ante metus dictum at tempor commodo ullamcorper a. Diam in arcu cursus euismod quis. 
        Convallis convallis tellus id interdum velit laoreet id donec ultrices. Sed turpis tincidunt id aliquet risus feugiat in ante.
        Mollis aliquam ut porttitor leo a. Congue quisque egestas diam in arcu cursus. 
        At ultrices mi tempus imperdiet nulla malesuada pellentesque elit. 
        Pulvinar sapien et ligula ullamcorper malesuada proin libero nunc consequat. 
        Felis bibendum ut tristique et egestas quis ipsum. Vel eros donec ac odio tempor orci. 
        Metus aliquam eleifend mi in nulla posuere. Morbi tempus iaculis urna id volutpat. 
        Etiam tempor orci eu lobortis elementum nibh tellus. Dolor sit amet consectetur adipiscing elit ut aliquam. 
        Varius vel pharetra vel turpis nunc eget lorem dolor. Cursus euismod quis viverra nibh cras pulvinar. 
        Scelerisque mauris pellentesque pulvinar pellentesque habitant. Nam libero justo laoreet sit amet cursus sit amet dictum. 
        Integer malesuada nunc vel risus commodo viverra maecenas.
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

    #[cfg(not(feature = "ssl"))]
    #[test]
    fn split() {
        let (request, total) = create_test_request();
        let address = "127.0.0.1:9999".parse().unwrap();
        let parts = Multipart::split(&request, &vec![], &address);
        assert_eq!(3, parts.len());
        check_part(parts.get(0).unwrap(), 0, total);
        check_part(parts.get(1).unwrap(), 1024, total);
        check_part(parts.get(2).unwrap(), 2048, total);
    }

    #[cfg(feature = "ssl")]
    #[test]
    fn split() {
        use crate::configuration::{Configuration, SSL};

        let conf = Configuration::new(SSL::default());
        let (request, total) = create_test_request();
        let address = "127.0.0.1:9999".parse().unwrap();
        let parts = Multipart::split(&request, &conf.ssl.public_key, &address);
        assert_eq!(24, parts.len());
        for i in 0..24 {
            check_part(parts.get(i).unwrap(), i * 128, total);
        }
    }

    #[cfg(feature = "ssl")]
    #[test]
    fn merge() {
        use crate::configuration::{Configuration, SSL};

        let conf = Configuration::new(SSL::default());
        let (request, _) = create_test_request();
        let address = "127.0.0.1:9999".parse().unwrap();
        let parts = Multipart::split(&request, &conf.ssl.public_key, &address);
        let merge = Multipart::merge(&parts, &conf);
        assert_eq!(request, merge.0);
        assert_eq!(address, merge.1);
    }
}
