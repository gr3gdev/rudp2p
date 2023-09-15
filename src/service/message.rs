use log::debug;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use crate::network::{events::Message, Request, Response};

pub(crate) struct MessageService;

impl MessageService {
    pub(crate) async fn execute<M>(
        request: &Request,
        peer_uid: &String,
        remote_addr: &SocketAddr,
        remote_public_key: Option<Vec<u8>>,
        on_message: Arc<Mutex<Box<M>>>,
    ) -> (Option<Response>, Vec<u8>)
    where
        M: FnMut(Message) -> Option<Response>,
    {
        debug!("Peer {peer_uid} - MESSAGE from {}", remote_addr);
        let mut on_message = on_message.lock().unwrap();
        let message = request.to_message_event(peer_uid);
        debug!("Peer {peer_uid} - Fire event {:?}", message);
        if let Some(pk) = remote_public_key {
            (on_message(message), pk)
        } else {
            (on_message(message), vec![])
        }
    }
}
