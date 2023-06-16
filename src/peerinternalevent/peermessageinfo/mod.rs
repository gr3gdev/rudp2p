use crate::{utils::encrypt, Content, PeerError, PeerMessageInfo};

impl Content for PeerMessageInfo {
    fn to_vec(&self, public_key_pem: Option<Vec<u8>>) -> Result<Vec<u8>, PeerError> {
        if let Some(public_key) = public_key_pem {
            encrypt(&public_key.as_slice(), &self.content)
        } else {
            Ok(self.content.clone())
        }
    }
}
