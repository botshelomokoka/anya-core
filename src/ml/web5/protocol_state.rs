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
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use log::{info, warn, error};

#[derive(Error, Debug)]
pub enum ProtocolStateError {
    #[error("Invalid state transition: {0}")]
    TransitionError(String),
    #[error("State validation failed: {0}")]
    ValidationError(String),
    #[error("Storage operation failed: {0}")]
    StorageError(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProtocolState {
    Initial,
    Training,
    Validating,
    Aggregating,
    Completed,
    Error(String),
}

pub struct Web5ProtocolStateManager {
    dwn: Arc<DWN>,
    did_manager: Arc<DIDManager>,
    data_handler: Arc<Web5DataHandler>,
    protocol_states: RwLock<HashMap<String, ProtocolState>>,
    metrics: StateMetrics,
}

impl Web5ProtocolStateManager {
    pub async fn new(
        dwn: Arc<DWN>,
        did_manager: Arc<DIDManager>,
        data_handler: Arc<Web5DataHandler>,
    ) -> Result<Self> {
        Ok(Self {
            dwn,
            did_manager,
            data_handler,
            protocol_states: RwLock::new(HashMap::new()),
            metrics: StateMetrics::new(),
        })
    }

    pub async fn update_protocol_state(
        &self, 
        protocol_id: &str, 
        new_state: ProtocolState,
    ) -> Result<()> {
        // Validate state transition
        self.validate_state_transition(protocol_id, &new_state).await?;
        
        // Update state in memory
        {
            let mut states = self.protocol_states.write().await;
            states.insert(protocol_id.to_string(), new_state.clone());
        }

        // Store state in DWN
        self.store_protocol_state(protocol_id, &new_state).await?;
        
        self.metrics.record_state_transition();
        Ok(())
    }

    async fn store_protocol_state(&self, protocol_id: &str, state: &ProtocolState) -> Result<()> {
        let record = MLDataRecord {
            protocol: "ml.protocol.state".to_string(),
            schema: "protocol-state".to_string(),
            data: serde_json::to_vec(&state)?,
            owner_did: self.did_manager.get_current_did().await?,
            permissions: vec![Permission::OwnerOnly],
        };

        self.data_handler.store_training_data(&record.data, &record.owner_did).await?;
        Ok(())
    }

    async fn validate_state_transition(
        &self, 
        protocol_id: &str, 
        new_state: &ProtocolState,
    ) -> Result<()> {
        let current_state = self.get_current_state(protocol_id).await?;
        
        match (&current_state, new_state) {
            (ProtocolState::Initial, ProtocolState::Training) => Ok(()),
            (ProtocolState::Training, ProtocolState::Validating) => Ok(()),
            (ProtocolState::Validating, ProtocolState::Aggregating) => Ok(()),
            (ProtocolState::Aggregating, ProtocolState::Completed) => Ok(()),
            (_, ProtocolState::Error(_)) => Ok(()),
            _ => Err(ProtocolStateError::TransitionError(
                format!("Invalid transition from {:?} to {:?}", current_state, new_state)
            ).into()),
        }
    }
}


