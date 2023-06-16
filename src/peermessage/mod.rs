use std::time::SystemTime;

use crate::{utils::from_ne_bytes, Content, PeerError, PeerInternalEvent, PeerMessage};

impl PeerMessage {
    pub(crate) fn new<I>(
        content: I,
        event: PeerInternalEvent,
        public_key_pem: Option<Vec<u8>>,
    ) -> Result<PeerMessage, PeerError>
    where
        I: Content,
    {
        content.to_vec(public_key_pem).and_then(|data| {
            Ok(PeerMessage {
                uid: SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
                    .to_string(),
                event,
                start: 0,
                total: data.len(),
                content: data,
            })
        })
    }

    pub(crate) fn encode(&mut self) -> Vec<u8> {
        // [uid size][uid][peer event code][start index][total content size][content]
        let mut encoded = Vec::new();
        let mut uid = self.uid.as_bytes().to_vec();
        let uid_size = uid.len();
        encoded.append(&mut uid_size.to_ne_bytes().to_vec());
        encoded.append(&mut uid);
        encoded.push(self.event.get_code());
        encoded.append(&mut self.start.to_ne_bytes().to_vec());
        encoded.append(&mut self.total.to_ne_bytes().to_vec());
        encoded.append(&mut self.content.clone());
        encoded
    }

    pub(crate) fn parse(data: Vec<u8>) -> Result<PeerMessage, PeerError> {
        let uid_size = from_ne_bytes(&data, 0);
        let uid = String::from_utf8(data[8..8 + uid_size].to_vec()).unwrap();
        let code = data[8 + uid_size];
        let start = from_ne_bytes(&data, 9 + uid_size);
        let total = from_ne_bytes(&data, 17 + uid_size);
        let content = data[25 + uid_size..data.len()].to_vec();
        PeerInternalEvent::from_code(code).and_then(|event| {
            Ok(PeerMessage {
                uid,
                event,
                start,
                total,
                content,
            })
        })
    }

    pub(crate) fn split(&self) -> Vec<PeerMessage> {
        let mut messages = Vec::new();
        let size = 1024;
        let content_size = self.content.len();
        for i in (0..content_size).step_by(size) {
            let mut max = i + size;
            if max > content_size {
                max = content_size;
            }
            let new_content = self.content[i..max].to_vec();
            messages.push(PeerMessage {
                uid: self.uid.clone(),
                event: self.event.clone(),
                start: i,
                total: self.total.clone(),
                content: new_content,
            })
        }
        messages
    }

    pub(crate) fn merge(messages: &Vec<PeerMessage>) -> Result<PeerMessage, PeerError> {
        let mut uid = None;
        let mut event = None;
        let mut total = 0;
        let mut content = vec![];
        for message in messages {
            total = message.total;
            if uid.is_none() {
                uid = Some(message.uid.clone());
                event = Some(message.event.clone());
            }
            if content.len() < total {
                content.append(&mut message.content.clone());
            }
        }
        if uid.is_none() || event.is_none() {
            Err(PeerError::new("Invalid structure of message"))
        } else {
            Ok(PeerMessage {
                uid: uid.unwrap(),
                event: event.unwrap(),
                start: 0,
                total,
                content,
            })
        }
    }

    pub(crate) fn is_completed(&self) -> bool {
        self.total == self.content.len()
    }
}

#[cfg(test)]
mod test {
    use crate::{PeerDisconnectingInfo, PeerInternalEvent, PeerMessage, PeerMessageInfo};

    fn compare(expected: PeerMessage, actual: &PeerMessage) {
        assert_eq!(expected.uid, actual.uid, "not same uid");
        assert_eq!(
            expected.event.get_code(),
            actual.event.get_code(),
            "not same event's code"
        );
        assert_eq!(expected.start, actual.start, "not same start index");
        assert_eq!(expected.total, actual.total, "not same total size");
        assert_eq!(expected.content, actual.content, "not same content");
    }

    fn encode_and_parse(mut message: PeerMessage) -> PeerMessage {
        let data = message.encode();
        let message_decoded = PeerMessage::parse(data).unwrap();
        compare(message, &message_decoded);
        message_decoded
    }

    #[test]
    fn test_encode_and_parse_disconnecting() {
        let message = PeerMessage::new(
            PeerDisconnectingInfo {
                uid: "TEST UID".to_string(),
            },
            PeerInternalEvent::DISCONNECTING,
            None,
        )
        .unwrap();
        let message_decoded = encode_and_parse(message);
        assert_eq!(
            "TEST UID",
            String::from_utf8(message_decoded.content).unwrap()
        );
    }

    #[test]
    fn test_encode_and_parse_message() {
        let message = PeerMessage::new(
            PeerMessageInfo {
                content: "Hello 4test !".as_bytes().to_vec(),
            },
            PeerInternalEvent::MESSAGE,
            None,
        )
        .unwrap();
        let message_decoded = encode_and_parse(message);
        assert_eq!(
            "Hello 4test !",
            String::from_utf8(message_decoded.content).unwrap(),
            "not same text"
        );
    }

    #[test]
    fn test_split_and_merge() {
        let mut content = Vec::new();
        for i in 0..2048 {
            content.push(i as u8);
        }
        let message = PeerMessage::new(
            PeerMessageInfo { content },
            PeerInternalEvent::MESSAGE,
            None,
        )
        .unwrap();
        let messages = message.split();
        assert_eq!(2, messages.len());
        let message_merged = PeerMessage::merge(&messages).unwrap();
        compare(message, &message_merged);
    }
}
