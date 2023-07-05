use std::{collections::HashMap, net::SocketAddr};

use crate::{
    flows::{
        connecting::ConnectingFlow, disconnecting::DisconnectingFlow, message::MessageFlow,
        sharepeers::SharePeersFlow, FlowProcessor,
    },
    Error, Event, InternalPeer, InternalTypeEvent, Message, RemotePeer,
};

impl Event {
    pub(crate) fn parse(
        peer_data: InternalPeer,
        uid_message: &String,
        sender: &SocketAddr,
        cache_messages: &HashMap<String, Vec<Message>>,
    ) -> Result<
        (
            Option<Event>,
            HashMap<String, RemotePeer>,
            HashMap<SocketAddr, Vec<Message>>,
        ),
        Error,
    > {
        let list_messages = cache_messages.get(uid_message).unwrap();
        Message::merge(list_messages).and_then(|message| {
            if message.is_completed() {
                match message.event {
                    InternalTypeEvent::CONNECTING => {
                        ConnectingFlow::process(&message, peer_data, sender)
                    }
                    InternalTypeEvent::DISCONNECTING => {
                        DisconnectingFlow::process(&message, peer_data, &sender)
                    }
                    InternalTypeEvent::MESSAGE => {
                        MessageFlow::process(&message, peer_data, &sender)
                    }
                    InternalTypeEvent::PEERS => {
                        SharePeersFlow::process(&message, peer_data, &sender)
                    }
                }
            } else {
                let peers_connected = peer_data.peers_connected.unwrap();
                Ok((None, peers_connected.clone(), HashMap::new()))
            }
        })
    }
}

#[cfg(test)]
mod test {
    use std::{
        collections::HashMap,
        net::{IpAddr, SocketAddr},
    };

    use crate::{
        Event, InternalPeer, InternalTypeEvent, Message, MessageMethod, RemotePeer, TypeEvent,
    };
    use openssl::rsa::Rsa;

    fn init_peer(port: u16, peers: HashMap<String, RemotePeer>) -> InternalPeer {
        let rsa = Rsa::generate(1024).unwrap();
        let public_key_pem = rsa.public_key_to_pem().unwrap();
        InternalPeer {
            uid: "PeerTest".to_owned(),
            public_key_pem,
            private_key: Some(rsa),
            addr: init_sender(port),
            peers_connected: Some(peers),
            rejects: vec![],
            share_peers: true,
        }
    }

    fn init_sender(port: u16) -> SocketAddr {
        let addr = "127.0.0.1".parse::<IpAddr>().unwrap();
        SocketAddr::new(addr, port)
    }

    fn test_parse(
        peer_data: InternalPeer,
        sender: SocketAddr,
        message: Message,
    ) -> (
        Option<Event>,
        HashMap<String, RemotePeer>,
        HashMap<SocketAddr, Vec<Message>>,
    ) {
        let uid_message = "Message001".to_owned();
        let mut cache_messages = HashMap::new();
        cache_messages.insert(uid_message.clone(), vec![message]);
        Event::parse(peer_data, &uid_message, &sender, &cache_messages).unwrap()
    }

    #[test]
    fn test_parse_connecting() {
        let mut peers_connected = HashMap::new();
        peers_connected.insert(
            "Other".to_owned(),
            RemotePeer {
                uid: "Other".to_owned(),
                addr: init_sender(9002),
                public_key: None,
            },
        );
        let peer_data = init_peer(9000, peers_connected);
        let sender = init_sender(9001);
        let message = Message::internal(
            InternalTypeEvent::CONNECTING,
            MessageMethod::Request,
            &peer_data,
            None,
        );
        let (event, peers, responses) = test_parse(peer_data, sender, message);
        assert!(event.is_some());
        assert_eq!(TypeEvent::CONNECTED, event.unwrap().type_event);
        assert_eq!(1, responses.len());
        let responses = responses.get(&sender).unwrap();
        assert_eq!(2, responses.len());
        assert_eq!(InternalTypeEvent::CONNECTING, responses[0].event);
        assert_eq!(InternalTypeEvent::PEERS, responses[1].event);
        assert!(responses[1].total > 0);
        assert_eq!(2, peers.len());
    }

    #[test]
    fn test_parse_disconnecting() {
        let sender = init_sender(9001);
        let mut peers_connected = HashMap::new();
        peers_connected.insert(
            "Other".to_owned(),
            RemotePeer {
                uid: "Other".to_owned(),
                addr: sender,
                public_key: None,
            },
        );
        let peer_data = init_peer(9000, peers_connected);
        let message = Message::new_internal(
            InternalTypeEvent::DISCONNECTING,
            MessageMethod::Request,
            vec![],
        );
        let (event, peers, responses) = test_parse(peer_data, sender, message);
        assert!(event.is_some());
        assert_eq!(TypeEvent::DISCONNECTED, event.unwrap().type_event);
        assert_eq!(1, responses.len());
        let responses = responses.get(&sender).unwrap();
        assert_eq!(1, responses.len());
        assert_eq!(InternalTypeEvent::DISCONNECTING, responses[0].event);
        assert_eq!(0, peers.len());
    }

    #[test]
    fn test_parse_peers() {
        let rsa = Rsa::generate(1024).unwrap();
        let sender_pk = rsa.public_key_to_pem().unwrap();
        let sender = init_sender(9001);
        let mut peers_connected = HashMap::new();
        peers_connected.insert(
            "Other".to_owned(),
            RemotePeer {
                uid: "Other".to_owned(),
                addr: sender,
                public_key: Some(sender_pk.clone()),
            },
        );
        let peer_data = init_peer(9000, peers_connected);
        let message = Message::internal(
            InternalTypeEvent::PEERS,
            MessageMethod::Request,
            &peer_data,
            Some(sender_pk),
        );
        let (event, peers, responses) = test_parse(peer_data, sender, message);
        assert!(event.is_none());
        assert_eq!(1, responses.len());
        let responses = responses.get(&sender).unwrap();
        assert_eq!(1, responses.len());
        assert_eq!(InternalTypeEvent::PEERS, responses[0].event);
        assert_eq!(1, peers.len());
    }
}
