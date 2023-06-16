use std::{
    collections::HashMap,
    fmt::Debug,
    net::{SocketAddr, UdpSocket},
};

use crate::{
    logger::Logger, Message, PeerError, PeerInternalEvent, PeerMessage, PeerMessageInfo,
    PeerSocket, Socket,
};
use openssl::rsa::Rsa;

impl Socket for PeerSocket {
    fn send_to<M>(&self, message: M, to: String) -> Result<(), PeerError>
    where
        M: Message,
    {
        if let Some(to_peer) = self.peers.get(&to) {
            PeerMessage::new(
                PeerMessageInfo {
                    content: message.content(),
                },
                PeerInternalEvent::MESSAGE,
                Some(self.public_key_pem.clone()),
            )
            .and_then(|message| self.send_at(message, to_peer.address))
        } else {
            Ok(())
        }
    }

    fn send_all<M>(&self, message: M) -> Result<(), PeerError>
    where
        M: Message,
    {
        PeerMessage::new(
            PeerMessageInfo {
                content: message.content(),
            },
            PeerInternalEvent::MESSAGE,
            Some(self.public_key_pem.clone()),
        )
        .and_then(|message| {
            for peer in &self.peers {
                self.send_at(message.clone(), peer.1.address)
                    .unwrap_or_else(|e| Logger::error(e));
            }
            Ok(())
        })
    }
}

impl Debug for PeerSocket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PeerSocket")
            .field("peers", &self.peers)
            .field("socket", &self.socket)
            .finish()
    }
}

impl Clone for PeerSocket {
    fn clone(&self) -> Self {
        Self {
            peers: self.peers.clone(),
            socket: self.socket.try_clone().unwrap(),
            public_key_pem: self.public_key_pem.clone(),
            private_key: self.private_key.clone(),
        }
    }
}

impl PeerSocket {
    pub(crate) fn new(socket: UdpSocket) -> Result<PeerSocket, PeerError> {
        Rsa::generate(2048)
            .or_else(|e| Err(PeerError::new_ssl("Error to generate RSA keys", e)))
            .and_then(|rsa| {
                rsa.public_key_to_pem()
                    .or_else(|e| Err(PeerError::new_ssl("Error to generate public key", e)))
                    .and_then(|public_key_pem| {
                        Ok(PeerSocket {
                            peers: HashMap::new(),
                            socket,
                            public_key_pem,
                            private_key: rsa,
                        })
                    })
            })
    }

    pub(crate) fn send_at(
        &self,
        message: PeerMessage,
        address: SocketAddr,
    ) -> Result<(), PeerError> {
        self.addr().and_then(|addr| {
            if addr != address {
                for mut m in message.split() {
                    self.send_buffer_to(m.encode().as_slice(), address)
                        .unwrap_or_else(|e| {
                            Logger::error(e);
                            0 as usize
                        });
                }
            } else {
                Logger::warn("Cannot send a message to me");
            }
            Ok(())
        })
    }

    pub(crate) fn send_at_all(&self, message: PeerMessage) {
        for peer in &self.peers {
            self.send_at(message.clone(), peer.1.address)
                .unwrap_or_else(|e| {
                    Logger::error(e);
                });
        }
    }

    pub(crate) fn send_buffer_to(&self, buf: &[u8], addr: SocketAddr) -> Result<usize, PeerError> {
        self.socket
            .send_to(buf, addr)
            .or_else(|e| Err(PeerError::new_io("Error to send a message", e)))
    }

    pub(crate) fn addr(&self) -> Result<SocketAddr, PeerError> {
        self.socket
            .local_addr()
            .or_else(|e| Err(PeerError::new_io("Error to get local addresse", e)))
    }
}
