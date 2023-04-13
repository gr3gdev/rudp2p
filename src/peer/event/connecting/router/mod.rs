use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::sync::MutexGuard;

use crate::peer::event::common::{complete_map, insert_peer, Merge, Parser, send_with_socket};
use crate::peer::event::connected::PeerConnectedEvent;
use crate::peer::event::connecting::PeerConnectingEvent;
use crate::peer::event::PeerEvent;
use crate::peer::RemotePeer;
use crate::peer::router::Router;

pub(crate) fn connecting(peer_uid: &String, addr: SocketAddr, sender: SocketAddr, socket: &UdpSocket,
                         mut peers: &mut MutexGuard<HashMap<String, RemotePeer>>,
                         peer_connecting: PeerConnectingEvent,
                         public_key_pem: &Vec<u8>) {
    println!("[ {} ] connecting with {}", peer_uid, peer_connecting.uid);
    if !RemotePeer::exists(&peers, &peer_connecting.uid) {
        insert_peer(&mut peers, &peer_connecting.uid, sender, &peer_connecting.public_key_pem);
        // Response CONNECTED
        let my_peers = peers.values().cloned().collect();
        println!("[ {} ] send connected to {}", peer_uid, peer_connecting.uid);
        send_with_socket(socket, PeerConnectedEvent::event(PeerConnectedEvent {
            uid: peer_uid.clone(),
            addr,
            peers: my_peers,
            public_key_pem: public_key_pem.clone(),
        }), &sender);
    }
}

// STRUCT

pub(crate) struct ConnectingRouter;

// IMPL

impl ConnectingRouter {
    pub(crate) fn execute(peer_event: PeerEvent, socket: &UdpSocket, sender: SocketAddr, router: &Router) {
        let mut peers = router.shared_peers.lock().unwrap();
        let mut connecting_map = router.shared_connecting_map.lock().unwrap();
        let uid = peer_event.uid.clone();
        complete_map(&mut connecting_map, peer_event, &uid);
        // Check connecting map
        let mut completed = Vec::new();
        for list in connecting_map.values() {
            let connecting_event = PeerEvent::merge(list);
            if connecting_event.is_complete() {
                let peer_connecting = PeerConnectingEvent::parse(&connecting_event.message);
                println!("[ {} ] complete connecting with {}", router.peer_uid, peer_connecting.uid);
                connecting(&router.peer_uid, router.addr, sender, socket, &mut peers, peer_connecting, &router.public_key_pem);
                completed.push(connecting_event.uid)
            }
        }
        connecting_map.retain(|k, _| !completed.contains(k));
    }
}
