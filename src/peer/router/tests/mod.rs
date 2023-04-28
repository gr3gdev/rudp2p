#[allow(unused_imports)]
use std::cell::RefCell;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::net::SocketAddr;
#[allow(unused_imports)]
use std::sync::{Arc, Mutex};

#[allow(unused_imports)]
use openssl::rsa::Rsa;
#[allow(unused_imports)]
use openssl::symm::Cipher;

#[allow(unused_imports)]
use crate::peer::event::PeerEvent;
use crate::peer::event::ResponseEvent;
use crate::peer::message::PeerMessage;
#[allow(unused_imports)]
use crate::peer::RemotePeer;
#[allow(unused_imports)]
use crate::peer::router::data::{DecodeData, Decoder, Encoder, RouteData};
#[allow(unused_imports)]
use crate::peer::router::event::{RouteEvent, RouterEvent};
#[allow(unused_imports)]
use crate::peer::router::Router;
use crate::peer::SimplePeer;

#[allow(dead_code)]
fn encode_decode(uid: String, peers: Vec<SimplePeer>) {
    let rsa = Rsa::generate(1024).unwrap();
    let passphrase = "P@ssW0rd!";
    let private_key_pem = rsa.private_key_to_pem_passphrase(Cipher::aes_256_cbc(), passphrase.as_bytes()).unwrap();
    let public_key_pem = rsa.public_key_to_pem().unwrap();

    let mut data = Vec::new();
    data.append(&mut RouteData::Uid(uid.clone()).encode());
    data.append(&mut RouteData::PublicKey(public_key_pem.clone()).encode());
    data.append(&mut RouteData::Peers(peers.clone(), public_key_pem.clone()).encode());

    let decode = RouteData::decode(data, vec![
        DecodeData::Uid,
        DecodeData::PublicKey,
        DecodeData::Peers(private_key_pem, passphrase.to_string()),
    ]);
    assert_eq!(3, decode.len());
    let RouteData::Uid(uid_decoded) = &decode[0] else { panic!("Uid not decoded") };
    assert_eq!(&uid, uid_decoded);
    let RouteData::PublicKey(pk_pem) = &decode[1] else { panic!("PublicKey not decoded") };
    assert_eq!(&public_key_pem.clone(), pk_pem);
    let RouteData::Peers(remote_peers, _) = &decode[2] else { panic!("Peers not decoded") };
    assert_eq!(peers.len(), remote_peers.len());
    if !peers.is_empty() {
        for index in 0..peers.len() {
            let peer = &peers[index];
            let remote_peer = &remote_peers[index];
            assert_eq!(peer.uid, remote_peer.uid);
            assert_eq!(peer.addr, remote_peer.addr);
        }
    }
}

#[test]
fn test_encode_decode_data_without_peers() {
    encode_decode("A0".to_string(), Vec::new());
}

#[test]
fn test_encode_decode_data_with_peers() {
    encode_decode("B0".to_string(), vec![
        SimplePeer {
            uid: "P1".to_string(),
            addr: "127.0.0.1:9001".parse::<SocketAddr>().unwrap(),
        },
        SimplePeer {
            uid: "P2".to_string(),
            addr: "127.0.0.1:9002".parse::<SocketAddr>().unwrap(),
        },
    ]);
}

