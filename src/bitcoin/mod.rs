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
//! `rust
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
//! Bitcoin module provides core Bitcoin functionality and Layer 2 integrations.

pub mod core;
pub mod lightning;
pub mod dlc;
pub mod privacy;
pub mod taproot;
pub mod standards;

pub use core::BitcoinCore;
pub use lightning::Lightning;
pub use dlc::DLCManager;
pub use privacy::PrivacyModule;
pub use taproot::TaprootModule;

// Re-export common Bitcoin types
pub use bitcoin::{
    Network,
    Transaction,
    Address,
    OutPoint,
    Script,
    ScriptBuf,
    util::psbt::PartiallySignedTransaction,
};


