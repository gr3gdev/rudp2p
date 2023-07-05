use std::net::SocketAddr;

use openssl::{
    pkey::Private,
    rsa::{Padding, Rsa},
};

use crate::{Error, InternalPeer};

pub(crate) enum Data {
    Uid,
    PublicKey,
    Peers,
}

impl Data {
    fn encode(
        &self,
        value: &InternalPeer,
        remote_public_key: Option<Vec<u8>>,
    ) -> Result<Vec<u8>, Error> {
        match self {
            Data::Uid => {
                let mut data = Vec::new();
                let mut uid_size = value.uid.len().to_ne_bytes().to_vec();
                data.append(&mut uid_size);
                data.append(&mut value.uid.as_bytes().to_vec());
                Ok(data)
            }
            Data::PublicKey => {
                if value.public_key_pem.is_empty() {
                    Err(Error::custom("Public key is empty !".to_owned()))
                } else {
                    let public_key = value.public_key_pem.clone();
                    let mut pk_size = public_key.len().to_ne_bytes().to_vec();
                    let mut data = Vec::new();
                    data.append(&mut pk_size);
                    data.append(&mut public_key.clone());
                    Ok(data)
                }
            }
            Data::Peers => {
                let mut peers = Vec::new();
                for (uid, remote) in value.peers_connected.as_ref().unwrap().clone() {
                    let mut concat = uid.clone();
                    concat.push('|');
                    concat.push_str(&remote.addr.to_string());
                    concat.push(',');
                    peers.append(&mut concat.as_bytes().to_vec());
                }
                let mut data_size = peers.len().to_ne_bytes().to_vec();
                let public_key = remote_public_key.unwrap();
                Encoder::encrypt(&public_key, &peers).and_then(|mut peers| {
                    let mut data = Vec::new();
                    data.append(&mut data_size);
                    data.append(&mut peers);
                    Ok(data)
                })
            }
        }
    }
}

pub(crate) struct Encoder;

impl Encoder {
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

    pub(crate) fn concat(
        data: Vec<Data>,
        peer: &InternalPeer,
        remote_public_key: Option<Vec<u8>>,
    ) -> Result<Vec<u8>, Error> {
        let mut res = Vec::new();
        for d in data {
            let mut encoded = d.encode(peer, remote_public_key.clone()).unwrap();
            res.append(&mut encoded);
        }
        Ok(res)
    }

    pub(crate) fn extract(
        data: Vec<Data>,
        value: &Vec<u8>,
        private_key: Option<&Rsa<Private>>,
    ) -> Result<(String, Vec<u8>, Vec<(String, SocketAddr)>), Error> {
        let mut index = 0;
        let mut uid = "".to_owned();
        let mut public_key_pem = vec![];
        let mut peers = Vec::new();
        for d in data {
            match d {
                Data::Uid => {
                    let uid_size = Encoder::from_ne_bytes(&value, index);
                    uid =
                        String::from_utf8(value[index + 8..index + 8 + uid_size].to_vec()).unwrap();
                    index = index + 8 + uid_size;
                }
                Data::PublicKey => {
                    let pk_size = Encoder::from_ne_bytes(&value, index);
                    public_key_pem = value[index + 8..index + 8 + pk_size].to_vec();
                    if public_key_pem.is_empty() {
                        panic!("Public key empty !")
                    }
                    index = index + 8 + pk_size;
                }
                Data::Peers => {
                    let peer_size = Encoder::from_ne_bytes(&value, index);
                    let encrypted = value[index + 8..].to_vec();
                    let decrypted =
                        Self::decrypt(private_key.unwrap(), &encrypted, peer_size).unwrap();
                    let list = String::from_utf8(decrypted).unwrap();
                    for peer in list.split(",") {
                        let infos = peer.split("|").collect::<Vec<&str>>();
                        if infos.len() == 2 {
                            let uid = infos.get(0).unwrap().to_string();
                            let addr = infos.get(1).unwrap().parse::<SocketAddr>().unwrap();
                            peers.push((uid, addr));
                        }
                    }
                }
            }
        }
        Ok((uid, public_key_pem, peers))
    }

