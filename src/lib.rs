//! Anya Core: A decentralized AI assistant framework
//!
//! This library provides the core functionality for the Anya project.

#![warn(missing_docs)]
#![warn(clippy::all)]

use slog::{info, o, Drain, Logger};
use std::sync::Mutex;
use config::{Config, ConfigError};

/// Initialize the logger for the Anya Core system
pub fn init_logger() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = Mutex::new(slog_term::FullFormat::new(decorator).build()).fuse();
    let logger = Logger::root(drain, o!("version" => env!("CARGO_PKG_VERSION")));
    info!(logger, "Anya Core logger initialized");
    logger
}

/// Main configuration structure for Anya Core
#[derive(Debug, Clone)]
pub struct AnyaConfig {
    pub log_level: String,
    pub api_key: String,
    pub network_type: String,
}

impl AnyaConfig {
    /// Create a new AnyaConfig instance
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(config::Environment::with_prefix("ANYA"))
            .build()?;

        Ok(AnyaConfig {
            log_level: config.get_string("log_level").unwrap_or_else(|_| "info".to_string()),
            api_key: config.get_string("api_key").unwrap_or_default(),
            network_type: config.get_string("network_type").unwrap_or_else(|_| "testnet".to_string()),
        })
    }
}

// Core modules (open source)
pub mod bitcoin_core;
pub mod lightning;
pub mod dlc;
pub mod ml_logic;

// Enterprise modules (API access)
#[cfg(feature = "enterprise")]
pub mod advanced_analytics;
#[cfg(feature = "enterprise")]
pub mod high_volume_trading;

// Add more modules as needed
pub mod user_management;
pub mod network_discovery;
pub mod blockchain;
pub mod identity;
pub mod data_storage;
pub mod smart_contracts;
pub mod interoperability;
pub mod privacy;
pub mod ui;

// Re-export important structs and functions
pub use user_management::UserManagement;
pub use network_discovery::NetworkDiscovery;
pub use blockchain::{BitcoinSupport, LightningSupport, StacksSupport, DLCSupport};
pub use ml_logic::FederatedLearning;
pub use identity::{DIDManager, VerifiableCredential};
pub use data_storage::{IPFSStorage, OrbitDB};
pub use smart_contracts::{ClarityContract, WasmContract};
pub use interoperability::{IBCProtocol, CosmosSDK, Polkadot};
pub use privacy::{ZeroKnowledgeProof, HomomorphicEncryption, SecureMultiPartyComputation};
pub use ui::{WebInterface, CLI, MobileApp};

// Re-export important structs and functions
pub use user_management::UserManagement;
pub use network_discovery::NetworkDiscovery;
pub use blockchain::{BitcoinSupport, LightningSupport, StacksSupport, DLCSupport};
pub use ml_logic::FederatedLearning;
pub use identity::{DIDManager, VerifiableCredential};
pub use data_storage::{IPFSStorage, OrbitDB};
pub use smart_contracts::{ClarityContract, WasmContract};
pub use interoperability::{IBCProtocol, CosmosSDK, Polkadot};
pub use privacy::{ZeroKnowledgeProof, HomomorphicEncryption, SecureMultiPartyComputation};
pub use ui::{WebInterface, CLI, MobileApp};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_logger() {
        let logger = init_logger();
        info!(logger, "Test log message");
    }

    #[test]
    fn test_anya_config() {
        let config = AnyaConfig::new().expect("Failed to create AnyaConfig");
        assert!(format!("{:?}", config).contains("AnyaConfig"));
    }
}
