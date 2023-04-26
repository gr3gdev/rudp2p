// TRAIT

use std::net::{SocketAddr, UdpSocket};

use openssl::rsa::{Padding, Rsa};

use crate::peer::event::PeerEvent;
use crate::server::Message;

pub trait Parser<O> {
    /// Parse message to object.
    fn parse(message: &Vec<u8>) -> O;
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

// COMMON FUNCTIONS

pub(crate) fn send_with_socket(socket: &UdpSocket, peer_event: PeerEvent, addr: &SocketAddr) {
    for event in PeerEvent::split(peer_event, 1024) {
        socket.send_to(event.content().as_slice(), addr).unwrap();
    }
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

pub(crate) fn get_size_from_ne_bytes(message: &Vec<u8>, start: usize) -> usize {
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
