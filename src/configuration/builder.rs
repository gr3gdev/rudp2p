use super::*;

/// Builder for create configuration.
pub struct ConfigurationBuilder {
    instance: Configuration,
}

impl ConfigurationBuilder {
    #[cfg(not(feature = "ssl"))]
    pub(crate) fn new() -> Self {
        Self {
            instance: Configuration::default(),
        }
    }

    #[cfg(feature = "ssl")]
    pub(crate) fn new(ssl: SSL) -> Self {
        Self {
            instance: Configuration::default_with_ssl(ssl),
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

    /// Change the database upgrate mode.
    pub fn upgrade_mod(mut self, mode: DatabaseUpgradeMode) -> Self {
        self.instance.database_upgrade_mode = mode;
        self
    }

    /// Configure sqlite database in memory (default) or with a file.
    #[cfg(feature = "sqlite")]
    pub fn database(mut self, mode: SqliteMode) -> Self {
        self.instance.database_mode = mode;
        self
    }

    /// Configure mysql database url.
    #[cfg(feature = "mysql")]
    pub fn database(mut self, url: &str) -> Self {
        self.instance.database_url = Some(String::from(url));
        self
    }

    /// Create the configuration.
    pub fn build(self) -> Configuration {
        self.instance
    }
}
