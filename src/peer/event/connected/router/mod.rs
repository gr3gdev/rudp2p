use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::sync::MutexGuard;

use crate::peer::event::common::{complete_map, DecryptParser, insert_peer, Merge};
use crate::peer::event::connected::PeerConnectedEvent;
use crate::peer::event::connecting::PeerConnectingEvent;
use crate::peer::event::connecting::router::connecting;
use crate::peer::event::PeerEvent;
use crate::peer::RemotePeer;
use crate::peer::router::Router;

fn connected(peer_uid: &String, addr: SocketAddr, socket: &UdpSocket,
             mut peers: &mut MutexGuard<HashMap<String, RemotePeer>>,
             peer_connected: &PeerConnectedEvent,
             public_key_pem: &Vec<u8>) {
    println!("connected : {} with {}", peer_uid, peer_connected.uid);
    insert_peer(&mut peers, &peer_connected.uid, peer_connected.addr, &peer_connected.public_key_pem);
    for other in &peer_connected.peers {
        connecting(&peer_uid, addr, other.addr, socket, &mut peers, PeerConnectingEvent {
            uid: other.uid.clone(),
            public_key_pem: public_key_pem.clone(),
        }, public_key_pem);
    }
}

// STRUCT

pub(crate) struct ConnectedRouter;

// IMPL

impl ConnectedRouter {
    pub(crate) fn execute(peer_event: PeerEvent, socket: &UdpSocket, router: &Router) {
        let mut peers = router.shared_peers.lock().unwrap();
        let mut connected_map = router.shared_connected_map.lock().unwrap();
        let guard_connected = router.shared_connected.lock().unwrap();
        let uid = peer_event.uid.clone();
        complete_map(&mut connected_map, peer_event, &uid);
        // Check connected map
        let mut completed = Vec::new();
        for list in connected_map.values() {
            let connected_event = PeerEvent::merge(list);
            if connected_event.is_complete() {
                let peer_connected = PeerConnectedEvent::parse(&connected_event.message, &router.private_key_pem, router.passphrase);
                println!("[ {} ] complete connected with {}", router.peer_uid, peer_connected.uid);
                if !RemotePeer::exists(&peers, &peer_connected.uid) {
                    connected(&router.peer_uid, router.addr, socket, &mut peers, &peer_connected, &router.public_key_pem);
                    if let Some(ref mut connected) = *guard_connected.borrow_mut() {
                        connected(&peer_connected.uid);
                    }
                }
                completed.push(connected_event.uid);
            }
        }
        connected_map.retain(|k, _| !completed.contains(k));
    }
}
