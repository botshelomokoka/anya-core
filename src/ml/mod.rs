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
//! `
ust
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
//! Machine Learning module provides AI and ML capabilities.
use anyhow::Result;
use log::{info, error};

// Current ML Module Structure
pub mod core;
pub mod agents;
pub mod research;
pub mod ragentic;
pub mod auto_adjust;
pub mod ml_core;
pub mod adaptive_model;
pub mod model_adaptation;
pub mod pipeline_optimizer;
pub mod web5;
pub mod nlp;
pub mod federated;
pub mod monitoring;
pub mod types;

pub use core::{MLCore, MLInput, MLOutput};
pub use agents::MLAgent;
pub use web5::Web5MLIntegration;
pub use nlp::NaturalLanguageProcessor;
pub use research::ResearchModule;
pub use federated::FederatedLearningModule;
pub use monitoring::MLMonitor;
pub use types::{MLConfig, MLMetrics};

// Bitcoin-specific features
mod bitcoin_models;
mod mlfee;

// Re-exports
pub use self::core::MLCore;
pub use self::types::{MLInput, MLOutput, MLError};
pub use self::manager::MLManager;
pub use self::bitcoin_models::BitcoinPricePredictor;

/// Initialize the ML module
pub async fn init() -> Result<()> {
    info!("Initializing ML module");
    
    // Initialize core components
    federated_learning::init().await?;
    differential_privacy::init().await?;
    secure_aggregation::init().await?;
    
    info!("ML module initialized successfully");
    Ok(())
}

// Re-export common ML types
pub use ndarray::{Array1, Array2};
pub use tch::{Tensor, Device};

pub use std::sync::Arc;
pub use tokio::sync::RwLock;

use crate::metrics::MetricsCollector;
use crate::ml::ragentic::RAGenticCoordinator;
use crate::ml::research::ResearchModule;
use crate::ml::agents::MLAgent;

pub struct MLSystem {
    metrics: Arc<MetricsCollector>,
    research_module: Arc<ResearchModule>,
    rag_coordinator: Arc<RAGenticCoordinator>,
    agents: Vec<Arc<dyn MLAgent>>,
}

impl MLSystem {
    pub fn new(metrics: Arc<MetricsCollector>) -> Result<Self> {
        let research_module = Arc::new(ResearchModule::new(metrics.clone(), Default::default()));
        let agents = Vec::new();
        let rag_coordinator = Arc::new(RAGenticCoordinator::new(
            metrics.clone(),
            Arc::clone(&research_module),
            agents.clone(),
        ));

        Ok(Self {
            metrics,
            research_module,
            rag_coordinator,
            agents,
        })
    }

    pub async fn initialize(&mut self) -> Result<()> {
        // Initialize RAG coordinator
        self.rag_coordinator.initialize_roles().await?;

        // Update research module with RAG coordinator
        self.research_module = Arc::new(
            ResearchModule::new(self.metrics.clone(), Default::default())
                .with_rag_coordinator(Arc::clone(&self.rag_coordinator))
        );

        Ok(())
    }

    pub async fn process_query(&self, query: &str) -> Result<String> {
        self.rag_coordinator.process_query(query).await
    }
}
