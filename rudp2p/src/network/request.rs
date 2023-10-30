use std::{cmp::Ordering, fmt::Debug, net::SocketAddr};

use serialize_bits::{des::DeserializerData, ser::SerializerData};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    Connection,
    Disconnection,
    Message,
    ShareConnection,
}

impl Type {
    pub fn from_code(code: u8) -> Self {
        match code {
            0 => Self::Connection,
            1 => Self::Disconnection,
            2 => Self::Message,
            8 => Self::ShareConnection,
            _ => panic!("Code is not valid !"),
        }
    }

    pub fn to_code(&self) -> u8 {
        match self {
            Type::Connection => 0,
            Type::Disconnection => 1,
            Type::Message => 2,
            Type::ShareConnection => 8,
        }
    }
}

impl SerializerData for Type {
    fn to_data(&self) -> Vec<u8> {
        self.to_code().to_data()
    }
}

impl DeserializerData for Type {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        let (value, index) = u8::from_data(data, index);
        (Type::from_code(value), index)
    }
}

#[derive(Clone, Eq, PartialEq, Ord)]
pub struct RequestPart {
    pub uid: String,
    pub request_type: Type,
    pub start: usize,
    pub total: usize,
    pub content_size: usize,
    pub content: Vec<u8>,
    pub sender: SocketAddr,
}

impl SerializerData for RequestPart {
    fn to_data(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.append(&mut self.uid.to_data());
        res.append(&mut self.request_type.to_data());
        res.append(&mut self.start.to_data());
        res.append(&mut self.total.to_data());
        res.append(&mut self.content_size.to_data());
        res.append(&mut self.content.to_data());
        res.append(&mut self.sender.to_data());
        res
    }
}

impl DeserializerData for RequestPart {
    fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize)
    where
        Self: Sized,
    {
        let (uid, index) = String::from_data(data, index);
        let (request_type, index) = Type::from_data(data, index);
        let (start, index) = usize::from_data(data, index);
        let (total, index) = usize::from_data(data, index);
        let (content_size, index) = usize::from_data(data, index);
        let (content, index) = Vec::from_data(data, index);
        let (sender, index) = SocketAddr::from_data(data, index);
        (
            Self {
                uid,
                request_type,
                start,
                total,
                content_size,
                content,
                sender,
            },
            index,
        )
    }
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
