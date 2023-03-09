use std::time::SystemTime;

use crate::peer::event::PeerEvent;

pub struct PeerMessage {
    uid: Vec<u8>,
    pub content: Vec<u8>,
}

impl Clone for PeerMessage {
    fn clone(&self) -> Self {
        PeerMessage {
            uid: self.uid.clone(),
            content: self.content.clone(),
        }
    }
}

impl PeerMessage {
    pub fn new(content: Vec<u8>) -> PeerMessage {
        PeerMessage {
            uid: Vec::from(SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap().as_millis()
                .to_string()),
            content,
        }
    }

    pub fn to_event(&self) -> PeerEvent {
        let mut data: Vec<u8> = Vec::new();
        let size = self.uid.len() as u8;
        data.push(size);
        for d in &self.uid {
            data.push(d.clone());
        }
        for c in &self.content {
            data.push(c.clone());
        }
        PeerEvent::message(data)
    }

    pub fn parse(data: Vec<u8>) -> PeerMessage {
        let size = data[0] as usize;
        PeerMessage {
            uid: data[1..(1 + size)].to_vec(),
            content: data[(1 + size)..data.len()].to_vec(),
        }
    }

    pub fn uid(message: &PeerMessage) -> String {
        String::from_utf8(message.uid.clone()).unwrap()
    }

    pub fn to_string(&self) -> String {
        String::from_utf8(self.content.clone()).unwrap()
    }
}
