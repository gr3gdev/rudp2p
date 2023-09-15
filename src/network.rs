use std::{
    fmt::Debug,
    net::{SocketAddr, UdpSocket},
};

use log::error;

use crate::{
    dao::remote::RemotePeer,
    utils::{decoder::Decoder, encoder::Encoder, multipart::Multipart},
};

use self::{events::*, request::Type};

pub mod events;
pub mod request;

pub trait Data {
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
        Self {
            request_type: Type::Message,
            content: data.to_vec().clone(),
        }
    }

    pub(crate) fn new_connection(peer_name: String, public_key: Vec<u8>) -> Self {
        let content = Vec::new();
        let content = Encoder::add_with_size(content, peer_name.as_bytes().to_vec());
        let content = Encoder::add_with_size(content, public_key);
        Self {
            request_type: Type::Connection,
            content,
        }
    }

    pub(crate) fn new_disconnection(peer_name: String) -> Self {
        let content = Vec::new();
        let content = Encoder::add(content, peer_name.as_bytes().to_vec());
        Self {
            request_type: Type::Disconnection,
            content,
        }
    }

    pub(crate) fn new_message(peer_name: String, data: Vec<u8>) -> Self {
        let mut content = Encoder::add_with_size(Vec::new(), peer_name.as_bytes().to_vec());
        content.append(&mut data.clone());
        Self {
            request_type: Type::Message,
            content,
        }
    }

    pub(crate) fn new_share_connection(peers: &Vec<RemotePeer>) -> Self {
        let mut content = Vec::new();
        for remote in peers {
            content.append(&mut remote.addr.to_string().as_bytes().to_vec());
            content.push(',' as u8);
        }
        Self {
            request_type: Type::ShareConnection,
            content,
        }
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
        res
    }

    pub(crate) fn to_connected_event(&self, peer_uid: &String, addr: &SocketAddr) -> Connected {
        let content = self.content.clone();
        let (peer_name_size, next_index) = Decoder::get_size(&content, 0);
        let peer_name =
            String::from_utf8(content[next_index..next_index + peer_name_size].to_vec())
                .expect("Unable to read content");
        let (public_key_size, next_index) =
            Decoder::get_size(&content, next_index + peer_name_size);
        let public_key = content[next_index..next_index + public_key_size].to_vec();
        Connected {
            uid: peer_uid.clone(),
            from: peer_name,
            address: addr.clone(),
            public_key,
        }
    }

    pub(crate) fn to_disconnected_event(
        &self,
        peer_uid: &String,
        addr: &SocketAddr,
    ) -> Disconnected {
        let peer_name = String::from_utf8(self.content.clone()).expect("Unable to read content");
        Disconnected {
            uid: peer_uid.clone(),
            from: peer_name,
            address: addr.clone(),
        }
    }

    pub(crate) fn to_message_event(&self, peer_uid: &String) -> Message {
        let content = self.content.clone();
        let (peer_name_size, next_index) = Decoder::get_size(&content, 0);
        let peer_name =
            String::from_utf8(content[next_index..next_index + peer_name_size].to_vec())
                .expect("Unable to read content");
        let content = content[next_index + peer_name_size..].to_vec();
        Message {
            uid: peer_uid.clone(),
            from: peer_name,
            content,
        }
    }

    pub(crate) fn send(&self, socket: &UdpSocket, addr: &SocketAddr, public_key: &Vec<u8>) -> () {
        let parts = Multipart::split(self, public_key);
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
        Self {
            address: None,
            data: value.as_bytes().to_vec(),
        }
    }

    pub(crate) fn to_request(&self, peer_name: String) -> Request {
        Request::new_message(peer_name, self.data.clone())
    }
}
