use std::env;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::secure_storage::SecureStorage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Secrets {
    bitcoin_rpc_auth: String,
    api_keys: ApiKeys,
    enterprise_license: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ApiKeys {
    lightning_api_key: String,
    stacks_api_key: String,
}

impl Secrets {
    pub fn load() -> Result<Self> {
        // First try environment variables
        if let Ok(secrets) = Self::from_env() {
            return Ok(secrets);
        }

        // Then try secure storage
        let storage = SecureStorage::new("anya");
        match storage.get::<Self>("secrets") {
            Ok(secrets) => Ok(secrets),
            Err(_) => anyhow::bail!("Could not load secrets from any source")
        }
    }

    pub fn save(&self) -> Result<()> {
        let storage = SecureStorage::new("anya");
        storage.store("secrets", self)
    }

    fn from_env() -> Result<Self> {
        Ok(Self {
            bitcoin_rpc_auth: env::var("ANYA_BITCOIN_RPC_AUTH")?,
            api_keys: ApiKeys {
                lightning_api_key: env::var("ANYA_LIGHTNING_API_KEY")?, 
                stacks_api_key: env::var("ANYA_STACKS_API_KEY")?,
            },
            enterprise_license: env::var("ANYA_ENTERPRISE_LICENSE").ok(),
        })
    }
}
