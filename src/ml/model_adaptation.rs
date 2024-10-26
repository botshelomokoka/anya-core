use crate::ml_core::{MLCore, MLInput, MLOutput};
use crate::blockchain::BlockchainInterface;
use crate::privacy::zksnarks::ZKSnarkSystem;
use thiserror::Error;
use log::{info, warn, error};
use metrics::{counter, gauge};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Error, Debug)]
pub enum ModelAdaptationError {
    #[error("Model update failed: {0}")]
    UpdateError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Privacy constraint violation: {0}")]
    PrivacyError(String),
}

pub struct ModelAdapter {
    ml_core: Arc<Mutex<MLCore>>,
    blockchain: Arc<BlockchainInterface>,
    zk_system: Arc<ZKSnarkSystem>,
    metrics: AdaptationMetrics,
}

impl ModelAdapter {
    pub fn new(
        ml_core: Arc<Mutex<MLCore>>,
        blockchain: Arc<BlockchainInterface>,
        zk_system: Arc<ZKSnarkSystem>,
    ) -> Self {
        Self {
            ml_core,
            blockchain,
            zk_system,
            metrics: AdaptationMetrics::new(),
        }
    }

    pub async fn adapt_model(&self, new_data: &[MLInput]) -> Result<(), ModelAdaptationError> {
        // Verify data privacy with ZK proofs
        self.verify_data_privacy(new_data).await?;

        // Get blockchain metrics for adaptation
        let blockchain_metrics = self.get_blockchain_metrics().await?;

        // Adapt model based on blockchain state
        let adapted_model = self.adapt_to_blockchain_state(new_data, &blockchain_metrics).await?;

        // Update ML core with adapted model
        self.update_ml_core(adapted_model).await?;

        self.metrics.record_successful_adaptation();
        Ok(())
    }

    async fn verify_data_privacy(&self, data: &[MLInput]) -> Result<(), ModelAdaptationError> {
        for input in data {
            let proof = self.zk_system.create_proof(&[
                &input.features.as_bytes(),
                &input.timestamp.timestamp().to_le_bytes(),
            ]).map_err(|e| ModelAdaptationError::PrivacyError(e.to_string()))?;

            if !self.zk_system.verify_proof(&proof, &[&input.features.as_bytes()])?? {
                return Err(ModelAdaptationError::PrivacyError("Invalid privacy proof".into()));
            }
        }
        Ok(())
    }

    async fn get_blockchain_metrics(&self) -> Result<BlockchainMetrics, ModelAdaptationError> {
        let mempool_size = self.blockchain.get_mempool_size().await
            .map_err(|e| ModelAdaptationError::UpdateError(e.to_string()))?;
        
        let network_load = self.blockchain.get_network_load().await
            .map_err(|e| ModelAdaptationError::UpdateError(e.to_string()))?;

        Ok(BlockchainMetrics {
            mempool_size,
            network_load,
        })
    }

    async fn adapt_to_blockchain_state(
        &self,
        data: &[MLInput],
        metrics: &BlockchainMetrics,
    ) -> Result<AdaptedModel, ModelAdaptationError> {
        let mut ml_core = self.ml_core.lock().await;
        
        // Adjust learning rate based on network load
        let learning_rate = self.calculate_learning_rate(metrics.network_load);
        
        // Update model with new data using adjusted parameters
        ml_core.update_with_params(data, learning_rate)
            .map_err(|e| ModelAdaptationError::UpdateError(e.to_string()))?;

        Ok(AdaptedModel {
            parameters: ml_core.get_parameters(),
            learning_rate,
        })
    }

    fn calculate_learning_rate(&self, network_load: f64) -> f64 {
        // Adjust learning rate inversely to network load
        let base_rate = 0.01;
        base_rate * (1.0 - network_load.min(0.9))
    }

    async fn update_ml_core(&self, model: AdaptedModel) -> Result<(), ModelAdaptationError> {
        let mut ml_core = self.ml_core.lock().await;
        ml_core.update_parameters(&model.parameters)
            .map_err(|e| ModelAdaptationError::UpdateError(e.to_string()))?;
        Ok(())
    }
}

struct AdaptationMetrics {
    successful_adaptations: Counter,
    failed_adaptations: Counter,
    average_learning_rate: Gauge,
}

impl AdaptationMetrics {
    fn new() -> Self {
        Self {
            successful_adaptations: counter!("model_adaptations_successful_total"),
            failed_adaptations: counter!("model_adaptations_failed_total"),
            average_learning_rate: gauge!("model_average_learning_rate"),
        }
    }

    fn record_successful_adaptation(&self) {
        self.successful_adaptations.increment(1);
    }

    fn record_failed_adaptation(&self) {
        self.failed_adaptations.increment(1);
    }

    fn update_learning_rate(&self, rate: f64) {
        self.average_learning_rate.set(rate);
    }
}

struct BlockchainMetrics {
    mempool_size: u64,
    network_load: f64,
}

struct AdaptedModel {
    parameters: Vec<f64>,
    learning_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_model_adaptation() {
        let ml_core = Arc::new(Mutex::new(MLCore::new()));
        let blockchain = Arc::new(BlockchainInterface::new());
        let zk_system = Arc::new(ZKSnarkSystem::new().unwrap());
        
        let adapter = ModelAdapter::new(ml_core, blockchain, zk_system);
        
        let test_data = vec![MLInput::default()];
        let result = adapter.adapt_model(&test_data).await;
        assert!(result.is_ok());
    }
}
