use self::builder::ConfigurationBuilder;

mod builder;

#[derive(Debug, Default)]
pub struct Configuration {
    pub(crate) port: u16,
    pub(crate) name: Option<String>,
    pub(crate) share_connections: bool,
}

impl Configuration {
    pub fn builder() -> ConfigurationBuilder {
        ConfigurationBuilder::new()
    }
}
