use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::net::{SocketAddr, UdpSocket};
use std::sync::MutexGuard;
use std::time::Instant;

use openssl::rsa::Rsa;
use openssl::symm::Cipher;

use event::PeerEvent;
use message::PeerMessage;

use crate::peer::event::common::Split;
use crate::peer::router::data::RouteData;
use crate::peer::router::event::RouterEvent;
use crate::peer::router::Router;
use crate::server::{Event, Message, Server, ServerStatus, Udp};
use crate::utils::{OptionalClosure, ThreadSafe};

pub(crate) mod event;
/// Module with Peer messages methods
pub mod message;
pub(crate) mod router;

// TRAIT

/// # Dispatch
///
/// This trait is used to manage connections and exchanges between peers.
///
/// When a peer send a CONNECTING event then he received CONNECTED and the dispatch automatically sends others CONNECTING events to other peers connected with him.
pub(crate) trait Dispatch {
    /// Manage the exchanges with peers.
    fn routing(&mut self);
}

/// # Exchange
///
/// This trait provides methods :
/// - to connect with one peer (by address)
/// - to send a message to all peers connected with him
/// - to send a message to a specific peer connected with him (by uid)
pub trait Exchange {
    /// Send a message to all peers.
    fn send_to_all(&self, message: PeerMessage) -> ();
    /// Send a message to a specific peer (by uid).
    fn send_to(&self, message: PeerMessage, uid: &String) -> ();
    /// Connect with an other peer (by address).
    fn connect(&mut self, addr: &SocketAddr) -> ();
}

// STRUCT

pub(crate) struct SimplePeer {
    /// Unique identifier.
    uid: String,
    /// Address of the peer.
    addr: SocketAddr,
}

pub(crate) struct RemotePeer {
    /// Data of peer.
    simple_peer: SimplePeer,
    /// Public key for encrypt messages.
    public_key_pem: Vec<u8>,
}

/// # Peer
///
/// A structure for P2P communications.
pub struct Peer {
    /// Unique identifier of the peer.
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
    /// Public key for encryption.
    public_key_pem: Vec<u8>,
    /// Private key for decryption.
    private_key_pem: Vec<u8>,
    /// Passphrase for decryption.
    passphrase: &'static str,
    /// Friendly peers.
    friendly_peers: ThreadSafe<Vec<String>>,
    /// Peers to block.
    blocked_peers: ThreadSafe<Vec<String>>,
}

// IMPL

impl Display for SimplePeer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.uid, self.addr)
    }
}

impl Debug for RemotePeer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.simple_peer)
    }
}

impl RemotePeer {
    fn exists(list: &MutexGuard<HashMap<String, RemotePeer>>, uid: &String) -> bool {
        list.get(uid).is_some()
    }
}

impl Clone for SimplePeer {
    fn clone(&self) -> Self {
        SimplePeer {
            uid: self.uid.clone(),
            addr: self.addr.clone(),
        }
    }
}

impl SimplePeer {
    fn new(uid: String, addr: SocketAddr) -> SimplePeer {
        SimplePeer {
            uid,
            addr,
        }
    }
}

impl Clone for RemotePeer {
    fn clone(&self) -> Self {
        RemotePeer {
            simple_peer: self.simple_peer.clone(),
            public_key_pem: self.public_key_pem.clone(),
        }
    }
}

impl RemotePeer {
    fn new(uid: String, addr: SocketAddr, public_key_pem: Vec<u8>) -> RemotePeer {
        RemotePeer {
            simple_peer: SimplePeer::new(uid, addr),
            public_key_pem,
        }
    }
}

