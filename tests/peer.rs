use std::{env, fs};
use std::collections::HashMap;
use std::net::SocketAddr;

use cucumber::{gherkin::Step, given, then, when, World};
use futures::future::LocalBoxFuture;
use futures::FutureExt;

use rudp2plib::peer::{Exchange, Peer};
use rudp2plib::peer::message::PeerMessage;
use rudp2plib::server::{Server, ServerStatus};
use rudp2plib::utils::{read_file, ThreadSafe};

use crate::common::wait_while_condition;

mod common;

#[derive(World, Debug)]
#[world(init = Self::new)]
struct PeersWorld {
    servers: Vec<PeerData>,
}

#[derive(Debug)]
struct PeerData {
    peer: Peer,
    addr: SocketAddr,
    on_message_received: ThreadSafe<Vec<PeerMessageData>>,
    on_peer_connected: ThreadSafe<Vec<String>>,
    on_peer_disconnected: ThreadSafe<Vec<String>>,
}

#[derive(Debug)]
struct PeerMessageData {
    message: PeerMessage,
    sender: String,
}

impl PeerData {
    fn new(name: String, port: u16, connect: Option<SocketAddr>, authorized_peers: Vec<String>) -> PeerData {
        let mut peer = Peer::new(port, Some(name));
        let peer_uid = peer.uid.clone();
        let addr = peer.addr();
        let on_message_received = ThreadSafe::new(Vec::new());
        let shared_on_message_received = on_message_received.clone();
        peer.set_on_message_received(move |m, u| {
            shared_on_message_received.lock().unwrap().push(PeerMessageData {
                message: m.clone(),
                sender: u.clone(),
            });
        });
        let on_peer_connected = ThreadSafe::new(Vec::new());
        let shared_on_peer_connected = on_peer_connected.clone();
        peer.set_on_peer_connected(move |u| {
            println!("{} connected with {}", peer_uid, u);
            shared_on_peer_connected.lock().unwrap().push(u.clone());
        });
        let on_peer_disconnected = ThreadSafe::new(Vec::new());
        let shared_on_peer_disconnected = on_peer_disconnected.clone();
        peer.set_on_peer_disconnected(move |u| {
            shared_on_peer_disconnected.lock().unwrap().push(u.clone());
        });
        if let Some(dispatcher) = connect {
            println!("Connect {} with {}", peer.uid, dispatcher);
            peer.connect(&dispatcher, authorized_peers);
        } else {
            println!("Start {}", peer.uid);
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

    fn is_connected_with(&self, uid: &String) {
        wait_while_condition("wait Connection", &||
                !self.on_peer_connected.lock().unwrap().contains(uid));
        let on_peer_connected = self.on_peer_connected.lock().unwrap();
        assert!(on_peer_connected.contains(uid));
    }

    fn is_not_connected_with(&self, uid: &String) {
        wait_while_condition("wait Connection", &||
            self.on_peer_connected.lock().unwrap().is_empty());
        let on_peer_connected = self.on_peer_connected.lock().unwrap();
        assert!(!on_peer_connected.contains(uid));
    }

    fn is_disconnected_with(&self, uid: &String) {
        wait_while_condition("wait Disconnection", &||
            !self.on_peer_disconnected.lock().unwrap().contains(uid));
        let on_peer_disconnected = self.on_peer_disconnected.lock().unwrap();
        assert!(on_peer_disconnected.contains(uid));
    }

    fn is_not_disconnected_with(&self, uid: &String) {
        wait_while_condition("wait Disconnection", &||
            self.on_peer_disconnected.lock().unwrap().is_empty());
        let on_peer_disconnected = self.on_peer_disconnected.lock().unwrap();
        assert!(!on_peer_disconnected.contains(uid));
    }

    fn is_not_message_received(&self, uid: &String) {
        wait_while_condition("wait Message reception", &|| self.on_message_received.lock().unwrap().is_empty());
        let on_message_received = self.on_message_received.lock().unwrap();
        let mut senders = Vec::new();
        for m in on_message_received.iter() {
            let sender = m.sender.clone();
            senders.push(sender);
        }
        assert!(!senders.contains(uid));
    }

    fn is_message_received(&self, event: String, uid: &String) {
        wait_while_condition("wait Message reception", &|| self.on_message_received.lock().unwrap().is_empty());
        let on_message_received = self.on_message_received.lock().unwrap();
        let mut messages_by_sender: HashMap<String, Vec<PeerMessage>> = HashMap::new();
        for m in on_message_received.iter() {
            let sender = m.sender.clone();
            let message = m.message.clone();
            if let Some(messages) = messages_by_sender.get(uid) {
                let mut messages = messages.clone();
                messages.push(message);
                messages_by_sender.insert(sender, messages);
            } else {
                messages_by_sender.insert(sender, vec![message]);
            }
        }
        let messages = messages_by_sender.get(uid).expect("Messages not found");
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
            servers: Vec::new(),
        }
    }

    fn find(&self, name: &String) -> &PeerData {
        self.servers.iter().find(|p| p.peer.uid.eq(name)).expect("Peer not found")
    }

    fn remove_peer(&mut self, name: String) {
        let peer = self.find(&name);
        peer.peer.close();
    }

    fn add_peer(&mut self, name: String, port: u16, connect: Option<String>, authorized_peers: Vec<String>) {
        let addr;
        if let Some(dispatcher_name) = connect {
            let peer_data = self.find(&dispatcher_name);
            addr = Some(peer_data.addr);
        } else {
            addr = None;
        }
        self.servers.push(PeerData::new(name.clone(), port, addr, authorized_peers));
    }

    fn close(&self) -> LocalBoxFuture<()> {
        async {
            for peer_data in self.servers.iter() {
                let peer = &peer_data.peer;
                peer.close();
                wait_while_condition("wait peer close", &|| peer.alive());
            }
        }.boxed_local()
    }
}

#[given(expr = "the following peers are started")]
async fn start_peers(w: &mut PeersWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) { // NOTE: skip header
            let name = &row[0];
            let port = row[1].parse::<u16>().unwrap();
            w.add_peer(name.clone(), port, None, Vec::new());
        }
    }
}

