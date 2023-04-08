use std::cell::RefCell;
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex, MutexGuard};

use openssl::rsa::Rsa;
use openssl::symm::Cipher;

use crate::peer::{Peer, RemotePeer};
use crate::peer::event::{CONNECTED, CONNECTING, DecryptParser, DISCONNECTED, DISCONNECTING, Merge, MESSAGE, Parser, PeerConnectedEvent, PeerConnectingEvent, PeerEvent, PeerIdentEvent, PeerMessageEvent, Split};
use crate::peer::message::PeerMessage;
use crate::server::{Event, Message, ServerStatus};

// COMMON FUNCTIONS

fn insert_peer(peers: &mut MutexGuard<HashMap<String, RemotePeer>>, uid: &String, address: SocketAddr, public_key_pem: &Vec<u8>) {
    match Rsa::public_key_from_pem(public_key_pem) {
        Ok(pk) => {
            peers.insert(uid.clone(), RemotePeer {
                uid: uid.clone(),
                addr: address,
                public_key: Some(pk),
            });
        }
        Err(..) => {
            panic!("Unable to generate public key from pem");
        }
    }
}

// STRUCT

pub struct Router {
    peer_uid: String,
    addr: SocketAddr,
    passphrase: &'static str,
    private_key_pem: Vec<u8>,
    public_pey_pem: Vec<u8>,
    shared_peers: Arc<Mutex<HashMap<String, RemotePeer>>>,
    shared_message: Arc<Mutex<RefCell<Option<Box<dyn FnMut(&PeerMessage, &String) -> () + Send + Sync>>>>>,
    shared_connected: Arc<Mutex<RefCell<Option<Box<dyn FnMut(&String) -> () + Send + Sync>>>>>,
    shared_disconnected: Arc<Mutex<RefCell<Option<Box<dyn FnMut(&String) -> () + Send + Sync>>>>>,
    shared_connecting_map: Arc<Mutex<HashMap<String, Vec<PeerEvent>>>>,
    shared_connected_map: Arc<Mutex<HashMap<String, Vec<PeerEvent>>>>,
}

// IMPL

