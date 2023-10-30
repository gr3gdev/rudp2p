#[cfg(feature = "ssl")]
use openssl::{
    pkey::Private,
    rsa::{Padding, Rsa},
};

#[cfg(feature = "ssl")]
pub(crate) struct Decoder;

#[cfg(feature = "ssl")]
impl Decoder {
    pub(crate) fn decrypt(rsa: &Rsa<Private>, data: &Vec<u8>, original_size: usize) -> Vec<u8> {
        let mut buf = vec![0; rsa.size() as usize];
        let res = match rsa.private_decrypt(&data, &mut buf, Padding::PKCS1) {
            Ok(_) => {
                buf.truncate(original_size);
                buf
            }
            Err(e) => {
                log::error!("{e}");
                vec![]
            }
        };
        log::trace!("decrypt({}, {original_size}) => {:?}", rsa.size(), res);
        res
    }
}
