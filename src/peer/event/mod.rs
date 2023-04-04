use std::net::SocketAddr;
use std::time::SystemTime;

use openssl::rsa::{Padding, Rsa};

use crate::peer::RemotePeer;
use crate::server::Message;

// CONSTANTS

pub static DISCONNECTING: u8 = 0;
pub static CONNECTING: u8 = 1;
pub static CONNECTED: u8 = 2;
pub static MESSAGE: u8 = 3;
pub static DISCONNECTED: u8 = 9;

// COMMON FUNCTIONS

fn encrypt(public_key_pem: &Vec<u8>, data: Vec<u8>) -> Vec<u8> {
    let rsa = Rsa::public_key_from_pem(public_key_pem.as_slice()).unwrap();
    let mut buf = vec![0; rsa.size() as usize];
    rsa.public_encrypt(data.as_slice(), &mut buf, Padding::PKCS1).unwrap();
    buf
}

fn decrypt(private_key_pem: &Vec<u8>, passphrase: &str, data: Vec<u8>) -> Vec<u8> {
    let rsa = Rsa::private_key_from_pem_passphrase(private_key_pem.as_slice(), passphrase.as_bytes()).unwrap();
    let mut buf = vec![0; rsa.size() as usize];
    rsa.private_decrypt(&data, &mut buf, Padding::PKCS1).unwrap();
    buf
}

fn append_uid(data: &mut Vec<u8>, uid: String) {
    data.push(uid.len() as u8);
    data.append(&mut uid.as_bytes().to_vec());
}

fn parse_uid(message: &Vec<u8>) -> Data<String> {
    let uid_size = message[0] as usize;
    Data {
        value: String::from_utf8(message[1..(1 + uid_size)].to_vec()).expect("Unable to read UID"),
        size: uid_size,
    }
}

fn append_public_pem(data: &mut Vec<u8>, mut public_key: Vec<u8>) {
    data.push(public_key.len() as u8);
    data.append(&mut public_key);
}

fn parse_public_pem(message: &Vec<u8>, start: usize) -> Data<Vec<u8>> {
    let pem_size = message[start] as usize;
    let pem = &message[(1 + start)..(1 + start + pem_size)];
    Data {
        value: pem.to_vec(),
        size: pem_size,
    }
}

fn append_message(data: &mut Vec<u8>, mut message: Vec<u8>) {
    data.append(&mut message);
}

fn append_address(data: &mut Vec<u8>, addr: SocketAddr, public_key_pem: &Vec<u8>) {
    let address = addr.to_string();
    data.push(address.len() as u8);
    let mut encrypt_address = encrypt(public_key_pem, address.as_bytes().to_vec());
    data.append(&mut encrypt_address);
}

fn parse_address(message: &Vec<u8>, start: usize, private_key_pem: &Vec<u8>, passphrase: &str) -> Data<SocketAddr> {
    let address_size = message[start] as usize;
    let address = &message[(1 + start)..(1 + start + address_size)];
    let encrypt_address = address.to_vec();
    let address = decrypt(private_key_pem, passphrase, encrypt_address);
    let address = String::from_utf8(address).unwrap();
    Data {
        value: address.parse().expect("Unable to parse socket address"),
        size: address_size,
    }
}

fn append_peers(data: &mut Vec<u8>, peers: Vec<RemotePeer>, public_key_pem: &Vec<u8>) {
    let mut list = Vec::new();
    for peer in peers {
        let peer_string = peer.uid + "|" + peer.addr.to_string().as_str();
        list.append(&mut peer_string.as_bytes().to_vec());
        list.append(&mut ",".as_bytes().to_vec());
    }
    let mut encrypt_peers = encrypt(public_key_pem, list);
    data.append(&mut encrypt_peers);
}

fn parse_peers(message: &Vec<u8>, start: usize, private_key_pem: &Vec<u8>, passphrase: &str) -> Data<Vec<RemotePeer>> {
    let peers = &message[start..message.len()];
    let encrypt_peers = peers.to_vec();
    let peers = decrypt(private_key_pem, passphrase, encrypt_peers);
    let list_peers = String::from_utf8(peers).unwrap();
    let peers_data = list_peers.split(",").collect::<Vec<&str>>();
    let mut peers = Vec::new();
    for peer in peers_data {
        let data = peer.split("|").collect::<Vec<&str>>();
        peers.push(RemotePeer {
            uid: data.get(0).unwrap().to_string(),
            addr: data.get(1).unwrap().to_string().parse().expect("Unable to parse socket address"),
            public_key: None,
        })
    }
    Data {
        value: peers,
        size: message.len(),
    }
}

// TRAIT

pub trait Parser<O> {
    /// Parse message to object.
    fn parse(message: &Vec<u8>) -> O;
}

pub trait DecryptParser<O> {
    /// Parse message to object.
    fn parse(message: &Vec<u8>, private_key_pem: &Vec<u8>, passphrase: &str) -> O;
}

pub trait Split<T> {
    /// Split an objet in list.
    fn split(data: T, size: usize) -> Vec<T>;
}

pub trait Merge<T> {
    /// Merge a list in an object.
    fn merge(data: &Vec<T>) -> T;
    /// Check if data is complete (len == total).
    fn is_complete(&self) -> bool;
}

// STRUCT

struct Data<T> {
    value: T,
    size: usize,
}

