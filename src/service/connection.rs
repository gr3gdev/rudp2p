use log::debug;
use std::{
    net::{SocketAddr, UdpSocket},
    sync::{Arc, Mutex},
};

use crate::{
    dao::remote::{self, RemotePeer},
    network::{Request, Response},
    observer::Observer,
    thread::PeerInstance,
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
            "[PEER {peer_uid}] share connection {:?} with {}",
            connected_peers, addr
        );
        req.send(socket, &addr, &vec![]);
    }
}

impl ConnectionService {
    pub(crate) async fn execute<O>(
        instance: &PeerInstance,
        request: &Request,
        remote_addr: &SocketAddr,
        observer: Arc<Mutex<O>>,
    ) -> (Option<Response>, Vec<u8>)
    where
        O: Observer,
    {
        let connected_peers = remote::select_all(&instance.uid, &instance.pool).await;
        let connection = request.to_connected_event(&instance.uid, remote_addr);
        if remote::select_by_uid(&instance.uid, &instance.pool, &connection.from)
            .await
            .is_empty()
        {
            debug!("[PEER {}] CONNECTION from {}", instance.uid, remote_addr);
            // Share connection ?
            share_connection(
                &instance.uid,
                &instance.socket,
                remote_addr,
                &connected_peers,
                instance.configuration.share_connections,
            );
            // Cache the connection with address and public key
            let remote = RemotePeer::new(&connection);
            remote::add(&instance.uid, &instance.pool, &remote).await;
            // Send connection to remote too
            Request::new_connection(&instance.uid, &instance.public_key).send(
                &instance.socket,
                &remote_addr,
                &vec![],
            );
            debug!("[PEER {}] Fire event {:?}", instance.uid, connection);
            (
                observer.lock().unwrap().on_connected(connection).await,
                vec![],
            )
        } else {
            (None, vec![])
        }
    }
}
