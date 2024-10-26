use serde::Deserialize;
use config::{Config, ConfigError, File, Environment};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub database_url: String,
    pub server_port: u16,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();
        s.merge(File::with_name("config/default"))?;
        s.merge(File::with_name("config/local").required(false))?;

        s.try_into()
    }
}