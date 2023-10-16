use async_trait::async_trait;
use rudp2plib::{
    configuration::Configuration,
    dao::InMemoryDao,
    network::{events::Message, Response},
    observer::Observer,
    peer::{Peer, RemotePeer},
};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone)]
struct TestObserver {
    connected: Arc<Mutex<Vec<SocketAddr>>>,
    messages: Arc<Mutex<Vec<String>>>,
}

#[async_trait]
impl Observer for TestObserver {
    async fn on_connected(&mut self, remote: &RemotePeer) -> Option<Response> {
        self.connected.lock().unwrap().push(remote.addr);
        None
    }

    async fn on_disconnected(&mut self, remote: &RemotePeer) -> Option<Response> {
        let mut connected = self.connected.lock().unwrap();
        let index = connected.iter().position(|a| a == &remote.addr).unwrap();
        connected.remove(index);
        None
    }

    async fn on_message(&mut self, message: &Message) -> Option<Response> {
        self.messages
            .lock()
            .unwrap()
            .push(String::from_utf8(message.content.clone()).unwrap());
        None
    }
}

#[wasm_bindgen]
pub struct PeerWasm {
    peer: Peer,
    observer: TestObserver,
}

#[wasm_bindgen]
impl PeerWasm {
    pub async fn start(port: u16) -> Self {
        let configuration = Configuration::builder()
            .share_connections(false)
            .port(port)
            .build();
        let observer = TestObserver {
            connected: Arc::new(Mutex::new(Vec::new())),
            messages: Arc::new(Mutex::new(Vec::new())),
        };
        let peer = Peer::new(configuration, InMemoryDao::default(), observer.clone()).await;
        PeerWasm { peer, observer }
    }

    pub async fn stop(&self) -> () {
        self.peer.close()
    }

    pub async fn get_connected(&self) -> String {
        self.observer
            .connected
            .lock()
            .unwrap()
            .iter()
            .map(SocketAddr::to_string)
            .collect::<Vec<String>>()
            .join(", ")
    }
}
