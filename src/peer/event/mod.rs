use std::net::SocketAddr;

pub static DISCONNECTING: u8 = 0;
pub static CONNECTING: u8 = 1;
pub static CONNECTED: u8 = 2;
pub static MESSAGE: u8 = 3;
pub static DISCONNECTED: u8 = 9;

pub trait AsBytes {
    /// Convert struct to bytes u8.
    fn as_bytes(&self) -> Vec<u8>;
}

pub struct PeerEvent {
    /// Code of the peer event.
    pub code: u8,
    /// Content message of the peer event.
    pub message: Vec<u8>,
}

impl AsBytes for PeerEvent {
    fn as_bytes(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.push(self.code);
        for m in self.message.clone() {
            data.push(m);
        }
        data
    }
}

impl PeerEvent {
    /// Event: connecting.
    pub fn connecting() -> PeerEvent {
        PeerEvent {
            code: CONNECTING,
            message: Vec::new(),
        }
    }

    /// Event: disconnecting.
    pub fn disconnecting() -> PeerEvent {
        PeerEvent {
            code: DISCONNECTING,
            message: Vec::new(),
        }
    }

    /// Event: connected.
    pub fn connected(addr: SocketAddr) -> PeerEvent {
        PeerEvent {
            code: CONNECTED,
            message: addr.to_string().as_bytes().to_vec(),
        }
    }

    /// Event: disconnected.
    pub fn disconnected(addr: SocketAddr) -> PeerEvent {
        PeerEvent {
            code: DISCONNECTED,
            message: addr.to_string().as_bytes().to_vec(),
        }
    }

    /// Event: message.
    pub fn message(message: Vec<u8>) -> PeerEvent {
        PeerEvent {
            code: MESSAGE,
            message,
        }
    }
}
