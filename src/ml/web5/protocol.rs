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

#[derive(Serialize, Deserialize)]
pub struct MLProtocolDefinition {
    name: String,
    version: String,
    types: Vec<MLDataType>,
    roles: Vec<Role>,
    rules: Vec<Rule>,
}

impl MLProtocolDefinition {
    pub fn new_training_protocol() -> Self  -> Result<(), Box<dyn Error>> {
        Self {
            name: "ml.training.protocol".to_string(),
            version: "1.0.0".to_string(),
            types: vec![
                MLDataType::TrainingData,
                MLDataType::ModelState,
                MLDataType::ValidationResults,
            ],
            roles: vec![
                Role::DataProvider,
                Role::ModelTrainer,
                Role::Validator,
            ],
            rules: vec![
                Rule::RequireDataEncryption,
                Rule::RequireOwnerConsent,
                Rule::RequireValidation,
            ],
        }
    }
}



