use std::{cmp::Ordering, fmt::Debug, net::SocketAddr};

use crate::utils::{decoder::Decoder, encoder::Encoder, unwrap::unwrap_result};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Type {
    Connection,
    Disconnection,
    Message,
    ShareConnection,
}

impl Type {
    pub(crate) fn from_code(code: u8) -> Self {
        match code {
            0 => Self::Connection,
            1 => Self::Disconnection,
            2 => Self::Message,
            8 => Self::ShareConnection,
            _ => panic!("Code is not valid !"),
        }
    }

    pub(crate) fn to_code(&self) -> u8 {
        match self {
            Type::Connection => 0,
            Type::Disconnection => 1,
            Type::Message => 2,
            Type::ShareConnection => 8,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord)]
pub struct RequestPart {
    pub(crate) uid: String,
    pub(crate) request_type: Type,
    pub(crate) start: usize,
    pub(crate) total: usize,
    pub(crate) content_size: usize,
    pub(crate) content: Vec<u8>,
    pub(crate) sender: SocketAddr,
}

impl Debug for RequestPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RequestPart")
            .field("uid", &self.uid)
            .field("request_type", &self.request_type)
            .field("start", &self.start)
            .field("total", &self.total)
            .field("content_size", &self.content_size)
            .field("sender", &self.sender)
            .finish()
    }
}

impl RequestPart {
    pub(crate) fn to_data(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(self.request_type.to_code());
        let data = Encoder::add_with_size(&data, &self.uid.as_bytes().to_vec());
        let data = Encoder::add_size(&data, self.start);
        let data = Encoder::add_size(&data, self.total);
        let mut data = Encoder::add_size(&data, self.content_size);
        data.append(&mut self.content.clone());
        log::trace!("RequestPart::to_data() => {}", data.len());
        data
    }

    pub(crate) fn parse(data: Vec<u8>, addr: SocketAddr) -> Self {
        let request_type = Type::from_code(data[0]);
        let (uid_size, next_index) = Decoder::get_size(&data, 1);
        let uid = unwrap_result(
            String::from_utf8(data[next_index..next_index + uid_size].to_vec()),
            "Unable to read the UID",
        );
        let (start, next_index) = Decoder::get_size(&data, next_index + uid_size);
        let (total, next_index) = Decoder::get_size(&data, next_index);
        let (content_size, next_index) = Decoder::get_size(&data, next_index);
        let res = Self {
            uid,
            request_type,
            start,
            total,
            content_size,
            content: data[next_index..].to_vec(),
            sender: addr,
        };
        log::trace!("RequestPart::parse({}, {addr}) => {:?}", data.len(), res);
        res
    }
}

impl PartialOrd for RequestPart {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.start < other.start {
            Some(Ordering::Less)
        } else if self.start > other.start {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}
