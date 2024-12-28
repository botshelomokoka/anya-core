//! ML*/Agent Error Types
//! 
//! This module defines the error types used throughout the ML*/Agent system.
//! It provides a comprehensive error handling framework that covers various
//! failure scenarios across different system components.
//!
//! # Error Categories
//!
//! The error system covers several categories:
//! - Configuration errors
//! - Runtime errors
//! - Security errors
//! - ML model errors
//! - Resource errors
//! - Validation errors
//!
//! # Example
//!
//! ```rust
//! use anya::agent::AgentError;
//!
//! fn handle_error(err: AgentError) {
//!     match err {
//!         AgentError::ConfigError(msg) => println!("Configuration error: {}", msg),
//!         AgentError::RuntimeError(msg) => println!("Runtime error: {}", msg),
//!         AgentError::SecurityError(msg) => println!("Security error: {}", msg),
//!         _ => println!("Other error: {}", err),
//!     }
//! }
//! ```

use std::fmt;
use std::error::Error;

/// Comprehensive error type for ML*/Agent system.
///
/// Covers various error categories:
/// - Configuration errors
/// - Runtime errors
/// - Security errors
/// - ML model errors
/// - Resource errors
/// - Validation errors
#[derive(Debug)]
pub enum AgentError {
    /// Configuration error with message
    ConfigError(String),
    /// Runtime error with message
    RuntimeError(String),
    /// Security error with message
    SecurityError(String),
    /// ML model error with message
    MLError(String),
    /// Resource error with message
    ResourceError(String),
    /// Validation error with message
    ValidationError(String),
    /// System error with message
    SystemError(String),
    /// Unknown error with message
    UnknownError(String),
}

impl fmt::Display for AgentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AgentError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            AgentError::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
            AgentError::SecurityError(msg) => write!(f, "Security error: {}", msg),
            AgentError::MLError(msg) => write!(f, "ML error: {}", msg),
            AgentError::ResourceError(msg) => write!(f, "Resource error: {}", msg),
            AgentError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AgentError::SystemError(msg) => write!(f, "System error: {}", msg),
            AgentError::UnknownError(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl Error for AgentError {}

impl From<std::io::Error> for AgentError {
    fn from(err: std::io::Error) -> Self {
        AgentError::SystemError(err.to_string())
    }
}

impl From<serde_json::Error> for AgentError {
    fn from(err: serde_json::Error) -> Self {
        AgentError::ConfigError(err.to_string())
    }
}

/// Helper function to create a configuration error.
///
/// # Arguments
///
/// * `msg` - Error message
///
/// # Returns
///
/// AgentError::ConfigError with the provided message
pub fn config_error<T: ToString>(msg: T) -> AgentError {
    AgentError::ConfigError(msg.to_string())
}

/// Helper function to create a runtime error.
///
/// # Arguments
///
/// * `msg` - Error message
///
/// # Returns
///
/// AgentError::RuntimeError with the provided message
pub fn runtime_error<T: ToString>(msg: T) -> AgentError {
    AgentError::RuntimeError(msg.to_string())
}

/// Helper function to create a security error.
///
/// # Arguments
///
/// * `msg` - Error message
///
/// # Returns
///
/// AgentError::SecurityError with the provided message
pub fn security_error<T: ToString>(msg: T) -> AgentError {
    AgentError::SecurityError(msg.to_string())
}

/// Helper function to create an ML error.
///
/// # Arguments
///
/// * `msg` - Error message
///
/// # Returns
///
/// AgentError::MLError with the provided message
pub fn ml_error<T: ToString>(msg: T) -> AgentError {
    AgentError::MLError(msg.to_string())
}

/// Helper function to create a resource error.
///
/// # Arguments
///
/// * `msg` - Error message
///
/// # Returns
///
/// AgentError::ResourceError with the provided message
pub fn resource_error<T: ToString>(msg: T) -> AgentError {
    AgentError::ResourceError(msg.to_string())
}

/// Helper function to create a validation error.
///
/// # Arguments
///
/// * `msg` - Error message
///
/// # Returns
///
/// AgentError::ValidationError with the provided message
pub fn validation_error<T: ToString>(msg: T) -> AgentError {
    AgentError::ValidationError(msg.to_string())
}

/// Helper function to create a system error.
///
/// # Arguments
///
/// * `msg` - Error message
///
/// # Returns
///
/// AgentError::SystemError with the provided message
pub fn system_error<T: ToString>(msg: T) -> AgentError {
    AgentError::SystemError(msg.to_string())
}

/// Helper function to create an unknown error.
///
/// # Arguments
///
/// * `msg` - Error message
///
/// # Returns
///
/// AgentError::UnknownError with the provided message
pub fn unknown_error<T: ToString>(msg: T) -> AgentError {
    AgentError::UnknownError(msg.to_string())
}
