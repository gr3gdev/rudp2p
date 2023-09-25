use super::*;

pub struct ConfigurationBuilder {
    instance: Configuration,
}

impl ConfigurationBuilder {
    pub(crate) fn new() -> Self {
        Self {
            instance: Configuration::default(),
        }
    }

    pub fn port(mut self, port: u16) -> Self {
        self.instance.port = port;
        self
    }

    pub fn share_connections(mut self, share: bool) -> Self {
        self.instance.share_connections = share;
        self
    }

    pub fn database(mut self, mode: DatabaseMode) -> Self {
        self.instance.database_mode = mode;
        self
    }

    pub fn build(self) -> Configuration {
        self.instance
    }
}
