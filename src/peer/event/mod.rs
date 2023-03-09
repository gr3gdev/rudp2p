use std::net::SocketAddr;

pub static DISCONNECTING: u8 = 0;
pub static CONNECTING: u8 = 1;
pub static CONNECTED: u8 = 2;
pub static MESSAGE: u8 = 3;
pub static DISCONNECTED: u8 = 9;

pub trait AsBytes {
    fn as_bytes(&self) -> Vec<u8>;
}

pub struct PeerEvent {
    pub code: u8,
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
    pub fn connecting() -> PeerEvent {
        PeerEvent {
            code: CONNECTING,
            message: Vec::new(),
        }
    }

    pub fn disconnecting() -> PeerEvent {
        PeerEvent {
            code: DISCONNECTING,
            message: Vec::new(),
        }
    }

    pub fn connected(addr: SocketAddr) -> PeerEvent {
        PeerEvent {
            code: CONNECTED,
            message: addr.to_string().as_bytes().to_vec(),
        }
    }

    pub fn disconnected(addr: SocketAddr) -> PeerEvent {
        PeerEvent {
            code: DISCONNECTED,
            message: addr.to_string().as_bytes().to_vec(),
        }
    }

    pub fn message(message: Vec<u8>) -> PeerEvent {
        PeerEvent {
            code: MESSAGE,
            message,
        }
    }
}
