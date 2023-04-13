#[allow(unused_imports)]
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[allow(unused_imports)]
use openssl::rsa::Rsa;
#[allow(unused_imports)]
use openssl::symm::Cipher;

#[allow(unused_imports)]
use crate::peer::event::common::DecryptParser;
#[allow(unused_imports)]
use crate::peer::event::connected::PeerConnectedEvent;

#[test]
fn test_parse_event() {
    let rsa = Rsa::generate(1024).unwrap();
    let public_key_pem = rsa.public_key_to_pem().unwrap();
    let passphrase = "Passphrase_for_Tests";
    let private_key_pem = rsa.private_key_to_pem_passphrase(Cipher::aes_256_cbc(),
                                                            passphrase.as_bytes()).unwrap();
    let event = PeerConnectedEvent::event(PeerConnectedEvent {
        uid: "P0".to_string(),
        addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8000),
        peers: vec![],
        public_key_pem: public_key_pem.clone(),
    });
    let data = PeerConnectedEvent::parse(&event.message, &private_key_pem, passphrase);
    assert_eq!("P0".to_string(), data.uid);
    assert_eq!(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8000), data.addr);
    assert_eq!(0, data.peers.len());
    assert_eq!(public_key_pem.clone(), data.public_key_pem);
}
