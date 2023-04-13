use std::cell::RefCell;
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};

use openssl::symm::Cipher;

use crate::peer::{Peer, RemotePeer};
use crate::peer::event::common::Parser;
use crate::peer::event::PeerEvent;
use crate::peer::message::PeerMessage;
use crate::peer::router::event::{Route, RouterEvent};
use crate::server::{Event, ServerStatus};

pub(crate) mod event;

// STRUCT

pub(crate) struct Router {
    pub(crate) peer_uid: String,
    pub(crate) addr: SocketAddr,
    pub(crate) passphrase: &'static str,
    pub(crate) private_key_pem: Vec<u8>,
    pub(crate) public_key_pem: Vec<u8>,
    pub(crate) shared_peers: Arc<Mutex<HashMap<String, RemotePeer>>>,
    pub(crate) shared_message: Arc<Mutex<RefCell<Option<Box<dyn FnMut(&PeerMessage, &String) -> () + Send + Sync>>>>>,
    pub(crate) shared_connected: Arc<Mutex<RefCell<Option<Box<dyn FnMut(&String) -> () + Send + Sync>>>>>,
    pub(crate) shared_disconnected: Arc<Mutex<RefCell<Option<Box<dyn FnMut(&String) -> () + Send + Sync>>>>>,
    pub(crate) shared_connecting_map: Arc<Mutex<HashMap<String, Vec<PeerEvent>>>>,
    pub(crate) shared_connected_map: Arc<Mutex<HashMap<String, Vec<PeerEvent>>>>,
}

// IMPL

impl Router {
    pub(crate) fn new(peer: &Peer) -> Router {
        let keys = peer.keys.clone();
        let passphrase = "TODO_change_password";
        let private_key_pem = keys.private_key_to_pem_passphrase(
            Cipher::aes_256_cbc(),
            passphrase.as_bytes(),
        ).unwrap();
        Router {
            peer_uid: peer.uid.clone(),
            addr: peer.addr().clone(),
            passphrase,
            private_key_pem,
            public_key_pem: keys.public_key_to_pem().unwrap(),
            shared_peers: peer.peers.clone(),
            shared_message: peer.on_message_received.shared(),
            shared_connected: peer.on_peer_connected.shared(),
            shared_disconnected: peer.on_peer_disconnected.shared(),
            shared_connecting_map: peer.connecting_map.clone(),
            shared_connected_map: peer.connected_map.clone(),
        }
    }

    pub(crate) fn route(&self, e: &Event, socket: &UdpSocket) {
        let peer_event = PeerEvent::parse(&e.content);
        let router_event = RouterEvent::find_by_code(peer_event.code);
        router_event.execute(peer_event, socket, e.sender, self);
    }
}
