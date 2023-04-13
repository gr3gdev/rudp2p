#[allow(unused_imports)]
use openssl::rsa::Rsa;

#[allow(unused_imports)]
use crate::peer::event::{CONNECTING, PeerEvent};
#[allow(unused_imports)]
use crate::peer::event::common::{Merge, Split};
#[allow(unused_imports)]
use crate::peer::event::connecting::PeerConnectingEvent;

#[allow(dead_code)]
struct Keys {
    public_key_pem: Vec<u8>,
}

#[allow(dead_code)]
fn init_keys() -> Keys {
    let rsa = Rsa::generate(2048).unwrap();
    let public_key_pem = rsa.public_key_to_pem().unwrap();
    Keys {
        public_key_pem,
    }
}

#[test]
fn test_split_event() {
    let keys = init_keys();
    let event = PeerConnectingEvent::event(PeerConnectingEvent {
        uid: String::from("0001"),
        public_key_pem: keys.public_key_pem,
    });
    let events = PeerEvent::split(event, 128);
    assert_eq!(4, events.len());
}

#[test]
fn test_merge_event() {
    let keys = init_keys();
    let event = PeerConnectingEvent::event(PeerConnectingEvent {
        uid: String::from("0001"),
        public_key_pem: keys.public_key_pem,
    });
    let message = event.clone().message;
    let events = PeerEvent::split(event, 64);
    let merge_event = PeerEvent::merge(&events);
    assert_eq!(CONNECTING, merge_event.code);
    assert_eq!(message, merge_event.message);
}