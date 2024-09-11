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
pub mod core {
    // Core functionality, free and open-source
    pub use crate::blockchain;
    pub use crate::networking;
    pub use crate::identity;
}

pub mod standard {
    // Standard features, free and open-source
    pub use crate::smart_contracts;
    pub use crate::federated_learning;
}

#[cfg(feature = "enterprise")]
pub mod enterprise {
    // Enterprise features, requires paid license
    pub mod advanced_analytics;
    pub mod high_volume_trading;
}

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

pub mod core;
pub mod network;
pub mod blockchain;
pub mod federated_learning;
pub mod identity;
pub mod smart_contracts;
pub mod interoperability;
pub mod privacy;
pub mod ui;

pub mod dlc_support;
pub mod kademlia;

use crate::network::{
    bitcoinadapter::BitcoinAdapter,
    lightningadapter::LightningAdapter,
    ipfsadapter::IPFSAdapter,
    stacksadapter::StacksAdapter,
};

// Re-export important traits and types
pub use crate::core::{NetworkNode, NetworkType, NetworkDiscovery, ConnectionManager, AdapterRunner};

// Initialize and run all network adapters
pub async fn run_network_adapters() {
    let bitcoin_adapter = Arc::new(BitcoinAdapter::new(/* params */));
    let lightning_adapter = Arc::new(LightningAdapter::new(/* params */));
    let ipfs_adapter = Arc::new(IPFSAdapter::new(/* params */));
    let stacks_adapter = Arc::new(StacksAdapter::new(/* params */));

    tokio::join!(
        bitcoin_adapter.run(),
        lightning_adapter.run(),
        ipfs_adapter.run(),
        stacks_adapter.run()
    );
}

// Other initialization and utility functions

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

pub mod interlink;

pub mod ml;
pub mod interlink;

// Re-export important structs and functions
pub use ml::{MLModel, SimpleLinearRegression};
pub use interlink::Interlink;

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

pub mod interlink;

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
