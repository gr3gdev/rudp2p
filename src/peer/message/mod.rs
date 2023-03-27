use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::time::SystemTime;

use crate::peer::event::PeerEvent;
use crate::utils::{read_file, write_file};

pub struct PeerMessage {
    /// UID of the message.
    pub uid: Vec<u8>,
    /// Start of the byte array of the content message.
    start: usize,
    /// Size of the byte array of the content message.
    total: usize,
    /// Part of the content message.
    pub content: Vec<u8>,
}

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
    pub fn from_text(text: &str) -> PeerMessage {
        PeerMessage::new(Vec::from(text))
    }

    pub fn from_file(path: &str) -> PeerMessage {
        PeerMessage::new(read_file(path))
    }

    fn new(content: Vec<u8>) -> PeerMessage {
        PeerMessage {
            uid: Vec::from(SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap().as_millis()
                .to_string()),
            start: 0,
            total: content.len(),
            content,
        }
    }

    pub fn concat(messages: &Vec<PeerMessage>) -> PeerMessage {
        let mut uid = Vec::new();
        let mut content = Vec::new();
        for message in messages {
            uid = message.uid.clone();
            if content.len() < message.total {
                for c in message.content.clone() {
                    content.push(c);
                }
            }
        }
        PeerMessage {
            uid,
            start: 0,
            total: content.len(),
            content,
        }
    }

    pub fn split(message: PeerMessage, size: usize) -> Vec<PeerMessage> {
        let mut messages = Vec::new();
        let total = message.content.len();
        for i1 in (0..message.content.len()).step_by(size) {
            let mut new_content = Vec::new();
            for i2 in i1..(i1 + size) {
                if let Some(data) = message.content.get(i2) {
                    new_content.push(*data);
                }
            }
            messages.push(PeerMessage {
                uid: message.uid.clone(),
                start: i1,
                total,
                content: new_content,
            })
        }
        messages
    }

    pub fn to_event(&self, uid: &String) -> PeerEvent {
        let mut data: Vec<u8> = Vec::new();
        let size = self.uid.len() as u8;
        data.push(size);
        for ne in self.start.to_ne_bytes() {
            data.push(ne);
        }
        for ne in self.total.to_ne_bytes() {
            data.push(ne);
        }
        for d in self.uid.clone() {
            data.push(d);
        }
        for c in self.content.clone() {
            data.push(c);
        }
        PeerEvent::message(uid.clone(), data)
    }

    pub fn parse(data: &Vec<u8>) -> PeerMessage {
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

    pub fn uid(message: &PeerMessage) -> String {
        String::from_utf8(message.uid.clone()).unwrap()
    }

    pub fn to_string(&self) -> String {
        String::from_utf8(self.content.clone()).unwrap()
    }

    pub fn to_file(&self, path: &str) -> File {
        write_file(self.content.as_slice(), path)
    }
}

impl Debug for PeerMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.uid.clone()).unwrap())
    }
}
