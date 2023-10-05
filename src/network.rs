use self::{events::*, request::Type};
use crate::{
    peer::RemotePeer,
    utils::{decoder::Decoder, encoder::Encoder, multipart::Multipart},
};
use log::error;
use std::{
    fmt::Debug,
    net::{SocketAddr, UdpSocket},
};

pub mod events;
pub mod request;

pub trait Data: Debug {
    fn to_vec(&self) -> Vec<u8>;
}

impl Data for String {
    fn to_vec(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl Data for &str {
    fn to_vec(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl Data for Vec<u8> {
    fn to_vec(&self) -> Vec<u8> {
        self.clone()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Request {
    pub(crate) request_type: Type,
    pub(crate) content: Vec<u8>,
}

impl Debug for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Request")
            .field("request_type", &self.request_type)
            .field("content", &self.content.len())
            .finish()
    }
}

impl Request {
    pub fn new<D>(data: D) -> Self
    where
        D: Data,
    {
        let instance = Self {
            request_type: Type::Message,
            content: data.to_vec().clone(),
        };
        log::trace!("Request::new({:?}) => {:?}", data, instance);
        instance
    }

    pub(crate) fn new_connection(public_key: &Vec<u8>) -> Self {
        let content = Vec::new();
        let content = Encoder::add_with_size(&content, &public_key);
        let instance = Self {
            request_type: Type::Connection,
            content,
        };
        log::trace!(
            "Request::new_connection({}) => {:?}",
            public_key.len(),
            instance
        );
        instance
    }

    pub(crate) fn new_disconnection() -> Self {
        let instance = Self {
            request_type: Type::Disconnection,
            content: vec![1],
        };
        log::trace!("Request::new_disconnection() => {:?}", instance);
        instance
    }

    pub(crate) fn new_message(data: &Vec<u8>) -> Self {
        let instance = Self {
            request_type: Type::Message,
            content: data.clone(),
        };
        log::trace!("Request::new_message({}) => {:?}", data.len(), instance);
        instance
    }

    pub(crate) fn new_share_connection(peers: &Vec<RemotePeer>) -> Self {
        let mut content = Vec::new();
        for remote in peers {
            content.append(&mut remote.addr.to_string().as_bytes().to_vec());
            content.push(',' as u8);
        }
        let instance = Self {
            request_type: Type::ShareConnection,
            content: content.clone(),
        };
        log::trace!(
            "Request::new_share_connection({:?}) => {:?}",
            content,
            instance
        );
        instance
    }

    pub(crate) fn to_peers_values(&self) -> Vec<SocketAddr> {
        let mut res = Vec::new();
        let mut current = Vec::new();
        for b in self.content.clone() {
            if b == ',' as u8 {
                let addr = String::from_utf8(current)
                    .expect("Unable to read address")
                    .parse()
                    .expect("Unable to parse address");
                res.push(addr);
                current = Vec::new();
            } else {
                current.push(b);
            }
        }
        log::trace!("Request::to_peers_values() => {:?}", res);
        res
    }

    pub(crate) fn parse_public_key(&self) -> Vec<u8> {
        let content = self.content.clone();
        let (public_key_size, next_index) = Decoder::get_size(&content, 0);
        let pk = content[next_index..next_index + public_key_size].to_vec();
        log::trace!("Request::parse_public_key() => {}", pk.len());
        pk
    }

    pub(crate) fn to_message_event(&self, remote: &RemotePeer) -> Message {
        let message = Message {
            from: remote.clone(),
            content: self.content.clone(),
        };
        log::trace!("Request::to_message_event({:?}) => {:?}", remote, message);
        message
    }

    pub(crate) fn send(&self, socket: &UdpSocket, addr: &SocketAddr, public_key: &Vec<u8>) -> () {
        log::trace!("Request::send({:?}, {addr}, {})", socket, public_key.len());
        let parts = Multipart::split(self, public_key, addr);
        for part in parts {
            socket.send_to(&part.to_data(), addr).unwrap_or_else(|e| {
                error!("Unable to send request : {e}");
                0
            });
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Response {
    pub(crate) address: Option<SocketAddr>,
    pub(crate) data: Vec<u8>,
}

impl Response {
    pub fn text(value: &str) -> Self {
        let instance = Self {
            address: None,
            data: value.as_bytes().to_vec(),
        };
        log::trace!("Response::text({value}) => {:?}", instance);
        instance
    }

    pub(crate) fn to_request(&self) -> Request {
        let req = Request::new_message(&self.data);
        log::trace!("Response::to_request() => {:?}", req);
        req
    }
}
