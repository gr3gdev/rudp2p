use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use crate::{
    dao::remote::{self},
    network::{Request, Response, send},
    observer::Observer,
    thread::PeerInstance,
};

pub(crate) struct DisconnectionService;

impl DisconnectionService {
    pub(crate) async fn execute<O>(
        instance: &PeerInstance,
        remote_addr: &SocketAddr,
        observer: Arc<Mutex<O>>,
    ) -> (Option<Response>, Vec<u8>)
    where
        O: Observer,
    {
        let exist = remote::select_by_address(&instance.pool, remote_addr).await;
        let res = if exist.is_empty() {
            (None, vec![])
        } else {
            let remote = exist.get(0).unwrap();
            if remote::remove(&instance.pool, &remote).await < 1 {
                log::error!("[DAO] Unable to remove remote peer");
                (None, vec![])
            } else {
                // Send disconnection to remote too
                let req = Request::new_disconnection();
                send(&instance.socket, &req, &remote.addr);
                (
                    observer.lock().unwrap().on_disconnected(remote).await,
                    vec![],
                )
            }
        };
        log::trace!(
            "DisconnectionService::execute({:?}, {remote_addr}, observer) => {:?}",
            instance,
            res
        );
        res
    }
}
