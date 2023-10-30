use self::{events::*, request::Type};
use crate::{
    configuration::Configuration,
    peer::RemotePeer,
    utils::{multipart::Multipart, unwrap::unwrap_result},
};
use log::error;
use serialize_bits::ser::SerializerData;
use std::{
    fmt::Debug,
    net::{SocketAddr, UdpSocket},
};

pub mod events;
pub mod request;

/// # Request
///
/// A request for exchange data with another Peer.
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
    /// New Request with Data.
    pub fn new<D>(data: D) -> Self
    where
        D: SerializerData,
    {
        let instance = Self {
            request_type: Type::Message,
            content: data.to_data().clone(),
        };
        log::trace!("Request::new(data) => {:?}", instance);
        instance
    }

    #[cfg(not(feature = "ssl"))]
    pub(crate) fn new_connection(configuration: &Configuration) -> Self {
        let instance = Self {
            request_type: Type::Connection,
            content: vec![0],
        };
        log::trace!(
            "Request::new_connection({:?}) => {:?}",
            configuration,
            instance
        );
        instance
    }

    #[cfg(feature = "ssl")]
    pub(crate) fn new_connection(configuration: &Configuration) -> Self {
        let content = configuration.ssl.public_key.to_data();
        let instance = Self {
            request_type: Type::Connection,
            content,
        };
        log::trace!(
            "Request::new_connection({:?}) => {:?}",
            configuration,
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
                let addr = unwrap_result(
                    unwrap_result(String::from_utf8(current), "Unable to read address").parse(),
                    "Unable to parse address",
                );
                res.push(addr);
                current = Vec::new();
            } else {
                current.push(b);
            }
        }
        log::trace!("Request::to_peers_values() => {:?}", res);
        res
    }

    #[cfg(feature = "ssl")]
    pub(crate) fn parse_public_key(&self) -> Vec<u8> {
        use serialize_bits::des::DeserializerData;

        let content = self.content.clone();
        let (pk, _) = Vec::from_data(&content, 0);
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

    #[cfg(not(feature = "ssl"))]
    pub(crate) fn send(&self, socket: &UdpSocket, addr: &SocketAddr) -> () {
        log::trace!("Request::send({:?}, {addr})", socket);
        let socket_address = unwrap_result(socket.local_addr(), "Unable to get the local address");
        let parts = Multipart::split(self, &vec![], &socket_address);
        for part in parts {
            socket.send_to(&part.to_data(), addr).unwrap_or_else(|e| {
                error!("Unable to send request : {e}");
                0
            });
        }
    }

    #[cfg(feature = "ssl")]
    pub(crate) fn send(&self, socket: &UdpSocket, addr: &SocketAddr, public_key: &Vec<u8>) -> () {
        log::trace!("Request::send({:?}, {addr}, {})", socket, public_key.len());
        let socket_address = unwrap_result(socket.local_addr(), "Unable to get the local address");
        let parts = Multipart::split(self, public_key, &socket_address);
        for part in parts {
            socket.send_to(&part.to_data(), addr).unwrap_or_else(|e| {
                error!("Unable to send request : {e}");
                0
            });
        }
    }
}

/// # Response
///
/// Response send after an event in Observer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Response {
    pub(crate) address: Option<SocketAddr>,
    pub(crate) data: Vec<u8>,
}

impl Response {
    /// New Response from text.
    pub fn text(value: &str) -> Self {
        let instance = Self {
            address: None,
            data: value.as_bytes().to_vec(),
        };
        log::trace!("Response::text({value}) => {:?}", instance);
        instance
    }

    /// New Response from Data.
    pub fn new<D>(data: D) -> Self
    where
        D: SerializerData,
    {
        let instance = Self {
            address: None,
            data: data.to_data(),
        };
        log::trace!("Response::new(data) => {:?}", instance);
        instance
    }

    pub(crate) fn to_request(&self) -> Request {
        let req = Request::new_message(&self.data);
        log::trace!("Response::to_request() => {:?}", req);
        req
    }
}

#[cfg(not(feature = "ssl"))]
pub(crate) fn send(socket: &UdpSocket, request: &Request, address: &SocketAddr) {
    request.send(socket, address);
}

#[cfg(feature = "ssl")]
pub(crate) fn send(socket: &UdpSocket, request: &Request, address: &SocketAddr) {
    request.send(socket, address, &vec![]);
}
