use log::debug;
use std::net::{SocketAddr, UdpSocket};

use crate::{
    dao::{remote, Pool},
    network::{Request, Response},
    observer::Observer,
};

pub(crate) struct DisconnectionService;

impl DisconnectionService {
    pub(crate) async fn execute<O>(
        pool: &Pool,
        socket: &UdpSocket,
        request: &Request,
        peer_uid: &String,
        remote_addr: &SocketAddr,
        mut observer: O,
    ) -> (Option<Response>, Vec<u8>)
    where
        O: Observer,
    {
        let disconnection = request.to_disconnected_event(peer_uid, remote_addr);
        let remote = remote::select_by_uid(pool, &disconnection.from).await;
        if !remote.is_empty() {
            debug!("Peer {peer_uid} - DISCONNECTION from {}", remote_addr);
            remote::remove_by_uid(pool, &disconnection.from).await;
            // Send disconnection to remote too
            Request::new_disconnection(peer_uid.clone()).send(socket, remote_addr, &vec![]);
            debug!("Peer {peer_uid} - Fire event {:?}", disconnection);
            (observer.on_disconnected(disconnection).await, vec![])
        } else {
            (None, vec![])
        }
    }
}
