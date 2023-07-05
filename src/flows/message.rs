use std::{collections::HashMap, net::SocketAddr};

use crate::{encoder::Encoder, Error, Event, InternalPeer, Message, RemotePeer, TypeEvent};

use super::FlowProcessor;

pub(crate) struct MessageFlow;

impl FlowProcessor for MessageFlow {
    fn process(
        message: &Message,
        peer_data: InternalPeer,
        sender: &SocketAddr,
    ) -> Result<
        (
            Option<Event>,
            HashMap<String, RemotePeer>,
            HashMap<SocketAddr, Vec<Message>>,
        ),
        Error,
    > {
        let peers_connected = peer_data.peers_connected.unwrap();
        if sender != &peer_data.addr {
            let peer = peers_connected
                .iter()
                .find(|p| &p.1.addr == sender)
                .unwrap();
            let private_key = peer_data.private_key.as_ref().unwrap();
            Encoder::decrypt(&private_key, &message.data, message.total).and_then(|data| {
                Ok((
                    Some(Event {
                        type_event: TypeEvent::MESSAGE,
                        to: peer_data.uid.clone(),
                        from: peer.0.clone(),
                        message: Some(Message::new(data)),
                    }),
                    peers_connected.clone(),
                    HashMap::new(),
                ))
            })
        } else {
            Ok((None, peers_connected.clone(), HashMap::new()))
        }
    }
}
