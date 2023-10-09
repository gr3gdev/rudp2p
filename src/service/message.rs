use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use crate::{
    dao::remote,
    network::{Request, Response},
    observer::Observer,
    thread::PeerInstance,
    utils::unwrap::{unwrap_option, unwrap_result},
};

pub(crate) struct MessageService;

impl MessageService {
    #[cfg(not(feature = "ssl"))]
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
            let remote = unwrap_option(remotes.first(), "Unable to get the first remote peer");
            let message = request.to_message_event(remote);
            let res = unwrap_result(observer.lock(), "Unable to send on_message event")
                .on_message(&message)
                .await;
            (res, vec![])
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

    #[cfg(feature = "ssl")]
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
            let remote = unwrap_option(remotes.first(), "Unable to get the first remote peer");
            let message = request.to_message_event(remote);
            let res = unwrap_result(observer.lock(), "Unable to send on_message event")
                .on_message(&message)
                .await;
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