    pub(crate) fn encrypt(public_key_pem: &[u8], data: &Vec<u8>) -> Result<Vec<u8>, Error> {
        if public_key_pem.is_empty() {
            Err(Error::custom("Invalid public key (empty)".to_owned()))
        } else {
            Rsa::public_key_from_pem(public_key_pem)
                .or_else(|e| Err(Error::ssl(e)))
                .and_then(|rsa| {
                    let mut buf = vec![0; rsa.size() as usize];
                    rsa.public_encrypt(data.as_slice(), &mut buf, Padding::PKCS1)
                        .and_then(|_| Ok(buf))
                        .or_else(|e| Err(Error::ssl(e)))
                })
        }
    }

    pub(crate) fn decrypt(
        rsa: &Rsa<Private>,
        data: &Vec<u8>,
        original_size: usize,
    ) -> Result<Vec<u8>, Error> {
        let mut buf = vec![0; rsa.size() as usize];
        rsa.private_decrypt(&data, &mut buf, Padding::PKCS1)
            .or_else(|e| Err(Error::ssl(e)))
            .and_then(|_| {
                buf.truncate(original_size);
                Ok(buf)
            })
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        net::{IpAddr, SocketAddr},
    };

    use crate::{InternalPeer, RemotePeer};

    use super::{Data, Encoder};
    use openssl::rsa::Rsa;

    #[test]
    fn test_encrypt_decrypt() {
        let rsa = Rsa::generate(1024).unwrap();
        let public_key_pem = rsa.public_key_to_pem().unwrap();
        let expected = "This is a test".as_bytes().to_vec();
        let encrypted = Encoder::encrypt(&public_key_pem, &expected).unwrap();
        let decrypted = Encoder::decrypt(&rsa, &encrypted, expected.len()).unwrap();
        assert_eq!(expected, decrypted);
    }

    #[test]
    fn test_from_ne_bytes() {
        let expected = "This is a test".as_bytes().to_vec();
        let data = expected.len().to_ne_bytes().to_vec();
        let actual = Encoder::from_ne_bytes(&data, 0);
        assert_eq!(expected.len(), actual);
    }

    #[test]
    fn test_concat_extract() {
        let rsa = Rsa::generate(1024).unwrap();
        let public_key_pem = rsa.public_key_to_pem().unwrap();
        let addr = "127.0.0.1".parse::<IpAddr>().unwrap();
        let mut peers = HashMap::new();
        peers.insert(
            "Other1".to_owned(),
            RemotePeer {
                uid: "Other1".to_owned(),
                addr: SocketAddr::new(addr, 9001),
                public_key: None,
            },
        );
        peers.insert(
            "Other2".to_owned(),
            RemotePeer {
                uid: "Other2".to_owned(),
                addr: SocketAddr::new(addr, 9002),
                public_key: None,
            },
        );
        let peer = InternalPeer {
            uid: "PeerTest".to_owned(),
            public_key_pem,
            private_key: Some(rsa),
            addr: SocketAddr::new(addr, 9000),
            peers_connected: Some(peers),
            rejects: vec![],
            share_peers: true,
        };
        let result = Encoder::concat(
            vec![Data::Uid, Data::PublicKey, Data::Peers],
            &peer,
            Some(peer.public_key_pem.clone()),
        )
        .unwrap();
        let (uid, public_key, peers) = Encoder::extract(
            vec![Data::Uid, Data::PublicKey, Data::Peers],
            &result,
            Some(&peer.private_key.unwrap()),
        )
        .unwrap();
        assert_eq!(peer.uid, uid);
        assert_eq!(peer.public_key_pem, public_key);
        assert_eq!(2, peers.len());
    }

    #[test]
    fn test_extract() {}
}
