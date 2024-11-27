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
use crate::dowe::{DoweOracle, OracleData};
use crate::ml_core::{MLCore, MLInput, MLOutput};
use crate::blockchain::BlockchainInterface;
use crate::privacy::zksnarks::ZKSnarkSystem;
use thiserror::Error;
use log::{info, warn, error};
use metrics::{counter, gauge};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Error, Debug)]
pub enum DataIntegrationError {
    #[error("Data validation failed: {0}")]
    ValidationError(String),
    #[error("ML processing error: {0}")]
    MLError(String),
    #[error("Blockchain error: {0}")]
    BlockchainError(String),
}

pub struct MLDataIntegration {
    ml_core: Arc<Mutex<MLCore>>,
    blockchain: Arc<BlockchainInterface>,
    dowe_oracle: Arc<DoweOracle>,
    zk_system: Arc<ZKSnarkSystem>,
    metrics: IntegrationMetrics,
}

impl MLDataIntegration {
    pub fn new(
        ml_core: Arc<Mutex<MLCore>>,
        blockchain: Arc<BlockchainInterface>,
        dowe_oracle: Arc<DoweOracle>,
        zk_system: Arc<ZKSnarkSystem>,
    ) -> Self {
        Self {
            ml_core,
            blockchain,
            dowe_oracle,
            zk_system,
            metrics: IntegrationMetrics::new(),
        }
    }

    pub async fn process_blockchain_data(&self) -> Result<MLOutput, DataIntegrationError> {
        // Get blockchain metrics
        let mempool_size = self.blockchain.get_mempool_size().await
            .map_err(|e| DataIntegrationError::BlockchainError(e.to_string()))?;
        
        let network_load = self.blockchain.get_network_load().await
            .map_err(|e| DataIntegrationError::BlockchainError(e.to_string()))?;

        // Create ML input with blockchain data
        let ml_input = MLInput {
            mempool_size,
            network_load,
            timestamp: chrono::Utc::now(),
        };

        // Get ML prediction
        let prediction = self.ml_core.lock().await.predict(&ml_input)
            .map_err(|e| DataIntegrationError::MLError(e.to_string()))?;

        // Create ZK proof of prediction
        let proof = self.zk_system.create_proof(&[
            &prediction.value.to_le_bytes(),
            &prediction.confidence.to_le_bytes(),
        ]).map_err(|e| DataIntegrationError::ValidationError(e.to_string()))?;

        // Submit to oracle with proof
        self.dowe_oracle.submit_data(OracleData {
            source: "blockchain_analysis".to_string(),
            timestamp: chrono::Utc::now(),
            value: serde_json::json!({
                "prediction": prediction.value,
                "confidence": prediction.confidence,
            }),
            signature: vec![],
            proof,
        }).await.map_err(|e| DataIntegrationError::ValidationError(e.to_string()))?;

        self.metrics.record_successful_processing();
        Ok(prediction)
    }
}

struct IntegrationMetrics {
    successful_processing: Counter,
    failed_processing: Counter,
    data_points_processed: Gauge,
}

impl IntegrationMetrics {
    fn new() -> Self {
        Self {
            successful_processing: counter!("ml_data_integration_successful_total"),
            failed_processing: counter!("ml_data_integration_failed_total"),
            data_points_processed: gauge!("ml_data_points_processed_total"),
        }
    }

    fn record_successful_processing(&self) {
        self.successful_processing.increment(1);
        self.data_points_processed.increment(1);
    }
}


