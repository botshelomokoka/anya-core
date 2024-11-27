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

#[derive(Error, Debug)]
pub enum AnyaError {
    #[error("Bitcoin error: {0}")]
    Bitcoin(#[from] BitcoinError),
    
    #[error("DLC error: {0}")]
    DLC(#[from] DLCError),
    
    #[error("ML error: {0}")]
    ML(#[from] MLError),
    
    #[error("Privacy error: {0}")]
    Privacy(#[from] PrivacyError),
    
    #[error("Security error: {0}")]
    Security(#[from] SecurityError),
    
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    
    #[error("Network error: {0}")]
    Network(#[from] NetworkError),
    
    #[error("Identity error: {0}")]
    Identity(#[from] IdentityError),
    
    #[error("Interoperability error: {0}")]
    Interop(#[from] InteroperabilityError),
    
    #[error("Data pipeline error: {0}")]
    DataPipeline(#[from] DataPipelineError),
}

#[derive(Error, Debug)]
pub enum BitcoinError {
    #[error("RPC error: {0}")]
    RpcError(String),
    #[error("Transaction error: {0}")]
    TransactionError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Quantum resistance error: {0}")]
    QuantumError(String),
}

#[derive(Error, Debug)]
pub enum DLCError {
    #[error("Contract error: {0}")]
    ContractError(String),
    #[error("Oracle error: {0}")]
    OracleError(String),
    #[error("Execution error: {0}")]
    ExecutionError(String),
}

#[derive(Error, Debug)]
pub enum MLError {
    #[error("Training error: {0}")]
    TrainingError(String),
    #[error("Prediction error: {0}")]
    PredictionError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Ethics violation: {0}")]
    EthicsViolation(String),
}

#[derive(Error, Debug)]
pub enum PrivacyError {
    #[error("Encryption error: {0}")]
    EncryptionError(String),
    #[error("ZK proof error: {0}")]
    ZKProofError(String),
    #[error("MPC error: {0}")]
    MPCError(String),
    #[error("Privacy constraint violation: {0}")]
    ConstraintViolation(String),
}

#[derive(Error, Debug)]
pub enum SecurityError {
    #[error("Authentication error: {0}")]
    AuthError(String),
    #[error("Authorization error: {0}")]
    AuthzError(String),
    #[error("Quantum resistance error: {0}")]
    QuantumError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Secure storage error: {0}")]
    SecureStorageError(String),
    #[error("Distributed storage error: {0}")]
    DistributedStorageError(String),
    #[error("Platform error: {0}")]
    PlatformError(String),
}

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("Connection error: {0}")]
    ConnectionError(String),
    #[error("Protocol error: {0}")]
    ProtocolError(String),
    #[error("P2P error: {0}")]
    P2PError(String),
}

#[derive(Error, Debug)]
pub enum IdentityError {
    #[error("DID resolution failed: {0}")]
    ResolutionError(String),
    #[error("Credential verification failed: {0}")]
    VerificationError(String),
    #[error("Invalid credential format: {0}")]
    InvalidCredential(String),
    #[error("Credential expired")]
    CredentialExpired,
}

#[derive(Error, Debug)]
pub enum InteroperabilityError {
    #[error("Cross-chain error: {0}")]
    CrossChainError(String),
    #[error("Protocol error: {0}")]
    ProtocolError(String),
    #[error("Bridge error: {0}")]
    BridgeError(String),
}

#[derive(Error, Debug)]
pub enum DataPipelineError {
    #[error("Data ingestion error: {0}")]
    IngestionError(String),
    #[error("Processing error: {0}")]
    ProcessingError(String),
    #[error("ML pipeline error: {0}")]
    MLPipelineError(String),
    #[error("Privacy constraint violation: {0}")]
    PrivacyError(String),
}

pub type Result<T> = std::result::Result<T, AnyaError>; 

