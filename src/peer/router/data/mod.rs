use crate::peer::event::common::{decrypt, encrypt, get_size_from_ne_bytes};
use crate::peer::message::PeerMessage;
use crate::peer::RemotePeer;

pub(crate) trait Encoder {
    fn encode(&self) -> Vec<u8>;
}

pub(crate) trait Decoder {
    fn decode(message: Vec<u8>, types: Vec<DecodeData>) -> Vec<RouteData>;
}

pub(crate) enum DecodeData {
    Uid,
    PublicKey,
    Peers(Vec<u8>, String),
    Message(Vec<u8>, String),
}

pub(crate) enum RouteData {
    Uid(String),
    PublicKey(Vec<u8>),
    Peers(Vec<RemotePeer>, Vec<u8>),
    Message(PeerMessage, Vec<u8>),
}

impl Decoder for RouteData {
    fn decode(message: Vec<u8>, types: Vec<DecodeData>) -> Vec<RouteData> {
        let mut size = 0;
        let mut data = Vec::new();
        for t in types {
            match t {
                DecodeData::Uid => {
                    let uid_size = message[0] as usize;
                    let uid_data = &message[1..1 + uid_size];
                    data.push(RouteData::Uid(String::from_utf8(uid_data.to_vec()).expect("Unable to decode UID")));
                    size += 1 + uid_size;
                }
                DecodeData::PublicKey => {
                    let public_key_size = get_size_from_ne_bytes(&message, size);
                    let public_key_data = &message[size + 8..size + 8 + public_key_size];
                    data.push(RouteData::PublicKey(public_key_data.to_vec()));
                    size += 8 + public_key_size;
                }
                DecodeData::Peers(private_key_pem, passphrase) => {
                    if size < message.len() {
                        let real_size = get_size_from_ne_bytes(&message, size);
                        let peers = &message[size + 8..message.len()];
                        let encrypt_peers = peers.to_vec();
                        let mut peers = decrypt(&private_key_pem, passphrase.as_str(), encrypt_peers);
                        peers.truncate(real_size);
                        let list_peers = String::from_utf8(peers).expect("Unable to read list of peers");
                        let peers_data = list_peers.split(",").collect::<Vec<&str>>();
                        let mut peers = Vec::new();
                        for peer in peers_data {
                            let data = peer.split("|").collect::<Vec<&str>>();
                            if data.len() >= 2 {
                                peers.push(RemotePeer {
                                    uid: data.get(0).expect("Uid not found").to_string(),
                                    addr: data.get(1).expect("Address not found").to_string().parse().expect("Unable to parse socket address"),
                                    public_key_pem: None,
                                })
                            }
                        }
                        data.push(RouteData::Peers(peers, vec![]));
                    } else {
                        data.push(RouteData::Peers(vec![], vec![]));
                    }
                }
                DecodeData::Message(private_key_pem, passphrase) => {
                    let uid_size = message[size] as usize;
                    let uid_data = &message[size + 1..size + 1 + uid_size];
                    let real_size = get_size_from_ne_bytes(&message, size + 1 + uid_size);
                    let mut content = decrypt(&private_key_pem, passphrase.as_str(), message[size + 9 + uid_size..message.len()].to_vec());
                    content.truncate(real_size);
                    data.push(RouteData::Message(PeerMessage::new(content, Some(uid_data.to_vec())), vec![]));
                }
            }
        }
        data
    }
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
                if !peers.is_empty() {
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
            RouteData::Message(message, public_key_pem) => {
                data.push(message.uid.len() as u8);
                data.append(&mut message.uid.clone());
                let mut encrypt_content = encrypt(public_key_pem, message.content.clone());
                data.append(&mut encrypt_content.len().to_ne_bytes().to_vec());
                data.append(&mut encrypt_content);
            }
        }
        data
    }
}
