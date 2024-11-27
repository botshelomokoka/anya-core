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
use log::info;
pub use plugin_manager::PluginManager;
pub use hexagonal::HexagonalArchitecture;
use thiserror::Error;

// Module declarations
mod plugin_manager;
mod hexagonal;
mod types;
mod errors;
mod circuit_breaker;
mod telemetry;
mod cache;
mod health;

// Public re-exports
pub use errors::{HexagonalError, HexagonalResult, ErrorContext, ErrorSeverity};
pub use types::{
    ModelStatus, NetworkStatus, ConnectionStatus, FeeEstimate,
    Trace, Span, SpanGuard, PerformanceMetrics, ErrorMetrics,
    ResourceMetrics, BusinessMetrics, CompleteMetrics, ResourceUtilization,
    HealthStatus, HealthState, ComponentHealth, HealthCheck,
    Cache, CacheConfig, CacheEntry,
};
pub use circuit_breaker::{CircuitBreaker, CircuitBreakerConfig, CircuitState};
pub use telemetry::{Telemetry, MetricsCollector};
pub use cache::{CacheLayer, CacheManager, CacheStats};
pub use health::{HealthChecker, HealthMonitor, HealthSubscriber, SystemHealthCheck, DatabaseHealthCheck};

/// Custom error type for the Architecture module
#[derive(Error, Debug)]
pub enum ArchitectureError {
    #[error("Plugin Manager Error: {0}")]
    PluginManagerError(#[from] plugin_manager::PluginManagerError),

    #[error("Hexagonal Architecture Error: {0}")]
    HexagonalError(#[from] hexagonal::HexagonalError),

    #[error("Circuit Breaker Error: {0}")]
    CircuitBreakerError(String),

    #[error("Cache Error: {0}")]
    CacheError(String),

    #[error("Health Check Error: {0}")]
    HealthCheckError(String),

    #[error("Telemetry Error: {0}")]
    TelemetryError(String),
}

/// Initializes the architecture module by setting up the plugin manager and hexagonal architecture.
/// 
/// # Errors
/// 
/// Returns `ArchitectureError::PluginManagerError` if the plugin manager fails to initialize.
/// Returns `ArchitectureError::HexagonalError` if the hexagonal architecture setup fails.
pub fn init() -> Result<(), ArchitectureError> {
    plugin_manager::init()?;
    hexagonal::init()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[test]
    fn test_init_success() {
        let result = init();
        assert!(result.is_ok());
    }

    #[test]
    fn test_init_plugin_manager_failure() {
        // Simulate PluginManager init failure
        // This requires mocking plugin_manager::init()
        // For simplicity, assume plugin_manager::init() returns an error
        // You might use a mocking library like `mockall`
    }

    #[test]
    fn test_init_hexagonal_failure() {
        // Simulate HexagonalArchitecture init failure
        // Similar to above, use mocking to simulate failure
    }
}
