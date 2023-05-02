use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

use openssl::symm::Cipher;

use crate::info;
use crate::peer::{Peer, RemotePeer};
use crate::peer::event::common::{Merge, Parser, send_with_socket};
use crate::peer::event::PeerEvent;
use crate::peer::message::PeerMessage;
use crate::peer::router::event::{RouteEvent, RouterEvent};
use crate::server::Event;

pub(crate) mod event;
pub(crate) mod data;
mod tests;

// STRUCT

pub(crate) struct Router {
    pub(crate) peer_uid: String,
    pub(crate) passphrase: &'static str,
    pub(crate) private_key_pem: Vec<u8>,
    pub(crate) public_key_pem: Vec<u8>,
    pub(crate) shared_peers: Arc<Mutex<HashMap<String, RemotePeer>>>,
    pub(crate) shared_message: Arc<Mutex<RefCell<Option<Box<dyn FnMut(&PeerMessage, &String) -> () + Send + Sync>>>>>,
    pub(crate) shared_connected: Arc<Mutex<RefCell<Option<Box<dyn FnMut(&String) -> () + Send + Sync>>>>>,
    pub(crate) shared_disconnected: Arc<Mutex<RefCell<Option<Box<dyn FnMut(&String) -> () + Send + Sync>>>>>,
    complete_event: HashMap<String, Vec<PeerEvent>>,
    friendly_peers: Arc<Mutex<Vec<String>>>,
    blocked_peers: Arc<Mutex<Vec<String>>>,
}

// IMPL

impl Router {
    pub(crate) fn new(peer: &Peer) -> Router {
        let keys = peer.keys.clone();
        let passphrase = "TODO_change_password";
        let private_key_pem = keys.private_key_to_pem_passphrase(
            Cipher::aes_256_cbc(),
            passphrase.as_bytes(),
        ).expect("Unable to generate private key pem");
        let public_key_pem = keys.public_key_to_pem()
            .expect("Unable to generate public key pem");
        Router {
            peer_uid: peer.uid.clone(),
            passphrase,
            private_key_pem,
            public_key_pem,
            shared_peers: peer.peers.clone(),
            shared_message: peer.on_message_received.shared(),
            shared_connected: peer.on_peer_connected.shared(),
            shared_disconnected: peer.on_peer_disconnected.shared(),
            complete_event: HashMap::new(),
            friendly_peers: peer.friendly_peers.clone(),
            blocked_peers: peer.blocked_peers.clone(),
        }
    }

    pub(crate) fn route(&mut self, e: &Event, socket: &UdpSocket) {
        let peer_event = self.parse_complete_event(e);
        let event_uid = peer_event.uid.clone();
        if peer_event.is_complete() {
            let router_event = RouterEvent::find_by_code(peer_event.code);
            info!("[{}] \x1b[33mReceive\x1b[0m \x1b[35m{}\x1b[0m event (\x1b[33m{}\x1b[0m) from \x1b[33m{}\x1b[0m", self, router_event, event_uid, e.sender);
            if let Some(responses_peer_event) = router_event.responses_event(peer_event, e.sender, self) {
                for response in responses_peer_event {
                    let res_router_event = RouterEvent::find_by_code(response.peer_event.code);
                    info!("[{}] \x1b[34mSend\x1b[0m \x1b[35m{}\x1b[0m event (\x1b[33m{}\x1b[0m) to \x1b[33m{}\x1b[0m", self, res_router_event, response.peer_event.uid, response.address);
                    send_with_socket(socket, response.peer_event, &response.address);
                }
            }
        } else {
            info!("[{}] event not completed : \x1b[33m{}\x1b[0m", self, event_uid);
        }
    }

    fn parse_complete_event(&mut self, e: &Event) -> PeerEvent {
        let peer_event = PeerEvent::parse(&e.content);
        let mut events = Vec::new();
        if let Some(e) = self.complete_event.get(&peer_event.uid) {
            events.append(&mut e.clone());
        }
        events.push(peer_event.clone());
        self.complete_event.insert(peer_event.uid, events.clone());
        let peer_event = PeerEvent::merge(&events.clone());
        peer_event
    }
}

impl Display for Router {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let peers = self.shared_peers.lock().unwrap();
        write!(f, "{} {:?}", self.peer_uid, peers.keys())
    }
}
