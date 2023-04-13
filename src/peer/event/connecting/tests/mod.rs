#[allow(unused_imports)]
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[allow(unused_imports)]
use openssl::rsa::Rsa;
#[allow(unused_imports)]
use openssl::symm::Cipher;

#[allow(unused_imports)]
use crate::peer::event::common::Parser;
#[allow(unused_imports)]
use crate::peer::event::connecting::PeerConnectingEvent;

#[test]
fn test_parse_event() {
    let rsa = Rsa::generate(1024).unwrap();
    let public_key_pem = rsa.public_key_to_pem().unwrap();
    let event = PeerConnectingEvent::event(PeerConnectingEvent {
        uid: "P0".to_string(),
        public_key_pem: public_key_pem.clone(),
    });
    let data = PeerConnectingEvent::parse(&event.message);
    assert_eq!("P0".to_string(), data.uid);
    assert_eq!(public_key_pem.clone(), data.public_key_pem);
}