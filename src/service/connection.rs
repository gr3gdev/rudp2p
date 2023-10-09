use crate::{
    dao::remote,
    network::{send, Request, Response},
    observer::Observer,
    peer::RemotePeer,
    thread::PeerInstance,
    utils::unwrap::unwrap_result,
};
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
) -> () {
    log::trace!(
        "share_connection({:?}, {addr}, {:?}, {share_connections})",
        socket,
        connected_peers
    );
    if share_connections {
        let req = Request::new_share_connection(connected_peers);
        send(socket, &req, addr);
    } else {
        log::debug!("share connections is disabled");
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
        let res = if exist.is_empty() {
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
            let req = Request::new_connection(&instance.configuration);
            send(&instance.socket, &req, &remote_addr);
            (
                unwrap_result(observer.lock(), "Unable to send on_connected event")
                    .on_connected(&remote)
                    .await,
                vec![],
            )
        } else {
            (None, vec![])
        };
        log::trace!(
            "ConnectionService::execute({:?}, {:?}, {remote_addr}, observer) => {:?}",
            instance,
            request,
            res
        );
        res
    }
}
