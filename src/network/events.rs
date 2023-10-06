use crate::peer::RemotePeer;
use std::fmt::Debug;

/// # Message
/// 
/// Message receive from another Peer.
#[derive(Clone)]
pub struct Message {
    /// The remote Peer.
    pub from: RemotePeer,
    /// The content of the message.
    pub content: Vec<u8>,
}

impl Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Message")
            .field("content", &self.content.len())
            .finish()
    }
}
