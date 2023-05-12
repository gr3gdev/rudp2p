use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::time::SystemTime;

use crate::peer::event::common::{decrypt, encrypt, get_size_from_ne_bytes};
use crate::utils::{read_file, write_file};

// STRUCT

/// # PeerMessage
///
/// A structure for send and receive messages.
pub struct PeerMessage {
    /// Unique identifier of the message.
    pub uid: Vec<u8>,
    /// Start of the byte array of the content message.
    start: usize,
    /// Size of the byte array of the content message.
    total: usize,
    /// Part of the content message.
    pub content: Vec<u8>,
}

// IMPL

impl Clone for PeerMessage {
    fn clone(&self) -> Self {
        PeerMessage {
            uid: self.uid.clone(),
            start: self.start,
            total: self.total,
            content: self.content.clone(),
        }
    }
}

impl PeerMessage {
    /// Generate a PeerMessage from text.
    pub fn from_text(text: &str) -> PeerMessage {
        PeerMessage::new(Vec::from(text), None)
    }

    /// Generate a PeerMessage from file.
    pub fn from_file(path: &str) -> PeerMessage {
        PeerMessage::new(read_file(path), None)
    }

    pub(crate) fn new(content: Vec<u8>, uid: Option<Vec<u8>>) -> PeerMessage {
        PeerMessage {
            uid: uid.or_else(|| Some(Vec::from(SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap().as_millis()
                .to_string()))).unwrap(),
            start: 0,
            total: content.len(),
            content,
        }
    }

    pub(crate) fn encode(&self, public_key_pem: &Vec<u8>) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(self.uid.len() as u8);
        data.append(&mut self.start.to_ne_bytes().to_vec());
        data.append(&mut self.total.to_ne_bytes().to_vec());
        data.append(&mut self.uid.clone());
        data.append(&mut self.content.clone());
        let real_size = data.len();
        let mut data_with_size = Vec::new();
        data_with_size.append(&mut real_size.to_ne_bytes().to_vec());
        data_with_size.append(&mut encrypt(public_key_pem, data));
        data_with_size
    }

    pub(crate) fn parse(encrypted_data: Vec<u8>, private_key_pem: &Vec<u8>, passphrase: String) -> PeerMessage {
        let real_size = get_size_from_ne_bytes(&encrypted_data, 0);
        let mut data = decrypt(private_key_pem, passphrase.as_str(), encrypted_data[8..encrypted_data.len()].to_vec());
        data.truncate(real_size);
        let uid_size = data[0] as usize;
        let start = usize::from_ne_bytes([data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8]]);
        let total = usize::from_ne_bytes([data[9], data[10], data[11], data[12], data[13], data[14], data[15], data[16]]);
        let uid = data[17..(17 + uid_size)].to_vec();
        let content = data[(17 + uid_size)..data.len()].to_vec();
        PeerMessage {
            uid,
            start,
            total,
            content,
        }
    }

    /// Get the UID of the PeerMessage.
    pub fn uid(message: &PeerMessage) -> String {
        String::from_utf8(message.uid.clone()).unwrap()
    }

    /// Convert the PeerMessage to text.
    pub fn to_string(&self) -> String {
        String::from_utf8(self.content.clone()).unwrap()
    }

    /// Convert the PeerMessage to file.
    pub fn to_file(&self, path: &str) -> File {
        write_file(self.content.as_slice(), path)
    }
}

impl Debug for PeerMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.uid.clone()).unwrap())
    }
}
