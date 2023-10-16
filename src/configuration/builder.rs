use super::*;

/// Builder for create configuration.
pub struct ConfigurationBuilder {
    instance: Configuration,
}

impl ConfigurationBuilder {
    #[cfg(not(feature = "ssl"))]
    pub(crate) fn new() -> Self {
        Self {
            instance: Configuration::new(),
        }
    }

    #[cfg(feature = "ssl")]
    pub(crate) fn new(ssl: SSL) -> Self {
        Self {
            instance: Configuration::new(ssl),
        }
    }

    /// Specify the port for the Peer socket.
    pub fn port(mut self, port: u16) -> Self {
        self.instance.port = port;
        self
    }

    /// Share peers connected with the other peers.
    pub fn share_connections(mut self, share: bool) -> Self {
        self.instance.share_connections = share;
        self
    }

    /// Create the configuration.
    pub fn build(self) -> Configuration {
        self.instance
    }
}
