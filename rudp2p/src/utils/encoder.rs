#[cfg(feature = "ssl")]
use openssl::rsa::{Padding, Rsa};

#[cfg(feature = "ssl")]
pub(crate) struct Encoder;

#[cfg(feature = "ssl")]
impl Encoder {
    pub(crate) fn encrypt(public_key_pem: &[u8], data: &Vec<u8>) -> Vec<u8> {
        let res = match Rsa::public_key_from_pem(public_key_pem) {
            Ok(rsa) => {
                let size = rsa.size() as usize;
                let mut buf = vec![0; size];
                match rsa.public_encrypt(data.as_slice(), &mut buf, Padding::PKCS1) {
                    Ok(_) => buf,
                    Err(e) => {
                        log::error!("{e}");
                        vec![]
                    }
                }
            }
            Err(e) => {
                log::error!("{e}");
                vec![]
            }
        };
        log::trace!(
            "encrypt({}, {}) => {:?}",
            public_key_pem.len(),
            data.len(),
            res
        );
        res
    }
}
