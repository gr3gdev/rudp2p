use std::{
    collections::HashMap,
    fmt::Display,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

mod response;

use crate::{
    logger::Logger,
    utils::{find_peer_by_address, find_peer_by_id, is_new_peer},
    Info, PeerConnectedInfo, PeerConnectingInfo, PeerDisconnectedInfo, PeerDisconnectingInfo,
    PeerError, PeerEvent, PeerInternalEvent, PeerMessage, PeerSocket, PeerTypeEvent, RemotePeer,
    Response,
};

impl Display for PeerEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PeerEvent")
            .field("type", &self.type_event)
            .field("from", &self.from)
            .field("to", &self.to)
            .finish()
    }
}

impl PeerEvent {
    pub(crate) fn connecting_to(
        peer_uid: &String,
        socket: &PeerSocket,
        remote: SocketAddr,
    ) -> Result<Response, PeerError> {
        Response::new(
            PeerConnectingInfo {
                uid: peer_uid.clone(),
                public_key: socket.public_key_pem.clone(),
            },
            PeerInternalEvent::CONNECTING,
            None,
            "unknown".to_string(),
            remote,
        )
    }

    pub(crate) fn disconnecting_to(
        peer_uid: &String,
        peers: Arc<Mutex<HashMap<String, RemotePeer>>>,
        remote_uid: &String,
    ) -> Result<Response, PeerError> {
        let guard_peers = peers.lock().unwrap();
        find_peer_by_id(guard_peers, remote_uid).and_then(|remote| {
            Response::new(
                PeerDisconnectingInfo {
                    uid: peer_uid.clone(),
                },
                PeerInternalEvent::DISCONNECTING,
                None,
                remote.uid,
                remote.address,
            )
        })
    }

    pub(crate) fn disconnecting_to_all(
        peer_uid: &String,
        socket: &PeerSocket,
    ) -> Result<(), PeerError> {
        PeerMessage::new(
            PeerDisconnectingInfo {
                uid: peer_uid.clone(),
            },
            PeerInternalEvent::DISCONNECTING,
            None,
        )
        .and_then(|message| {
            socket.send_at_all(message);
            Ok(())
        })
    }

