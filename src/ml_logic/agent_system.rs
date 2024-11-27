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
use tokio::sync::{Mutex, RwLock};
use log::{info, warn, error};
use metrics::{counter, gauge};

#[derive(Error, Debug)]
pub enum AgentSystemError {
    #[error("ML Agent error: {0}")]
    AgentError(String),
    #[error("Integration error: {0}")]
    IntegrationError(String),
    #[error("System validation failed: {0}")]
    ValidationError(String),
}

pub struct MLAgentSystem {
    unified_system: Arc<UnifiedSystem>,
    integrated_system: Arc<IntegratedMLSystem>,
    system_evaluator: Arc<SystemEvaluator>,
    metrics: AgentMetrics,
}

impl MLAgentSystem {
    pub async fn new(
        ml_core: Arc<Mutex<MLCore>>,
        blockchain: Arc<BlockchainInterface>,
        zk_system: Arc<ZKSnarkSystem>,
    ) -> Result<Self> {
        // Referenced from unified_system.rs
        let unified_system = Arc::new(UnifiedSystem::new(
            Arc::clone(&ml_core),
            Arc::clone(&blockchain),
            Arc::clone(&zk_system),
        ).await?);

        // Referenced from integrated_system.rs
        let integrated_system = Arc::new(IntegratedMLSystem::new(
            Arc::clone(&ml_core),
            Arc::clone(&blockchain),
            Arc::clone(&zk_system),
        ).await?);

        // Referenced from system_evaluation.rs
        let system_evaluator = Arc::new(SystemEvaluator::new(
            blockchain.clone(),
            BitcoinSupport::new(),
            STXSupport::new(),
            LightningSupport::new(),
            Web5Support::new(),
            Config::default(),
            DataManager::new(),
            SecurityManager::new(),
        ));

        Ok(Self {
            unified_system,
            integrated_system,
            system_evaluator,
            metrics: AgentMetrics::new(),
        })
    }

    pub async fn process_agent_update(&self) -> Result<(), AgentSystemError> {
        // Process unified system update
        self.unified_system.process_system_update().await
            .map_err(|e| AgentSystemError::IntegrationError(e.to_string()))?;

        // Process integrated system update
        self.integrated_system.process_update().await
            .map_err(|e| AgentSystemError::IntegrationError(e.to_string()))?;

        // Evaluate system performance
        let performance = self.system_evaluator
            .evaluate_performance(&self.federated_learning).await
            .map_err(|e| AgentSystemError::ValidationError(e.to_string()))?;

        self.update_agent_metrics(performance).await?;
        
        Ok(())
    }

    async fn update_agent_metrics(&self, performance: f64) -> Result<(), AgentSystemError> {
        self.metrics.system_performance.set(performance);
        
        if performance < 0.8 {
            warn!("Agent system performance below threshold: {}", performance);
            self.metrics.performance_issues.increment(1);
        } else {
            self.metrics.successful_updates.increment(1);
        }

        Ok(())
    }
}


