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
pub enum AgentRegistryError {
    #[error("Agent registration failed: {0}")]
    RegistrationError(String),
    #[error("Agent verification failed: {0}")]
    VerificationError(String),
    #[error("Protocol error: {0}")]
    ProtocolError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentRegistration {
    agent_did: String,
    capabilities: Vec<AgentCapability>,
    status: AgentStatus,
    last_heartbeat: u64,
    registration_time: u64,
}

pub struct Web5AgentRegistry {
    dwn: Arc<DWN>,
    did_manager: Arc<DIDManager>,
    data_handler: Arc<Web5DataHandler>,
    protocol_handler: Arc<ProtocolHandler>,
    registered_agents: RwLock<HashMap<String, AgentRegistration>>,
    metrics: RegistryMetrics,
}

impl Web5AgentRegistry {
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
            registered_agents: RwLock::new(HashMap::new()),
            metrics: RegistryMetrics::new(),
        })
    }

    pub async fn register_agent(&self, registration: AgentRegistration) -> Result<()> {
        // Verify agent DID
        self.verify_agent_did(&registration.agent_did).await?;
        
        // Store registration
        self.store_agent_registration(&registration).await?;
        
        // Update registry
        self.registered_agents.write().await
            .insert(registration.agent_did.clone(), registration);
        
        self.metrics.record_registration();
        Ok(())
    }

    async fn store_agent_registration(&self, registration: &AgentRegistration) -> Result<()> {
        let record = MLDataRecord {
            protocol: "ml.agent.registry".to_string(),
            schema: "agent-registration".to_string(),
            data: serde_json::to_vec(registration)?,
            owner_did: self.did_manager.get_current_did().await?,
            permissions: vec![Permission::OwnerOnly],
        };

        self.data_handler.store_training_data(&record.data, &record.owner_did).await?;
        Ok(())
    }

    pub async fn verify_agent_status(&self, agent_did: &str) -> Result<AgentStatus> {
        let registration = self.get_agent_registration(agent_did).await?;
        
        // Check last heartbeat
        let current_time = chrono::Utc::now().timestamp() as u64;
        if current_time - registration.last_heartbeat > AGENT_TIMEOUT {
            return Ok(AgentStatus::Offline);
        }
        
        Ok(registration.status)
    }
}


