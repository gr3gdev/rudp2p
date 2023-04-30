use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::net::{SocketAddr, UdpSocket};
use std::sync::MutexGuard;
use std::time::Instant;

use openssl::pkey::Private;
use openssl::rsa::Rsa;

use event::PeerEvent;
use message::PeerMessage;

use crate::peer::event::common::Split;
use crate::peer::router::data::RouteData;
use crate::peer::router::event::RouterEvent;
use crate::peer::router::Router;
use crate::server::{Event, Message, Server, ServerStatus, Udp};
use crate::utils::{OptionalClosure, ThreadSafe};

pub(crate) mod event;
pub mod message;
pub(crate) mod router;

// TRAIT

pub(crate) trait Dispatch {
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

pub(crate) struct SimplePeer {
    /// Uid.
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
    /// Keys for encryption.
    keys: Rsa<Private>,
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
            RouteData::PublicKey(self.keys.public_key_to_pem().expect("Unable to get pem from public key")),
        ]), addr);
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
            keys: Rsa::generate(1024).expect("Unable to generate keys"),
            friendly_peers: ThreadSafe::new(Vec::new()),
            blocked_peers: ThreadSafe::new(Vec::new()),
        }
    }

    pub fn add_friendly_peers(&self, mut peers: Vec<String>) {
        let shared = self.friendly_peers.clone();
        shared.lock().unwrap().append(&mut peers);
    }

    pub fn block_peers(&self, mut peers: Vec<String>) {
        let shared = self.blocked_peers.clone();
        shared.lock().unwrap().append(&mut peers);
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
        let addr = remote_peer.simple_peer.addr;
        if addr != self.addr() {
            let peer_event = RouterEvent::Message.new_event(vec![
                RouteData::Uid(self.uid.clone()),
                RouteData::Message(message, self.keys.public_key_to_pem().expect("Unable to get pem from public key")),
            ]);
            self.send_with_server(peer_event, &addr);
        }
    }

    fn send_with_server(&self, peer_event: PeerEvent, addr: &SocketAddr) {
        for event in PeerEvent::split(peer_event, 1024) {
            self.server.send(event, addr);
        }
    }

    pub fn open(&mut self) {
        self.start();
    }

    pub fn close(&self) {
        self.stop();
    }
}

impl Debug for Peer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.uid)
    }
}
