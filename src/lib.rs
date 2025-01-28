//! Anya Core Library
//!
//! This is the core library for the Anya AI IDE system, providing fundamental
//! functionality for machine learning, Web5 integration, and Bitcoin operations.
//!
//! # Architecture
//!
//! The library is organized into several main modules:
//! - `ml`: Machine learning components and AI agent system
//! - `web5`: Web5 protocol integration and decentralized identity
//! - `bitcoin`: Bitcoin and Lightning Network functionality
//! - `utils`: Common utilities and helper functions
//!
//! # Features
//!
//! - Advanced ML capabilities with PyTorch integration
//! - Web5 protocol implementation for decentralized data management
//! - Bitcoin and Lightning Network support
//! - Comprehensive security and privacy features
//!
//! # Examples
//!
//! ```rust
//! use anya_core::{ml, web5, bitcoin};
//!
//! async fn example() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize ML system
//!     let ml_system = ml::MLSystem::new()?;
//!
//!     // Set up Web5 DID
//!     let did = web5::identity::DID::new()?;
//!
//!     // Create Bitcoin wallet
//!     let wallet = bitcoin::wallet::HDWallet::new()?;
//!
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(clippy::cargo)]
#![deny(clippy::nursery)]

use std::error::Error;
use std::fmt;

pub mod ml;
pub mod web5;
pub mod bitcoin;
pub mod utils;

/// Core error type for the Anya system
#[derive(Debug)]
pub enum AnyaError {
    /// ML-related errors
    ML(String),
    /// Web5-related errors
    Web5(String),
    /// Bitcoin-related errors
    Bitcoin(String),
    /// General system errors
    System(String),
}

impl fmt::Display for AnyaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnyaError::ML(msg) => write!(f, "ML error: {}", msg),
            AnyaError::Web5(msg) => write!(f, "Web5 error: {}", msg),
            AnyaError::Bitcoin(msg) => write!(f, "Bitcoin error: {}", msg),
            AnyaError::System(msg) => write!(f, "System error: {}", msg),
        }
    }
}

impl Error for AnyaError {}

/// Result type for Anya operations
pub type AnyaResult<T> = Result<T, AnyaError>;

/// Core configuration for the Anya system
#[derive(Debug, Clone)]
pub struct AnyaConfig {
    /// ML system configuration
    pub ml_config: ml::MLConfig,
    /// Web5 configuration
    pub web5_config: web5::Web5Config,
    /// Bitcoin network configuration
    pub bitcoin_config: bitcoin::BitcoinConfig,
}

impl Default for AnyaConfig {
    fn default() -> Self {
        Self {
            ml_config: ml::MLConfig::default(),
            web5_config: web5::Web5Config::default(),
            bitcoin_config: bitcoin::BitcoinConfig::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = AnyaConfig::default();
        assert!(config.ml_config.enabled);
        assert!(config.web5_config.enabled);
        assert!(config.bitcoin_config.enabled);
    }

    #[test]
    fn test_error_display() {
        let err = AnyaError::ML("test error".to_string());
        assert_eq!(err.to_string(), "ML error: test error");
    }
}