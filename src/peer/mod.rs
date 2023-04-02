use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::net::{SocketAddr, UdpSocket};
use std::sync::MutexGuard;
use std::time::Instant;

use openssl::pkey::{Private, Public};
use openssl::rsa::Rsa;

use event::PeerEvent;
use message::PeerMessage;

use crate::peer::event::{AsBytes, CONNECTED, CONNECTING, DISCONNECTED, DISCONNECTING, MESSAGE, Parser, PeerConnectedEvent, PeerConnectingEvent, PeerMessageEvent};
use crate::server::{Event, Message, Server, ServerStatus, Udp};
use crate::utils::{OptionalClosure, ThreadSafe};

mod event;
pub mod message;

// COMMON FUNCTIONS

fn send_message_to_peers(socket: &UdpSocket, peer_event: PeerEvent, peers: &Vec<&RemotePeer>, white_list: Vec<String>) {
    println!("send_message_to_peers {} {:?} {:?}", socket.local_addr().unwrap(), peers, white_list);
    for peer in peers {
        if white_list.is_empty() || white_list.contains(&peer.uid) {
            socket.send_to(peer_event.as_bytes().as_slice(), peer.addr).unwrap();
        }
    }
}

fn insert_peer(peers: &mut MutexGuard<HashMap<String, RemotePeer>>, uid: &String, address: SocketAddr) {
    peers.insert(uid.clone(), RemotePeer {
        uid: uid.clone(),
        addr: address,
        public_key: None,
        rsa: None,
    });
}

// TRAIT

pub trait Dispatch {
    /// Manage the exchanges with peers.
    fn routing(&mut self);
}

pub trait Exchange {
    /// Send a message to all peers.
    fn send_to_all(&self, message: PeerMessage) -> ();
    /// Send a message to a specific peer.
    fn send_to(&self, message: PeerMessage, uid: &String) -> ();
    /// Connect with an other server.
    fn connect(&mut self, addr: &SocketAddr) -> ();
}

// STRUCT

pub struct RemotePeer {
    /// Uid.
    uid: String,
    /// Address of the peer.
    addr: SocketAddr,
    /// RSA public key for encrypt data.
    public_key: Option<Rsa<Public>>,
    /// RSA private key for decrypt remote message and public key for remote encryption.
    rsa: Option<Rsa<Private>>,
}

pub struct Peer {
    /// Uid.
    pub uid: String,
    /// UDP Server used by the peer to communicate.
    server: Udp,
    /// List of the other peers.
    peers: ThreadSafe<HashMap<String, RemotePeer>>,
    /// Listener trigger when a message is received.
    on_message_received: OptionalClosure<dyn FnMut(&PeerMessage, &String) -> () + Send + Sync>,
    /// Listener trigger when a peer is connected.
    on_peer_connected: OptionalClosure<dyn FnMut(&String) -> () + Send + Sync>,
    /// Listener trigger when a peer is disconnected.
    on_peer_disconnected: OptionalClosure<dyn FnMut(&String) -> () + Send + Sync>,
}

// IMPL

impl Debug for RemotePeer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.uid.clone(), self.addr)
    }
}

impl RemotePeer {
    fn exists(list: &MutexGuard<HashMap<String, RemotePeer>>, uid: &String) -> bool {
        list.get(uid).is_some()
    }
}

impl Clone for RemotePeer {
    fn clone(&self) -> Self {
        RemotePeer {
            uid: self.uid.clone(),
            addr: self.addr.clone(),
            public_key: None,
            rsa: None,
        }
    }
}

impl Dispatch for Peer {
    fn routing(&mut self) {
        let peer_uid = self.uid.clone();
        let addr = self.addr().clone();
        let shared_peers = self.peers.clone();
        let shared_message = self.on_message_received.shared();
        let shared_connected = self.on_peer_connected.shared();
        let shared_disconnected = self.on_peer_disconnected.shared();
        self.server.set_on_received(Box::new(move |e: &Event, socket: &UdpSocket| {
            let peer_event = PeerEvent::convert_to_peer_event(e.content.clone());
            let mut peers = shared_peers.lock().unwrap();
            if peer_event.code == CONNECTING {
                let peer_connecting = PeerConnectingEvent::parse(&peer_event.message);
                Self::connecting(&peer_uid, addr, e.sender, socket, &mut peers, peer_connecting)
            }
            if peer_event.code == DISCONNECTING {
                let peer_disconnecting = PeerConnectingEvent::parse(&peer_event.message);
                Self::disconnecting(&peer_uid, addr, e.sender, socket, &mut peers, peer_disconnecting);
            }
            if peer_event.code == CONNECTED {
                let guard_connected = shared_connected.lock().unwrap();
                let peer_connected = PeerConnectedEvent::parse(&peer_event.message);
                if !RemotePeer::exists(&peers, &peer_connected.uid) {
                    Self::connected(&peer_uid, socket, &mut peers, &peer_connected);
                    if let Some(ref mut connected) = *guard_connected.borrow_mut() {
                        connected(&peer_connected.uid);
                    }
                }
            }
            if peer_event.code == DISCONNECTED {
                let guard_disconnected = shared_disconnected.lock().unwrap();
                let peer_disconnected = PeerConnectingEvent::parse(&peer_event.message);
                Self::disconnected(peers, &peer_disconnected);
                if let Some(ref mut disconnected) = *guard_disconnected.borrow_mut() {
                    disconnected(&peer_disconnected.uid);
                };
            }
            if peer_event.code == MESSAGE {
                let guard_message = shared_message.lock().unwrap();
                if let Some(ref mut observer) = *guard_message.borrow_mut() {
                    let peer_message = PeerMessageEvent::parse(&peer_event.message);
                    observer(&PeerMessage::parse(&peer_message.content), &peer_message.uid);
                } else {
                    println!("No observer for {}", addr);
                };
            }
        }));
    }
}

