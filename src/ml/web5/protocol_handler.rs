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
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::{Mutex, broadcast};
use log::{info, warn, error};
use metrics::{counter, gauge};

#[derive(Error, Debug)]
pub enum ProtocolHandlerError {
    #[error("Protocol registration failed: {0}")]
    RegistrationError(String),
    #[error("Protocol validation failed: {0}")]
    ValidationError(String),
    #[error("Data processing error: {0}")]
    DataError(String),
}

pub struct Web5ProtocolHandler {
    dwn: Arc<DWN>,
    did_manager: Arc<DIDManager>,
    protocol_registry: Arc<ProtocolRegistry>,
    metrics: ProtocolMetrics,
}

impl Web5ProtocolHandler {
    pub async fn new(
        dwn: Arc<DWN>,
        did_manager: Arc<DIDManager>,
    ) -> Result<Self> {
        let protocol_registry = Arc::new(ProtocolRegistry::new());
        
        let handler = Self {
            dwn,
            did_manager,
            protocol_registry,
            metrics: ProtocolMetrics::new(),
        };

        handler.initialize_protocols().await?;
        Ok(handler)
    }

    async fn initialize_protocols(&self) -> Result<()> {
        // Referenced from system/coordinator.rs
        let protocols = self.create_ml_protocols()?;
        self.register_protocols(protocols).await?;
        Ok(())
    }

    fn create_ml_protocols(&self) -> Result<Vec<ProtocolDefinition>> {
        let mut protocols = Vec::new();
        
        // Referenced from ml/web5/mod.rs
        protocols.push(
            ProtocolDefinition::new("ml.training.data")
                .with_schema("training-data")
                .with_encryption()
                .build()?
        );

        protocols.push(
            ProtocolDefinition::new("ml.model.state")
                .with_schema("model-state")
                .with_encryption()
                .with_permissions(vec!["read", "execute"])
                .build()?
        );

        Ok(protocols)
    }

    pub async fn process_protocol_message(&self, message: ProtocolMessage) -> Result<()> {
        match message.protocol_type {
            ProtocolType::Training => {
                self.handle_training_protocol(message).await?;
            },
            ProtocolType::ModelState => {
                self.handle_model_state_protocol(message).await?;
            }
        }

        self.metrics.record_protocol_message();
        Ok(())
    }

    async fn handle_training_protocol(&self, message: ProtocolMessage) -> Result<()> {
        let record = self.create_training_record(message)?;
        self.dwn.create_record(record).await
            .map_err(|e| ProtocolHandlerError::DataError(e.to_string()))?;
        Ok(())
    }

    async fn handle_model_state_protocol(&self, message: ProtocolMessage) -> Result<()> {
        let record = self.create_model_state_record(message)?;
        self.dwn.create_record(record).await
            .map_err(|e| ProtocolHandlerError::DataError(e.to_string()))?;
        Ok(())
    }
}


