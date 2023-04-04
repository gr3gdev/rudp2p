#[cfg(test)]
mod tests {
    use openssl::rsa::Rsa;
    use openssl::symm::Cipher;

    use rudp2plib::peer::event::{CONNECTING, Merge, PeerConnectingEvent, PeerEvent, Split};

    struct Keys {
        private_key_pem: Vec<u8>,
        public_key_pem: Vec<u8>,
    }

    fn init_keys(passphrase: &str) -> Keys {
        let rsa = Rsa::generate(2048).unwrap();
        let private_key_pem = rsa.private_key_to_pem_passphrase(
            Cipher::aes_256_cbc(),
            passphrase.as_bytes(),
        ).unwrap();
        let public_key_pem = rsa.public_key_to_pem().unwrap();
        Keys {
            private_key_pem,
            public_key_pem,
        }
    }

    #[test]
    fn test_split_event() {
        let keys = init_keys("pass for split event");
        let event = PeerEvent::connecting(PeerConnectingEvent {
            uid: String::from("0001"),
            public_key_pem: keys.public_key_pem,
        });
        let events = PeerEvent::split(event, 128);
        assert_eq!(4, events.len());
    }

    #[test]
    fn test_merge_event() {
        let keys = init_keys("pass for merge event");
        let event = PeerEvent::connecting(PeerConnectingEvent {
            uid: String::from("0001"),
            public_key_pem: keys.public_key_pem,
        });
        let message = event.clone().message;
        let events = PeerEvent::split(event, 64);
        let merge_event = PeerEvent::merge(&events);
        assert_eq!(CONNECTING, merge_event.code);
        assert_eq!(message, merge_event.message);
    }
}