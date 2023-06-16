use std::fmt::Display;
use std::io::Error;
use std::net::AddrParseError;

use openssl::error::ErrorStack;

use crate::PeerError;

impl Display for PeerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PeerError")
            .field("message", &self.message)
            .field("io_cause", &self.io_cause)
            .field("ssl_cause", &self.ssl_cause)
            .field("addr_cause", &self.addr_cause)
            .finish()
    }
}

impl PeerError {
    pub(crate) fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            io_cause: None,
            ssl_cause: None,
            addr_cause: None,
        }
    }

    pub(crate) fn new_io(message: &str, error: Error) -> Self {
        Self {
            message: message.to_string(),
            io_cause: Some(error),
            ssl_cause: None,
            addr_cause: None,
        }
    }

    pub(crate) fn new_ssl(message: &str, error: ErrorStack) -> Self {
        Self {
            message: message.to_string(),
            io_cause: None,
            ssl_cause: Some(error),
            addr_cause: None,
        }
    }

    pub(crate) fn new_addr(message: &str, error: AddrParseError) -> Self {
        Self {
            message: message.to_string(),
            io_cause: None,
            ssl_cause: None,
            addr_cause: Some(error),
        }
    }
}