impl Router {
    pub(crate) fn new(peer: &Peer) -> Router {
        let keys = peer.keys.clone();
        let passphrase = "TODO";
        let private_key_pem = keys.private_key_to_pem_passphrase(
            Cipher::aes_256_cbc(),
            passphrase.as_bytes(),
        ).unwrap();
        Router {
            peer_uid: peer.uid.clone(),
            addr: peer.addr().clone(),
            passphrase,
            private_key_pem,
            public_pey_pem: keys.public_key_to_pem().unwrap(),
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
        let mut peers = self.shared_peers.lock().unwrap();
        let mut connecting_map = self.shared_connecting_map.lock().unwrap();
        let mut connected_map = self.shared_connected_map.lock().unwrap();
        if peer_event.code == CONNECTING {
            let uid = peer_event.uid.clone();
            let mut list = connecting_map.get(&uid).unwrap_or(&Vec::new()).to_vec();
            list.push(peer_event);
            connecting_map.insert(uid, list);
            // Check connecting map
            let mut completed = Vec::new();
            for list in connecting_map.values() {
                let connecting_event = PeerEvent::merge(list);
                if connecting_event.is_complete() {
                    let peer_connecting = PeerConnectingEvent::parse(&connecting_event.message);
                    println!("[ {} ] complete connecting with {}", self.peer_uid, peer_connecting.uid);
                    Self::connecting(&self.peer_uid, self.addr, e.sender, socket, &mut peers, peer_connecting, &self.public_pey_pem);
                    completed.push(connecting_event.uid)
                }
            }
            for uid in completed {
                connecting_map.remove(&uid);
            }
        } else if peer_event.code == DISCONNECTING {
            let peer_disconnecting = PeerIdentEvent::parse(&peer_event.message);
            Self::disconnecting(&self.peer_uid, self.addr, e.sender, socket, &mut peers, peer_disconnecting, &self.public_pey_pem);
        } else if peer_event.code == CONNECTED {
            let uid = peer_event.uid.clone();
            let mut list = connected_map.get(&uid).unwrap_or(&Vec::new()).to_vec();
            list.push(peer_event);
            connected_map.insert(uid, list);
            // Check connected map
            let mut completed = Vec::new();
            for list in connected_map.values() {
                let connected_event = PeerEvent::merge(list);
                if connected_event.is_complete() {
                    let guard_connected = self.shared_connected.lock().unwrap();
                    let peer_connected = PeerConnectedEvent::parse(&connected_event.message, &self.private_key_pem, self.passphrase);
                    println!("[ {} ] complete connected with {}", self.peer_uid, peer_connected.uid);
                    if !RemotePeer::exists(&peers, &peer_connected.uid) {
                        Self::connected(&self.peer_uid, self.addr, socket, &mut peers, &peer_connected, &self.public_pey_pem);
                        if let Some(ref mut connected) = *guard_connected.borrow_mut() {
                            connected(&peer_connected.uid);
                        }
                    }
                    completed.push(connected_event.uid);
                }
            }
            for uid in completed {
                connected_map.remove(&uid);
            }
        } else if peer_event.code == DISCONNECTED {
            let guard_disconnected = self.shared_disconnected.lock().unwrap();
            let peer_disconnected = PeerIdentEvent::parse(&peer_event.message);
            Self::disconnected(peers, &peer_disconnected);
            if let Some(ref mut disconnected) = *guard_disconnected.borrow_mut() {
                disconnected(&peer_disconnected.uid);
            };
        } else if peer_event.code == MESSAGE {
            let guard_message = self.shared_message.lock().unwrap();
            if let Some(ref mut observer) = *guard_message.borrow_mut() {
                let peer_message = PeerMessageEvent::parse(&peer_event.message);
                observer(&PeerMessage::parse(&peer_message.content), &peer_message.uid);
            } else {
                println!("No observer for {}", self.addr);
            };
        }
    }

    fn send_with_socket(socket: &UdpSocket, peer_event: PeerEvent, addr: &SocketAddr) {
        for event in PeerEvent::split(peer_event, 1024) {
            socket.send_to(event.content().as_slice(), addr).unwrap();
        }
    }

    fn connecting(peer_uid: &String, addr: SocketAddr, sender: SocketAddr, socket: &UdpSocket,
                  mut peers: &mut MutexGuard<HashMap<String, RemotePeer>>,
                  peer_connecting: PeerConnectingEvent,
                  public_key_pem: &Vec<u8>,
    ) {
        println!("[ {} ] connecting with {}", peer_uid, peer_connecting.uid);
        if !RemotePeer::exists(&peers, &peer_connecting.uid) {
            insert_peer(&mut peers, &peer_connecting.uid, sender, &peer_connecting.public_key_pem);
            // Response CONNECTED
            let my_peers = peers.values().cloned().collect();
            println!("[ {} ] send connected to {}", peer_uid, peer_connecting.uid);
            Self::send_with_socket(socket, PeerEvent::connected(PeerConnectedEvent {
                uid: peer_uid.clone(),
                addr,
                peers: my_peers,
                public_key_pem: public_key_pem.clone(),
            }, public_key_pem), &sender);
        }
    }

    fn disconnecting(peer_uid: &String, addr: SocketAddr, sender: SocketAddr, socket: &UdpSocket,
                     peers: &mut MutexGuard<HashMap<String, RemotePeer>>,
                     peer_disconnecting: PeerIdentEvent,
                     public_key_pem: &Vec<u8>,
    ) {
        peers.remove(&peer_disconnecting.uid);
        Self::send_with_socket(socket, PeerEvent::disconnected(peer_uid.clone(), addr, public_key_pem), &sender);
    }

    fn connected(peer_uid: &String, addr: SocketAddr, socket: &UdpSocket,
                 mut peers: &mut MutexGuard<HashMap<String, RemotePeer>>,
                 peer_connected: &PeerConnectedEvent,
                 public_key_pem: &Vec<u8>,
    ) {
        println!("connected : {} with {}", peer_uid, peer_connected.uid);
        insert_peer(&mut peers, &peer_connected.uid, peer_connected.addr, &peer_connected.public_key_pem);
        for other in &peer_connected.peers {
            Self::connecting(&peer_uid, addr, other.addr, socket, &mut peers, PeerConnectingEvent {
                uid: other.uid.clone(),
                public_key_pem: public_key_pem.clone(),
            }, public_key_pem);
        }
    }

    fn disconnected(mut peers: MutexGuard<HashMap<String, RemotePeer>>, peer_disconnected: &PeerIdentEvent) {
        peers.remove(&peer_disconnected.uid);
    }
}
