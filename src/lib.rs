use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    sync::{Arc, Mutex},
    thread::JoinHandle,
};

use openssl::{error::ErrorStack, pkey::Private, rsa::Rsa};

pub(crate) mod encoder;
pub(crate) mod error;
pub(crate) mod event;
pub(crate) mod flows;
pub(crate) mod internaltypeevent;
pub(crate) mod messagemethod;
pub(crate) mod typeevent;

pub mod message;
pub mod peer;

/// # Peer
///
/// Example
/// ```
/// use rudp2plib::{Message, Peer, TypeEvent};
/// 
/// let peer1 = Peer::start(9001, Some("Peer1"), true, move |event| {
///     if event.type_event == TypeEvent::CONNECTED {
///         Some(Message::new("Hello I am Peer1 !".as_bytes().to_vec()))
///     } else {
///         None
///     }
/// }).unwrap();
///
/// let peer2 = Peer::start(9002, Some("Peer2"), true, move |event| {
///     if event.type_event == TypeEvent::CONNECTED {
///         Some(Message::new("Hello I am Peer2 !".as_bytes().to_vec()))
///     } else {
///         None
///     }
/// }).unwrap();
///
/// peer1.connect_to(peer2.addr());
/// 
/// peer1.close();
/// peer2.close();
/// ```
///
pub struct Peer {
    /// Unique identifier.
    pub uid: String,
    /// Current thread.
    job: Option<JoinHandle<()>>,
    /// UDP socket.
    udp_socket: UdpSocket,
    /// The other peers connected.
    peers: Arc<Mutex<HashMap<String, RemotePeer>>>,
    /// The list of uid's peer rejected.
    rejects: Arc<Mutex<Vec<String>>>,
    /// PEM of the public key for remote encryption.
    public_key_pem: Vec<u8>,
    /// Share connection with other peers.
    share_peers: bool,
}

/// # Message
///
/// Structure for a message.
#[derive(Clone)]
pub struct Message {
    uid: String,
    event: InternalTypeEvent,
    method: MessageMethod,
    start: usize,
    total: usize,
    pub data: Vec<u8>,
}

/// # Error
#[derive(Debug)]
pub struct Error {
    /// The error message.
    pub message: String,
    io_error: Option<std::io::Error>,
    ssl_error: Option<ErrorStack>,
}

/// # Event
///
/// Struct return by the Peer's listener when an event is fired.
#[derive(Debug)]
pub struct Event {
    pub type_event: TypeEvent,
    pub to: String,
    pub from: String,
    pub message: Option<Message>,
}

/// # InternalPeer
///
/// Internal peer data.
pub(crate) struct InternalPeer {
    uid: String,
    public_key_pem: Vec<u8>,
    private_key: Option<Rsa<Private>>,
    addr: SocketAddr,
    peers_connected: Option<HashMap<String, RemotePeer>>,
    rejects: Vec<String>,
    share_peers: bool,
}

/// # RemotePeer
///
/// Remote peer data.
#[derive(Clone)]
pub(crate) struct RemotePeer {
    uid: String,
    addr: SocketAddr,
    public_key: Option<Vec<u8>>,
}

/// # TypeEvent
///
/// - CONNECTED
/// - DISCONNECTED
/// - MESSAGE
#[derive(Debug, Clone, PartialEq)]
pub enum TypeEvent {
    CONNECTED,
    DISCONNECTED,
    MESSAGE,
}

/// # InternalTypeEvent
///
/// - CONNECTING
/// - DISCONNECTING
/// - MESSAGE
/// - PEERS
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum InternalTypeEvent {
    CONNECTING,
    DISCONNECTING,
    MESSAGE,
    PEERS,
}

/// # MessageMethod
///
/// Type of message :
/// - Request
/// - Response
/// - RequestAndResponse
/// - Other (for external messages)
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum MessageMethod {
    Request,
    Response,
    RequestAndResponse,
    Other,
}
