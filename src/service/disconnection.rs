use log::debug;
use std::{
    net::{SocketAddr, UdpSocket},
    sync::{Arc, Mutex},
};

use crate::{
    dao::{remote, Pool},
    network::{events::Disconnected, Request, Response},
};

pub(crate) struct DisconnectionService;

impl DisconnectionService {
    pub(crate) async fn execute<D>(
        pool: &Pool,
        socket: &UdpSocket,
        request: &Request,
        peer_uid: &String,
        remote_addr: &SocketAddr,
        on_disconnected: Arc<Mutex<Box<D>>>,
    ) -> (Option<Response>, Vec<u8>)
    where
        D: FnMut(Disconnected) -> Option<Response>,
    {
        let disconnection = request.to_disconnected_event(peer_uid, remote_addr);
        let remote = remote::select_by_uid(pool, &disconnection.from).await;
        if !remote.is_empty() {
            debug!("Peer {peer_uid} - DISCONNECTION from {}", remote_addr);
            remote::remove_by_uid(pool, &disconnection.from).await;
            // Send disconnection to remote too
            Request::new_disconnection(peer_uid.clone()).send(socket, remote_addr, &vec![]);
            let mut on_disconnected = on_disconnected.lock().unwrap();
            debug!("Peer {peer_uid} - Fire event {:?}", disconnection);
            (on_disconnected(disconnection), vec![])
        } else {
            (None, vec![])
        }
    }
}
