use std::time::Duration;
use async_trait::async_trait;
use cucumber::{gherkin::Step, World};
use futures::{future::LocalBoxFuture, FutureExt};
use log::debug;
use r2d2_sqlite::SqliteConnectionManager;
use rudp2plib::{
    configuration::Configuration,
    network::{events::*, Response},
    observer::Observer,
    peer::*,
};

use crate::{
    dao::{
        add_connection, add_disconnection, add_message, get_peer_messages_from, init,
        is_peer_connected_with, is_peer_disconnected_with, ConnectedEvent, DisconnectedEvent,
        MessageEvent, Pool,
    },
    utils::{get_time, read_file, wait_until},
};

#[derive(World, Debug)]
#[world(init = Self::new)]
pub(crate) struct PeersWorld {
    peers: Vec<Peer>,
    pool: Pool,
}

#[derive(Debug)]
pub(crate) struct PeerData {
    pub(crate) name: String,
    pub(crate) port: u16,
}

#[derive(Debug, Clone)]
pub(crate) struct Event {
    pub(crate) event: String,
    content: Vec<u8>,
    from: String,
}

pub(crate) trait DataTable {
    fn read_from_datatable(step: &Step) -> Vec<Self>
    where
        Self: Sized;
}

impl DataTable for PeerData {
    fn read_from_datatable(step: &Step) -> Vec<Self>
    where
        Self: Sized,
    {
        let mut data = Vec::new();
        if let Some(table) = step.table.as_ref() {
            for row in table.rows.iter().skip(1) {
                // NOTE: skip header
                let name = row[0].clone();
                let port = row[1].parse::<u16>().expect("Unable to read port number");
                data.push(PeerData { name, port });
            }
        }
        data
    }
}

impl DataTable for Event {
    fn read_from_datatable(step: &Step) -> Vec<Self>
    where
        Self: Sized,
    {
        let mut data = Vec::new();
        if let Some(table) = step.table.as_ref() {
            for row in table.rows.iter().skip(1) {
                // NOTE: skip header
                let event = row[0].clone();
                let content;
                let from;
                if row.len() == 3 {
                    let data = row[1].clone();
                    if data.starts_with("file:") {
                        content = read_file(&data[5..].trim());
                    } else {
                        content = data.as_bytes().to_vec();
                    }
                    from = row[2].clone();
                } else {
                    content = vec![];
                    from = row[1].clone();
                }
                data.push(Event {
                    event,
                    content,
                    from,
                });
            }
        }
        data
    }
}

struct TestObserver {
    pool: Pool,
}

#[async_trait]
impl Observer for TestObserver {
    async fn on_connected(&mut self, c: Connected) -> Option<Response> {
        add_connection(
            &self.pool,
            ConnectedEvent {
                from: c.from,
                to: c.uid,
            },
        )
        .await;
        None
    }

    async fn on_disconnected(&mut self, d: Disconnected) -> Option<Response> {
        add_disconnection(
            &self.pool,
            DisconnectedEvent {
                from: d.from,
                to: d.uid,
            },
        )
        .await;
        None
    }

    async fn on_message(&mut self, m: Message) -> Option<Response> {
        add_message(
            &self.pool,
            MessageEvent {
                from: m.from,
                to: m.uid,
                content: m.content,
            },
        )
        .await;
        None
    }
}

impl PeersWorld {
    async fn new() -> Self {
        let manager = SqliteConnectionManager::memory();
        let pool = Pool::new(manager).expect("Unable to initialize pool");
        init(&pool).await;
        Self {
            peers: Vec::new(),
            pool,
        }
    }

    pub(crate) async fn add_all(&mut self, peers: Vec<PeerData>) {
        for peer_data in peers {
            debug!("\x1b[33m[TEST]\x1b[0m Add peer {:?}", peer_data);
            let conf = Configuration::builder()
                .port(peer_data.port)
                .name(&peer_data.name)
                .share_connections(true)
                .build();
            let test_observer = TestObserver {
                pool: self.pool.clone(),
            };
            let peer = Peer::new(conf, test_observer).await;
            self.peers.push(peer);
        }
    }

    pub(crate) fn get_peer(&self, name: String) -> &Peer {
        self.peers
            .iter()
            .find(|p| p.uid == name)
            .expect("Peer not found")
    }

    pub(crate) fn close(&self) -> LocalBoxFuture<()> {
        async {
            for peer in self.peers.iter() {
                peer.close();
                // Wait while the server is alive
                let start = get_time();
                while peer.is_alive().await {
                    std::thread::sleep(Duration::from_millis(1000));
                    if get_time() - start > 10000 {
                        panic!("Peer is not stopped !");
                    }
                }
            }
        }
        .boxed_local()
    }

    pub(crate) async fn check_peer_receive(
        &self,
        peer: String,
        events: Vec<Event>,
        type_event: String,
    ) -> () {
        if type_event == "CONNECTED" {
            for e in events {
                assert!(
                    wait_until(&|| is_peer_connected_with(&self.pool, &peer, &e.from), 5000).await,
                    "Peer {peer} is not connected with {}",
                    e.from
                );
            }
        } else if type_event == "DISCONNECTED" {
            for e in events {
                assert!(
                    wait_until(
                        &|| is_peer_disconnected_with(&self.pool, &peer, &e.from),
                        5000
                    )
                    .await,
                    "Peer {peer} is not disconnected with {}",
                    e.from
                );
            }
        } else if type_event == "MESSAGE" {
            for e in events {
                assert!(
                    wait_until(
                        &|| async {
                            let messages = get_peer_messages_from(&self.pool, &peer, &e.from).await;
                            messages.iter().any(|m| m.content == e.content)
                        },
                        10000
                    )
                    .await,
                    "Peer {peer} has not received the message from {}",
                    e.from
                );
            }
        }
    }

    pub(crate) async fn check_peer_not_receive(
        &self,
        peer: String,
        events: Vec<Event>,
        type_event: String,
    ) -> () {
        std::thread::sleep(Duration::from_millis(1000));
        if type_event == "CONNECTED" {
            for e in events {
                assert!(
                    wait_until(
                        &|| async {
                            is_peer_connected_with(&self.pool, &peer, &e.from).await == false
                        },
                        5000
                    )
                    .await,
                    "Peer {peer} is connected with {}",
                    e.from
                );
            }
        } else if type_event == "DISCONNECTED" {
            for e in events {
                assert!(
                    wait_until(
                        &|| async {
                            is_peer_disconnected_with(&self.pool, &peer, &e.from).await == false
                        },
                        5000
                    )
                    .await,
                    "Peer {peer} is disconnected with {}",
                    e.from
                );
            }
        } else if type_event == "MESSAGE" {
            for e in events {
                assert!(
                    wait_until(
                        &|| async {
                            let messages = get_peer_messages_from(&self.pool, &peer, &e.from).await;
                            messages.is_empty() || messages.iter().all(|m| m.content != e.content)
                        },
                        10000
                    )
                    .await,
                    "Peer {peer} has received the message from {}",
                    e.from
                );
            }
        }
    }
}
