use std::net::SocketAddr;

use crate::peer::event::{CONNECTED, PeerEvent};
use crate::peer::event::common::{append_address, append_peers, append_public_pem, append_uid, DecryptParser, parse_address, parse_peers, parse_public_pem, parse_uid};
use crate::peer::RemotePeer;

pub(crate) mod router;
mod tests;

// STRUCT

pub struct PeerConnectedEvent {
    /// Uid of the peer connected.
    pub uid: String,
    /// Address of the peer connected.
    pub addr: SocketAddr,
    /// List of peers with which you can connect.
    pub peers: Vec<RemotePeer>,
    /// Public key for encrypt messages.
    pub public_key_pem: Vec<u8>,
}

// IMPL

impl PeerConnectedEvent {
    pub(crate) fn event(peer_connected_event: PeerConnectedEvent) -> PeerEvent {
        let mut list = Vec::new();
        append_uid(&mut list, peer_connected_event.uid);
        append_public_pem(&mut list, &peer_connected_event.public_key_pem);
        append_address(&mut list, peer_connected_event.addr, &peer_connected_event.public_key_pem);
        append_peers(&mut list, peer_connected_event.peers, &peer_connected_event.public_key_pem);
        PeerEvent::new(CONNECTED, list)
    }
}

impl DecryptParser<PeerConnectedEvent> for PeerConnectedEvent {
    fn parse(message: &Vec<u8>, private_key_pem: &Vec<u8>, passphrase: &str) -> PeerConnectedEvent {
        let uid_data = parse_uid(message);
        let public_key_data = parse_public_pem(message, uid_data.end);
        let address_data = parse_address(message, public_key_data.end, private_key_pem, passphrase);
        let peers_data = parse_peers(message, address_data.end, private_key_pem, passphrase);
        PeerConnectedEvent {
            uid: uid_data.value,
            addr: address_data.value,
            peers: peers_data.value,
            public_key_pem: public_key_data.value,
        }
    }
}
