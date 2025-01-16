use std::env;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Environment variable error: {0}")]
    EnvVar(#[from] std::env::VarError),
    #[error("Invalid auth provider: {0}")]
    InvalidAuthProvider(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub auth: AuthConfig,
    pub metrics: MetricsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub provider_type: AuthProviderType,
    pub credentials: AuthCredentials,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthProviderType {
    Stacks,
    Lightning,
    Web5,
    Default,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthCredentials {
    pub api_key: String,
    pub endpoint: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let provider_type = match env::var("AUTH_PROVIDER_TYPE")
            .unwrap_or_else(|_| "default".to_string())
            .to_lowercase()
            .as_str() 
        {
            "stacks" => AuthProviderType::Stacks,
            "lightning" => AuthProviderType::Lightning,
            "web5" => AuthProviderType::Web5,
            "default" => AuthProviderType::Default,
            invalid => return Err(ConfigError::InvalidAuthProvider(invalid.to_string())),
        };

        Ok(Self {
            auth: AuthConfig {
                provider_type,
                credentials: AuthCredentials {
                    api_key: env::var("AUTH_API_KEY").expect("AUTH_API_KEY must be set"),
                    endpoint: env::var("AUTH_ENDPOINT").expect("AUTH_ENDPOINT must be set"),
                }
            },
            // ... other config
        })
    }
}
