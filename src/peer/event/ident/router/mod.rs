use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::sync::MutexGuard;

use crate::peer::event::common::{Parser, send_with_socket};
use crate::peer::event::ident::{PeerDisconnectedEvent, PeerIdentEvent};
use crate::peer::event::PeerEvent;
use crate::peer::RemotePeer;
use crate::peer::router::Router;

fn disconnecting(peer_uid: &String, addr: SocketAddr, sender: SocketAddr, socket: &UdpSocket,
                 peers: &mut MutexGuard<HashMap<String, RemotePeer>>,
                 peer_disconnecting: PeerIdentEvent,
                 public_key_pem: &Vec<u8>) {
    peers.remove(&peer_disconnecting.uid);
    send_with_socket(socket, PeerDisconnectedEvent::event(peer_uid.clone(), addr, public_key_pem), &sender);
}

// STRUCT

pub(crate) struct DisconnectingRouter;

pub(crate) struct DisconnectedRouter;

// IMPL

impl DisconnectingRouter {
    pub(crate) fn execute(peer_event: PeerEvent, socket: &UdpSocket, sender: SocketAddr, router: &Router) {
        let mut peers = router.shared_peers.lock().unwrap();
        let peer_disconnecting = PeerIdentEvent::parse(&peer_event.message);
        disconnecting(&router.peer_uid, router.addr, sender, socket, &mut peers, peer_disconnecting, &router.public_key_pem);
    }
}

impl DisconnectedRouter {
    pub(crate) fn execute(peer_event: PeerEvent, router: &Router) {
        let mut peers = router.shared_peers.lock().unwrap();
        let guard_disconnected = router.shared_disconnected.lock().unwrap();
        let peer_disconnected = PeerIdentEvent::parse(&peer_event.message);
        peers.remove(&peer_disconnected.uid);
        if let Some(ref mut disconnected) = *guard_disconnected.borrow_mut() {
            disconnected(&peer_disconnected.uid);
        };
    }
}