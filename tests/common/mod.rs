use std::{env, fs, thread};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::{Duration, SystemTime};

use cucumber::World;
use futures::future::LocalBoxFuture;
use futures::FutureExt;

use rudp2plib::peer::{Exchange, Peer};
use rudp2plib::peer::message::PeerMessage;
use rudp2plib::server::ServerStatus;
use rudp2plib::utils::{read_file, ThreadSafe};

#[derive(World, Debug)]
#[world(init = Self::new)]
pub(crate) struct PeersWorld {
    servers: Vec<PeerData>,
}

#[derive(Debug)]
pub(crate) struct PeerData {
    pub(crate) peer: Peer,
    addr: SocketAddr,
    on_message_received: ThreadSafe<Vec<PeerMessageData>>,
    on_peer_connected: ThreadSafe<Vec<String>>,
    on_peer_disconnected: ThreadSafe<Vec<String>>,
}

#[derive(Debug)]
pub(crate) struct PeerMessageData {
    message: PeerMessage,
    sender: String,
}

#[derive(Debug)]
pub(crate) struct SimplePeerData {
    name: String,
    addr: SocketAddr,
}

pub(crate) fn log(message: String) {
    println!("\x1b[44m\x1b[36m[TEST]\x1b[0m {}", message);
}

pub(crate) fn wait_while_condition(condition: &dyn Fn() -> bool) {
    let start = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap().as_millis();
    while condition() {
        thread::sleep(Duration::from_millis(1000));
        let current = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap().as_millis();
        if current - start > 5000 {
            panic!("Timeout !");
        }
    }
}

impl PeerData {
    fn new(name: String, port: u16, connect: Option<SimplePeerData>, authorized_peers: Vec<String>) -> PeerData {
        let mut peer = Peer::new(port, Some(name));
        peer.add_friendly_peers(authorized_peers);
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
            log(format!("{} connected with {}", peer_uid, u));
            shared_on_peer_connected.lock().unwrap().push(u.clone());
        });
        let on_peer_disconnected = ThreadSafe::new(Vec::new());
        let shared_on_peer_disconnected = on_peer_disconnected.clone();
        peer.set_on_peer_disconnected(move |u| {
            shared_on_peer_disconnected.lock().unwrap().push(u.clone());
        });
        if let Some(dispatcher) = connect {
            log(format!("Connect {} with {}", peer.uid, dispatcher.name));
            peer.connect(&dispatcher.addr);
        } else {
            log(format!("Start {}", peer.uid));
            peer.open();
        }
        PeerData {
            peer,
            addr,
            on_message_received,
            on_peer_connected,
            on_peer_disconnected,
        }
    }

    pub(crate) fn get_message(data: &String) -> PeerMessage {
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

    pub(crate) fn send_to_all(&self, data: String) {
        let message = Self::get_message(&data);
        log(format!("{} sends [{}] to all", self.addr, data));
        self.peer.send_to_all(message);
    }

    pub(crate) fn send_to(&self, data: String, other: &String) {
        let message = Self::get_message(&data);
        log(format!("{} sends [{}] to {}", self.addr, data, other));
        self.peer.send_to(message, &other);
    }

    pub(crate) fn is_connected_with(&self, uid: &String) {
        let peer_uid = self.peer.uid.clone();
        wait_while_condition(&|| {
            let guard = self.on_peer_connected.lock().unwrap();
            log(format!("Waiting {} is connected with {} - {:?}", peer_uid, uid, guard));
            !guard.contains(uid)
        });
        let on_peer_connected = self.on_peer_connected.lock().unwrap();
        assert!(on_peer_connected.contains(uid));
    }

    pub(crate) fn is_not_connected_with(&self, uid: &String) {
        let peer_uid = self.peer.uid.clone();
        wait_while_condition(&|| {
            let guard = self.on_peer_connected.lock().unwrap();
            log(format!("Waiting {} is not connected with {} - {:?}", peer_uid, uid, guard));
            guard.contains(uid)
        });
        let on_peer_connected = self.on_peer_connected.lock().unwrap();
        assert!(!on_peer_connected.contains(uid));
    }

    pub(crate) fn is_disconnected_with(&self, uid: &String) {
        let peer_uid = self.peer.uid.clone();
        wait_while_condition(&|| {
            let guard = self.on_peer_disconnected.lock().unwrap();
            log(format!("Waiting {} is disconnected with {} - {:?}", peer_uid, uid, guard));
            !guard.contains(uid)
        });
        let on_peer_disconnected = self.on_peer_disconnected.lock().unwrap();
        assert!(on_peer_disconnected.contains(uid));
    }

    pub(crate) fn is_not_disconnected_with(&self, uid: &String) {
        let peer_uid = self.peer.uid.clone();
        wait_while_condition(&|| {
            let guard = self.on_peer_disconnected.lock().unwrap();
            log(format!("Waiting {} is not disconnected with {} - {:?}", peer_uid, uid, guard));
            guard.contains(uid)
        });
        let on_peer_disconnected = self.on_peer_disconnected.lock().unwrap();
        assert!(!on_peer_disconnected.contains(uid));
    }

    pub(crate) fn is_not_message_received(&self, uid: &String) {
        let peer_uid = self.peer.uid.clone();
        wait_while_condition(&|| {
            let guard = self.on_message_received.lock().unwrap();
            log(format!("Waiting {} is not received message from {} - {:?}", peer_uid, uid, guard));
            guard.iter().find(|d| d.sender.eq(uid)).is_some()
        });
        let on_message_received = self.on_message_received.lock().unwrap();
        let mut senders = Vec::new();
        for m in on_message_received.iter() {
            let sender = m.sender.clone();
            senders.push(sender);
        }
        assert!(!senders.contains(uid));
    }

    pub(crate) fn is_message_received(&self, event: String, uid: &String) {
        let peer_uid = self.peer.uid.clone();
        wait_while_condition(&|| {
            let guard = self.on_message_received.lock().unwrap();
            log(format!("Waiting {} is received message from {} - {:?}", peer_uid, uid, guard));
            guard.iter().find(|d| d.sender.eq(uid)).is_none()
        });
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
        for message in messages {
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
}

impl PeersWorld {
    fn new() -> Self {
        Self {
            servers: Vec::new(),
        }
    }

    pub(crate) fn find(&self, name: &String) -> &PeerData {
        self.servers.iter().find(|p| p.peer.uid.eq(name)).expect("Peer not found")
    }

    pub(crate) fn remove_peer(&mut self, name: String) {
        let peer = self.find(&name);
        peer.peer.close();
    }

    pub(crate) fn add_peer(&mut self, name: String, port: u16, authorized_peers: Vec<String>, connect: Option<String>) {
        let addr;
        if let Some(dispatcher_name) = connect {
            let peer_data = self.find(&dispatcher_name);
            addr = Some(SimplePeerData {
                name: dispatcher_name,
                addr: peer_data.addr,
            });
        } else {
            addr = None;
        }
        self.servers.push(PeerData::new(name.clone(), port, addr, authorized_peers));
    }

    pub(crate) fn close(&self) -> LocalBoxFuture<()> {
        async {
            for peer_data in self.servers.iter() {
                let peer = &peer_data.peer;
                peer.close();
                wait_while_condition(&|| {
                    log(format!("Waiting while peer is alive : {}", peer.alive()));
                    peer.alive()
                });
            }
        }.boxed_local()
    }
}
