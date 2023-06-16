use std::{fmt::Display, net::SocketAddr};

use crate::{logger::Logger, Content, PeerError, PeerInternalEvent, PeerMessage, PeerSocket};

use super::Response;

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Response")
            .field("event", &self.message.event)
            .field("to", &self.to)
            .finish()
    }
}

impl Response {
    pub(crate) fn new<I>(
        info: I,
        event: PeerInternalEvent,
        public_key: Option<Vec<u8>>,
        to: String,
        address: SocketAddr,
    ) -> Result<Self, PeerError>
    where
        I: Content,
    {
        PeerMessage::new(info, event, public_key).and_then(|message| {
            Ok(Self {
                message,
                to,
                address,
            })
        })
    }

    pub(crate) fn run(&self, socket: &PeerSocket) -> Result<(), PeerError> {
        Logger::info(format!("Execute response {self}").as_str());
        socket.send_at(self.message.clone(), self.address)
    }
}
