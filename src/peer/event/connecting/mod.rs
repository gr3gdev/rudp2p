use crate::peer::event::{CONNECTING, PeerEvent};
use crate::peer::event::common::{append_public_pem, append_uid, parse_public_pem, parse_uid, Parser};

pub(crate) mod router;
mod tests;

// STRUCT

pub struct PeerConnectingEvent {
    /// Uid of the peer that connects.
    pub uid: String,
    /// Public key for encrypt messages.
    pub public_key_pem: Vec<u8>,
}

// IMPL

impl PeerConnectingEvent {
    pub(crate) fn event(peer_connecting: PeerConnectingEvent) -> PeerEvent {
        let mut list = Vec::new();
        append_uid(&mut list, peer_connecting.uid);
        append_public_pem(&mut list, &peer_connecting.public_key_pem);
        PeerEvent::new(CONNECTING, list)
    }
}

impl Parser<PeerConnectingEvent> for PeerConnectingEvent {
    fn parse(message: &Vec<u8>) -> PeerConnectingEvent {
        let uid_data = parse_uid(message);
        let public_key_data = parse_public_pem(message, uid_data.end);
        PeerConnectingEvent {
            uid: uid_data.value,
            public_key_pem: public_key_data.value,
        }
    }
}
