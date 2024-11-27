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
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Bitcoin key derivation error: {0}")]
    KeyDerivation(String),
    
    #[error("Signing error: {0}")]
    Signing(String),
    
    #[error("Invalid key format: {0}")]
    InvalidKey(String),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    
    #[error("Invalid credentials: {0}")]
    InvalidCredentials(String),
}