#[allow(dead_code)]
fn init_router(uid: &str,
               peers: Vec<RemotePeer>,
               on_message: Option<Box<dyn FnMut(&PeerMessage, &String) -> () + Send + Sync>>,
               on_connected: Option<Box<dyn FnMut(&String) -> () + Send + Sync>>,
               on_disconnected: Option<Box<dyn FnMut(&String) -> () + Send + Sync>>) -> Router {
    let rsa = Rsa::generate(1024).unwrap();
    let passphrase = "P@ssW0rd!";
    let private_key_pem = rsa.private_key_to_pem_passphrase(Cipher::aes_256_cbc(), passphrase.as_bytes()).unwrap();
    let public_key_pem = rsa.public_key_to_pem().unwrap();
    let mut map_peers = HashMap::new();
    for p in peers {
        map_peers.insert(p.simple_peer.uid.clone(), p.clone());
    }
    Router {
        peer_uid: uid.to_string(),
        passphrase,
        private_key_pem,
        public_key_pem: public_key_pem.clone(),
        shared_peers: Arc::new(Mutex::new(map_peers)),
        shared_message: Arc::new(Mutex::new(RefCell::new(on_message))),
        shared_connected: Arc::new(Mutex::new(RefCell::new(on_connected))),
        shared_disconnected: Arc::new(Mutex::new(RefCell::new(on_disconnected))),
        complete_event: Default::default(),
    }
}

#[allow(dead_code)]
fn responses_event(peer_event: PeerEvent, router: &Router, remote_addr: SocketAddr,
                   mut valid_peers: Box<dyn FnMut(Vec<String>) -> () + Send + Sync>) -> Option<Vec<ResponseEvent>> {
    let router_event = RouterEvent::find_by_code(peer_event.code);
    let responses = router_event.responses_event(peer_event, remote_addr, router);
    let peers = router.shared_peers.lock().unwrap().keys().cloned().collect();
    valid_peers(peers);
    responses
}

#[test]
fn test_connecting_responses_event() {
    let rsa = Rsa::generate(1024).unwrap();
    let connecting_event = RouterEvent::Connecting.new_event(vec![
        RouteData::Uid("P3".to_string()),
        RouteData::PublicKey(rsa.public_key_to_pem().unwrap()),
    ]);
    let p1_rsa = Rsa::generate(1024).unwrap();
    let p2_rsa = Rsa::generate(1024).unwrap();
    let router = init_router(
        "P0",
        vec![
            RemotePeer {
                uid: "P1".to_string(),
                addr: "127.0.0.1:9001".parse::<SocketAddr>().unwrap(),
                public_key_pem: p1_rsa.public_key_to_pem().unwrap(),
            },
            RemotePeer {
                uid: "P2".to_string(),
                addr: "127.0.0.1:9002".parse::<SocketAddr>().unwrap(),
                public_key_pem: p2_rsa.public_key_to_pem().unwrap(),
            },
        ],
        None, None, None,
    );
    let responses = responses_event(connecting_event, &router,
                                    "127.0.0.1:9003".parse::<SocketAddr>().unwrap(),
                                    Box::new(|p0_peers| {
                                        assert!(p0_peers.contains(&"P3".to_string()));
                                    }));
    assert!(responses.is_some());
    let responses = responses.unwrap();
    assert_eq!(1, responses.len());
    assert_eq!(2, responses[0].peer_event.code);
}

#[test]
fn test_connected_responses_event() {
    let rsa = Rsa::generate(1024).unwrap();
    let p0_public_key_pem = rsa.public_key_to_pem().unwrap();
    let router = init_router(
        "P3",
        vec![],
        None, None, None,
    );
    let connected_event = RouterEvent::Connected.new_event(vec![
        RouteData::Uid("P0".to_string()),
        RouteData::PublicKey(p0_public_key_pem.clone()),
        RouteData::Peers(vec![
            SimplePeer {
                uid: "P1".to_string(),
                addr: "127.0.0.1:9001".parse::<SocketAddr>().unwrap(),
            },
            SimplePeer {
                uid: "P2".to_string(),
                addr: "127.0.0.1:9002".parse::<SocketAddr>().unwrap(),
            },
        ], router.public_key_pem.clone()),
    ]);
    let responses = responses_event(connected_event, &router,
                                    "127.0.0.1:9000".parse::<SocketAddr>().unwrap(),
                                    Box::new(|p3_peers| {
                                        assert!(p3_peers.contains(&"P0".to_string()));
                                    }));
    assert!(responses.is_some());
    let responses = responses.unwrap();
    assert_eq!(2, responses.len());
    assert_eq!(1, responses[0].peer_event.code);
    assert_eq!("127.0.0.1:9001".parse::<SocketAddr>().unwrap(), responses[0].address);
    assert_eq!(1, responses[1].peer_event.code);
    assert_eq!("127.0.0.1:9002".parse::<SocketAddr>().unwrap(), responses[1].address);
}

