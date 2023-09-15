use std::{fmt::Debug, net::SocketAddr};

#[derive(Clone)]
pub struct Connected {
    pub uid: String,
    pub from: String,
    pub address: SocketAddr,
    pub(crate) public_key: Vec<u8>,
}

impl Debug for Connected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConnectedEvent")
            .field("uid", &self.uid)
            .field("from", &self.from)
            .field("address", &self.address)
            .field("public_key", &self.public_key.len())
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct Disconnected {
    pub uid: String,
    pub from: String,
    pub address: SocketAddr,
}

#[derive(Clone)]
pub struct Message {
    pub uid: String,
    pub from: String,
    pub content: Vec<u8>,
}

impl Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Message")
            .field("uid", &self.uid)
            .field("from", &self.from)
            .field("content", &self.content.len())
            .finish()
    }
}
