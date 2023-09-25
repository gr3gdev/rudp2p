use crate::{
    dao::remote,
    network::{Request, Response},
    observer::Observer,
    peer::RemotePeer,
    thread::PeerInstance,
};
use log::debug;
use std::{
    net::{SocketAddr, UdpSocket},
    sync::{Arc, Mutex},
};

pub(crate) struct ConnectionService;

fn share_connection(
    socket: &UdpSocket,
    addr: &SocketAddr,
    connected_peers: &Vec<RemotePeer>,
    share_connections: bool,
) {
    if share_connections {
        let req = Request::new_share_connection(connected_peers);
        debug!("share connection {:?} with {}", connected_peers, addr);
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
        let connected_peers = remote::select_all(&instance.pool).await;
        let public_key = request.parse_public_key();
        let exist = remote::select_by_address(&instance.pool, remote_addr).await;
        if exist.is_empty() {
            debug!("CONNECTION from {remote_addr}");
            // Share connection ?
            share_connection(
                &instance.socket,
                &remote_addr,
                &connected_peers,
                instance.configuration.share_connections,
            );
            // Cache the connection with address and public key
            let remote = remote::add(&instance.pool, &remote_addr, &public_key).await;
            // Send connection to remote too
            Request::new_connection(&instance.public_key).send(
                &instance.socket,
                &remote_addr,
                &vec![],
            );
            debug!("Fire event : on_connected({:?})", remote);
            (observer.lock().unwrap().on_connected(&remote).await, vec![])
        } else {
            (None, vec![])
        }
    }
}
