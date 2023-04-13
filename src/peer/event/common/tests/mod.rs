#[allow(unused_imports)]
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[allow(unused_imports)]
use openssl::rsa::Rsa;
#[allow(unused_imports)]
use openssl::symm::Cipher;

#[allow(unused_imports)]
use crate::peer::event::common::{append_address, append_peers, append_public_pem, append_uid, decrypt, encrypt, parse_address, parse_peers, parse_public_pem, parse_uid};
#[allow(unused_imports)]
use crate::peer::RemotePeer;

#[test]
fn test_encrypt_decrypt() {
    let rsa = Rsa::generate(1024).unwrap();
    let public_key_pem = rsa.public_key_to_pem().unwrap();
    let passphrase = "Passphrase_for_Tests";
    let private_key_pem = rsa.private_key_to_pem_passphrase(Cipher::aes_256_cbc(),
                                                            passphrase.as_bytes()).unwrap();
    let data = "Test encrypt message".as_bytes().to_vec();
    let encrypted = encrypt(&public_key_pem, data.clone());
    let mut decrypted = decrypt(&private_key_pem, &passphrase, encrypted);
    decrypted.truncate(data.len());
    assert_eq!("Test encrypt message", String::from_utf8(decrypted).unwrap())
}

#[test]
fn test_append_parse_uid() {
    let mut data: Vec<u8> = Vec::new();
    let expected = "P1".to_string();
    append_uid(&mut data, expected.clone());
    let actual = parse_uid(&data);
    assert_eq!(expected, actual.value);
}

#[test]
fn test_append_parse_public_pem() {
    let mut data: Vec<u8> = Vec::new();
    let rsa = Rsa::generate(1024).unwrap();
    let expected = rsa.public_key_to_pem().unwrap();
    append_public_pem(&mut data, &expected.clone());
    let actual = parse_public_pem(&data, 0);
    assert_eq!(expected, actual.value);
}

#[test]
fn test_append_parse_address() {
    let rsa = Rsa::generate(1024).unwrap();
    let public_key_pem = rsa.public_key_to_pem().unwrap();
    let passphrase = "Passphrase_for_Tests";
    let private_key_pem = rsa.private_key_to_pem_passphrase(Cipher::aes_256_cbc(),
                                                            passphrase.as_bytes()).unwrap();
    let mut data: Vec<u8> = Vec::new();
    let expected = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8000);
    append_address(&mut data, expected, &public_key_pem);
    let actual = parse_address(&data, 0, &private_key_pem, passphrase);
    assert_eq!(expected, actual.value);
}

#[test]
fn test_append_parse_peers() {
    let rsa = Rsa::generate(1024).unwrap();
    let public_key_pem = rsa.public_key_to_pem().unwrap();
    let passphrase = "Passphrase_for_Tests";
    let private_key_pem = rsa.private_key_to_pem_passphrase(Cipher::aes_256_cbc(),
                                                            passphrase.as_bytes()).unwrap();
    let mut data: Vec<u8> = Vec::new();
    let peers = vec![
        RemotePeer {
            uid: "P1".to_string(),
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8001),
            public_key_pem: None,
        }, RemotePeer {
            uid: "P2".to_string(),
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8002),
            public_key_pem: None,
        },
    ];
    append_peers(&mut data, peers.clone(), &public_key_pem);
    let actual = parse_peers(&data, 0, &private_key_pem, passphrase);
    assert_eq!(peers.len(), actual.value.len());
    for peer in actual.value {
        let filter = peers.iter().find(|p| p.uid.eq(&peer.uid));
        assert!(filter.is_some());
        assert_eq!(filter.unwrap().addr, peer.addr);
        assert_eq!(None, peer.public_key_pem);
    }
}
