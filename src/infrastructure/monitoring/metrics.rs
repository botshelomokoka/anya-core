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
use metrics::{Counter, Gauge, Histogram};
use std::time::Instant;

#[derive(Clone)]
pub struct SecurityMetrics {
    pub failed_auth_attempts: Counter,
    pub key_operations: Counter,
    pub encryption_duration: Histogram,
    pub active_sessions: Gauge,
}

#[derive(Clone)]
pub struct MLMetrics {
    pub training_samples: Counter,
    pub prediction_accuracy: Gauge,
    pub feature_extraction_duration: Histogram,
    pub model_version: Gauge,
}

impl SecurityMetrics {
    pub fn new() -> Self  -> Result<(), Box<dyn Error>> {
        Self {
            failed_auth_attempts: register_counter!("security_failed_auth_total"),
            key_operations: register_counter!("security_key_ops_total"),
            encryption_duration: register_histogram!("security_encryption_duration"),
            active_sessions: register_gauge!("security_active_sessions"),
        }
    }
}

impl MLMetrics {
    pub fn new() -> Self  -> Result<(), Box<dyn Error>> {
        Self {
            training_samples: register_counter!("ml_training_samples_total"),
            prediction_accuracy: register_gauge!("ml_prediction_accuracy"),
            feature_extraction_duration: register_histogram!("ml_feature_extraction_duration"),
            model_version: register_gauge!("ml_model_version"),
        }
    }
}


