use std::{collections::HashMap, net::SocketAddr};

use openssl::pkey::Private;
use openssl::rsa::Rsa;

use crate::{
    utils::{decrypt, encrypt, from_ne_bytes},
    Content, Info, PeerConnectedInfo, PeerError, PeerMessage,
};

impl Content for PeerConnectedInfo {
    fn to_vec(&self, public_key_pem: Option<Vec<u8>>) -> Result<Vec<u8>, PeerError> {
        let mut data = Vec::new();
        let mut uid_size = self.uid.len().to_ne_bytes().to_vec();
        data.append(&mut uid_size);
        data.append(&mut self.uid.as_bytes().to_vec());
        let mut remote_peers = Vec::new();
        for remote_peer in &self.remote_peers {
            let mut concat = remote_peer.0.clone();
            concat.push('|');
            concat.push_str(&remote_peer.1.to_string());
            concat.push(',');
            remote_peers.append(&mut concat.as_bytes().to_vec());
        }
        data.append(&mut remote_peers.len().to_ne_bytes().to_vec());
        if let Some(public_key) = public_key_pem {
            encrypt(&public_key.as_slice(), &remote_peers).and_then(|mut encrypt_data| {
                data.append(&mut encrypt_data);
                Ok(data)
            })
        } else {
            data.append(&mut remote_peers);
            Ok(data)
        }
    }
}

impl Info for PeerConnectedInfo {
    fn parse(message: &PeerMessage, decrypt_data: Option<Rsa<Private>>) -> Result<Self, PeerError>
    where
        Self: Sized,
    {
        let data = &message.content;
        let uid_size = from_ne_bytes(data, 0);
        let uid = String::from_utf8(data[8..8 + uid_size].to_vec()).unwrap();
        let peers_size = from_ne_bytes(data, 8 + uid_size);
        let concat_peers;
        if let Some(key) = decrypt_data {
            let peers_decrypted = decrypt(key, &data[16 + uid_size..data.len()].to_vec())
                .and_then(|mut peers_decrypted| {
                    peers_decrypted.truncate(peers_size);
                    Ok(peers_decrypted)
                })
                .unwrap();
            concat_peers = String::from_utf8(peers_decrypted).unwrap();
        } else {
            concat_peers = String::from_utf8(data[16 + uid_size..data.len()].to_vec()).unwrap();
        }
        let peers = concat_peers.split(",").collect::<Vec<&str>>();
        let mut remote_peers = HashMap::new();
        for peer in peers {
            let infos = peer.split("|").collect::<Vec<&str>>();
            if infos.len() == 2 {
                let remote_uid = infos.get(0).unwrap().to_string();
                let addr = infos.get(1).unwrap().parse::<SocketAddr>().unwrap();
                remote_peers.insert(remote_uid, addr);
            }
        }
        Ok(PeerConnectedInfo { uid, remote_peers })
    }
}

#[cfg(test)]
mod test {
    use openssl::rsa::Rsa;
    use std::{
        collections::HashMap,
        net::{IpAddr, Ipv4Addr, SocketAddr},
    };

    use crate::{Content, Info, PeerConnectedInfo, PeerMessage};

    fn test(remote_peers: HashMap<String, SocketAddr>) {
        let rsa = Rsa::generate(2048).unwrap();
        let public_key = rsa.public_key_to_pem().unwrap();
        let peer_info = PeerConnectedInfo {
            uid: "UID00000001".to_string(),
            remote_peers,
        };
        let content = peer_info.to_vec(Some(public_key)).unwrap();
        let message = PeerMessage {
            uid: "MSG00000001".to_string(),
            event: crate::PeerInternalEvent::CONNECTING,
            start: 0,
            total: content.len(),
            content,
        };
        let peer_info_decoded = PeerConnectedInfo::parse(&message, Some(rsa)).unwrap();
        assert_eq!(peer_info.uid, peer_info_decoded.uid, "not same uid");
        assert_eq!(
            peer_info.remote_peers, peer_info_decoded.remote_peers,
            "not same list of remote peers"
        );
    }

    #[test]
    fn test_empty_remote_peers() {
        test(HashMap::new());
    }

    #[test]
    fn test_not_empty_remote_peers() {
        let mut remote_peers = HashMap::new();
        remote_peers.insert(
            "P001".to_string(),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9001),
        );
        remote_peers.insert(
            "P002".to_string(),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)), 9002),
        );
        test(remote_peers);
    }
}
