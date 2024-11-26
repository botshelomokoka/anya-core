//! Storage module provides secure and distributed storage capabilities.
//! This includes platform-specific secure storage and distributed storage systems.

pub mod secure;
pub mod distributed;

use thiserror::Error;
use metrics::{counter, gauge};
use log::{info, error};

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Secure storage error: {0}")]
    SecureStorageError(String),
    #[error("Distributed storage error: {0}")]
    DistributedStorageError(String),
    #[error("Platform error: {0}")]
    PlatformError(String),
}

/// Core storage metrics for monitoring and reliability
struct StorageMetrics {
    storage_operations: counter::Counter,
    storage_size: gauge::Gauge,
    operation_latency: gauge::Gauge,
    error_count: counter::Counter,
}

impl StorageMetrics {
    fn new() -> Self {
        Self {
            storage_operations: counter!("storage_operations_total"),
            storage_size: gauge!("storage_size_bytes"),
            operation_latency: gauge!("storage_operation_latency_ms"),
            error_count: counter!("storage_errors_total"),
        }
    }
}

/// Platform-specific secure storage configuration
#[cfg(target_os = "linux")]
pub use secure::linux::LinuxSecureStorage as SecureStorage;
#[cfg(target_os = "windows")]
pub use secure::windows::WindowsSecureStorage as SecureStorage;
#[cfg(target_os = "macos")]
pub use secure::macos::MacOSSecureStorage as SecureStorage;
#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub use secure::fallback::FallbackSecureStorage as SecureStorage;
