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

#[derive(Clone)]
pub struct Web5Metrics {
    pub dwn_sync_duration: Histogram,
    pub dwn_sync_errors: Counter,
    pub records_synced: Counter,
    pub active_connections: Gauge,
    pub protocol_operations: Counter,
    pub did_resolutions: Counter,
}

impl Web5Metrics {
    pub fn new() -> Self  -> Result<(), Box<dyn Error>> {
        Self {
            dwn_sync_duration: register_histogram!("web5_dwn_sync_duration_seconds"),
            dwn_sync_errors: register_counter!("web5_dwn_sync_errors_total"),
            records_synced: register_counter!("web5_records_synced_total"),
            active_connections: register_gauge!("web5_active_connections"),
            protocol_operations: register_counter!("web5_protocol_operations_total"),
            did_resolutions: register_counter!("web5_did_resolutions_total"),
        }
    }
}


