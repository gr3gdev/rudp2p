use openssl::pkey::Private;
use openssl::rsa::Rsa;

use crate::{Content, Info, PeerDisconnectingInfo, PeerError, PeerMessage};

impl Content for PeerDisconnectingInfo {
    fn to_vec(&self, _public_key_pem: Option<Vec<u8>>) -> Result<Vec<u8>, PeerError> {
        Ok(self.uid.clone().into_bytes().to_vec())
    }
}

impl Info for PeerDisconnectingInfo {
    fn parse(message: &PeerMessage, _decrypt_data: Option<Rsa<Private>>) -> Result<Self, PeerError>
    where
        Self: Sized,
    {
        let uid = String::from_utf8(message.content.clone()).unwrap();
        Ok(PeerDisconnectingInfo { uid })
    }
}
