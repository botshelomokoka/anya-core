use serde::Deserialize;
use config::{Config, ConfigError, File, Environment};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub database_url: String,
    pub server_port: u16,
    pub rate_limit: RateLimitConfig,
}

#[derive(Debug, Deserialize)]
pub struct RateLimitConfig {
    pub capacity: u32,
    pub refill_rate: f64,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();
        s.merge(File::with_name("config/default"))?;
        s.merge(File::with_name("config/local").required(false))?;
        s.merge(Environment::with_prefix("APP"))?;

        s.try_into()
    }
}
