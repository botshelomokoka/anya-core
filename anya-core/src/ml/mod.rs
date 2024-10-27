//! Machine Learning module for Anya Core
use anyhow::Result;
use log::{info, error};

// Current ML Module Structure
pub mod core;
pub mod logic;
pub mod dao_rules;
pub mod federated_learning;
pub mod system_evaluation;
pub mod differential_privacy;
pub mod secure_aggregation;

// Bitcoin-specific features
mod bitcoin_models;
mod mlfee;

// Re-exports
pub use self::core::MLCore;
pub use self::types::{MLInput, MLOutput, MLError};
pub use self::manager::MLManager;
pub use self::bitcoin_models::BitcoinPricePredictor;

/// Initialize the ML module
pub async fn init() -> Result<()> {
    info!("Initializing ML module");
    
    // Initialize core components
    federated_learning::init().await?;
    differential_privacy::init().await?;
    secure_aggregation::init().await?;
    
    info!("ML module initialized successfully");
    Ok(())
}
