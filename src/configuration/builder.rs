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

    pub fn name(mut self, name: &str) -> Self {
        self.instance.name = Some(String::from(name));
        self
    }

    pub fn share_connections(mut self, share: bool) -> Self {
        self.instance.share_connections = share;
        self
    }

    pub fn build(self) -> Configuration {
        self.instance
    }
}
