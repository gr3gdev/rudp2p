use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::net::{SocketAddr, UdpSocket};
use std::time::Instant;

use event::PeerEvent;
use message::PeerMessage;

use crate::peer::event::{AsBytes, CONNECTED, CONNECTING, DISCONNECTED, DISCONNECTING, MESSAGE};
use crate::server::{Event, Server, ServerStatus, Udp};
use crate::utils::{OptionalClosure, ThreadSafe};

mod event;
pub mod message;

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
    fn connect(&mut self, addr: &SocketAddr) -> ();
}

pub struct Peer {
    pub uid: String,
    server: Udp,
    dispatcher_addr: RefCell<Option<SocketAddr>>,
    peers: ThreadSafe<Vec<SocketAddr>>,
    on_message_received: OptionalClosure<dyn FnMut(&PeerMessage, &SocketAddr) -> () + Send + Sync>,
    on_peer_connected: OptionalClosure<dyn FnMut(&SocketAddr) -> () + Send + Sync>,
    on_peer_disconnected: OptionalClosure<dyn FnMut(&SocketAddr) -> () + Send + Sync>,
}

fn get_address(e: &Event) -> SocketAddr {
    let msg_address = String::from_utf8(e.content[1..e.content.len()].to_vec()).unwrap();
    let address: SocketAddr = msg_address.parse().expect("Unable to parse socket address");
    address
}

fn send_message_to_peers(socket: &UdpSocket, peer_event: PeerEvent, peers: &Vec<SocketAddr>) {
    for peer in peers {
        socket.send_to(peer_event.as_bytes().as_slice(), peer).unwrap();
    }
}

impl Dispatch for Peer {
    fn routing(&mut self) {
        let addr = self.addr().clone();
        let shared_peers = self.peers.clone();
        let shared_message = self.on_message_received.shared();
        let shared_connected = self.on_peer_connected.shared();
        let shared_disconnected = self.on_peer_disconnected.shared();
        self.server.set_on_received(Box::new(move |e: &Event, socket: &UdpSocket| {
            let mut peers = shared_peers.lock().unwrap();
            let guard_message = shared_message.lock().unwrap();
            let guard_connected = shared_connected.lock().unwrap();
            let guard_disconnected = shared_disconnected.lock().unwrap();
            if e.content[0] == CONNECTING {
                if !peers.contains(&e.sender) {
                    peers.push(e.sender);
                    send_message_to_peers(socket, PeerEvent::connected(e.sender), &peers);
                }
            }
            if e.content[0] == DISCONNECTING {
                if let Some(index) = peers.iter().position(|p| p == &e.sender) {
                    peers.remove(index);
                } else {
                    println!("Peer {} not found", e.sender);
                }
                send_message_to_peers(socket, PeerEvent::disconnected(e.sender), &peers);
            }
            if e.content[0] == CONNECTED {
                let address = get_address(e);
                if !peers.contains(&address) {
                    peers.push(address);
                    if let Some(ref mut connected) = *guard_connected.borrow_mut() {
                        connected(&address);
                    }
                    if address != addr {
                        socket.send_to(PeerEvent::connected(addr).as_bytes().as_slice(), address).unwrap();
                    }
                }
            }
            if e.content[0] == DISCONNECTED {
                let address = get_address(e);
                if let Some(index) = peers.iter().position(|p| p == &address) {
                    peers.remove(index);
                    if let Some(ref mut disconnected) = *guard_disconnected.borrow_mut() {
                        disconnected(&address);
                    }
                }
            }
            if e.content[0] == MESSAGE {
                if let Some(ref mut observer) = *guard_message.borrow_mut() {
                    observer(&PeerMessage::parse(e.content[1..e.content.len()].to_vec()), &e.sender);
                } else {
                    println!("No observer for {}", addr);
                }
            }
        }));
    }
}

impl Server<Peer> for Peer {
    fn new(port: u16) -> Peer {
        Peer {
            uid: Instant::now().elapsed().as_millis().to_string(),
            server: Udp::new(port),
            dispatcher_addr: RefCell::new(None),
            peers: ThreadSafe::new(Vec::new()),
            on_message_received: OptionalClosure::new(None),
            on_peer_connected: OptionalClosure::new(None),
            on_peer_disconnected: OptionalClosure::new(None),
        }
    }

    fn start(&mut self) -> () {
        if !self.alive() {
            self.server.start();
            self.routing();
        }
    }

    fn close(&self) -> () {
        let addr = self.dispatcher_addr.borrow_mut();
        if addr.is_some() {
            self.send(PeerEvent::disconnecting().as_bytes().as_slice(), &addr.unwrap());
        }
        self.server.close();
    }

    fn send(&self, msg: &[u8], addr: &SocketAddr) -> () {
        self.server.send(msg, addr)
    }
}

impl Exchange for Peer {
    fn send_to_all(&self, message: PeerMessage) -> () {
        let shared_peers = self.peers.clone();
        let recipients = shared_peers.lock().unwrap();
        for msg in PeerMessage::split(message, 1024) {
            for other in recipients.iter() {
                if other != &self.addr() {
                    self.server.send(msg.to_event().as_bytes().as_slice(), other)
                }
            }
        }
    }

    fn send_to(&self, message: PeerMessage, addr: &SocketAddr) -> () {
        for msg in PeerMessage::split(message, 1024) {
            if addr != &self.addr() {
                self.server.send(msg.to_event().as_bytes().as_slice(), addr)
            }
        }
    }

    fn connect(&mut self, addr: &SocketAddr) -> () {
        self.start();
        self.dispatcher_addr = RefCell::new(Some(addr.clone()));
        self.server.send(PeerEvent::connecting().as_bytes().as_slice(), addr);
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
    pub fn set_on_message_received<F>(&mut self, on_message_received: F) where F: FnMut(&PeerMessage, &SocketAddr) -> () + Send + Sync + 'static {
        OptionalClosure::set(&self.on_message_received, Box::new(on_message_received));
    }

    pub fn set_on_peer_connected<F>(&mut self, on_peer_connected: F) where F: FnMut(&SocketAddr) -> () + Send + Sync + 'static {
        OptionalClosure::set(&self.on_peer_connected, Box::new(on_peer_connected));
    }

    pub fn set_on_peer_disconnected<F>(&mut self, on_peer_disconnected: F) where F: FnMut(&SocketAddr) -> () + Send + Sync + 'static {
        OptionalClosure::set(&self.on_peer_disconnected, Box::new(on_peer_disconnected));
    }
}

impl Debug for Peer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.uid)
    }
}