impl Dispatch for Peer {
    fn routing(&mut self) {
        let mut router = Router::new(self);
        let local_addr = self.server.addr();
        self.server.set_on_received(Box::new(move |e: &Event, socket: &UdpSocket| {
            if local_addr != e.sender {
                // Ignore internal routes
                router.route(e, socket);
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

    fn stop(&self) -> () {
        let shared_peers = self.peers.clone();
        let peers = shared_peers.lock().unwrap();
        for peer in peers.values() {
            self.send(RouterEvent::Disconnecting.new_event(vec![
                RouteData::Uid(self.uid.clone())
            ]), &peer.simple_peer.addr);
        }
        self.server.stop();
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
        self.server.send(RouterEvent::Connecting.new_event(vec![
            RouteData::Uid(self.uid.clone()),
            RouteData::PublicKey(self.public_key_pem.clone()),
        ]), addr);
    }
}

impl Peer {
    /// Create a new Peer.
    ///
    /// - **port** : the port number (local)
    /// - **uid** : an UID (optional, generated by default)
    pub fn new(port: u16, uid: Option<String>) -> Peer {
        let uid = uid.or_else(|| Some(Instant::now().elapsed().as_millis().to_string())).unwrap();
        let rsa = Rsa::generate(1024).expect("Unable to generate keys");
        let passphrase = "P@ssword_temp";
        Peer {
            uid,
            server: Udp::new(port),
            peers: ThreadSafe::new(HashMap::new()),
            on_message_received: OptionalClosure::new(None),
            on_peer_connected: OptionalClosure::new(None),
            on_peer_disconnected: OptionalClosure::new(None),
            public_key_pem: rsa.public_key_to_pem().expect("Unable to generate public key pem"),
            private_key_pem: rsa.private_key_to_pem_passphrase(Cipher::aes_256_cbc(), passphrase.as_bytes()).expect("Unable to generate private key pem"),
            passphrase,
            friendly_peers: ThreadSafe::new(Vec::new()),
            blocked_peers: ThreadSafe::new(Vec::new()),
        }
    }

    /// Add a list of friendly peers.
    ///
    /// - **peers** : list of UID
    pub fn add_friendly_peers(&self, mut peers: Vec<String>) {
        let shared = self.friendly_peers.clone();
        shared.lock().unwrap().append(&mut peers);
    }

    /// Add a list of block peers.
    ///
    /// - **peers** : list of UID
    pub fn block_peers(&self, mut peers: Vec<String>) {
        let shared_authorized = self.friendly_peers.clone();
        let mut guard = shared_authorized.lock().unwrap();
        let shared_peers = self.peers.clone();
        for uid in &peers {
            // Remove friendly peer
            if let Some(position) = guard.iter().position(|p| p.eq(uid)) {
                guard.remove(position);
            }
            // Remove peer connected with
            shared_peers.lock().unwrap().remove(uid.as_str());
        }
        // Add in black list
        let shared = self.blocked_peers.clone();
        shared.lock().unwrap().append(&mut peers);
    }

    /// Add on message received observer.
    pub fn set_on_message_received<F>(&mut self, on_message_received: F) where F: FnMut(&PeerMessage, &String) -> () + Send + Sync + 'static {
        OptionalClosure::set(&self.on_message_received, Box::new(on_message_received));
    }

    /// Add on peer connected observer.
    pub fn set_on_peer_connected<F>(&mut self, on_peer_connected: F) where F: FnMut(&String) -> () + Send + Sync + 'static {
        OptionalClosure::set(&self.on_peer_connected, Box::new(on_peer_connected));
    }

    /// Add on peer disconnected observer.
    pub fn set_on_peer_disconnected<F>(&mut self, on_peer_disconnected: F) where F: FnMut(&String) -> () + Send + Sync + 'static {
        OptionalClosure::set(&self.on_peer_disconnected, Box::new(on_peer_disconnected));
    }

    fn send_to_remote_peer(&self, message: PeerMessage, remote_peer: &RemotePeer) {
        let addr = remote_peer.simple_peer.addr;
        if addr != self.server.addr() {
            let peer_event = RouterEvent::Message.new_event(vec![
                RouteData::Uid(self.uid.clone()),
                RouteData::Message(message, remote_peer.public_key_pem.clone()),
            ]);
            self.send_with_server(peer_event, &addr);
        }
    }

    fn send_with_server(&self, peer_event: PeerEvent, addr: &SocketAddr) {
        for event in PeerEvent::split(peer_event, 1024) {
            self.server.send(event, addr);
        }
    }

    /// Open the Peer.
    pub fn open(&mut self) {
        self.start();
    }

    /// Close the Peer.
    pub fn close(&self) {
        self.stop();
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

impl Debug for Peer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.uid)
    }
}
