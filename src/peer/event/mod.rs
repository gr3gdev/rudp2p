use std::net::SocketAddr;

use crate::peer::RemotePeer;
use crate::server::Message;

// CONSTANTS

pub static DISCONNECTING: u8 = 0;
pub static CONNECTING: u8 = 1;
pub static CONNECTED: u8 = 2;
pub static MESSAGE: u8 = 3;
pub static DISCONNECTED: u8 = 9;

// COMMON FUNCTIONS

fn init_with_data(uid: String, mut list: Vec<u8>) -> Vec<u8> {
    let mut data = vec!(uid.len() as u8);
    data.append(&mut uid.as_bytes().to_vec());
    if !list.is_empty() {
        data.append(&mut list);
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

pub struct PeerConnectingEvent {
    /// Uid of the peer that connects.
    pub uid: String,
}

pub struct PeerConnectedEvent {
    /// Uid of the peer connected.
    pub uid: String,
    /// Address of the peer connected.
    pub addr: SocketAddr,
    /// List of peers with which you can connect.
    pub peers: Vec<RemotePeer>,
}

pub struct PeerMessageEvent {
    /// Uid of the peer connected.
    pub uid: String,
    /// Content of the message.
    pub content: Vec<u8>,
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

impl PeerEvent {
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

    /// Event: connecting.
    pub fn connecting(peer_connecting: PeerConnectingEvent) -> PeerEvent {
        let data = init_with_data(peer_connecting.uid, Vec::new());
        PeerEvent {
            code: CONNECTING,
            message: data.clone(),
        }
    }

    pub fn read_peer_connecting(content: &Vec<u8>) -> PeerConnectingEvent {
        let uid_size = content[0] as usize;
        let uid = content[1..(1 + uid_size)].to_vec();
        let uid = String::from_utf8(uid).unwrap();
        PeerConnectingEvent {
            uid
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
    pub fn connected(peer_connected_event: PeerConnectedEvent) -> PeerEvent {
        let uid = peer_connected_event.uid;
        let addr = peer_connected_event.addr;
        let peers = peer_connected_event.peers;
        let mut addr_list = addr.to_string().as_bytes().to_vec();
        let mut list = vec!(addr_list.len() as u8);
        list.append(&mut addr_list);
        for peer in peers {
            let peer_string = peer.uid + "|" + peer.addr.to_string().as_str();
            list.append(&mut peer_string.as_bytes().to_vec());
            list.append(&mut ",".as_bytes().to_vec());
        }
        PeerEvent {
            code: CONNECTED,
            message: init_with_data(uid, list),
        }
    }

    pub fn read_peer_connected(content: &Vec<u8>) -> PeerConnectedEvent {
        let uid_size = content[0] as usize;
        let uid = content[1..(1 + uid_size)].to_vec();
        let uid = String::from_utf8(uid).unwrap();
        let address_size = content[(1 + uid_size)] as usize;
        let address = &content[(2 + uid_size)..(2 + uid_size + address_size)];
        let address = String::from_utf8(address.to_vec()).unwrap();
        let address: SocketAddr = address.parse().expect("Unable to parse socket address");
        let peers = &content[(2 + uid_size + address_size)..content.len()];
        let list_peers = String::from_utf8(peers.to_vec()).unwrap();
        let peers_data = list_peers.split(",").collect::<Vec<&str>>();
        let mut peers = Vec::new();
        for peer in peers_data {
            let data = peer.split("|").collect::<Vec<&str>>();
            peers.push(RemotePeer {
                uid: data.get(0).unwrap().to_string(),
                addr: data.get(1).unwrap().to_string().parse().expect("Unable to parse socket address"),
                public_key: None,
                rsa: None,
            })
        }
        PeerConnectedEvent {
            uid,
            addr: address,
            peers,
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

    pub fn read_peer_message(content: &Vec<u8>) -> PeerMessageEvent {
        let uid_size = content[0] as usize;
        let uid = content[1..(1 + uid_size)].to_vec();
        let uid = String::from_utf8(uid).unwrap();
        PeerMessageEvent {
            uid,
            content: content[(1 + uid_size)..content.len()].to_vec().clone()
        }
    }

}

impl Message for PeerEvent {
    fn content(&self) -> Vec<u8> {
        self.as_bytes()
    }
}
