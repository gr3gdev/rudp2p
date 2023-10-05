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

    #[cfg(feature = "sqlite")]
    pub fn database(mut self, mode: SqliteMode) -> Self {
        self.instance.database_mode = mode;
        self
    }

    #[cfg(feature = "mysql")]
    pub fn database(mut self, url: &str) -> Self {
        self.instance.database_url = Some(String::from(url));
        self
    }

    pub fn build(self) -> Configuration {
        self.instance
    }
}
