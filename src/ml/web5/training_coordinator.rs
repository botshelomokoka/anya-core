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
use tokio::sync::{RwLock, broadcast};
use serde::{Serialize, Deserialize};
use log::{info, warn, error};

#[derive(Error, Debug)]
pub enum TrainingCoordinatorError {
    #[error("Training validation failed: {0}")]
    ValidationError(String),
    #[error("Data synchronization failed: {0}")]
    SyncError(String),
    #[error("Protocol error: {0}")]
    ProtocolError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingSession {
    session_id: String,
    model_id: String,
    participants: Vec<String>,
    round: u64,
    status: TrainingStatus,
    metrics: TrainingMetrics,
}

pub struct Web5TrainingCoordinator {
    dwn: Arc<DWN>,
    did_manager: Arc<DIDManager>,
    data_handler: Arc<Web5DataHandler>,
    protocol_handler: Arc<ProtocolHandler>,
    active_sessions: RwLock<HashMap<String, TrainingSession>>,
    metrics: CoordinatorMetrics,
}

impl Web5TrainingCoordinator {
    pub async fn new(
        dwn: Arc<DWN>,
        did_manager: Arc<DIDManager>,
        data_handler: Arc<Web5DataHandler>,
        protocol_handler: Arc<ProtocolHandler>,
    ) -> Result<Self> {
        Ok(Self {
            dwn,
            did_manager,
            data_handler,
            protocol_handler,
            active_sessions: RwLock::new(HashMap::new()),
            metrics: CoordinatorMetrics::new(),
        })
    }

    pub async fn initiate_training_session(&self, model_id: &str) -> Result<String> {
        let session_id = self.generate_session_id();
        let session = TrainingSession {
            session_id: session_id.clone(),
            model_id: model_id.to_string(),
            participants: vec![self.did_manager.get_current_did().await?],
            round: 0,
            status: TrainingStatus::Initializing,
            metrics: TrainingMetrics::default(),
        };

        self.store_session(&session).await?;
        self.broadcast_session_update(&session).await?;

        Ok(session_id)
    }

    async fn store_session(&self, session: &TrainingSession) -> Result<()> {
        let record = MLDataRecord {
            protocol: "ml.training.session".to_string(),
            schema: "training-session".to_string(),
            data: serde_json::to_vec(session)?,
            owner_did: self.did_manager.get_current_did().await?,
            permissions: vec![Permission::OwnerOnly],
        };

        self.data_handler.store_training_data(&record.data, &record.owner_did).await?;
        
        // Update active sessions
        self.active_sessions.write().await
            .insert(session.session_id.clone(), session.clone());
        
        Ok(())
    }

    async fn broadcast_session_update(&self, session: &TrainingSession) -> Result<()> {
        let message = ProtocolMessage::new()
            .with_type(ProtocolType::Training)
            .with_action("session_update")
            .with_data(serde_json::to_vec(session)?);

        self.protocol_handler.process_protocol_message(message).await?;
        Ok(())
    }
}


