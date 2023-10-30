use async_trait::async_trait;
use cucumber::{gherkin::Step, World};
use futures::{future::LocalBoxFuture, FutureExt};
use log::debug;
use rudp2plib::{
    dao::InMemoryDao,
    network::{events::*, Response},
    observer::Observer,
    peer::*,
};
use serialize_bits::ser::SerializerData;
use std::{collections::HashMap, time::Duration};

use crate::{
    dao::{
        add_connection, add_disconnection, add_message, get_peer_messages_from, init,
        is_peer_connected_with, is_peer_disconnected_with, ConnectedEvent, DisconnectedEvent,
        MessageEvent,
    },
    utils::{get_time, read_file, wait_until},
};

const TIMEOUT_CONNECTION: u128 = 3000;
const TIMEOUT_MESSAGE: u128 = 5000;

#[derive(World, Debug)]
#[world(init = Self::new)]
pub(crate) struct PeersWorld {
    peers: HashMap<String, Peer>,
    pool: crate::sqlite::Pool,
}

#[derive(Debug)]
pub(crate) struct PeerData {
    pub(crate) name: String,
    pub(crate) port: u16,
    pub(crate) database: String,
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
                let database = row[2].clone();
                data.push(PeerData {
                    name,
                    port,
                    database,
                });
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
                        content = read_file(&data[5..].trim()).to_data();
                    } else {
                        content = data.to_data();
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
    name: String,
    pool: crate::sqlite::Pool,
}

#[async_trait]
impl Observer for TestObserver {
    async fn on_connected(&mut self, remote: &RemotePeer) -> Option<Response> {
        let event = ConnectedEvent {
            from: remote.addr,
            to: self.name.clone(),
        };
        add_connection(&self.pool, event).await;
        None
    }

    async fn on_disconnected(&mut self, remote: &RemotePeer) -> Option<Response> {
        let event = DisconnectedEvent {
            from: remote.addr,
            to: self.name.clone(),
        };
        add_disconnection(&self.pool, event).await;
        None
    }

    async fn on_message(&mut self, m: &Message) -> Option<Response> {
        let event = MessageEvent {
            from: m.from.addr,
            to: self.name.clone(),
            content: m.content.clone(),
        };
        add_message(&self.pool, event).await;
        None
    }
}

impl PeersWorld {
    async fn new() -> Self {
        let manager = r2d2_sqlite::SqliteConnectionManager::memory();
        let pool = crate::sqlite::Pool::new(manager).expect("Unable to initialize pool");
        init(&pool).await;
        Self {
            peers: HashMap::new(),
            pool,
        }
    }

    pub(crate) async fn add_all(&mut self, peers: Vec<PeerData>) {
        for peer_data in peers {
            debug!("[TEST] Add peer {:?}", peer_data);
            let conf = super::configure(peer_data.port);
            let test_observer = TestObserver {
                name: peer_data.name.clone(),
                pool: self.pool.clone(),
            };
            let peer = if peer_data.database == "Sqlite" {
                let manager = r2d2_sqlite::SqliteConnectionManager::memory().with_init(|c| {
                    c.execute_batch(
                        "PRAGMA journal_mode=wal2; PRAGMA synchronous=NORMAL; PRAGMA foreign_keys=1;",
                    )
                });
                let peer_pool = crate::sqlite::Pool::builder()
                    .max_size(16)
                    .build(manager)
                    .expect("Unable to initialize pool");
                Peer::new(
                    conf,
                    crate::sqlite::SqlitePeerDao::new(&peer_pool),
                    test_observer,
                )
                .await
            } else if peer_data.database.contains("mysql:") {
                let opts = mysql::Opts::from_url(&peer_data.database)
                    .or_else(|e| {
                        log::error!("{e}");
                        Err(e)
                    })
                    .unwrap();
                let params = mysql::OptsBuilder::from_opts(opts);
                let manager = r2d2_mysql::MySqlConnectionManager::new(params);
                let peer_pool = crate::mysql::Pool::builder()
                    .max_size(16)
                    .build(manager)
                    .or_else(|e| {
                        log::error!("{e}");
                        Err(e)
                    })
                    .unwrap();
                Peer::new(
                    conf,
                    crate::mysql::MysqlPeerDao::new(&peer_pool),
                    test_observer,
                )
                .await
            } else {
                Peer::new(conf, InMemoryDao::default(), test_observer).await
            };
            self.peers.insert(peer_data.name, peer);
        }
    }

    pub(crate) fn get_peer(&self, name: &String) -> &Peer {
        self.peers.get(name).expect("Peer not found")
    }

    pub(crate) fn close(&self) -> LocalBoxFuture<()> {
        async {
            for (_, peer) in self.peers.clone() {
                peer.close();
                // Wait while the server is alive
                let start = get_time();
                while peer.is_alive().await {
                    std::thread::sleep(Duration::from_millis(100));
                    if get_time() - start > 30000 {
                        panic!("Peer is not stopped after 30 seconds !");
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
                let other = self.get_peer(&e.from).addr();
                assert!(
                    wait_until(
                        &|| is_peer_connected_with(&self.pool, &peer, &other),
                        TIMEOUT_CONNECTION
                    )
                    .await,
                    "Peer {peer} is not connected with {}",
                    e.from
                );
            }
        } else if type_event == "DISCONNECTED" {
            for e in events {
                let other = self.get_peer(&e.from).addr();
                assert!(
                    wait_until(
                        &|| is_peer_disconnected_with(&self.pool, &peer, &other),
                        TIMEOUT_CONNECTION
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
                            let messages = get_peer_messages_from(
                                &self.pool,
                                &peer,
                                &self.get_peer(&e.from).addr(),
                            )
                            .await;
                            messages.iter().any(|m| m.content == e.content)
                        },
                        TIMEOUT_MESSAGE
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
                            is_peer_connected_with(
                                &self.pool,
                                &peer,
                                &self.get_peer(&e.from).addr(),
                            )
                            .await
                                == false
                        },
                        TIMEOUT_CONNECTION
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
                            is_peer_disconnected_with(
                                &self.pool,
                                &peer,
                                &self.get_peer(&e.from).addr(),
                            )
                            .await
                                == false
                        },
                        TIMEOUT_CONNECTION
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
                            let messages = get_peer_messages_from(
                                &self.pool,
                                &peer,
                                &self.get_peer(&e.from).addr(),
                            )
                            .await;
                            messages.is_empty() || messages.iter().all(|m| m.content != e.content)
                        },
                        TIMEOUT_MESSAGE
                    )
                    .await,
                    "Peer {peer} has received the message from {}",
                    e.from
                );
            }
        }
    }
}
