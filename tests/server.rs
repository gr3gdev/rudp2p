use std::collections::HashMap;

use cucumber::{given, then, when, World};

use rudp2plib::server::{Event, Server, ServerStatus, Udp};
use rudp2plib::utils::ThreadSafe;

use crate::common::wait_while_condition;

mod common;

#[derive(cucumber::World, Debug)]
#[world(init = Self::new)]
struct ServersWorld {
    servers: HashMap<String, Udp>,
    on_received: ThreadSafe<Vec<ServerData>>,
}

#[derive(Debug)]
struct ServerData {
    server_name: Vec<u8>,
    event: Event,
}

impl Clone for ServerData {
    fn clone(&self) -> Self {
        ServerData {
            server_name: self.server_name.clone(),
            event: self.event.clone(),
        }
    }
}

impl ServersWorld {
    fn new() -> Self {
        Self {
            servers: HashMap::new(),
            on_received: ThreadSafe::new(Vec::new()),
        }
    }
}

#[given(expr = "a server {string} is started on port {int}")]
async fn start_server(w: &mut ServersWorld, name: String, port: u16) {
    let mut server = Udp::new(port);
    let shared_name = name.clone();
    let shared_received = w.on_received.clone();
    server.set_on_received(move |e, _socket| {
        let mut guard = shared_received.lock().unwrap();
        guard.push(ServerData {
            server_name: shared_name.as_bytes().to_vec(),
            event: e.clone(),
        })
    });
    server.start();
    w.servers.insert(name, server);
}

#[when(expr = "{string} sends {string} to {string}")]
async fn server_send_message(w: &mut ServersWorld, s1: String, message: String, s2: String) {
    let server1 = w.servers.get(&s1).unwrap();
    let server2 = w.servers.get(&s2).unwrap();
    server1.send(message.as_bytes(), &server2.addr());
}

#[then(expr = "{string} receives {string} from {string}")]
async fn server_receive_message(w: &mut ServersWorld, s2: String, message: String, s1: String) {
    let server1 = w.servers.get(&s1).unwrap();
    let addr = server1.addr();
    wait_while_condition("wait message", &|| w.on_received.lock().unwrap().is_empty());
    let data = w.on_received.lock().unwrap().to_vec();
    assert_eq!(1, data.len());
    for d in data.iter() {
        assert_eq!(s2, String::from_utf8(d.server_name.clone()).unwrap());
        assert_eq!(addr, d.event.sender);
        assert_eq!(message, String::from_utf8(d.event.content.clone()).unwrap());
    }
}

fn main() {
    futures::executor::block_on(ServersWorld::run("features/server"));
}
