use log::error;
use openssl::{
    pkey::Private,
    rsa::{Padding, Rsa},
};

pub(crate) struct Decoder;

impl Decoder {
    pub(crate) fn get_size(list: &Vec<u8>, index: usize) -> (usize, usize) {
        let size = usize::from_ne_bytes([
            list[index],
            list[index + 1],
            list[index + 2],
            list[index + 3],
            list[index + 4],
            list[index + 5],
            list[index + 6],
            list[index + 7],
        ]);
        (size, index + 8)
    }

    pub(crate) fn decrypt(rsa: &Rsa<Private>, data: &Vec<u8>, original_size: usize) -> Vec<u8> {
        let mut buf = vec![0; rsa.size() as usize];
        match rsa.private_decrypt(&data, &mut buf, Padding::PKCS1) {
            Ok(_) => {
                buf.truncate(original_size);
                buf
            }
            Err(e) => {
                error!("{e}");
                vec![]
            }
        }
    }
}
