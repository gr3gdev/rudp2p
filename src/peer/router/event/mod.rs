use std::fmt::{Display, Formatter};
use std::net::SocketAddr;

use crate::logger::Logger;
use crate::peer::{RemotePeer, SimplePeer};
use crate::peer::event::{PeerEvent, ResponseEvent};
use crate::peer::router::data::{DecodeData, Decoder, Encoder, RouteData};
use crate::peer::router::Router;

pub(crate) trait RouteEvent {
    fn responses_event(&self, peer_event: PeerEvent, remote_addr: SocketAddr, router: &Router) -> Option<Vec<ResponseEvent>>;
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

    fn add_new_remote_peer(remote_addr: SocketAddr, router: &Router, data: &Vec<RouteData>) {
        let guard_connected = router.shared_connected.lock().expect("Unable to get shared connected");
        let mut peers = router.shared_peers.lock().expect("Unable to get shared peers");
        let RouteData::Uid(uid) = &data[0] else { panic!("UID not found !") };
        let RouteData::PublicKey(public_key_pem) = &data[1] else { panic!("Public KEY not found !") };
        if !RemotePeer::exists(&peers, &uid) {
            Logger::info(format!("[{}] New peer : {}", router.peer_uid, uid));
            peers.insert(uid.clone(), RemotePeer {
                uid: uid.clone(),
                addr: remote_addr,
                public_key_pem: public_key_pem.clone(),
            });
        }
        if let Some(ref mut connected) = *guard_connected.borrow_mut() {
            connected(&uid.clone());
        };
    }

    fn remove_remote_peer(router: &Router, data: &Vec<RouteData>) {
        let RouteData::Uid(uid) = &data[0] else { panic!("UID not found !") };
        let mut peers = router.shared_peers.lock().unwrap();
        if RemotePeer::exists(&peers, &uid) {
            Logger::info(format!("[{}] Remove peer : {}", router.peer_uid, uid));
            peers.remove(&uid.clone());
        }
        let guard_disconnected = router.shared_disconnected.lock().unwrap();
        if let Some(ref mut disconnected) = *guard_disconnected.borrow_mut() {
            disconnected(&uid.clone());
        };
    }
}

impl RouteEvent for RouterEvent {
    fn responses_event(&self, peer_event: PeerEvent, remote_addr: SocketAddr, router: &Router) -> Option<Vec<ResponseEvent>> {
        Logger::info(format!("[{}] Generate response for {} ...", router, self));
        let message = peer_event.message;
        match self {
            RouterEvent::Disconnecting => {
                let data = RouteData::decode(message, vec![
                    DecodeData::Uid
                ]);
                Self::remove_remote_peer(router, &data);
                let disconnected_event = RouterEvent::Disconnected.new_event(data);
                Some(vec![ResponseEvent {
                    peer_event: disconnected_event,
                    address: remote_addr,
                }])
            }
            RouterEvent::Connecting => {
                let data = RouteData::decode(message, vec![
                    DecodeData::Uid,
                    DecodeData::PublicKey,
                ]);
                let mut simple_peers = Vec::new();
                for p in router.shared_peers.lock().unwrap().values() {
                    simple_peers.push(SimplePeer {
                        uid: p.uid.clone(),
                        addr: p.addr,
                    })
                }
                let RouteData::PublicKey(remote_public_key_pem) = &data[1] else { panic!("Public KEY not found !") };
                let connected_event = RouterEvent::Connected.new_event(vec![
                    RouteData::Uid(router.peer_uid.clone()),
                    RouteData::PublicKey(router.public_key_pem.clone()),
                    RouteData::Peers(simple_peers, remote_public_key_pem.clone()),
                ]);
                Self::add_new_remote_peer(remote_addr, router, &data);
                Some(vec![ResponseEvent {
                    peer_event: connected_event,
                    address: remote_addr,
                }])
            }
            RouterEvent::Connected => {
                let data = RouteData::decode(message, vec![
                    DecodeData::Uid,
                    DecodeData::PublicKey,
                    DecodeData::Peers(router.private_key_pem.clone(), router.passphrase.to_string()),
                ]);
                Self::add_new_remote_peer(remote_addr, router, &data);
                let RouteData::Peers(remote_peers, remote_public_key) = &data[2] else { panic!("Peers not found !") };
                if !remote_peers.is_empty() {
                    // Share peers
                    let mut connecting_peers = Vec::new();
                    for remote in remote_peers {
                        let connecting_event = RouterEvent::Connecting.new_event(vec![
                            RouteData::Uid(remote.uid.clone()),
                            RouteData::PublicKey(remote_public_key.clone()),
                        ]);
                        connecting_peers.push(ResponseEvent {
                            peer_event: connecting_event,
                            address: remote.addr,
                        });
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
                let data = RouteData::decode(message, vec![
                    DecodeData::Uid
                ]);
                Self::remove_remote_peer(router, &data);
                None
            }
        }
    }
}

impl Display for RouterEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RouterEvent::Disconnecting => write!(f, "Disconnecting"),
            RouterEvent::Connecting => write!(f, "Connecting"),
            RouterEvent::Connected => write!(f, "Connected"),
            RouterEvent::Message => write!(f, "Message"),
            RouterEvent::Disconnected => write!(f, "Disconnected"),
        }
    }
}
