use log::debug;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use crate::{
    dao::remote::{self},
    network::{Request, Response},
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
        debug!("DISCONNECTION from {remote_addr}");
        let exist = remote::select_by_address(&instance.pool, remote_addr).await;
        if exist.is_empty() {
            (None, vec![])
        } else {
            let remote = exist.get(0).unwrap();
            remote::remove(&instance.pool, &remote).await;
            // Send disconnection to remote too
            Request::new_disconnection().send(&instance.socket, &remote.addr, &vec![]);
            debug!("Fire event : on_disconnected({:?})", remote);
            (
                observer.lock().unwrap().on_disconnected(remote).await,
                vec![],
            )
        }
    }
}