#[when(expr = "the following peers connect to {string}")]
async fn connect_peer(w: &mut PeersWorld, dispatcher_name: String, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) { // NOTE: skip header
            let peer_name = &row[0];
            let port = row[1].parse::<u16>().unwrap();
            let mut authorized_peers = Vec::new();
            if row.len() == 3 {
                let peers = &row[2].split(",").collect::<Vec<&str>>();
                for peer in peers {
                    authorized_peers.push(peer.to_string());
                }
                println!("{} {:?}", peer_name, authorized_peers);
            }
            w.add_peer(peer_name.clone(), port, Some(dispatcher_name.clone()), authorized_peers);
        }
    }
}

#[when(expr = "the peer {string} disconnects")]
async fn disconnect_peer(w: &mut PeersWorld, peer_name: String) {
    w.remove_peer(peer_name);
}

#[when(expr = "the peer {string} sends {string} to all")]
async fn peer_sends_to_all(w: &mut PeersWorld, peer_name: String, data: String) {
    let peer_data = w.find(&peer_name);
    peer_data.send_to_all(data);
}

#[when(expr = "the peer {string} sends {string} to {string}")]
async fn peer_sends_to_peer(w: &mut PeersWorld, peer_name: String, data: String, other_peer: String) {
    let peer_data = w.find(&peer_name);
    let other_peer_data = w.servers.iter().find(|p| p.peer.uid == other_peer).expect("Peer not found");
    peer_data.send_to(data, other_peer_data);
}

#[then(expr = "the peer {string} does not receives")]
async fn not_receive_event(w: &mut PeersWorld, peer_name: String, step: &Step) {
    let peer_data = w.find(&peer_name);
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) { // NOTE: skip header
            let event = &row[0];
            let sender_name = &row[1];
            if event == "Connected" {
                peer_data.is_not_connected_with(sender_name);
            } else if event == "Disconnected" {
                peer_data.is_not_disconnected_with(sender_name);
            } else {
                peer_data.is_not_message_received(sender_name);
            }
        }
    }
}

#[then(expr = "the peer {string} receives")]
async fn receive_event(w: &mut PeersWorld, peer_name: String, step: &Step) {
    let peer_data = w.find(&peer_name);
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) { // NOTE: skip header
            let event = &row[0];
            let sender_name = &row[1];
            if event == "Connected" {
                peer_data.is_connected_with(sender_name);
            } else if event == "Disconnected" {
                peer_data.is_disconnected_with(sender_name);
            } else {
                peer_data.is_message_received(event.clone(), sender_name);
            }
        }
    }
}

fn main() {
    futures::executor::block_on(PeersWorld::cucumber()
        .after(|_feature, _rule, _scenario, _ev, world| {
            world.unwrap().close()
        })
        .run_and_exit("features/peer"));
}
