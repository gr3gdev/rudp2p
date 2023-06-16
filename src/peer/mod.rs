use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::SystemTime,
};

use crate::{
    logger::Logger, Message, Peer, PeerError, PeerEvent, PeerMessage,
    PeerServer, Socket,
};

impl Peer {
    /// Start a new Peer.
    ///
    /// Parameters :
    /// - port : the port number, for example 8000
    /// - listener : the event's listener. This closure take 1 parameter (PeerEvent).
    /// - uid : optional unique identifier
    pub fn start<L>(port: u16, mut listener: L, uid: Option<String>) -> Result<Peer, PeerError>
    where
        L: FnMut(PeerEvent) + Send + Sync + 'static,
    {
        let blacklist = Arc::new(Mutex::new(Vec::new()));
        let shared_blacklist = Arc::clone(&blacklist);
        PeerServer::new(port).and_then(|mut server| {
            let mut cache_messages: HashMap<String, Vec<PeerMessage>> = HashMap::new();
            let mut default_uid = "P".to_owned();
            default_uid.push_str(
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
                    .to_string()
                    .as_str(),
            );
            let uid = uid.unwrap_or(default_uid);
            let peer_uid = uid.clone();
            let socket = server.socket.clone();
            let peers = Arc::clone(&server.peers);
            server.start(move |data, sender| {
                PeerMessage::parse(data)
                    .and_then(|message| {
                        // Put the message in cache
                        let mut list_messages = Vec::new();
                        if let Some(messages) = cache_messages.get(&message.uid) {
                            list_messages.append(&mut messages.clone());
                        }
                        list_messages.push(message.clone());
                        cache_messages.insert(message.uid.clone(), list_messages);
                        // Check if message is completed
                        PeerMessage::merge(cache_messages.get(&message.uid).unwrap()).and_then(
                            |tmp_complete_message| {
                                if tmp_complete_message.is_completed() {
                                    // Remove message from cache
                                    cache_messages.remove(&message.uid);
                                    let guard_blacklist = shared_blacklist.lock().unwrap();
                                    PeerEvent::parse_response(
                                        &peer_uid,
                                        &tmp_complete_message,
                                        peers.clone(),
                                        socket.clone(),
                                        &guard_blacklist,
                                        sender,
                                    )
                                    .and_then(
                                        |(response_event, responses)| {
                                            if let Some(event) = response_event {
                                                let sender_uid = &event.from;
                                                if !guard_blacklist.contains(sender_uid) {
                                                    // Send event
                                                    listener(event);
                                                }
                                            }
                                            responses
                                                .into_iter()
                                                .map(|res| res.run(&socket.clone()))
                                                .for_each(|r| {
                                                    r.unwrap_or_else(|e| Logger::error(e))
                                                });
                                            Ok(())
                                        },
                                    )
                                } else {
                                    Ok(())
                                }
                            },
                        )
                    })
                    .unwrap_or_else(|e| Logger::error(e));
            });
            Ok(Peer {
                uid,
                server,
                blacklist,
            })
        })
    }

    /// Block a list of peers.
    pub fn block_peers(&self, peers: Vec<String>) {
        let mut blacklist = self.blacklist.lock().unwrap();
        for peer in peers {
            blacklist.push(peer);
        }
    }

    /// The peer address.
    pub fn addr(&self) -> Result<SocketAddr, PeerError> {
        self.server.socket.addr()
    }

    /// True if peer is alive.
    pub fn alive(&self) -> bool {
        self.server.is_alive()
    }

    /// Close the Peer and send a disconnecting event to the others peers.
    pub fn close(&self) {
        self.disconnect_to_all()
            .and_then(|_| self.server.stop())
            .unwrap_or_else(|e| Logger::error(e));
    }

    /// Connect with a remote address.
    pub fn connect(&self, remote: SocketAddr) -> Result<(), PeerError> {
        if !self.alive() {
            Err(PeerError::new("The peer is not started"))
        } else {
            PeerEvent::connecting_to(&self.uid, &self.server.socket, remote)
                .and_then(|res| res.run(&self.server.socket))
        }
    }

    /// Disconnect with a peer.
    pub fn disconnect(&self, remote_uid: String) -> Result<(), PeerError> {
        if self.alive() {
            PeerEvent::disconnecting_to(&self.uid, Arc::clone(&self.server.peers), &remote_uid)
                .and_then(|res| res.run(&self.server.socket))
        } else {
            Err(PeerError::new("The peer is not started"))
        }
    }

    /// Disconnect with all peers.
    pub fn disconnect_to_all(&self) -> Result<(), PeerError> {
        if self.alive() {
            let socket = &self.server.socket;
            println!("peers {:?}", socket.peers);
            if !socket.peers.is_empty() {
                PeerEvent::disconnecting_to_all(&self.uid, &socket)
            } else {
                Logger::warn("The peer has not other peers, not disconnected");
                Ok(())
            }
        } else {
            Err(PeerError::new(
                "The peer is not started, cannot disconnected",
            ))
        }
    }

    /// Send a message to all peers.
    pub fn send_all<M>(&self, message: M) -> Result<(), PeerError>
    where
        M: Message,
    {
        self.server.socket.send_all(message)
    }

    /// Send a message to a specific peer.
    pub fn send_to<M>(&self, message: M, remote_uid: String) -> Result<(), PeerError>
    where
        M: Message,
    {
        self.server.socket.send_to(message, remote_uid)
    }
}
