use std::{collections::HashMap, net::SocketAddr};

use crate::{
    encoder::{Data, Encoder},
    Error, Event, InternalPeer, InternalTypeEvent, Message, MessageMethod, RemotePeer, TypeEvent,
};

use super::FlowProcessor;

pub(crate) struct ConnectingFlow;

impl FlowProcessor for ConnectingFlow {
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
        Encoder::extract(vec![Data::Uid, Data::PublicKey], &message.data, None).and_then(
            |(peer_uid, public_key, _)| {
                let mut responses = HashMap::new();
                let mut event = None;
                let mut peers_connected = peer_data.peers_connected.clone().unwrap();
                if peer_data.rejects.contains(&peer_uid) {
                    let mut message = String::from("Peer ");
                    message.push_str(peer_uid.as_str());
                    message.push_str(" is blocked");
                    Err(Error::custom(&message))
                } else {
                    if !peers_connected.contains_key(&peer_uid) && sender != &peer_data.addr {
                        // Add remote peer to list of connected (with public key)
                        peers_connected.insert(
                            peer_uid.clone(),
                            RemotePeer {
                                uid: peer_uid.clone(),
                                addr: sender.clone(),
                                public_key: Some(public_key.clone()),
                            },
                        );
                        // Send connecting response to remote
                        let mut messages = Vec::new();
                        if message.method == MessageMethod::Request {
                            messages.push(Message::internal(
                                InternalTypeEvent::CONNECTING,
                                MessageMethod::RequestAndResponse,
                                &peer_data,
                                None,
                            ));
                        } else if message.method == MessageMethod::RequestAndResponse {
                            messages.push(Message::internal(
                                InternalTypeEvent::CONNECTING,
                                MessageMethod::Response,
                                &peer_data,
                                None,
                            ));
                        }
                        if message.method == MessageMethod::Request
                            || message.method == MessageMethod::RequestAndResponse
                        {
                            messages.push(Message::internal(
                                InternalTypeEvent::PEERS,
                                MessageMethod::Request,
                                &peer_data,
                                Some(public_key.clone()),
                            ));
                            // Connected Event
                            event = Some(Event {
                                type_event: TypeEvent::CONNECTED,
                                to: peer_data.uid.clone(),
                                from: peer_uid.clone(),
                                message: None,
                            });
                        }
                        responses.insert(sender.clone(), messages);
                    }
                    Ok((event, peers_connected.clone(), responses))
                }
            },
        )
    }
}
