use crate::utils::unwrap::unwrap_result;
use std::time::SystemTime;

pub(crate) mod decoder;
pub(crate) mod encoder;
pub(crate) mod multipart;
pub(crate) mod unwrap;

pub(crate) fn generate_uid(prefix: &str) -> String {
    let mut uid = String::from(prefix);
    uid.push_str(
        unwrap_result(
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH),
            "Error when generate uid",
        )
        .as_nanos()
        .to_string()
        .as_str(),
    );
    log::trace!("generate_uid({prefix}) => {uid}");
    uid
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "ssl")]
    fn encrypt_decrypt() {
        use crate::utils::{encoder::Encoder, decoder::Decoder};

        let rsa = openssl::rsa::Rsa::generate(1024).unwrap();
        let pk = rsa.public_key_to_pem().unwrap();
        let data = "Simple text's test.".as_bytes().to_vec();
        let encrypted = Encoder::encrypt(&pk, &data);
        assert_eq!(128, encrypted.len());
        let decrypted = Decoder::decrypt(&rsa, &encrypted, data.len());
        assert_eq!(data, decrypted);
    }
}
