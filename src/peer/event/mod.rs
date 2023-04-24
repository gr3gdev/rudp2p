use std::time::SystemTime;

use crate::peer::event::common::{Merge, Parser, Split};
use crate::server::Message;

pub(crate) mod common;
mod tests;

// STRUCT

pub struct PeerEvent {
    /// Unique ID.
    pub(crate) uid: String,
    /// Start of the byte array of the content message.
    start: usize,
    /// Size of the byte array of the content message.
    total: usize,
    /// Code of the peer connecting.
    pub code: u8,
    /// Content message of the peer connecting.
    pub message: Vec<u8>,
}

// IMPL

impl Parser<PeerEvent> for PeerEvent {
    fn parse(data: &Vec<u8>) -> PeerEvent {
        let uid_size = data[0] as usize;
        let start = usize::from_ne_bytes([data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8]]);
        let total = usize::from_ne_bytes([data[9], data[10], data[11], data[12], data[13], data[14], data[15], data[16]]);
        let uid = data[17..(17 + uid_size)].to_vec();
        let code = data[17 + uid_size];
        let content = data[(18 + uid_size)..data.len()].to_vec();
        PeerEvent {
            uid: String::from_utf8(uid).expect("Unable to read uid of connecting"),
            start,
            total,
            code,
            message: content,
        }
    }
}

impl Clone for PeerEvent {
    fn clone(&self) -> Self {
        PeerEvent {
            uid: self.uid.clone(),
            start: self.start.clone(),
            total: self.total.clone(),
            code: self.code.clone(),
            message: self.message.clone(),
        }
    }
}

impl Message for PeerEvent {
    fn content(&self) -> Vec<u8> {
        let mut data = Vec::new();
        let mut uid_data = self.uid.as_bytes().to_vec();
        data.push(uid_data.len() as u8);
        data.append(&mut self.start.to_ne_bytes().to_vec());
        data.append(&mut self.total.to_ne_bytes().to_vec());
        data.append(&mut uid_data);
        data.push(self.code);
        data.append(&mut self.message.clone());
        data
    }
}

impl Split<PeerEvent> for PeerEvent {
    fn split(data: PeerEvent, size: usize) -> Vec<PeerEvent> {
        let mut list = Vec::new();
        for i in (0..data.message.len()).step_by(size) {
            let mut max = i + size;
            if max > data.message.len() {
                max = data.message.len();
            }
            let new_content = data.message[i..max].to_vec();
            list.push(PeerEvent {
                uid: data.uid.clone(),
                start: i,
                total: data.total,
                code: data.code,
                message: new_content,
            })
        }
        list
    }
}

impl Merge<PeerEvent> for PeerEvent {
    fn merge(data: &Vec<PeerEvent>) -> PeerEvent {
        let mut uid = None;
        let mut code = None;
        let mut message = Vec::new();
        for event in data {
            if uid.is_none() && code.is_none() {
                uid = Some(event.uid.clone());
                code = Some(event.code);
            }
            if uid.is_some() && code.is_some() && uid.eq(&Some(event.uid.clone())) && code.eq(&Some(event.code)) {
                if message.len() < event.total {
                    for c in event.message.clone() {
                        message.push(c);
                    }
                }
            }
        }
        PeerEvent {
            uid: uid.expect("UID not found"),
            start: 0,
            total: message.len(),
            code: code.expect("Code not found"),
            message,
        }
    }

    fn is_complete(&self) -> bool {
        self.total > 0 && self.message.len() > 0 && self.total == self.message.len()
    }
}

impl PeerEvent {
    pub(crate) fn new(code: u8, data: Vec<u8>) -> PeerEvent {
        PeerEvent {
            uid: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap().as_millis()
                .to_string(),
            start: 0,
            total: data.len(),
            code,
            message: data,
        }
    }
}
