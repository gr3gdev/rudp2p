use std::net::SocketAddr;

use crate::peer::event::PeerEvent;
use crate::peer::RemotePeer;
use crate::peer::router::data::{DecodeData, Decoder, Encoder, RouteData};
use crate::peer::router::Router;

pub(crate) trait RouteEvent {
    fn responses_event(&self, peer_event: PeerEvent, remote_addr: SocketAddr, router: &Router) -> Option<Vec<PeerEvent>>;
}

pub(crate) enum RouterEvent {
    Disconnecting,
    Connecting,
    Connected,
    Message,
    Disconnected,
}

impl RouterEvent {
    pub(crate) fn find_by_code(code: u8) -> RouterEvent {
        match code {
            0 => RouterEvent::Disconnecting,
            1 => RouterEvent::Connecting,
            2 => RouterEvent::Connected,
            3 => RouterEvent::Message,
            4 => RouterEvent::Disconnected,
            _ => panic!("Event not found !")
        }
    }

    pub(crate) fn new_event(&self, route_data: Vec<RouteData>) -> PeerEvent {
        let code;
        let mut data = Vec::new();
        match self {
            RouterEvent::Disconnecting => {
                code = 0;
            }
            RouterEvent::Connecting => {
                code = 1;
            }
            RouterEvent::Connected => {
                code = 2;
            }
            RouterEvent::Message => {
                code = 3;
            }
            RouterEvent::Disconnected => {
                code = 4;
            }
        }
        for d in &route_data {
            data.append(&mut d.encode());
        }
        PeerEvent::new(code, data)
    }
}

impl RouteEvent for RouterEvent {
    fn responses_event(&self, peer_event: PeerEvent, remote_addr: SocketAddr, router: &Router) -> Option<Vec<PeerEvent>> {
        let message = peer_event.message;
        match self {
            RouterEvent::Disconnecting => {
                Some(vec![
                    RouterEvent::Disconnected.new_event(
                        RouteData::decode(message, vec![
                            DecodeData::Uid
                        ]))
                ])
            }
            RouterEvent::Connecting => {
                Some(vec![
                    RouterEvent::Connected.new_event(
                        RouteData::decode(message, vec![
                            DecodeData::Uid,
                            DecodeData::PublicKey,
                            DecodeData::Peers(router.private_key_pem.clone(), router.passphrase.to_string()),
                        ]))
                ])
            }
            RouterEvent::Connected => {
                let guard_connected = router.shared_connected.lock().unwrap();
                let mut peers = router.shared_peers.lock().unwrap();
                let data = RouteData::decode(message, vec![
                    DecodeData::Uid,
                    DecodeData::PublicKey,
                    DecodeData::Peers(router.private_key_pem.clone(), router.passphrase.to_string()),
                ]);
                let RouteData::Uid(uid) = &data[0] else { panic!("UID not found !") };
                let RouteData::PublicKey(public_key_pem) = &data[1] else { panic!("Public KEY not found !") };
                let RouteData::Peers(remote_peers, _) = &data[2] else { panic!("Peers not found !") };
                if !RemotePeer::exists(&peers, &uid) {
                    peers.insert(uid.clone(), RemotePeer {
                        uid: uid.clone(),
                        addr: remote_addr,
                        public_key_pem: Some(public_key_pem.clone()),
                    });
                }
                if let Some(ref mut connected) = *guard_connected.borrow_mut() {
                    connected(&uid.clone());
                }
                if !remote_peers.is_empty() {
                    // Share peers
                    let mut connecting_peers = Vec::new();
                    for remote in remote_peers {
                        connecting_peers.push(RouterEvent::Connecting.new_event(
                            vec![
                                RouteData::Uid(remote.uid.clone()),
                                RouteData::PublicKey(remote.public_key_pem.clone().unwrap()),
                            ]
                        ));
                    }
                    Some(connecting_peers)
                } else {
                    None
                }
            }
            RouterEvent::Message => {
                let guard_message = router.shared_message.lock().unwrap();
                let data = RouteData::decode(message, vec![
                    DecodeData::Uid,
                    DecodeData::Message(router.private_key_pem.clone(), router.passphrase.to_string()),
                ]);
                let RouteData::Uid(uid) = &data[0] else { panic!("UID not found !") };
                let RouteData::Message(message, _) = &data[1] else { panic!("Message not found !") };
                if let Some(ref mut message_received) = *guard_message.borrow_mut() {
                    message_received(message, &uid.clone());
                }
                None
            }
            RouterEvent::Disconnected => {
                let guard_disconnected = router.shared_disconnected.lock().unwrap();
                let mut peers = router.shared_peers.lock().unwrap();
                let data = RouteData::decode(message, vec![
                    DecodeData::Uid
                ]);
                let RouteData::Uid(uid) = &data[0] else { panic!("UID not found !") };
                if RemotePeer::exists(&peers, &uid) {
                    peers.remove(&uid.clone());
                }
                if let Some(ref mut disconnected) = *guard_disconnected.borrow_mut() {
                    disconnected(&uid.clone());
                }
                None
            }
        }
    }
}