impl Server<Peer> for Peer {
    fn start(&mut self) -> () {
        if !self.alive() {
            self.server.start();
            self.routing();
        }
    }

    fn close(&self) -> () {
        let shared_peers = self.peers.clone();
        let peers = shared_peers.lock().unwrap();
        for peer in peers.values() {
            self.send(PeerEvent::disconnecting(self.uid.clone()), &peer.addr);
        }
        self.server.close();
    }

    fn send<M>(&self, msg: M, addr: &SocketAddr) where M: Message {
        self.server.send(msg, addr)
    }
}

impl Exchange for Peer {
    fn send_to_all(&self, message: PeerMessage) -> () {
        let shared_peers = self.peers.clone();
        let recipients = shared_peers.lock().unwrap();
        for other in recipients.iter() {
            self.send_to_remote_peer(message.clone(), other.1);
        }
    }

    fn send_to(&self, message: PeerMessage, uid: &String) -> () {
        let shared_peers = self.peers.clone();
        let peers = shared_peers.lock().unwrap();
        let remote_peer = peers.get(uid).expect("UID not found");
        self.send_to_remote_peer(message, remote_peer);
    }

    fn connect(&mut self, addr: &SocketAddr) -> () {
        self.start();
        self.server.send(PeerEvent::connecting(PeerConnectingEvent {
            uid: self.uid.clone(),
        }), addr);
    }
}

impl ServerStatus for Peer {
    fn alive(&self) -> bool {
        self.server.alive()
    }

    fn addr(&self) -> SocketAddr {
        self.server.addr()
    }
}

impl Peer {
    pub fn new(port: u16, uid: Option<String>) -> Peer {
        let uid = uid.or_else(|| Some(Instant::now().elapsed().as_millis().to_string())).unwrap();
        Peer {
            uid,
            server: Udp::new(port),
            peers: ThreadSafe::new(HashMap::new()),
            on_message_received: OptionalClosure::new(None),
            on_peer_connected: OptionalClosure::new(None),
            on_peer_disconnected: OptionalClosure::new(None),
        }
    }

    pub fn set_on_message_received<F>(&mut self, on_message_received: F) where F: FnMut(&PeerMessage, &String) -> () + Send + Sync + 'static {
        OptionalClosure::set(&self.on_message_received, Box::new(on_message_received));
    }

    pub fn set_on_peer_connected<F>(&mut self, on_peer_connected: F) where F: FnMut(&String) -> () + Send + Sync + 'static {
        OptionalClosure::set(&self.on_peer_connected, Box::new(on_peer_connected));
    }

    pub fn set_on_peer_disconnected<F>(&mut self, on_peer_disconnected: F) where F: FnMut(&String) -> () + Send + Sync + 'static {
        OptionalClosure::set(&self.on_peer_disconnected, Box::new(on_peer_disconnected));
    }

    fn send_to_remote_peer(&self, message: PeerMessage, remote_peer: &RemotePeer) {
        let addr = remote_peer.addr;
        for msg in PeerMessage::split(message, 1024) {
            if addr != self.addr() {
                self.server.send(msg.to_event(&self.uid), &addr)
            }
        }
    }

    fn connecting(peer_uid: &String, addr: SocketAddr, sender: SocketAddr, socket: &UdpSocket,
                  mut peers: &mut MutexGuard<HashMap<String, RemotePeer>>,
                  peer_connecting: PeerConnectingEvent) {
        if !RemotePeer::exists(&peers, &peer_connecting.uid) {
            insert_peer(&mut peers, &peer_connecting.uid, sender);
            // Response CONNECTED
            let my_peers = peers.values().cloned().collect();
            socket.send_to(PeerEvent::connected(PeerConnectedEvent {
                uid: peer_uid.clone(),
                addr,
                peers: my_peers,
            }).content().as_slice(), &sender).unwrap();
        }
    }

    fn disconnecting(peer_uid: &String, addr: SocketAddr, sender: SocketAddr, socket: &UdpSocket,
                     peers: &mut MutexGuard<HashMap<String, RemotePeer>>,
                     peer_disconnecting: PeerConnectingEvent) {
        peers.remove(&peer_disconnecting.uid);
        socket.send_to(PeerEvent::disconnected(peer_uid.clone(), addr).content().as_slice(), sender).unwrap();
    }

    fn connected(peer_uid: &String, socket: &UdpSocket,
                 mut peers: &mut MutexGuard<HashMap<String, RemotePeer>>,
                 peer_connected: &PeerConnectedEvent) {
        insert_peer(&mut peers, &peer_connected.uid, peer_connected.addr);
        for other in &peer_connected.peers {
            if !RemotePeer::exists(&peers, &other.uid) {
                // New peer
                socket.send_to(PeerEvent::connecting(PeerConnectingEvent {
                    uid: peer_uid.clone(),
                }).content().as_slice(), other.addr).unwrap();
            }
        }
    }

    fn disconnected(mut peers: MutexGuard<HashMap<String, RemotePeer>>, peer_disconnected: &PeerConnectingEvent) {
        peers.remove(&peer_disconnected.uid);
    }
}

impl Debug for Peer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.uid)
    }
}
