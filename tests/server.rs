use std::collections::HashMap;

use cucumber::{gherkin::Step, given, then, when, World};
use futures::future::LocalBoxFuture;
use futures::FutureExt;

use rudp2plib::server::{Event, Message, Server, ServerStatus, Udp};
use rudp2plib::utils::ThreadSafe;

use crate::common::wait_while_condition;

mod common;

struct ServerMessage {
    message: String,
}

impl Message for ServerMessage {
    fn content(&self) -> Vec<u8> {
        self.message.as_bytes().to_vec()
    }
}

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
    fn close(&self) -> LocalBoxFuture<()> {
        async {
            for udp in self.servers.values() {
                udp.close();
                wait_while_condition("wait server close", &|| udp.alive());
            }
        }.boxed_local()
    }
}

#[given(expr = "the following servers are started")]
async fn start_server(w: &mut ServersWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) { // NOTE: skip header
            let name = &row[0];
            let port = row[1].parse::<u16>().unwrap();
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
            w.servers.insert(name.clone(), server);
        }
    }
}

#[when(expr = "the server {string} sends {string} to the server {string}")]
async fn server_send_message(w: &mut ServersWorld, s1: String, message: String, s2: String) {
    let server1 = w.servers.get(&s1).unwrap();
    let server2 = w.servers.get(&s2).unwrap();
    server1.send(ServerMessage {
        message,
    }, &server2.addr());
}

#[then(expr = "the server {string} receives {string} from the server {string}")]
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
    futures::executor::block_on(ServersWorld::cucumber()
        .after(|_feature, _rule, _scenario, _ev, world| {
            world.unwrap().close()
        })
        .run_and_exit("features/server"));
}
