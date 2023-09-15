use log::error;
use openssl::rsa::{Padding, Rsa};

pub(crate) struct Encoder;

impl Encoder {
    pub(crate) fn add(list: Vec<u8>, mut data: Vec<u8>) -> Vec<u8> {
        let mut list = list.clone();
        list.append(&mut data);
        list
    }

    pub(crate) fn add_size(list: Vec<u8>, size: usize) -> Vec<u8> {
        Self::add(list, size.to_ne_bytes().to_vec())
    }

    pub(crate) fn add_with_size(list: Vec<u8>, data: Vec<u8>) -> Vec<u8> {
        let list = Self::add_size(list, data.len());
        let list = Self::add(list, data);
        list
    }

    pub(crate) fn encrypt(public_key_pem: &[u8], data: &Vec<u8>) -> Vec<u8> {
        match Rsa::public_key_from_pem(public_key_pem) {
            Ok(rsa) => {
                let size = rsa.size() as usize;
                let mut buf = vec![0; size];
                match rsa.public_encrypt(data.as_slice(), &mut buf, Padding::PKCS1) {
                    Ok(_) => buf,
                    Err(e) => {
                        error!("{e}");
                        vec![]
                    }
                }
            }
            Err(e) => {
                error!("{e}");
                vec![]
            }
        }
    }
}