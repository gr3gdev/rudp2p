use std::{collections::HashMap, net::SocketAddr, sync::MutexGuard};

use openssl::{
    pkey::Private,
    rsa::{Padding, Rsa},
};

use crate::{PeerError, RemotePeer};

pub(crate) fn encrypt(public_key_pem: &[u8], data: &Vec<u8>) -> Result<Vec<u8>, PeerError> {
    if public_key_pem.is_empty() {
        Err(PeerError::new("Invalid public key (empty)"))
    } else {
        Rsa::public_key_from_pem(public_key_pem)
            .or_else(|e| Err(PeerError::new_ssl("Error when generate public key", e)))
            .and_then(|rsa| {
                let mut buf = vec![0; rsa.size() as usize];
                rsa.public_encrypt(data.as_slice(), &mut buf, Padding::PKCS1)
                    .and_then(|_| Ok(buf))
                    .or_else(|e| Err(PeerError::new_ssl("Error when encrypt data", e)))
            })
    }
}

pub(crate) fn decrypt(rsa: Rsa<Private>, data: &Vec<u8>) -> Result<Vec<u8>, PeerError> {
    let mut buf = vec![0; rsa.size() as usize];
    rsa.private_decrypt(&data, &mut buf, Padding::PKCS1)
        .or_else(|e| Err(PeerError::new_ssl("Error when decrypt data", e)))
        .and_then(|_| Ok(buf))
}

pub(crate) fn from_ne_bytes(data: &Vec<u8>, index: usize) -> usize {
    usize::from_ne_bytes([
        data[index],
        data[index + 1],
        data[index + 2],
        data[index + 3],
        data[index + 4],
        data[index + 5],
        data[index + 6],
        data[index + 7],
    ])
}

pub(crate) fn is_new_peer(
    peers: MutexGuard<'_, HashMap<String, RemotePeer>>,
    uid: &String,
) -> bool {
    !peers.contains_key(uid)
}

pub(crate) fn find_peer_by_id(
    peers: MutexGuard<'_, HashMap<String, RemotePeer>>,
    uid: &String,
) -> Result<RemotePeer, PeerError> {
    peers
        .get(uid)
        .ok_or(PeerError::new("Peer not found"))
        .map(|r| r.clone())
}

pub(crate) fn find_peer_by_address(
    peers: MutexGuard<'_, HashMap<String, RemotePeer>>,
    address: SocketAddr,
) -> Result<String, PeerError> {
    peers
        .iter()
        .find(|s| s.1.address == address)
        .map(|s| s.0.clone())
        .ok_or(PeerError::new("Peer not found"))
}

#[cfg(test)]
mod test {
    use openssl::rsa::Rsa;

    use super::{decrypt, encrypt};

    #[test]
    fn test_encrypt_decrypt() {
        let rsa = Rsa::generate(2048).unwrap();
        let public_key = rsa.public_key_to_pem().unwrap();
        let data = "This is a text for tests".as_bytes().to_vec();
        let encrypted = encrypt(public_key.as_slice(), &data).unwrap();
        let mut decrypted = decrypt(rsa, &encrypted).unwrap();
        decrypted.truncate(data.len());
        assert_eq!(data, decrypted);
    }
}
