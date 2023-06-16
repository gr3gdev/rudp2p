use openssl::pkey::Private;

use crate::{
    utils::from_ne_bytes, Content, Info, PeerConnectingInfo, PeerError, PeerMessage,
};
use openssl::rsa::Rsa;

impl Content for PeerConnectingInfo {
    fn to_vec(&self, _public_key_pem: Option<Vec<u8>>) -> Result<Vec<u8>, PeerError> {
        let mut data = Vec::new();
        let mut uid = self.uid.as_bytes().to_vec();
        let mut uid_size = uid.len().to_ne_bytes().to_vec();
        data.append(&mut uid_size);
        data.append(&mut uid);
        data.append(&mut self.public_key.clone());
        Ok(data)
    }
}

impl Info for PeerConnectingInfo {
    fn parse(message: &PeerMessage, _decrypt_data: Option<Rsa<Private>>) -> Result<Self, PeerError>
    where
        Self: Sized,
    {
        let data = message.content.clone();
        let uid_size = from_ne_bytes(&data, 0);
        let uid = String::from_utf8(data[8..8 + uid_size].to_vec()).unwrap();
        let public_key = data[8 + uid_size..data.len()].to_vec();
        Ok(PeerConnectingInfo { uid, public_key })
    }
}
