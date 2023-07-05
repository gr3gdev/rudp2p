use std::{fmt::Debug, time::SystemTime};

use crate::{
    encoder::{Data, Encoder},
    Error, InternalPeer, InternalTypeEvent, Message, MessageMethod,
};

impl Message {
    /// Create a new message with data
    pub fn new(data: Vec<u8>) -> Self {
        Self::new_internal(InternalTypeEvent::MESSAGE, MessageMethod::Other, data)
    }

    pub(crate) fn new_internal(
        event: InternalTypeEvent,
        method: MessageMethod,
        data: Vec<u8>,
    ) -> Self {
        Self {
            uid: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
                .to_string(),
            event,
            method,
            start: 0,
            total: data.len(),
            data,
        }
    }

    pub(crate) fn internal(
        event: InternalTypeEvent,
        method: MessageMethod,
        peer: &InternalPeer,
        remote_public_key: Option<Vec<u8>>,
    ) -> Self {
        let data;
        match event {
            InternalTypeEvent::CONNECTING => data = vec![Data::Uid, Data::PublicKey],
            InternalTypeEvent::DISCONNECTING => data = vec![],
            InternalTypeEvent::MESSAGE => data = vec![],
            InternalTypeEvent::PEERS => data = vec![Data::Peers],
        }
        let data = Encoder::concat(data, &peer, remote_public_key).unwrap();
        Self::new_internal(event, method, data)
    }

    pub(crate) fn write(&self) -> Vec<u8> {
        // [uid size][uid][peer event code][start index][total content size][content]
        let mut encoded = Vec::new();
        let mut uid = self.uid.as_bytes().to_vec();
        let uid_size = uid.len();
        encoded.append(&mut uid_size.to_ne_bytes().to_vec());
        encoded.append(&mut uid);
        encoded.push(self.event.get_code());
        encoded.push(self.method.get_code());
        encoded.append(&mut self.start.to_ne_bytes().to_vec());
        encoded.append(&mut self.total.to_ne_bytes().to_vec());
        encoded.append(&mut self.data.clone());
        encoded
    }

    pub(crate) fn read(data: &Vec<u8>) -> Result<Self, Error> {
        let uid_size = Encoder::from_ne_bytes(data, 0);
        let uid_message = String::from_utf8(data[8..8 + uid_size].to_vec()).unwrap();
        let event_code = data[8 + uid_size];
        InternalTypeEvent::from_code(event_code).and_then(|type_event| {
            let method_code = data[9 + uid_size];
            MessageMethod::from_code(method_code).and_then(|method| {
                let start = Encoder::from_ne_bytes(data, 10 + uid_size);
                let total = Encoder::from_ne_bytes(data, 18 + uid_size);
                Ok(Self {
                    uid: uid_message,
                    event: type_event,
                    method,
                    start,
                    total,
                    data: data[26 + uid_size..].to_vec(),
                })
            })
        })
    }

    pub(crate) fn split(&self) -> Vec<Message> {
        let mut messages = Vec::new();
        if self.total == 0 {
            messages.push(Message {
                uid: self.uid.clone(),
                event: self.event.clone(),
                method: self.method.clone(),
                start: self.start,
                total: self.total,
                data: self.data.clone(),
            });
        } else {
            let size = 1024;
            let content_size = self.data.len();
            for i in (0..content_size).step_by(size) {
                let mut max = i + size;
                if max > content_size {
                    max = content_size;
                }
                let new_content = self.data[i..max].to_vec();
                messages.push(Message {
                    uid: self.uid.clone(),
                    event: self.event.clone(),
                    method: self.method.clone(),
                    start: i,
                    total: self.total,
                    data: new_content,
                })
            }
        }
        messages
    }

