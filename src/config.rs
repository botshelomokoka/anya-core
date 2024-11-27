//! Module documentation for $moduleName
//!
//! # Overview
//! This module is part of the Anya Core project, located at $modulePath.
//!
//! # Architecture
//! [Add module-specific architecture details]
//!
//! # API Reference
//! [Document public functions and types]
//!
//! # Usage Examples
//! `ust
//! // Add usage examples
//! `
//!
//! # Error Handling
//! This module uses proper error handling with Result types.
//!
//! # Security Considerations
//! [Document security features and considerations]
//!
//! # Performance
//! [Document performance characteristics]

use std::error::Error;
use serde::{Deserialize, Serialize};
use config::{Config, ConfigError, Environment};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnyaConfig {
    // Network configuration
    pub network_type: String,
    pub bitcoin_network: String,
    
    // RPC configuration
    pub bitcoin_rpc_url: String,
    
    // System configuration
    pub log_level: String,
    pub data_dir: PathBuf,
    
    // ML configuration
    pub ml_config: MLConfig,
    
    // Private configuration
    #[serde(skip_serializing)]
    secrets: Secrets,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLConfig {
    pub federated_learning_enabled: bool,
    pub privacy_threshold: f64,
    pub model_update_interval: u64,
}

impl AnyaConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let mut config = Config::new();
        
        // Set defaults
        config.set_default("network_type", "testnet")?;
        config.set_default("bitcoin_network", "testnet")?;
        config.set_default("log_level", "info")?;
        
        // Load from environment
        config.merge(Environment::with_prefix("ANYA"))?;
        
        // Load secrets
        let secrets = Secrets::load()?;
        
        config.try_into().map(|mut c: AnyaConfig| {
            c.secrets = secrets;
            c
        })
    }
}


