use log::debug;
use std::net::SocketAddr;

use crate::{
    network::{Request, Response},
    observer::Observer,
};

pub(crate) struct MessageService;

impl MessageService {
    pub(crate) async fn execute<O>(
        request: &Request,
        peer_uid: &String,
        remote_addr: &SocketAddr,
        remote_public_key: Option<Vec<u8>>,
        mut observer: O,
    ) -> (Option<Response>, Vec<u8>)
    where
        O: Observer,
    {
        debug!("Peer {peer_uid} - MESSAGE from {}", remote_addr);
        let message = request.to_message_event(peer_uid);
        debug!("Peer {peer_uid} - Fire event {:?}", message);
        let res = observer.on_message(message).await;
        if let Some(pk) = remote_public_key {
            (res, pk)
        } else {
            (res, vec![])
        }
    }
}
