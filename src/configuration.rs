use self::builder::ConfigurationBuilder;

mod builder;

#[cfg(feature = "sqlite")]
#[derive(Debug, Default, Clone)]
pub enum SqliteMode {
    #[default]
    Memory,
    File(String),
}

#[derive(Debug, Default, Clone)]
pub struct Configuration {
    pub(crate) port: u16,
    pub(crate) share_connections: bool,
    #[cfg(feature = "sqlite")]
    pub(crate) database_mode: SqliteMode,
    #[cfg(feature = "mysql")]
    pub(crate) database_url: Option<String>,
}

impl Configuration {
    pub fn builder() -> ConfigurationBuilder {
        ConfigurationBuilder::new()
    }
}
