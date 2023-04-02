use std::net::SocketAddr;

use openssl::bn::BigNum;
use openssl::pkey::Public;
use openssl::rsa::Rsa;

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

pub trait Parser<O> {
    /// Parse message to object.
    fn parse(message: &Vec<u8>) -> O;
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
    /// Public key for encrypt messages.
    pub public_key: Rsa<Public>,
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

impl Parser<PeerConnectingEvent> for PeerConnectingEvent {
    fn parse(message: &Vec<u8>) -> PeerConnectingEvent {
        let uid_size = message[0] as usize;
        let uid = message[1..(1 + uid_size)].to_vec();
        let public_n_size = message[(1 + uid_size)] as usize;
        let public_n = message[(2 + uid_size)..(2 + uid_size + public_n_size)].to_vec();
        let n = BigNum::from_slice(public_n.as_slice()).unwrap();
        let public_e_size = message[(2 + uid_size + public_n_size)] as usize;
        let public_e = message[(3 + uid_size + public_n_size)..(3 + uid_size + public_n_size + public_e_size)].to_vec();
        let e = BigNum::from_slice(public_e.as_slice()).unwrap();
        let uid = String::from_utf8(uid).unwrap();
        PeerConnectingEvent {
            uid,
            public_key: Rsa::from_public_components(n, e).unwrap(),
        }
    }
}

impl Parser<PeerConnectedEvent> for PeerConnectedEvent {
    fn parse(message: &Vec<u8>) -> PeerConnectedEvent {
        let uid_size = message[0] as usize;
        let uid = message[1..(1 + uid_size)].to_vec();
        let uid = String::from_utf8(uid).unwrap();
        let address_size = message[(1 + uid_size)] as usize;
        let address = &message[(2 + uid_size)..(2 + uid_size + address_size)];
        let address = String::from_utf8(address.to_vec()).unwrap();
        let address: SocketAddr = address.parse().expect("Unable to parse socket address");
        let peers = &message[(2 + uid_size + address_size)..message.len()];
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
}

impl Parser<PeerMessageEvent> for PeerMessageEvent {
    fn parse(message: &Vec<u8>) -> PeerMessageEvent {
        let uid_size = message[0] as usize;
        let uid = message[1..(1 + uid_size)].to_vec();
        let uid = String::from_utf8(uid).unwrap();
        PeerMessageEvent {
            uid,
            content: message[(1 + uid_size)..message.len()].to_vec().clone(),
        }
    }
}

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
    pub fn convert_to_peer_event(content: Vec<u8>) -> PeerEvent {
        PeerEvent {
            code: content[0],
            message: content[1..content.len()].to_vec(),
        }
    }

    /// Event: connecting.
    pub fn connecting(peer_connecting: PeerConnectingEvent) -> PeerEvent {
        let mut public_n = peer_connecting.public_key.n().to_vec();
        let mut public_e = peer_connecting.public_key.e().to_vec();
        let mut pk_data = vec![public_n.len() as u8];
        pk_data.append(&mut public_n);
        pk_data.push(public_e.len() as u8);
        pk_data.append(&mut public_e);
        let data = init_with_data(peer_connecting.uid, pk_data);
        PeerEvent {
            code: CONNECTING,
            message: data.clone(),
        }
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