pub struct PeerEvent {
    /// Unique ID.
    pub(crate) uid: String,
    /// Start of the byte array of the content message.
    start: usize,
    /// Size of the byte array of the content message.
    total: usize,
    /// Code of the peer event.
    pub code: u8,
    /// Content message of the peer event.
    pub message: Vec<u8>,
}

pub struct PeerIdentEvent {
    /// Uid of the peer that connects.
    pub uid: String,
}

pub struct PeerConnectingEvent {
    /// Uid of the peer that connects.
    pub uid: String,
    /// Public key for encrypt messages.
    pub public_key_pem: Vec<u8>,
}

pub struct PeerConnectedEvent {
    /// Uid of the peer connected.
    pub uid: String,
    /// Address of the peer connected.
    pub addr: SocketAddr,
    /// List of peers with which you can connect.
    pub peers: Vec<RemotePeer>,
    /// Public key for encrypt messages.
    pub public_key_pem: Vec<u8>,
}

pub struct PeerMessageEvent {
    /// Uid of the peer connected.
    pub uid: String,
    /// Content of the message.
    pub content: Vec<u8>,
}

// IMPL

impl Parser<PeerIdentEvent> for PeerIdentEvent {
    fn parse(message: &Vec<u8>) -> PeerIdentEvent {
        let uid_data = parse_uid(message);
        PeerIdentEvent {
            uid: uid_data.value,
        }
    }
}

impl Parser<PeerConnectingEvent> for PeerConnectingEvent {
    fn parse(message: &Vec<u8>) -> PeerConnectingEvent {
        let uid_data = parse_uid(message);
        let public_key_data = parse_public_pem(message, 1 + uid_data.size);
        PeerConnectingEvent {
            uid: uid_data.value,
            public_key_pem: public_key_data.value,
        }
    }
}

impl DecryptParser<PeerConnectedEvent> for PeerConnectedEvent {
    fn parse(message: &Vec<u8>, private_key_pem: &Vec<u8>, passphrase: &str) -> PeerConnectedEvent {
        let uid_data = parse_uid(message);
        let public_key_data = parse_public_pem(message, 1 + uid_data.size);
        let address_data = parse_address(message, 1 + uid_data.size + public_key_data.size, private_key_pem, passphrase);
        let peers_data = parse_peers(message, 1 + uid_data.size + public_key_data.size + address_data.size, private_key_pem, passphrase);
        PeerConnectedEvent {
            uid: uid_data.value,
            addr: address_data.value,
            peers: peers_data.value,
            public_key_pem: public_key_data.value,
        }
    }
}

impl Parser<PeerMessageEvent> for PeerMessageEvent {
    fn parse(message: &Vec<u8>) -> PeerMessageEvent {
        let uid_data = parse_uid(message);
        PeerMessageEvent {
            uid: uid_data.value,
            content: message[(1 + uid_data.size)..message.len()].to_vec().clone(),
        }
    }
}

impl Parser<PeerEvent> for PeerEvent {
    fn parse(data: &Vec<u8>) -> PeerEvent {
        let uid_size = data[0] as usize;
        let start = usize::from_ne_bytes([data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8]]);
        let total = usize::from_ne_bytes([data[9], data[10], data[11], data[12], data[13], data[14], data[15], data[16]]);
        let uid = data[17..(17 + uid_size)].to_vec();
        let code = data[17 + uid_size];
        let content = data[(18 + uid_size)..data.len()].to_vec();
        PeerEvent {
            uid: String::from_utf8(uid).expect("Unable to read uid of event"),
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
        append_uid(&mut data, self.uid.clone());
        data.append(&mut self.start.to_ne_bytes().to_vec());
        data.append(&mut self.total.to_ne_bytes().to_vec());
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
    fn new(code: u8, data: Vec<u8>) -> PeerEvent {
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

    /// Event: connecting.
    pub fn connecting(peer_connecting: PeerConnectingEvent) -> PeerEvent {
        let mut list = Vec::new();
        append_uid(&mut list, peer_connecting.uid);
        append_public_pem(&mut list, peer_connecting.public_key_pem);
        PeerEvent::new(CONNECTING, list)
    }

    /// Event: disconnecting.
    pub fn disconnecting(uid: String) -> PeerEvent {
        let mut list = Vec::new();
        append_uid(&mut list, uid);
        PeerEvent::new(DISCONNECTING, list)
    }

    /// Event: connected.
    pub fn connected(peer_connected_event: PeerConnectedEvent, public_key_pem: &Vec<u8>) -> PeerEvent {
        let mut list = Vec::new();
        append_uid(&mut list, peer_connected_event.uid);
        append_public_pem(&mut list, peer_connected_event.public_key_pem);
        append_address(&mut list, peer_connected_event.addr, public_key_pem);
        append_peers(&mut list, peer_connected_event.peers, public_key_pem);
        PeerEvent::new(CONNECTED, list)
    }

    /// Event: disconnected.
    pub fn disconnected(uid: String, addr: SocketAddr, public_key_pem: &Vec<u8>) -> PeerEvent {
        let mut list = Vec::new();
        append_uid(&mut list, uid);
        append_address(&mut list, addr, public_key_pem);
        PeerEvent::new(DISCONNECTED, list)
    }

    /// Event: message.
    pub fn message(uid: String, message: Vec<u8>) -> PeerEvent {
        let mut list = Vec::new();
        append_uid(&mut list, uid);
        append_message(&mut list, message);
        PeerEvent::new(MESSAGE, list)
    }
}
