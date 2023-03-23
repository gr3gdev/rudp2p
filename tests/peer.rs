use std::{env, fs};
use std::collections::HashMap;
use std::net::SocketAddr;

use cucumber::{given, then, when, World};

use rudp2plib::peer::{Exchange, Peer};
use rudp2plib::peer::message::PeerMessage;
use rudp2plib::server::{Server, ServerStatus};
use rudp2plib::utils::{read_file, ThreadSafe};

use crate::common::wait_while_condition;

mod common;

#[derive(World, Debug)]
#[world(init = Self::new)]
struct PeersWorld {
    servers: HashMap<String, PeerData>,
}

#[derive(Debug)]
struct PeerData {
    peer: Peer,
    addr: SocketAddr,
    on_message_received: ThreadSafe<Vec<PeerMessageData>>,
    on_peer_connected: ThreadSafe<Vec<SocketAddr>>,
    on_peer_disconnected: ThreadSafe<Vec<SocketAddr>>,
}

#[derive(Debug)]
struct PeerMessageData {
    message: PeerMessage,
    sender: SocketAddr,
}

impl PeerData {
    fn new(port: u16, connect: Option<SocketAddr>) -> PeerData {
        let mut peer = Peer::new(port);
        let addr = peer.addr();
        let on_message_received = ThreadSafe::new(Vec::new());
        let shared_on_message_received = on_message_received.clone();
        peer.set_on_message_received(move |m, s| {
            println!("{} received message from {}", addr, s);
            shared_on_message_received.lock().unwrap().push(PeerMessageData {
                message: m.clone(),
                sender: s.clone(),
            });
        });
        let on_peer_connected = ThreadSafe::new(Vec::new());
        let shared_on_peer_connected = on_peer_connected.clone();
        peer.set_on_peer_connected(move |s| {
            println!("{} connected with {}", addr, s);
            shared_on_peer_connected.lock().unwrap().push(s.clone());
        });
        let on_peer_disconnected = ThreadSafe::new(Vec::new());
        let shared_on_peer_disconnected = on_peer_disconnected.clone();
        peer.set_on_peer_disconnected(move |s| {
            println!("{} disconnected with {}", addr, s);
            shared_on_peer_disconnected.lock().unwrap().push(s.clone());
        });
        if let Some(dispatcher) = connect {
            println!("Connect {} with {}", addr, dispatcher);
            peer.connect(&dispatcher);
        } else {
            peer.start();
        }
        PeerData {
            peer,
            addr,
            on_message_received,
            on_peer_connected,
            on_peer_disconnected,
        }
    }

    fn get_message(data: &String) -> PeerMessage {
        let message;
        if data.contains("file:") {
            let current_dir = env::current_dir().unwrap();
            let mut path = current_dir.display().to_string();
            path.push_str(data[5..].trim());
            message = PeerMessage::from_file(path.as_str());
        } else {
            message = PeerMessage::from_text(data.as_str());
        };
        message
    }

    fn send_to_all(&self, data: String) {
        let message = Self::get_message(&data);
        println!("{} sends [{}] to all", self.addr, data);
        self.peer.send_to_all(message);
    }

    fn send_to(&self, data: String, other: &PeerData) {
        let message = Self::get_message(&data);
        let addr = other.addr;
        println!("{} sends [{}] to {}", self.addr, data, addr);
        self.peer.send_to(message, &addr);
    }

    fn is_connected_with(&self, addr: SocketAddr) {
        wait_while_condition("wait Connected", &||
            !self.on_peer_connected.lock().unwrap().contains(&addr));
        let on_peer_connected = self.on_peer_connected.lock().unwrap();
        assert!(on_peer_connected.contains(&addr));
    }

    fn is_disconnected_with(&self, addr: SocketAddr) {
        wait_while_condition("wait Disconnected", &||
            !self.on_peer_disconnected.lock().unwrap().contains(&addr));
        let on_peer_disconnected = self.on_peer_disconnected.lock().unwrap();
        assert!(on_peer_disconnected.contains(&addr));
    }

