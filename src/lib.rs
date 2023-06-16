use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    net::{AddrParseError, SocketAddr, UdpSocket},
    sync::{Arc, Mutex},
    thread::JoinHandle,
};

use openssl::error::ErrorStack;
use openssl::{pkey::Private, rsa::Rsa};

pub(crate) mod logger;
pub mod peer;
pub mod peererror;
pub mod peerevent;
pub(crate) mod peerinternalevent;
pub mod peermessage;
pub(crate) mod peerserver;
pub mod peersocket;
pub(crate) mod utils;

/// # PeerServer
///
/// Internal UDP server for exchange message with others peers.
pub(crate) struct PeerServer {
    /// UDP socket of the peer.
    pub(crate) socket: PeerSocket,
    /// The current thread.
    job: Option<JoinHandle<()>>,
    /// List of remote peers.
    peers: Arc<Mutex<HashMap<String, RemotePeer>>>,
}

/// # PeerSocket
///
/// Struct with list of others peers and UDP socket for send and receive messages.
pub struct PeerSocket {
    /// List of remote peers.
    peers: HashMap<String, RemotePeer>,
    /// UDP socket.
    socket: UdpSocket,
    /// Public key for encryption.
    public_key_pem: Vec<u8>,
    /// Private key for decryption.
    private_key: Rsa<Private>,
}

#[derive(Clone, Debug)]
pub(crate) struct RemotePeer {
    pub(crate) uid: String,
    pub(crate) address: SocketAddr,
    pub(crate) public_key_pem: Vec<u8>,
}

/// # Peer
///
/// Struct of a peer.
#[derive(Debug)]
pub struct Peer {
    pub uid: String,
    server: PeerServer,
    blacklist: Arc<Mutex<Vec<String>>>,
}

/// # PeerError
///
/// Struct for throw errors.
#[derive(Debug)]
pub struct PeerError {
    pub message: String,
    io_cause: Option<std::io::Error>,
    ssl_cause: Option<ErrorStack>,
    addr_cause: Option<AddrParseError>,
}

/// # PeerEvent
///
/// Enumeration with the list of events.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum PeerInternalEvent {
    CONNECTING,
    CONNECTED,
    MESSAGE,
    DISCONNECTING,
    DISCONNECTED,
}

/// # PeerTypeEvent
///
/// Enumeration with the list of type events.
#[derive(Clone, Debug, PartialEq)]
pub enum PeerTypeEvent {
    CONNECTED,
    MESSAGE,
    DISCONNECTED,
}

impl Display for PeerTypeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PeerTypeEvent::CONNECTED => write!(f, "CONNECTED"),
            PeerTypeEvent::MESSAGE => write!(f, "MESSAGE"),
            PeerTypeEvent::DISCONNECTED => write!(f, "DISCONNECTED"),
        }
    }
}

/// # PeerEvent
///
/// The event send to the Peer listener.
#[derive(Debug)]
pub struct PeerEvent {
    pub type_event: PeerTypeEvent,
    pub to: String,
    pub from: String,
    pub socket: PeerSocket,
    pub data: Vec<u8>,
}

/// # Response
///
/// See PeerEvent.
#[derive(Debug, Clone)]
pub(crate) struct Response {
    message: PeerMessage,
    pub(crate) to: String,
    address: SocketAddr,
}

/// # PeerMessage
///
/// Message exchange with others peers.
#[derive(Clone, Debug)]
pub(crate) struct PeerMessage {
    pub(crate) uid: String,
    pub(crate) event: PeerInternalEvent,
    pub(crate) start: usize,
    pub(crate) total: usize,
    pub(crate) content: Vec<u8>,
}

/// # PeerConnectedInfo
///
/// Informations send when connected.
#[derive(Clone, Debug)]
pub(crate) struct PeerConnectedInfo {
    pub(crate) uid: String,
    pub(crate) remote_peers: HashMap<String, SocketAddr>,
}

/// # PeerConnectingInfo
///
/// Informations send when connecting.
#[derive(Clone, Debug)]
pub(crate) struct PeerConnectingInfo {
    pub(crate) uid: String,
    pub(crate) public_key: Vec<u8>,
}

/// # PeerDisconnectingInfo
///
/// Informations send when disconnecting.
#[derive(Clone, Debug)]
pub(crate) struct PeerDisconnectingInfo {
    pub(crate) uid: String,
}

/// # PeerDisconnectedInfo
///
/// Informations send when disconnected.
#[derive(Clone, Debug)]
pub(crate) struct PeerDisconnectedInfo {
    pub(crate) uid: String,
}

/// # PeerMessageInfo
///
/// Informations send when message sent.
pub(crate) struct PeerMessageInfo {
    pub(crate) content: Vec<u8>,
}

/// # Content
///
/// Trait for convert messages.
pub(crate) trait Content {
    fn to_vec(&self, public_key_pem: Option<Vec<u8>>) -> Result<Vec<u8>, PeerError>;
}

/// # Info
///
/// Trait for internal events.
pub(crate) trait Info {
    fn parse(message: &PeerMessage, decrypt_data: Option<Rsa<Private>>) -> Result<Self, PeerError>
    where
        Self: Sized;
}

/// # Message
pub trait Message {
    /// Content of the message.
    fn content(&self) -> Vec<u8>;
}

/// # Socket
///
/// Trait for send requests.
pub trait Socket {
    fn send_to<M>(&self, message: M, to: String) -> Result<(), PeerError>
    where
        M: Message;
    fn send_all<M>(&self, message: M) -> Result<(), PeerError>
    where
        M: Message;
}
