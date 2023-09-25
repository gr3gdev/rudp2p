use self::builder::ConfigurationBuilder;

mod builder;

#[derive(Debug, Default, Clone)]
pub enum DatabaseMode {
    #[default]
    Memory,
    File(String),
}

#[derive(Debug, Default, Clone)]
pub struct Configuration {
    pub(crate) port: u16,
    pub(crate) share_connections: bool,
    pub(crate) database_mode: DatabaseMode,
}

impl Configuration {
    pub fn builder() -> ConfigurationBuilder {
        ConfigurationBuilder::new()
    }
}
