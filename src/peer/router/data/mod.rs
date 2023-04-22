use crate::peer::event::common::encrypt;
use crate::peer::RemotePeer;

pub(crate) trait Encoder {
    fn encode(&self) -> Vec<u8>;
}

pub(crate) enum RouteData {
    Uid(String),
    PublicKey(Vec<u8>),
    Peers(Vec<RemotePeer>, Vec<u8>),
}

impl Encoder for RouteData {
    fn encode(&self) -> Vec<u8> {
        let mut data = Vec::new();
        match self {
            RouteData::Uid(uid) => {
                data.push(uid.len() as u8);
                data.append(&mut uid.as_bytes().to_vec());
            }
            RouteData::PublicKey(public_key) => {
                data.append(&mut public_key.len().to_ne_bytes().to_vec());
                data.append(&mut public_key.clone());
            }
            RouteData::Peers(peers, public_key_pem) => {
                let mut list = Vec::new();
                for peer in peers {
                    let peer_string = peer.uid.clone() + "|" + peer.addr.to_string().as_str();
                    list.append(&mut peer_string.as_bytes().to_vec());
                    list.append(&mut ",".as_bytes().to_vec());
                }
                let mut encrypt_peers = encrypt(public_key_pem, list.clone());
                data.append(&mut list.len().to_ne_bytes().to_vec());
                data.append(&mut encrypt_peers);
            }
        }
        data
    }
}
