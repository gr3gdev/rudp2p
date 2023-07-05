use std::{collections::HashMap, net::SocketAddr};

use crate::{
    Error, Event, InternalPeer, InternalTypeEvent, Message, MessageMethod, RemotePeer, TypeEvent,
};

use super::FlowProcessor;

pub(crate) struct DisconnectingFlow;

impl FlowProcessor for DisconnectingFlow {
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
        let mut peers_connected = peer_data.peers_connected.unwrap();
        if let Some(peer) = peers_connected.iter().find(|p| &p.1.addr == sender) {
            if sender != &peer_data.addr {
                let mut responses = HashMap::new();
                let mut event = None;
                let peer_uid = peer.0.clone();
                // Remove remote peer from the list of connected
                peers_connected.remove(&peer_uid);
                let mut messages = Vec::new();
                // Send disconnecting response to remote
                if message.method == MessageMethod::Request {
                    messages.push(Message::new_internal(
                        InternalTypeEvent::DISCONNECTING,
                        MessageMethod::RequestAndResponse,
                        vec![],
                    ));
                } else if message.method == MessageMethod::RequestAndResponse {
                    messages.push(Message::new_internal(
                        InternalTypeEvent::DISCONNECTING,
                        MessageMethod::Response,
                        vec![],
                    ));
                }
                if message.method == MessageMethod::Request
                    || message.method == MessageMethod::RequestAndResponse
                {
                    event = Some(Event {
                        type_event: TypeEvent::DISCONNECTED,
                        to: peer_data.uid.clone(),
                        from: peer_uid.clone(),
                        message: None,
                    });
                }
                responses.insert(sender.clone(), messages);
                // Disconnected Event
                Ok((event, peers_connected.clone(), responses))
            } else {
                Ok((None, peers_connected.clone(), HashMap::new()))
            }
        } else {
            Ok((None, peers_connected.clone(), HashMap::new()))
        }
    }
}
