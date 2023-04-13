// TRAIT

use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::sync::MutexGuard;

use openssl::rsa::{Padding, Rsa};

use crate::peer::event::PeerEvent;
use crate::peer::RemotePeer;
use crate::server::Message;

mod tests;

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

pub(crate) struct Data<T> {
    pub(crate) value: T,
    pub(crate) end: usize,
}

// COMMON FUNCTIONS

pub(crate) fn complete_map(map: &mut MutexGuard<HashMap<String, Vec<PeerEvent>>>, peer_event: PeerEvent, uid: &String) {
    let mut list = map.get(uid).unwrap_or(&Vec::new()).to_vec();
    list.push(peer_event);
    map.insert(uid.clone(), list);
}

pub(crate) fn send_with_socket(socket: &UdpSocket, peer_event: PeerEvent, addr: &SocketAddr) {
    for event in PeerEvent::split(peer_event, 1024) {
        println!("[ Socket ] {} send data to {}", socket.local_addr().unwrap(), addr);
        socket.send_to(event.content().as_slice(), addr).unwrap();
    }
}

pub(crate) fn insert_peer(peers: &mut MutexGuard<HashMap<String, RemotePeer>>, uid: &String, address: SocketAddr, public_key_pem: &Vec<u8>) {
    peers.insert(uid.clone(), RemotePeer {
        uid: uid.clone(),
        addr: address,
        public_key_pem: Some(public_key_pem.clone()),
    });
}

pub(crate) fn encrypt(public_key_pem: &Vec<u8>, data: Vec<u8>) -> Vec<u8> {
    let rsa = Rsa::public_key_from_pem(public_key_pem.as_slice()).unwrap();
    let mut buf = vec![0; rsa.size() as usize];
    rsa.public_encrypt(data.as_slice(), &mut buf, Padding::PKCS1).unwrap();
    buf
}

pub(crate) fn decrypt(private_key_pem: &Vec<u8>, passphrase: &str, data: Vec<u8>) -> Vec<u8> {
    let rsa = Rsa::private_key_from_pem_passphrase(private_key_pem.as_slice(), passphrase.as_bytes())
        .expect("Unable to create private key from pem and passphrase");
    let mut buf = vec![0; rsa.size() as usize];
    rsa.private_decrypt(&data, &mut buf, Padding::PKCS1).expect("Unable to decrypt data");
    buf
}

pub(crate) fn append_uid(data: &mut Vec<u8>, uid: String) {
    data.push(uid.len() as u8);
    data.append(&mut uid.as_bytes().to_vec());
}

pub(crate) fn parse_uid(message: &Vec<u8>) -> Data<String> {
    let uid_size = message[0] as usize;
    let uid = String::from_utf8(message[1..(1 + uid_size)].to_vec()).expect("Unable to read UID");
    Data {
        value: uid,
        end: 1 + uid_size,
    }
}

pub(crate) fn append_public_pem(data: &mut Vec<u8>, public_key: &Vec<u8>) {
    data.append(&mut public_key.len().to_ne_bytes().to_vec());
    data.append(&mut public_key.clone());
}

fn get_size_from_ne_bytes(message: &Vec<u8>, start: usize) -> usize {
    usize::from_ne_bytes([
        message[start],
        message[start + 1],
        message[start + 2],
        message[start + 3],
        message[start + 4],
        message[start + 5],
        message[start + 6],
        message[start + 7]
    ])
}

pub(crate) fn parse_public_pem(message: &Vec<u8>, start: usize) -> Data<Vec<u8>> {
    let pem_size = get_size_from_ne_bytes(message, start);
    let pem = &message[(8 + start)..(8 + start + pem_size)];
    Data {
        value: pem.to_vec(),
        end: 8 + start + pem_size,
    }
}

pub(crate) fn append_message(data: &mut Vec<u8>, mut message: Vec<u8>) {
    // TODO encrypt
    data.append(&mut message);
}

pub(crate) fn append_address(data: &mut Vec<u8>, addr: SocketAddr, public_key_pem: &Vec<u8>) {
    let address = addr.to_string();
    let mut encrypt_address = encrypt(public_key_pem, address.as_bytes().to_vec());
    data.push(address.len() as u8);
    data.append(&mut encrypt_address.len().to_ne_bytes().to_vec());
    data.append(&mut encrypt_address);
}

pub(crate) fn parse_address(message: &Vec<u8>, start: usize, private_key_pem: &Vec<u8>, passphrase: &str) -> Data<SocketAddr> {
    let address_size = message[start] as usize;
    let encrypted_address_size = get_size_from_ne_bytes(message, start + 1);
    let address = &message[(9 + start)..(9 + start + encrypted_address_size)];
    let encrypt_address = address.to_vec();
    let mut address = decrypt(private_key_pem, passphrase, encrypt_address);
    address.truncate(address_size);
    println!("address: {:?}", address);
    let address = String::from_utf8(address).expect("Unable to create String");
    println!("address: {:?}", address);
    let address = address.parse().expect("Unable to parse socket address");
    Data {
        value: address,
        end: 9 + start + encrypted_address_size,
    }
}

pub(crate) fn append_peers(data: &mut Vec<u8>, peers: Vec<RemotePeer>, public_key_pem: &Vec<u8>) {
    let mut list = Vec::new();
    for peer in peers {
        let peer_string = peer.uid + "|" + peer.addr.to_string().as_str();
        list.append(&mut peer_string.as_bytes().to_vec());
        list.append(&mut ",".as_bytes().to_vec());
    }
    let mut encrypt_peers = encrypt(public_key_pem, list.clone());
    data.append(&mut list.len().to_ne_bytes().to_vec());
    data.append(&mut encrypt_peers);
}

pub(crate) fn parse_peers(message: &Vec<u8>, start: usize, private_key_pem: &Vec<u8>, passphrase: &str) -> Data<Vec<RemotePeer>> {
    let real_size = get_size_from_ne_bytes(message, start);
    let peers = &message[8 + start..message.len()];
    let encrypt_peers = peers.to_vec();
    let mut peers = decrypt(private_key_pem, passphrase, encrypt_peers);
    peers.truncate(real_size);
    let list_peers = String::from_utf8(peers).unwrap();
    let peers_data = list_peers.split(",").collect::<Vec<&str>>();
    let mut peers = Vec::new();
    for peer in peers_data {
        let data = peer.split("|").collect::<Vec<&str>>();
        if data.len() >= 2 {
            peers.push(RemotePeer {
                uid: data.get(0).expect("Uid not found").to_string(),
                addr: data.get(1).expect("Address not found").to_string().parse().expect("Unable to parse socket address"),
                public_key_pem: None,
            })
        }
    }
    Data {
        value: peers,
        end: message.len(),
    }
}
