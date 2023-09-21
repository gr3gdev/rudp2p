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

pub(crate) struct MessageService;

impl MessageService {
    pub(crate) async fn execute<O>(
        instance: &PeerInstance,
        request: &Request,
        remote_addr: &SocketAddr,
        observer: Arc<Mutex<O>>,
    ) -> (Option<Response>, Vec<u8>)
    where
        O: Observer,
    {
        let remote = remote::select_by_address(&instance.uid, &instance.pool, remote_addr).await;
        debug!("[PEER {}] MESSAGE from {}", instance.uid, remote_addr);
        let message = request.to_message_event(&instance.uid);
        debug!("[PEER {}] Fire event {:?}", instance.uid, message);
        let res = observer.lock().unwrap().on_message(message).await;
        (res, remote.public_key)
    }
}