    fn is_message_received(&self, event: String, addr: SocketAddr) {
        wait_while_condition("wait Message received", &|| self.on_message_received.lock().unwrap().is_empty());
        let on_message_received = self.on_message_received.lock().unwrap();
        let mut messages_by_sender: HashMap<SocketAddr, Vec<PeerMessage>> = HashMap::new();
        for m in on_message_received.iter() {
            let sender = m.sender.clone();
            let message = m.message.clone();
            if let Some(messages) = messages_by_sender.get(&sender) {
                let mut messages = messages.clone();
                messages.push(message);
                messages_by_sender.insert(sender, messages);
            } else {
                messages_by_sender.insert(sender, vec![message]);
            }
        }
        let messages = messages_by_sender.get(&addr).expect("Messages not found");
        let message = PeerMessage::concat(&messages);
        if event.contains("file:") {
            let current_dir = env::current_dir().unwrap();
            let mut path = current_dir.display().to_string();
            path.push_str(event[5..].trim());
            let mut out = current_dir.display().to_string();
            out.push_str("/target/out.txt");
            message.to_file(out.as_str());
            let expected_data = read_file(path.as_str());
            let actual_data = read_file(out.as_str());
            fs::remove_file(out).expect("Unable to remove out.txt");
            assert_eq!(expected_data, actual_data);
        } else {
            assert_eq!(event, message.to_string());
        }
    }
}

impl PeersWorld {
    fn new() -> Self {
        Self {
            servers: HashMap::new(),
        }
    }

    fn remove_peer(&mut self, name: String) {
        let peer = self.servers.get(&name).expect("Peer not found");
        peer.peer.close();
    }

    fn add_peer(&mut self, name: String, port: u16, connect: Option<String>) {
        let addr;
        if let Some(dispatcher_name) = connect {
            let dispatcher = self.servers.get(&dispatcher_name).expect("Dispatcher not found");
            addr = Some(dispatcher.addr);
        } else {
            addr = None;
        }
        self.servers.insert(name, PeerData::new(port, addr));
    }
}

#[given(expr = "a peer {string} is started on port {int}")]
async fn start_dispatcher(w: &mut PeersWorld, dispatcher_name: String, port: u16) {
    w.add_peer(dispatcher_name, port, None);
}

#[when(expr = "a peer {string} connects to {string} with the port {int}")]
async fn connect_peer(w: &mut PeersWorld, peer_name: String, dispatcher_name: String, port: u16) {
    w.add_peer(peer_name, port, Some(dispatcher_name));
}

#[when(expr = "a peer {string} disconnects")]
async fn disconnect_peer(w: &mut PeersWorld, peer_name: String) {
    w.remove_peer(peer_name);
}

#[when(expr = "the peer {string} sends {string} to all")]
async fn peer_sends_to_all(w: &mut PeersWorld, peer_name: String, data: String) {
    let peer_data = w.servers.get(&peer_name).expect("Peer not found");
    peer_data.send_to_all(data);
}

#[when(expr = "the peer {string} sends {string} to {string}")]
async fn peer_sends_to_peer(w: &mut PeersWorld, peer_name: String, data: String, other_peer: String) {
    let peer_data = w.servers.get(&peer_name).expect("Peer not found");
    let other_peer_data = w.servers.get(&other_peer).expect("Peer not found");
    peer_data.send_to(data, other_peer_data);
}

#[then(expr = "the peer {string} receives {string} from {string}")]
async fn receive_event(w: &mut PeersWorld, peer_name: String, event: String, sender_name: String) {
    let peer_data = w.servers.get(peer_name.as_str()).expect("Peer not found");
    let sender = w.servers.get(sender_name.as_str()).expect("Peer not found");
    let sender_addr = sender.peer.addr();
    if event == "Connected" {
        peer_data.is_connected_with(sender_addr);
    } else if event == "Disconnected" {
        peer_data.is_disconnected_with(sender_addr);
    } else {
        peer_data.is_message_received(event, sender_addr);
    }
}

fn main() {
    futures::executor::block_on(PeersWorld::run("features/peer"));
}
