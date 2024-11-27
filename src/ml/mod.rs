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
//! Machine Learning module provides AI and ML capabilities.
use anyhow::Result;
use log::{info, error};

// Current ML Module Structure
pub mod core;
pub mod agents;
pub mod web5;
pub mod nlp;
pub mod research;
pub mod federated;
pub mod monitoring;
pub mod types;

pub use core::{MLCore, MLInput, MLOutput};
pub use agents::MLAgent;
pub use web5::Web5MLIntegration;
pub use nlp::NaturalLanguageProcessor;
pub use research::ResearchModule;
pub use federated::FederatedLearningModule;
pub use monitoring::MLMonitor;
pub use types::{MLConfig, MLMetrics};

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

// Re-export common ML types
pub use ndarray::{Array1, Array2};
pub use tch::{Tensor, Device};


