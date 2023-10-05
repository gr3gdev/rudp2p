use crate::peer::RemotePeer;
use std::fmt::Debug;

#[derive(Clone)]
pub struct Message {
    pub from: RemotePeer,
    pub content: Vec<u8>,
}

impl Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Message")
            .field("content", &self.content.len())
            .finish()
    }
}
