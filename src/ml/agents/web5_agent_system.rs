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
pub enum Web5AgentError {
    #[error("DID operation failed: {0}")]
    DIDError(String),
    #[error("DWN storage error: {0}")]
    DWNError(String),
    #[error("ML integration error: {0}")]
    MLError(String),
}

pub struct Web5AgentSystem {
    unified_system: Arc<UnifiedSystem>,
    agent_coordinator: Arc<Mutex<AgentCoordinator>>,
    web5_integration: Arc<Web5MLIntegration>,
    did_manager: Arc<DIDManager>,
    dwn_storage: Arc<DWNStorage>,
    metrics: Web5AgentMetrics,
}

impl Web5AgentSystem {
    pub async fn new(
        ml_core: Arc<Mutex<MLCore>>,
        blockchain: Arc<BlockchainInterface>,
        zk_system: Arc<ZKSnarkSystem>,
    ) -> Result<Self> {
        // Initialize Web5 components
        let did_manager = Arc::new(DIDManager::new().await?);
        let dwn_storage = Arc::new(DWNStorage::new().await?);
        
        // Initialize ML registry for Web5
        let ml_registry = Arc::new(MLRegistry::new());
        
        // Initialize Web5 integration
        let web5_integration = Arc::new(Web5MLIntegration::new(ml_registry.clone()).await?);
        
        // Initialize unified system
        let unified_system = Arc::new(UnifiedSystem::new(
            ml_core.clone(),
            blockchain.clone(),
            zk_system.clone(),
        ).await?);

        // Initialize agent coordinator with Web5 support
        let agent_coordinator = AgentCoordinator::new(
            unified_system.clone(),
            web5_integration.clone(),
            did_manager.clone(),
        );

        Ok(Self {
            unified_system,
            agent_coordinator: Arc::new(Mutex::new(agent_coordinator)),
            web5_integration,
            did_manager,
            dwn_storage,
            metrics: Web5AgentMetrics::new(),
        })
    }

    pub async fn process_web5_cycle(&self) -> Result<(), Web5AgentError> {
        // Process agent coordination
        let mut coordinator = self.agent_coordinator.lock().await;
        coordinator.coordinate_cycle().await
            .map_err(|e| Web5AgentError::MLError(e.to_string()))?;

        // Process DID operations
        self.process_did_operations().await?;

        // Process DWN storage operations
        self.process_dwn_operations().await?;

        // Update ML models with Web5 data
        self.update_web5_ml_models().await?;

        self.metrics.record_successful_cycle();
        Ok(())
    }

    async fn process_did_operations(&self) -> Result<(), Web5AgentError> {
        self.did_manager.process_pending_operations().await
            .map_err(|e| Web5AgentError::DIDError(e.to_string()))?;
        Ok(())
    }

    async fn process_dwn_operations(&self) -> Result<(), Web5AgentError> {
        self.dwn_storage.process_pending_records().await
            .map_err(|e| Web5AgentError::DWNError(e.to_string()))?;
        Ok(())
    }

    async fn update_web5_ml_models(&self) -> Result<(), Web5AgentError> {
        self.web5_integration.update_models_for_web5().await
            .map_err(|e| Web5AgentError::MLError(e.to_string()))?;
        Ok(())
    }
}


