use log::debug;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use crate::{
    dao::remote,
    network::{Request, Response},
    observer::Observer,
    thread::PeerInstance,
};

pub(crate) struct DisconnectionService;

impl DisconnectionService {
    pub(crate) async fn execute<O>(
        instance: &PeerInstance,
        request: &Request,
        remote_addr: &SocketAddr,
        observer: Arc<Mutex<O>>,
    ) -> (Option<Response>, Vec<u8>)
    where
        O: Observer,
    {
        let disconnection = request.to_disconnected_event(&instance.uid, remote_addr);
        let remote =
            remote::select_by_uid(&instance.uid, &instance.pool, &disconnection.from).await;
        if !remote.is_empty() {
            debug!("[PEER {}] DISCONNECTION from {}", instance.uid, remote_addr);
            remote::remove_by_uid(&instance.uid, &instance.pool, &disconnection.from).await;
            // Send disconnection to remote too
            Request::new_disconnection(&instance.uid).send(&instance.socket, remote_addr, &vec![]);
            debug!("[PEER {}] Fire event {:?}", instance.uid, disconnection);
            (
                observer
                    .lock()
                    .unwrap()
                    .on_disconnected(disconnection)
                    .await,
                vec![],
            )
        } else {
            (None, vec![])
        }
    }
}