    pub(crate) fn merge(messages: &Vec<Message>) -> Result<Message, Error> {
        let mut uid = None;
        let mut event = None;
        let mut method = None;
        let mut total = 0;
        let mut data = vec![];
        for message in messages {
            total = message.total;
            if uid.is_none() {
                uid = Some(message.uid.clone());
                event = Some(message.event.clone());
                method = Some(message.method.clone());
            }
            if data.len() < total {
                data.append(&mut message.data.clone());
            }
        }
        if uid.is_none() || event.is_none() {
            Err(Error::custom("Invalid structure of message".to_owned()))
        } else {
            Ok(Message {
                uid: uid.unwrap(),
                event: event.unwrap(),
                method: method.unwrap(),
                start: 0,
                total,
                data,
            })
        }
    }

    pub(crate) fn is_completed(&self) -> bool {
        self.total <= self.data.len()
    }
}

impl Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Message")
            .field("uid", &self.uid)
            .field("event", &self.event)
            .field("method", &self.method)
            .field("start", &self.start)
            .field("total", &self.total)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::Message;

    #[test]
    fn test_read_write() {
        let message = Message::new("This is a test message".as_bytes().to_vec());
        let data = message.write();
        let read = Message::read(&data).unwrap();
        assert_eq!(message.uid, read.uid);
        assert_eq!(message.event, read.event);
        assert_eq!(message.method, read.method);
        assert_eq!(message.start, read.start);
        assert_eq!(message.total, read.total);
        assert_eq!(message.data, read.data);
    }

    #[test]
    fn test_split_merge() {
        let message = Message::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed non risus. Suspendisse lectus tortor, dignissim sit amet, adipiscing nec, ultricies sed, dolor. Cras elementum ultrices diam. Maecenas ligula massa, varius a, semper congue, euismod non, mi. Proin porttitor, orci nec nonummy molestie, enim est eleifend mi, non fermentum diam nisl sit amet erat. Duis semper. Duis arcu massa, scelerisque vitae, consequat in, pretium a, enim. Pellentesque congue. Ut in risus volutpat libero pharetra tempor. Cras vestibulum bibendum augue. Praesent egestas leo in pede. Praesent blandit odio eu enim. Pellentesque sed dui ut augue blandit sodales. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Aliquam nibh. Mauris ac mauris sed pede pellentesque fermentum. Maecenas adipiscing ante non diam sodales hendrerit.
        Ut velit mauris, egestas sed, gravida nec, ornare ut, mi. Aenean ut orci vel massa suscipit pulvinar. Nulla sollicitudin. Fusce varius, ligula non tempus aliquam, nunc turpis ullamcorper nibh, in tempus sapien eros vitae ligula. Pellentesque rhoncus nunc et augue. Integer id felis. Curabitur aliquet pellentesque diam. Integer quis metus vitae elit lobortis egestas. Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Morbi vel erat non mauris convallis vehicula. Nulla et sapien. Integer tortor tellus, aliquam faucibus, convallis id, congue eu, quam. Mauris ullamcorper felis vitae erat. Proin feugiat, augue non elementum posuere, metus purus iaculis lectus, et tristique ligula justo vitae magna.
        Aliquam convallis sollicitudin purus. Praesent aliquam, enim at fermentum mollis, ligula massa adipiscing nisl, ac euismod nibh nisl eu lectus. Fusce vulputate sem at sapien. Vivamus leo. Aliquam euismod libero eu enim. Nulla nec felis sed leo placerat imperdiet. Aenean suscipit nulla in justo. Suspendisse cursus rutrum augue. Nulla tincidunt tincidunt mi. Curabitur iaculis, lorem vel rhoncus faucibus, felis magna fermentum augue, et ultricies lacus lorem varius purus. Curabitur eu amet.".as_bytes().to_vec());
        let list = message.split();
        assert_eq!(3, list.len());
        let merge = Message::merge(&list).unwrap();
        assert_eq!(message.uid, merge.uid);
        assert_eq!(message.event, merge.event);
        assert_eq!(message.method, merge.method);
        assert_eq!(message.start, merge.start);
        assert_eq!(message.total, merge.total);
        assert_eq!(message.data, merge.data);
    }
}
