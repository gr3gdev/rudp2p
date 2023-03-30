use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::net::{SocketAddr, UdpSocket};
use std::sync::MutexGuard;
use std::time::Instant;

use event::PeerEvent;
use message::PeerMessage;

use crate::peer::event::{AsBytes, CONNECTED, CONNECTING, DISCONNECTED, DISCONNECTING, MESSAGE, PeerConnecting};
use crate::server::{Event, Message, Server, ServerStatus, Udp};
use crate::utils::{OptionalClosure, ThreadSafe};

mod event;
pub mod message;

// COMMON FUNCTIONS

fn send_message_to_peers(socket: &UdpSocket, peer_event: PeerEvent, peers: &Vec<SimplePeer>, white_list: Vec<String>) {
    println!("send_message_to_peers {} {:?} {:?}", socket.local_addr().unwrap(), peers, white_list);
    for peer in peers {
        if white_list.is_empty() || white_list.contains(&peer.uid) {
            socket.send_to(peer_event.as_bytes().as_slice(), peer.addr).unwrap();
        }
    }
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
    fn send_to(&self, message: PeerMessage, addr: &SocketAddr) -> ();
    /// Connect with an other server.
    fn connect(&mut self, addr: &SocketAddr, white_list: Vec<String>) -> ();
}

// STRUCT

struct SimplePeer {
    /// Uid.
    uid: String,
    /// Address of the peer.
    addr: SocketAddr,
}

pub struct Peer {
    /// Uid.
    pub uid: String,
    /// UDP Server used by the peer to communicate.
    server: Udp,
    /// Address of the dispatch (optional).
    dispatcher_addr: RefCell<Option<SocketAddr>>,
    /// List of the other peers.
    peers: ThreadSafe<Vec<SimplePeer>>,
    /// Listener trigger when a message is received.
    on_message_received: OptionalClosure<dyn FnMut(&PeerMessage, &String) -> () + Send + Sync>,
    /// Listener trigger when a peer is connected.
    on_peer_connected: OptionalClosure<dyn FnMut(&String) -> () + Send + Sync>,
    /// Listener trigger when a peer is disconnected.
    on_peer_disconnected: OptionalClosure<dyn FnMut(&String) -> () + Send + Sync>,
}

// IMPL

impl Debug for SimplePeer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.uid.clone(), self.addr)
    }
}

impl SimplePeer {
    fn exists(list: &MutexGuard<Vec<SimplePeer>>, uid: &String) -> bool {
        list.iter().find(|p| p.uid.eq(uid)).is_some()
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
            let uid = PeerEvent::read_uid(&peer_event.message);
            let content = PeerEvent::read_after_uid(&peer_event.message);
            let mut peers = shared_peers.lock().unwrap();
            let guard_message = shared_message.lock().unwrap();
            let guard_connected = shared_connected.lock().unwrap();
            let guard_disconnected = shared_disconnected.lock().unwrap();
            if peer_event.code == CONNECTING {
                let white_list = PeerConnecting::read_white_list(&content);
                if !SimplePeer::exists(&peers, &uid) {
                    peers.push(SimplePeer {
                        uid: uid.clone(),
                        addr: e.sender,
                    });
                    send_message_to_peers(socket, PeerEvent::connected(uid.clone(), e.sender), &peers, white_list);
                }
            }
            if peer_event.code == DISCONNECTING {
                if let Some(index) = peers.iter().position(|p| p.uid.eq(&uid)) {
                    peers.remove(index);
                } else {
                    println!("Peer {} not found", e.sender);
                }
                send_message_to_peers(socket, PeerEvent::disconnected(uid.clone(), e.sender), &peers, Vec::new());
            }
            if peer_event.code == CONNECTED {
                let address = PeerConnecting::read_address(&content);
                println!("{} peers {:?} - connected : {} {}", addr, peers, uid, address);
                if !SimplePeer::exists(&peers, &uid) {
                    peers.push(SimplePeer {
                        uid: uid.clone(),
                        addr: address,
                    });
                    if let Some(ref mut connected) = *guard_connected.borrow_mut() {
                        connected(&uid);
                    }
                    if address != addr {
                        socket.send_to(PeerEvent::connected(peer_uid.clone(), addr).as_bytes().as_slice(), address).unwrap();
                    }
                }
            }
            if peer_event.code == DISCONNECTED {
                if let Some(index) = peers.iter().position(|p| p.uid.eq(&uid)) {
                    peers.remove(index);
                    if let Some(ref mut disconnected) = *guard_disconnected.borrow_mut() {
                        disconnected(&uid);
                    }
                }
            }
            if peer_event.code == MESSAGE {
                if let Some(ref mut observer) = *guard_message.borrow_mut() {
                    observer(&PeerMessage::parse(&content), &uid);
                } else {
                    println!("No observer for {}", addr);
                }
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
        let addr = self.dispatcher_addr.borrow_mut();
        if addr.is_some() {
            self.send(PeerEvent::disconnecting(self.uid.clone()), &addr.unwrap());
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
        for msg in PeerMessage::split(message, 1024) {
            for other in recipients.iter() {
                if other.uid.ne(&self.uid) {
                    self.server.send(msg.to_event(&self.uid), &other.addr)
                }
            }
        }
    }

    fn send_to(&self, message: PeerMessage, addr: &SocketAddr) -> () {
        for msg in PeerMessage::split(message, 1024) {
            if addr != &self.addr() {
                self.server.send(msg.to_event(&self.uid), addr)
            }
        }
    }

    fn connect(&mut self, addr: &SocketAddr, white_list: Vec<String>) -> () {
        self.start();
        self.dispatcher_addr = RefCell::new(Some(addr.clone()));
        self.server.send(PeerEvent::connecting(PeerConnecting {
            uid: self.uid.clone(),
            white_list,
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
            dispatcher_addr: RefCell::new(None),
            peers: ThreadSafe::new(Vec::new()),
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
}

impl Debug for Peer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.uid)
    }
}
