use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::utils::{log, wait_while_condition};
use cucumber::{gherkin::Step, World};
use futures::{future::LocalBoxFuture, FutureExt};
use rudp2plib::{Message, Peer};

use crate::utils::read_file;

#[derive(World, Debug)]
#[world(init = Self::new)]
pub(crate) struct PeersWorld {
    peers: Vec<Peer>,
    events: Arc<Mutex<HashMap<String, Vec<ReceiveEvent>>>>,
}

#[derive(Debug)]
pub(crate) struct PeerData {
    pub(crate) name: String,
    pub(crate) port: u16,
}

#[derive(Debug)]
pub(crate) struct ReceiveEvent {
    pub(crate) event: String,
    pub(crate) content: Vec<u8>,
    pub(crate) from: String,
}

#[derive(Debug, Clone)]
pub(crate) struct Event {
    event: String,
    content: Vec<u8>,
    from: String,
}

pub(crate) struct TextMessage {
    pub(crate) text: String,
}

pub(crate) trait DataTable {
    fn read_from_datatable(step: &Step) -> Vec<Self>
    where
        Self: Sized;
}

impl Message for TextMessage {
    fn content(&self) -> Vec<u8> {
        self.text.as_bytes().to_vec()
    }
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

impl ReceiveEvent {
    pub(crate) fn presents_in(&self, events: Vec<Event>) -> bool {
        events
            .iter()
            .find(|e| e.event == self.event && e.from == self.from && e.content == self.content)
            .is_some()
    }
}

impl PeersWorld {
    fn new() -> Self {
        Self {
            peers: Vec::new(),
            events: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub(crate) fn add_all(&mut self, peers: Vec<PeerData>) {
        for peer_data in peers {
            let events = Arc::clone(&self.events);
            let peer = Peer::start(
                peer_data.port,
                move |event| {
                    log(format!(
                        "{} receive event {} from {}",
                        event.to,
                        event.type_event.to_string(),
                        event.from
                    ));
                    let mut e = events.lock().expect("Unable to lock event");
                    let uid = event.to.clone();
                    let event = ReceiveEvent {
                        event: event.type_event.to_string(),
                        content: event.data.clone(),
                        from: event.from,
                    };
                    let mut peer_events = vec![event];
                    let list = e.entry(uid.clone()).or_insert(vec![]);
                    peer_events.append(list);
                    e.insert(uid, peer_events);
                },
                Some(peer_data.name.clone()),
            );
            let peer = peer.expect("Error when create peer");
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
                wait_while_condition(format!("{} is alive", peer.uid), &|| peer.alive());
            }
        }
        .boxed_local()
    }

    pub(crate) fn check_peer_receive(&self, peer: String, events: Vec<Event>) {
        wait_while_condition(format!("{} receives events {:?}", peer, events), &|| {
            let map = self.events.lock().expect("Unable to lock events");
            let empty = vec![];
            let list = map.get(&peer).unwrap_or(&empty);
            list.iter()
                .find(|e| e.presents_in(events.clone()))
                .is_none()
        });
    }

    pub(crate) fn check_peer_not_receive(&self, peer: String, events: Vec<Event>) {
        wait_while_condition(format!("{} not receives events {:?}", peer, events), &|| {
            let map = self.events.lock().expect("Unable to lock events");
            let empty = vec![];
            let list = map.get(&peer).unwrap_or(&empty);
            list.iter()
                .find(|e| e.presents_in(events.clone()))
                .is_some()
        });
    }
}
