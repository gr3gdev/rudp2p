use std::net::SocketAddr;
use crate::server::Message;

// CONSTANTS

pub static DISCONNECTING: u8 = 0;
pub static CONNECTING: u8 = 1;
pub static CONNECTED: u8 = 2;
pub static MESSAGE: u8 = 3;
pub static DISCONNECTED: u8 = 9;

// COMMON FUNCTIONS

fn init_with_data(uid: String, list: Vec<u8>) -> Vec<u8> {
    let mut data = Vec::new();
    let uid_size = uid.len() as u8;
    data.push(uid_size);
    for b in uid.as_bytes().to_vec() {
        data.push(b);
    }
    if !list.is_empty() {
        for b in list {
            data.push(b);
        }
    }
    data
}

// TRAIT

pub trait AsBytes {
    /// Convert struct to bytes u8.
    fn as_bytes(&self) -> Vec<u8>;
}

// STRUCT

pub struct PeerEvent {
    /// Code of the peer event.
    pub code: u8,
    /// Content message of the peer event.
    pub message: Vec<u8>,
}

pub struct PeerConnecting {
    /// Uid of the peer that connects.
    pub uid: String,
    /// Whitelist of Uid with whom peer can communicate.
    pub white_list: Vec<String>,
}

// IMPL

impl AsBytes for PeerEvent {
    fn as_bytes(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.code);
        for m in self.message.clone() {
            data.push(m);
        }
        data
    }
}

impl PeerConnecting {
    pub fn read_address(message: &Vec<u8>) -> SocketAddr {
        let msg_address = String::from_utf8(message.clone()).unwrap();
        let address: SocketAddr = msg_address.parse().expect("Unable to parse socket address");
        address
    }

    pub fn read_white_list(message: &Vec<u8>) -> Vec<String> {
        let white_list = String::from_utf8(message.clone()).unwrap();
        println!("white_list : {}", white_list);
        let mut list = Vec::new();
        for uid in white_list.split(",") {
            if !uid.is_empty() {
                list.push(uid.to_string());
            }
        }
        list
    }
}

impl PeerEvent {
    /// Event: connecting.
    pub fn connecting(peer_connecting: PeerConnecting) -> PeerEvent {
        let mut data = init_with_data(peer_connecting.uid, Vec::new());
        for w in peer_connecting.white_list {
            for b in w.as_bytes().to_vec() {
                data.push(b);
            }
            for e in ",".as_bytes() {
                data.push(e.clone());
            }
        }
        PeerEvent {
            code: CONNECTING,
            message: data.clone(),
        }
    }

    pub fn convert_to_peer_event(content: Vec<u8>) -> PeerEvent {
        PeerEvent {
            code: content[0],
            message: content[1..content.len()].to_vec(),
        }
    }

    pub fn read_uid(content: &Vec<u8>) -> String {
        let uid_size = content[0] as usize;
        let uid = content[1..(1 + uid_size)].to_vec();
        String::from_utf8(uid).unwrap()
    }

    pub fn read_after_uid(content: &Vec<u8>) -> Vec<u8> {
        let uid_size = content[0] as usize;
        content[(1 + uid_size)..content.len()].to_vec()
    }

    /// Event: disconnecting.
    pub fn disconnecting(uid: String) -> PeerEvent {
        PeerEvent {
            code: DISCONNECTING,
            message: init_with_data(uid, Vec::new()),
        }
    }

    /// Event: connected.
    pub fn connected(uid: String, addr: SocketAddr) -> PeerEvent {
        PeerEvent {
            code: CONNECTED,
            message: init_with_data(uid, addr.to_string().as_bytes().to_vec()),
        }
    }

    /// Event: disconnected.
    pub fn disconnected(uid: String, addr: SocketAddr) -> PeerEvent {
        PeerEvent {
            code: DISCONNECTED,
            message: init_with_data(uid, addr.to_string().as_bytes().to_vec()),
        }
    }

    /// Event: message.
    pub fn message(uid: String, message: Vec<u8>) -> PeerEvent {
        PeerEvent {
            code: MESSAGE,
            message: init_with_data(uid, message),
        }
    }
}

impl Message for PeerEvent {
    fn content(&self) -> Vec<u8> {
        self.as_bytes()
    }
}
