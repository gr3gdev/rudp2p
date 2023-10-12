use crate::{
    dao::PeerDao,
    network::{send, Request, Response},
    observer::Observer,
    thread::PeerInstance,
    utils::unwrap::{unwrap_option, unwrap_result},
};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

pub(crate) struct DisconnectionService;

impl DisconnectionService {
    pub(crate) async fn execute<O, D>(
        instance: &PeerInstance,
        remote_addr: &SocketAddr,
        observer: Arc<Mutex<O>>,
        dao: Arc<Mutex<D>>,
    ) -> (Option<Response>, Vec<u8>)
    where
        O: Observer,
        D: PeerDao,
    {
        let exist = dao
            .lock()
            .unwrap()
            .find_remotes_by_address(remote_addr)
            .await;
        let res = if exist.is_empty() {
            (None, vec![])
        } else {
            let remote = unwrap_option(exist.first(), "Unable to get the first remote peer");
            if dao.lock().unwrap().remove_remote(&remote).await < 1 {
                log::error!("[DAO] Unable to remove remote peer");
                (None, vec![])
            } else {
                // Send disconnection to remote too
                let req = Request::new_disconnection();
                send(&instance.socket, &req, &remote.addr);
                (
                    unwrap_result(observer.lock(), "Unable to send on_disconnected event")
                        .on_disconnected(remote)
                        .await,
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
