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
pub enum IdentityError {
    #[error("DID resolution failed: {0}")]
    ResolutionError(String),

    #[error("Credential verification failed: {0}")]
    VerificationError(String),

    #[error("Invalid credential format: {0}")]
    InvalidCredential(String),

    #[error("Credential has expired")]
    CredentialExpired,

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("DID parse error: {0}")]
    DIDParseError(String),

    #[error("JSON serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Invalid proof: {0}")]
    InvalidProof(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Web5 DWN error: {0}")]
    DWNError(String),

    #[error("Hex decode error: {0}")]
    HexError(#[from] hex::FromHexError),

    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("Decryption error: {0}")]
    DecryptionError(String),

    #[error("Credential revoked")]
    CredentialRevoked,

    #[error("Invalid credential update: {0}")]
    InvalidUpdate(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Invalid schema: {0}")]
    InvalidSchema(String),

    #[error("Network timeout")]
    NetworkTimeout,

    #[error("Synchronization error: {0}")]
    SyncError(String),
}

impl From<did_key::Error> for IdentityError {
    fn from(err: did_key::Error) -> Self  -> Result<(), Box<dyn Error>> {
        Self::ResolutionError(err.to_string())
    }
}