#[test]
fn test_disconnecting_responses_event() {
    let p1_rsa = Rsa::generate(1024).unwrap();
    let p2_rsa = Rsa::generate(1024).unwrap();
    let p3_rsa = Rsa::generate(1024).unwrap();
    let router = init_router(
        "P0",
        vec![
            RemotePeer {
                uid: "P1".to_string(),
                addr: "127.0.0.1:9001".parse::<SocketAddr>().unwrap(),
                public_key_pem: p1_rsa.public_key_to_pem().unwrap(),
            },
            RemotePeer {
                uid: "P2".to_string(),
                addr: "127.0.0.1:9002".parse::<SocketAddr>().unwrap(),
                public_key_pem: p2_rsa.public_key_to_pem().unwrap(),
            },
            RemotePeer {
                uid: "P3".to_string(),
                addr: "127.0.0.1:9003".parse::<SocketAddr>().unwrap(),
                public_key_pem: p3_rsa.public_key_to_pem().unwrap(),
            },
        ],
        None, None, None,
    );
    let disconnecting_event = RouterEvent::Disconnecting.new_event(vec![
        RouteData::Uid("P3".to_string()),
    ]);
    let responses = responses_event(disconnecting_event, &router,
                                    "127.0.0.1:9003".parse::<SocketAddr>().unwrap(),
                                    Box::new(|p0_peers| {
                                        assert!(p0_peers.contains(&"P1".to_string()));
                                        assert!(p0_peers.contains(&"P2".to_string()));
                                        assert!(!p0_peers.contains(&"P3".to_string()));
                                    }));
    assert!(responses.is_some());
    let responses = responses.unwrap();
    assert_eq!(1, responses.len());
    assert_eq!(4, responses[0].peer_event.code);
    assert_eq!("127.0.0.1:9003".parse::<SocketAddr>().unwrap(), responses[0].address);
}

#[test]
fn test_disconnected_responses_event() {
    let p1_rsa = Rsa::generate(1024).unwrap();
    let p2_rsa = Rsa::generate(1024).unwrap();
    let p0_rsa = Rsa::generate(1024).unwrap();
    let router = init_router(
        "P3",
        vec![
            RemotePeer {
                uid: "P1".to_string(),
                addr: "127.0.0.1:9001".parse::<SocketAddr>().unwrap(),
                public_key_pem: p1_rsa.public_key_to_pem().unwrap(),
            },
            RemotePeer {
                uid: "P2".to_string(),
                addr: "127.0.0.1:9002".parse::<SocketAddr>().unwrap(),
                public_key_pem: p2_rsa.public_key_to_pem().unwrap(),
            },
            RemotePeer {
                uid: "P0".to_string(),
                addr: "127.0.0.1:9000".parse::<SocketAddr>().unwrap(),
                public_key_pem: p0_rsa.public_key_to_pem().unwrap(),
            },
        ],
        None, None, None,
    );
    let disconnecting_event = RouterEvent::Disconnected.new_event(vec![
        RouteData::Uid("P0".to_string()),
    ]);
    let responses = responses_event(disconnecting_event, &router,
                                    "127.0.0.1:9000".parse::<SocketAddr>().unwrap(),
                                    Box::new(|p0_peers| {
                                        assert!(p0_peers.contains(&"P1".to_string()));
                                        assert!(p0_peers.contains(&"P2".to_string()));
                                        assert!(!p0_peers.contains(&"P0".to_string()));
                                    }));
    assert!(responses.is_none());
}
