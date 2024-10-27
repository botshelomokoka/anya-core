//! Anya Core: A decentralized AI assistant framework
//!
//! This library provides the core functionality for the Anya project.

#![warn(missing_docs)]
#![warn(clippy::all)]

use std::sync::Arc;
use anyhow::{Result, Error};
use slog::{info, o, Drain, Logger};
use tokio::sync::Mutex;

// Core modules
pub mod blockchain;
pub mod identity;
pub mod ml;
pub mod network;
pub mod secure_storage;
pub mod secrets;
pub mod gorules;
pub mod config;

// Feature modules
pub mod smart_contracts;
pub mod interoperability;
pub mod privacy;
pub mod federated_learning;

// Infrastructure

pub mod data_storage;
pub mod user_management;

// Re-exports
pub use blockchain::{BitcoinCore, Lightning, Stacks};
pub use identity::{DIDManager, VerifiableCredential};
pub use ml::{MLModel, FederatedLearning, MLManager};
pub use network::{NetworkDiscovery, ConnectionManager};
pub use gorules::GoRulesManager;
pub use config::AnyaConfig;

use crate::ml::directory_manager::DirectoryManager;
use crate::config::Config;

/// Main Anya struct that holds all components
pub struct Anya {
    config: Config,
    logger: Logger,
    directory_manager: Arc<DirectoryManager>,
    bitcoin_core: Arc<BitcoinCore>,
    lightning: Arc<Lightning>,
    identity: Arc<DIDManager>,
}

impl Anya {
    /// Create a new Anya instance
    pub async fn new() -> Result<Self> {
        let config = Config::new()?;
        let logger = init_logger();
        let directory_manager = Arc::new(DirectoryManager::new().await?);

        let bitcoin_core = Arc::new(BitcoinCore::new(&config)?);
        let lightning = Arc::new(Lightning::new(&config)?);
        let identity = Arc::new(DIDManager::new(&config)?);

        Ok(Self {
            config,
            logger,
            directory_manager,
            bitcoin_core,
            lightning,
            identity,
        })
    }

    /// Initialize the system
    pub async fn init(&self) -> Result<()> {
        info!(self.logger, "Initializing Anya Core");
        
        // Scan and organize directory structure
        self.directory_manager.scan_directory().await?;
        
        Ok(())
    }

    /// Get the current Bitcoin block count
    pub async fn get_bitcoin_block_count(&self) -> Result<u64> {
        self.bitcoin_core.get_block_count().await
    }

    /// Create a new DID
    pub async fn create_did(&self) -> Result<String> {
        self.identity.create_did().await
    }
}

/// Initialize the logger
fn init_logger() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_async::Async::new(
        slog_term::FullFormat::new(decorator).build().fuse()
    ).build().fuse();
    
    Logger::root(drain, o!("version" => env!("CARGO_PKG_VERSION")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_anya_init() {
        let anya = Anya::new().await.unwrap();
        assert!(anya.init().await.is_ok());
    }
}
