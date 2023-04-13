use std::net::SocketAddr;

use crate::peer::event::{DISCONNECTED, DISCONNECTING, PeerEvent};
use crate::peer::event::common::{append_address, append_uid, parse_uid, Parser};

pub(crate) mod router;

// STRUCT

pub struct PeerIdentEvent {
    /// Uid of the peer that connects.
    pub uid: String,
}

pub struct PeerDisconnectedEvent;

pub struct PeerDisconnectingEvent;

// IMPL

impl PeerDisconnectedEvent {
    pub(crate) fn event(uid: String, addr: SocketAddr, public_key_pem: &Vec<u8>) -> PeerEvent {
        let mut list = Vec::new();
        append_uid(&mut list, uid);
        append_address(&mut list, addr, public_key_pem);
        PeerEvent::new(DISCONNECTED, list)
    }
}

impl PeerDisconnectingEvent {
    pub(crate) fn event(uid: String) -> PeerEvent {
        let mut list = Vec::new();
        append_uid(&mut list, uid);
        PeerEvent::new(DISCONNECTING, list)
    }
}

impl Parser<PeerIdentEvent> for PeerIdentEvent {
    fn parse(message: &Vec<u8>) -> PeerIdentEvent {
        let uid_data = parse_uid(message);
        PeerIdentEvent {
            uid: uid_data.value,
        }
    }
}
