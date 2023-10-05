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
        let remotes = remote::select_by_address(&instance.pool, remote_addr).await;
        let res = if !remotes.is_empty() {
            let remote = remotes.get(0).unwrap();
            let message = request.to_message_event(remote);
            let res = observer.lock().unwrap().on_message(&message).await;
            (res, remote.public_key.clone())
        } else {
            (None, vec![])
        };
        log::trace!(
            "MessageService::execute({:?}, {:?}, {remote_addr}, observer) => {:?}",
            instance,
            request,
            res
        );
        res
    }
}
