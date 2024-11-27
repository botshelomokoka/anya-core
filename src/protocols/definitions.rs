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
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemProtocols {
    pub ml_training: ProtocolDefinition,
    pub ml_model: ProtocolDefinition,
    pub identity: ProtocolDefinition,
    pub data_exchange: ProtocolDefinition,
}

impl SystemProtocols {
    pub fn new() -> Self  -> Result<(), Box<dyn Error>> {
        Self {
            ml_training: ProtocolDefinition::new("ml.training")
                .with_actions(vec![
                    "TrainingDataSubmit",
                    "ModelUpdate",
                    "ValidationRequest",
                ])
                .with_schemas(vec![
                    "TrainingData",
                    "ModelState",
                    "ValidationResult",
                ])
                .build(),

            ml_model: ProtocolDefinition::new("ml.model")
                .with_actions(vec![
                    "ModelQuery",
                    "PredictionRequest",
                    "ModelValidation",
                ])
                .build(),

            identity: ProtocolDefinition::new("system.identity")
                .with_actions(vec![
                    "IdentityCreate",
                    "IdentityUpdate",
                    "IdentityVerify",
                ])
                .build(),

            data_exchange: ProtocolDefinition::new("system.data")
                .with_actions(vec![
                    "DataSubmit",
                    "DataRequest",
                    "DataValidate",
                ])
                .build(),
        }
    }
}



