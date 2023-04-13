use crate::peer::event::common::{parse_uid, Parser};

pub(crate) mod router;

// STRUCT

pub struct PeerMessageEvent {
    /// Uid of the peer connected.
    pub uid: String,
    /// Content of the message.
    pub content: Vec<u8>,
}

// IMPL

impl Parser<PeerMessageEvent> for PeerMessageEvent {
    fn parse(message: &Vec<u8>) -> PeerMessageEvent {
        let uid_data = parse_uid(message);
        PeerMessageEvent {
            uid: uid_data.value,
            content: message[uid_data.end..message.len()].to_vec().clone(),
        }
    }
}
