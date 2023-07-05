use std::{collections::HashMap, net::SocketAddr};

use crate::{
    encoder::{Data, Encoder},
    Error, Event, InternalPeer, InternalTypeEvent, Message, MessageMethod, RemotePeer,
};

use super::FlowProcessor;

pub(crate) struct SharePeersFlow;

impl FlowProcessor for SharePeersFlow {
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
        let peers_connected = peer_data.peers_connected.clone().unwrap();
        if peer_data.share_peers {
            let peer = peers_connected
                .iter()
                .find(|p| &p.1.addr == sender)
                .unwrap();
            if message.method == MessageMethod::Request {
                let mut responses = HashMap::new();
                responses.insert(
                    peer.1.addr,
                    vec![Message::internal(
                        InternalTypeEvent::PEERS,
                        MessageMethod::Response,
                        &peer_data,
                        peer.1.public_key.clone(),
                    )],
                );
                Ok((None, peers_connected.clone(), responses))
            } else {
                let mut responses = HashMap::new();
                let (_, _, peers) = Encoder::extract(
                    vec![Data::Peers],
                    &message.data,
                    Some(&peer_data.private_key.as_ref().unwrap()),
                )
                .unwrap();
                for (uid, addr) in peers {
                    if uid != peer_data.uid {
                        responses.insert(
                            addr,
                            vec![Message::internal(
                                InternalTypeEvent::CONNECTING,
                                MessageMethod::Request,
                                &peer_data,
                                None,
                            )],
                        );
                    }
                }
                Ok((None, peers_connected.clone(), responses))
            }
        } else {
            Ok((None, peers_connected.clone(), HashMap::new()))
        }
    }
}
