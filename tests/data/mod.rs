use std::time::Duration;

use cucumber::{gherkin::Step, World};
use futures::{executor::block_on, future::LocalBoxFuture, FutureExt};
use log::debug;
use r2d2_sqlite::SqliteConnectionManager;
use rudp2plib::{configuration::Configuration, peer::*};

use crate::{
    dao::{
        add_connection, add_disconnection, add_message, get_connection_for_peer,
        get_disconnection_for_peer, get_message_for_peer, init, ConnectedEvent, DisconnectedEvent,
        MessageEvent, Pool,
    },
    utils::{get_time, read_file, wait_while_condition},
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

impl PeersWorld {
    fn new() -> Self {
        let manager = SqliteConnectionManager::file("target/features.db");
        let pool = Pool::new(manager).expect("Unable to initialize pool");
        block_on(init(&pool));
        Self {
            peers: Vec::new(),
            pool,
        }
    }

    pub(crate) async fn add_all(&mut self, peers: Vec<PeerData>) {
        for peer_data in peers {
            let conf = Configuration::builder()
                .port(peer_data.port)
                .name(&peer_data.name)
                .share_connections(true)
                .build();
            let pool_c = self.pool.clone();
            let pool_d = self.pool.clone();
            let pool_m = self.pool.clone();
            let peer = Peer::new(
                conf,
                move |c| {
                    add_connection(
                        &pool_c,
                        ConnectedEvent {
                            from: c.from,
                            to: c.uid,
                        },
                    )
                    .await;
                    None
                },
                move |d| {
                    add_disconnection(
                        &pool_d,
                        DisconnectedEvent {
                            from: d.from,
                            to: d.uid,
                        },
                    )
                    .await;
                    None
                },
                move |m| {
                    add_message(
                        &pool_m,
                        MessageEvent {
                            from: m.from,
                            to: m.uid,
                            content: m.content,
                        },
                    )
                    .await;
                    None
                },
            )
            .await;
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
    ) -> bool {
        if type_event == "CONNECTED" {
            let connections = get_connection_for_peer(&self.pool, &peer).await;
            debug!(
                "Check {peer} connected with {:?} - {:?}",
                events, connections
            );
            events
                .iter()
                .all(|e| connections.iter().any(|c| c.from == e.from))
        } else if type_event == "DISCONNECTED" {
            let disconnections = get_disconnection_for_peer(&self.pool, &peer).await;
            debug!(
                "Check {peer} disconnected with {:?} - {:?}",
                events, disconnections
            );
            events
                .iter()
                .all(|e| disconnections.iter().any(|d| d.from == e.from))
        } else if type_event == "MESSAGE" {
            let messages = get_message_for_peer(&self.pool, &peer).await;
            debug!(
                "Check {peer} receive messages {:?} - {:?}",
                events, messages
            );
            events.iter().all(|e| {
                messages
                    .iter()
                    .any(|m| m.from == e.from && m.content == e.content)
            })
        } else {
            false
        }
    }

    pub(crate) async fn check_peer_not_receive(
        &self,
        peer: String,
        events: Vec<Event>,
        type_event: String,
    ) -> bool {
        std::thread::sleep(Duration::from_millis(1000));
        if type_event == "CONNECTED" {
            let connections = get_connection_for_peer(&self.pool, &peer).await;
            events
                .iter()
                .all(|e| connections.iter().all(|c| c.from != e.from))
        } else if type_event == "DISCONNECTED" {
            let disconnections = get_disconnection_for_peer(&self.pool, &peer).await;
            events
                .iter()
                .all(|e| disconnections.iter().all(|d| d.from != e.from))
        } else if type_event == "MESSAGE" {
            let messages = get_message_for_peer(&self.pool, &peer).await;
            events.iter().all(|e| {
                messages
                    .iter()
                    .all(|m| m.from != e.from && m.content != e.content)
            })
        } else {
            false
        }
    }
}