    pub(crate) fn parse_response(
        peer_uid: &String,
        message: &PeerMessage,
        peers: Arc<Mutex<HashMap<String, RemotePeer>>>,
        socket: PeerSocket,
        blacklist: &Vec<String>,
        sender: SocketAddr,
    ) -> Result<(Option<PeerEvent>, Vec<Response>), PeerError> {
        match message.event {
            PeerInternalEvent::CONNECTING => {
                PeerConnectingInfo::parse(message, None).and_then(|connecting_info| {
                    let sender_uid = connecting_info.uid;
                    let mut responses = Vec::new();
                    if (blacklist.is_empty() || !blacklist.contains(&sender_uid))
                        && is_new_peer(peers.lock().unwrap(), &sender_uid)
                    {
                        let mut remote_addresses = HashMap::new();
                        for remote in peers.lock().unwrap().iter() {
                            remote_addresses.insert(remote.0.clone(), remote.1.address);
                        }
                        // Send connected with the list of peers to the peer connecting
                        Response::new(
                            PeerConnectedInfo {
                                uid: peer_uid.clone(),
                                remote_peers: remote_addresses,
                            },
                            PeerInternalEvent::CONNECTED,
                            Some(connecting_info.public_key),
                            sender_uid.clone(),
                            sender,
                        )
                        .and_then(|res| {
                            responses.push(res);
                            // Send connecting to the remote peer connected
                            Response::new(
                                PeerConnectingInfo {
                                    uid: peer_uid.clone(),
                                    public_key: socket.public_key_pem.clone(),
                                },
                                PeerInternalEvent::CONNECTING,
                                None,
                                sender_uid.clone(),
                                sender,
                            )
                            .and_then(|res| {
                                responses.push(res);
                                Ok((None, responses))
                            })
                        })
                    } else {
                        Ok((None, responses))
                    }
                })
            }
            PeerInternalEvent::CONNECTED => {
                PeerConnectedInfo::parse(message, Some(socket.private_key.clone())).and_then(
                    |connected_info| {
                        let mut responses = Vec::new();
                        // Send connecting to the remote peers of the peer connected
                        for peer in connected_info.remote_peers {
                            if peer_uid != &peer.0 {
                                if is_new_peer(peers.lock().unwrap(), &peer.0) {
                                    Response::new(
                                        PeerConnectingInfo {
                                            uid: peer_uid.clone(),
                                            public_key: socket.public_key_pem.clone(),
                                        },
                                        PeerInternalEvent::CONNECTING,
                                        None,
                                        peer.0,
                                        peer.1,
                                    )
                                    .and_then(|res| {
                                        responses.push(res);
                                        Ok(())
                                    })
                                    .unwrap_or_else(|e| Logger::error(e));
                                } else {
                                    Logger::warn(
                                        format!("Peer already added : {}", peer.0).as_str(),
                                    );
                                }
                            } else {
                                Logger::warn("Cannot connected with me");
                            }
                        }
                        Ok((
                            Some(PeerEvent {
                                type_event: PeerTypeEvent::CONNECTED,
                                to: peer_uid.clone(),
                                from: connected_info.uid,
                                socket: socket.clone(),
                                data: vec![],
                            }),
                            responses,
                        ))
                    },
                )
            }
            PeerInternalEvent::MESSAGE => find_peer_by_address(peers.lock().unwrap(), sender)
                .and_then(|remote_peer| {
                    Ok((
                        Some(PeerEvent {
                            type_event: PeerTypeEvent::MESSAGE,
                            to: peer_uid.clone(),
                            from: remote_peer,
                            socket: socket.clone(),
                            data: message.content.clone(),
                        }),
                        vec![],
                    ))
                }),
            PeerInternalEvent::DISCONNECTING => {
                PeerDisconnectingInfo::parse(message, None).and_then(|disconnecting_info| {
                    let mut responses = Vec::new();
                    // Send disconnected to the peer disconnecting
                    Response::new(
                        PeerDisconnectedInfo {
                            uid: peer_uid.clone(),
                        },
                        PeerInternalEvent::DISCONNECTED,
                        None,
                        disconnecting_info.uid.clone(),
                        sender,
                    )
                    .and_then(|res| {
                        responses.push(res);
                        Response::new(
                            PeerDisconnectingInfo {
                                uid: peer_uid.clone(),
                            },
                            PeerInternalEvent::DISCONNECTING,
                            None,
                            disconnecting_info.uid.clone(),
                            sender,
                        )
                        .and_then(|res| {
                            responses.push(res);
                            Ok((None, responses))
                        })
                    })
                })
            }
            PeerInternalEvent::DISCONNECTED => {
                PeerDisconnectedInfo::parse(message, None).and_then(|disconnected_info| {
                    Ok((
                        Some(PeerEvent {
                            type_event: PeerTypeEvent::DISCONNECTED,
                            to: peer_uid.clone(),
                            from: disconnected_info.uid,
                            socket: socket.clone(),
                            data: vec![],
                        }),
                        vec![],
                    ))
                })
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::{
        collections::HashMap,
        net::{IpAddr, SocketAddr, UdpSocket},
        sync::{Arc, Mutex},
    };

    use crate::{
        Content, PeerConnectedInfo, PeerConnectingInfo, PeerDisconnectedInfo,
        PeerDisconnectingInfo, PeerEvent, PeerInternalEvent, PeerMessage, PeerSocket,
        PeerTypeEvent, Response,
    };

    fn init_address(port: u16) -> SocketAddr {
        let addr = "127.0.0.1".parse::<IpAddr>().unwrap();
        SocketAddr::new(addr, port)
    }

    fn init_socket(port: u16) -> PeerSocket {
        let socket_addr = init_address(port);
        let socket = UdpSocket::bind(socket_addr).unwrap();
        PeerSocket::new(socket).unwrap()
    }

    fn parse_response<I>(
        local_peer: String,
        local_socket: PeerSocket,
        remote_socket: PeerSocket,
        info: I,
        event: PeerInternalEvent,
    ) -> (Option<PeerEvent>, Vec<Response>)
    where
        I: Content,
    {
        PeerEvent::parse_response(
            &local_peer,
            &PeerMessage::new(info, event, Some(local_socket.public_key_pem.clone())).unwrap(),
            Arc::new(Mutex::new(HashMap::new())),
            local_socket,
            &vec![],
            remote_socket.addr().unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn test_parse_response_connecting() {
        let local_socket = init_socket(9001);
        let remote_socket = init_socket(9002);
        let info = PeerConnectingInfo {
            uid: "Peer2".to_string(),
            public_key: remote_socket.public_key_pem.clone(),
        };
        let (event, responses) = parse_response(
            "Peer1".to_string(),
            local_socket,
            remote_socket.clone(),
            info,
            PeerInternalEvent::CONNECTING,
        );
        assert!(event.is_none());
        assert_eq!(2, responses.len());
        let r1 = responses.get(0).unwrap();
        assert_eq!(PeerInternalEvent::CONNECTED, r1.message.event);
        assert_eq!("Peer2", r1.to);
        assert_eq!(remote_socket.addr().unwrap(), r1.address);
        let r2 = responses.get(1).unwrap();
        assert_eq!(PeerInternalEvent::CONNECTING, r2.message.event);
        assert_eq!("Peer2", r2.to);
        assert_eq!(remote_socket.addr().unwrap(), r2.address);
    }

    #[test]
    fn test_parse_response_disconnecting() {
        let local_socket = init_socket(9003);
        let remote_socket = init_socket(9004);
        let info = PeerDisconnectingInfo {
            uid: "Peer2".to_string(),
        };
        let (event, responses) = parse_response(
            "Peer1".to_string(),
            local_socket,
            remote_socket.clone(),
            info,
            PeerInternalEvent::DISCONNECTING,
        );
        assert!(event.is_none());
        assert_eq!(2, responses.len());
        let r1 = responses.get(0).unwrap();
        assert_eq!(PeerInternalEvent::DISCONNECTED, r1.message.event);
        assert_eq!("Peer2", r1.to);
        assert_eq!(remote_socket.addr().unwrap(), r1.address);
        let r2 = responses.get(1).unwrap();
        assert_eq!(PeerInternalEvent::DISCONNECTING, r2.message.event);
        assert_eq!("Peer2", r2.to);
        assert_eq!(remote_socket.addr().unwrap(), r2.address);
    }

    #[test]
    fn test_parse_response_connected_without_remote_peers() {
        let local_socket = init_socket(9005);
        let remote_socket = init_socket(9006);
        let info = PeerConnectedInfo {
            uid: "Peer2".to_string(),
            remote_peers: HashMap::new(),
        };
        let (event, responses) = parse_response(
            "Peer1".to_string(),
            local_socket,
            remote_socket,
            info,
            PeerInternalEvent::CONNECTED,
        );
        assert!(event.is_some());
        let event: PeerEvent = event.unwrap();
        assert_eq!(PeerTypeEvent::CONNECTED, event.type_event);
        assert_eq!("Peer2", event.from);
        assert_eq!("Peer1", event.to);
        assert_eq!(0, responses.len());
    }

    #[test]
    fn test_parse_response_connected_with_remote_peers() {
        let local_socket = init_socket(9007);
        let remote_socket = init_socket(9008);
        let mut remote_peers = HashMap::new();
        let addr3: SocketAddr = init_address(9009);
        remote_peers.insert("Peer3".to_string(), addr3);
        let info = PeerConnectedInfo {
            uid: "Peer2".to_string(),
            remote_peers,
        };
        let (event, responses) = parse_response(
            "Peer1".to_string(),
            local_socket,
            remote_socket.clone(),
            info,
            PeerInternalEvent::CONNECTED,
        );
        assert!(event.is_some());
        let event: PeerEvent = event.unwrap();
        assert_eq!(PeerTypeEvent::CONNECTED, event.type_event);
        assert_eq!("Peer2", event.from);
        assert_eq!("Peer1", event.to);
        assert_eq!(1, responses.len());
        let r1 = responses.get(0).unwrap();
        assert_eq!(PeerInternalEvent::CONNECTING, r1.message.event);
        assert_eq!("Peer3", r1.to);
        assert_eq!(addr3, r1.address);
    }

    #[test]
    fn test_parse_response_disconnected() {
        let local_socket = init_socket(9010);
        let remote_socket = init_socket(9011);
        let info = PeerDisconnectedInfo {
            uid: "Peer2".to_string(),
        };
        let (event, responses) = parse_response(
            "Peer1".to_string(),
            local_socket,
            remote_socket.clone(),
            info,
            PeerInternalEvent::DISCONNECTED,
        );
        assert!(event.is_some());
        let event: PeerEvent = event.unwrap();
        assert_eq!(PeerTypeEvent::DISCONNECTED, event.type_event);
        assert_eq!("Peer2", event.from);
        assert_eq!("Peer1", event.to);
        assert_eq!(0, responses.len());
    }
}
