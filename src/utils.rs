use std::time::SystemTime;

pub(crate) mod decoder;
pub(crate) mod encoder;
pub(crate) mod multipart;

pub(crate) fn generate_uid(prefix: &str) -> String {
    let mut uid = String::from(prefix);
    uid.push_str(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .to_string()
            .as_str(),
    );
    log::trace!("generate_uid({prefix}) => {uid}");
    uid
}

#[cfg(test)]
mod tests {
    use openssl::rsa::Rsa;

    use crate::utils::{decoder::Decoder, encoder::Encoder};

    #[test]
    fn add() {
        let list = Encoder::add(&Vec::new(), &vec![1, 2, 3]);
        assert_eq!(vec![1, 2, 3], list);
    }

    #[test]
    fn add_size() {
        let list = Encoder::add_size(&Vec::new(), 3);
        assert_eq!(vec![3, 0, 0, 0, 0, 0, 0, 0], list);
    }

    #[test]
    fn add_with_size() {
        let list = Encoder::add_with_size(&Vec::new(), &vec![1, 2, 3]);
        assert_eq!(vec![3, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3], list);
    }

    #[test]
    fn get_size() {
        let (size, next_index) = Decoder::get_size(&vec![3, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3], 0);
        assert_eq!(3, size);
        assert_eq!(8, next_index);
    }

    #[test]
    fn encrypt_decrypt() {
        let rsa = Rsa::generate(1024).unwrap();
        let pk = rsa.public_key_to_pem().unwrap();
        let data = "Simple text's test.".as_bytes().to_vec();
        let encrypted = Encoder::encrypt(&pk, &data);
        assert_eq!(128, encrypted.len());
        let decrypted = Decoder::decrypt(&rsa, &encrypted, data.len());
        assert_eq!(data, decrypted);
    }
}
