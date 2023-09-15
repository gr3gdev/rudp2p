use log::debug;
use std::{
    net::{SocketAddr, UdpSocket},
    sync::{Arc, Mutex},
};

use crate::{
    dao::{
        remote::{self, RemotePeer},
        Pool,
    },
    network::{events::Connected, Request, Response},
};

pub(crate) struct ConnectionService;

fn share_connection(
    peer_uid: &String,
    socket: &UdpSocket,
    addr: &SocketAddr,
    connected_peers: &Vec<RemotePeer>,
    share_connections: bool,
) {
    if share_connections {
        let req = Request::new_share_connection(connected_peers);
        debug!(
            "Peer {peer_uid} - share connection {:?} with {}",
            connected_peers, addr
        );
        req.send(socket, &addr, &vec![]);
    }
}

impl ConnectionService {
    pub(crate) async fn execute<C>(
        pool: &Pool,
        socket: &UdpSocket,
        request: &Request,
        peer_uid: &String,
        public_key: &Vec<u8>,
        remote_addr: &SocketAddr,
        connected_peers: &Vec<RemotePeer>,
        share_connections: bool,
        on_connected: Arc<Mutex<Box<C>>>,
    ) -> (Option<Response>, Vec<u8>)
    where
        C: FnMut(Connected) -> Option<Response>,
    {
        let connection = request.to_connected_event(peer_uid, remote_addr);
        if remote::select_by_uid(pool, &connection.from)
            .await
            .is_empty()
        {
            debug!("Peer {peer_uid} - CONNECTION from {}", remote_addr);
            // Share connection ?
            share_connection(
                peer_uid,
                socket,
                remote_addr,
                connected_peers,
                share_connections,
            );
            // Cache the connection with address and public key
            let remote = RemotePeer::new(connection.clone());
            remote::add(pool, &remote).await;
            // Send connection to remote too
            Request::new_connection(peer_uid.clone(), public_key.clone()).send(
                socket,
                &remote_addr,
                &vec![],
            );
            let mut on_connected = on_connected.lock().unwrap();
            debug!("Peer {peer_uid} - Fire event {:?}", connection);
            (on_connected(connection), vec![])
        } else {
            (None, vec![])
        }
    }
}
