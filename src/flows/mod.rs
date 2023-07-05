use std::{collections::HashMap, net::SocketAddr};

use crate::{Error, Event, InternalPeer, Message, RemotePeer};

pub(crate) mod connecting;
pub(crate) mod disconnecting;
pub(crate) mod message;
pub(crate) mod sharepeers;

pub(crate) trait FlowProcessor {
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
    >;
}
