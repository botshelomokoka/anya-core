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
use did_key::{DIDKey, KeyMaterial};
use dwn_sdk_rs::{DWN, Record, RecordQuery};
use anyhow::Result;

pub struct Web5MLIntegration {
    dwn: Arc<DWN>,
    did_manager: Arc<DIDManager>,
    ml_registry: Arc<MLRegistry>,
    data_protocols: HashMap<String, ProtocolDefinition>,
}

impl Web5MLIntegration {
    pub async fn new(ml_registry: Arc<MLRegistry>) -> Result<Self> {
        let dwn = DWN::new(Config::default()).await?;
        let did_manager = DIDManager::new().await?;
        
        let mut data_protocols = HashMap::new();
        // Define ML-specific protocols
        data_protocols.insert(
            "ml.training.data".to_string(),
            ProtocolDefinition::new()
                .with_schema("training-data")
                .with_encryption()
                .build()
        );
        
        Ok(Self {
            dwn,
            did_manager,
            ml_registry,
            data_protocols,
        })
    }

    async fn register_ml_protocols(&self) -> Result<()> {
        for (protocol_name, definition) in &self.data_protocols {
            self.dwn.register_protocol(protocol_name, definition).await?;
        }
        Ok(())
    }
}



