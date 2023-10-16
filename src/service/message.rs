use crate::{
    dao::PeerDao,
    network::{Request, Response},
    observer::Observer,
    thread::PeerInstance,
    utils::unwrap::{unwrap_option, unwrap_result},
};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

pub(crate) struct MessageService;

impl MessageService {
    #[cfg(not(feature = "ssl"))]
    pub(crate) async fn execute<O, D>(
        instance: &PeerInstance,
        request: &Request,
        remote_addr: &SocketAddr,
        observer: Arc<Mutex<O>>,
        dao: Arc<Mutex<D>>,
    ) -> (Option<Response>, Vec<u8>)
    where
        O: Observer,
        D: PeerDao,
    {
        let remotes = dao
            .lock()
            .unwrap()
            .find_remotes_by_address(remote_addr)
            .await;
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
    pub(crate) async fn execute<O, D>(
        instance: &PeerInstance,
        request: &Request,
        remote_addr: &SocketAddr,
        observer: Arc<Mutex<O>>,
        dao: Arc<Mutex<D>>,
    ) -> (Option<Response>, Vec<u8>)
    where
        O: Observer,
        D: PeerDao,
    {
        let remotes = dao
            .lock()
            .unwrap()
            .find_remotes_by_address(remote_addr)
            .await;
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
