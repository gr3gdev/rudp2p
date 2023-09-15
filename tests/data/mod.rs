use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
    time::Duration,
};

use cucumber::{gherkin::Step, World};
use futures::{future::LocalBoxFuture, FutureExt};
use rudp2plib::{configuration::Configuration, peer::*};

use crate::utils::{get_time, read_file, wait_while_condition};

#[derive(World, Debug)]
#[world(init = Self::new)]
pub(crate) struct PeersWorld {
    peers: Vec<Peer>,
    connected_events: Arc<Mutex<HashMap<String, Vec<ReceiveEvent>>>>,
    disconnected_events: Arc<Mutex<HashMap<String, Vec<ReceiveEvent>>>>,
    message_events: Arc<Mutex<HashMap<String, Vec<ReceiveEvent>>>>,
}

#[derive(Debug)]
pub(crate) struct PeerData {
    pub(crate) name: String,
    pub(crate) port: u16,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ReceiveEvent {
    pub(crate) content: Option<Vec<u8>>,
    pub(crate) from: String,
}

#[derive(Debug, Clone)]
pub(crate) struct Event {
    pub(crate) event: String,
    content: Option<Vec<u8>>,
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
                        content = Some(read_file(&data[5..].trim()));
                    } else {
                        content = Some(data.as_bytes().to_vec());
                    }
                    from = row[2].clone();
                } else {
                    content = None;
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

impl ReceiveEvent {
    pub(crate) fn presents_in(&self, events: Vec<Event>, type_event: String) -> bool {
        events
            .iter()
            .find(|e| e.event == type_event && e.from == self.from && e.content == self.content)
            .is_some()
    }
}

impl PeersWorld {
    fn new() -> Self {
        Self {
            peers: Vec::new(),
            connected_events: Arc::new(Mutex::new(HashMap::new())),
            disconnected_events: Arc::new(Mutex::new(HashMap::new())),
            message_events: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn complete_event(
        mut e: MutexGuard<HashMap<String, Vec<ReceiveEvent>>>,
        uid: String,
        event: ReceiveEvent,
    ) {
        match e.entry(uid.clone()) {
            std::collections::hash_map::Entry::Occupied(mut o) => {
                let list = o.get_mut();
                if !list.contains(&event) {
                    list.push(event);
                }
            }
            std::collections::hash_map::Entry::Vacant(_) => {
                e.insert(uid, vec![event]);
            }
        }
    }

    pub(crate) async fn add_all(&mut self, peers: Vec<PeerData>) {
        for peer_data in peers {
            let connected_events = Arc::clone(&self.connected_events);
            let disconnected_events = Arc::clone(&self.disconnected_events);
            let message_events = Arc::clone(&self.message_events);
            let conf = Configuration::builder()
                .port(peer_data.port)
                .name(&peer_data.name)
                .share_connections(true)
                .build();
            let peer = Peer::new(
                conf,
                move |c| {
                    let e = connected_events.lock().expect("Unable to lock event");
                    let event = ReceiveEvent {
                        content: None,
                        from: c.from.clone(),
                    };
                    Self::complete_event(e, c.uid.clone(), event);
                    None
                },
                move |d| {
                    let e = disconnected_events.lock().expect("Unable to lock event");
                    let event = ReceiveEvent {
                        content: None,
                        from: d.from.clone(),
                    };
                    Self::complete_event(e, d.uid.clone(), event);
                    None
                },
                move |m| {
                    let e = message_events.lock().expect("Unable to lock event");
                    let event = ReceiveEvent {
                        content: Some(m.content.clone()),
                        from: m.from.clone(),
                    };
                    Self::complete_event(e, m.from.clone(), event);
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

    pub(crate) fn get_events(
        &self,
        type_event: &str,
    ) -> std::sync::MutexGuard<'_, HashMap<String, Vec<ReceiveEvent>>> {
        match type_event {
            "CONNECTED" => self
                .connected_events
                .lock()
                .expect("Unable to lock connected_events"),
            "DISCONNECTED" => self
                .disconnected_events
                .lock()
                .expect("Unable to lock disconnected_events"),
            "MESSAGE" => self
                .message_events
                .lock()
                .expect("Unable to lock message_events"),
            _ => panic!("Unknown event"),
        }
    }

    pub(crate) fn check_peer_receive(&self, peer: String, events: Vec<Event>, type_event: String) {
        wait_while_condition(&|| {
            let map = self.get_events(&type_event);
            let empty = vec![];
            let list = map.get(&peer).unwrap_or(&empty);
            list.iter()
                .find(|e| e.presents_in(events.clone(), type_event.clone()))
                .is_none()
        });
    }

    pub(crate) fn check_peer_not_receive(
        &self,
        peer: String,
        events: Vec<Event>,
        type_event: String,
    ) {
        wait_while_condition(&|| {
            let map = self.get_events(&type_event);
            let empty = vec![];
            let list = map.get(&peer).unwrap_or(&empty);
            list.iter()
                .find(|e| e.presents_in(events.clone(), type_event.clone()))
                .is_some()
        });
    }
}
