use crate::ml_core::{MLCore, MLInput, MLOutput, MLError};
use crate::research::bitcoin_layers_crawler::BitcoinLayersCrawler;
use crate::research::federated_learning_research::FederatedLearningResearcher;
use crate::privacy::zksnarks::ZKSnarkSystem;
use crate::metrics::{counter, gauge};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, error};

pub struct UnifiedMLSystem {
    ml_core: Arc<Mutex<MLCore>>,
    bitcoin_layers_crawler: Arc<BitcoinLayersCrawler>,
    fl_researcher: Arc<FederatedLearningResearcher>,
    zk_system: Arc<ZKSnarkSystem>,
    metrics: MLMetrics,
}

impl UnifiedMLSystem {
    pub async fn new() -> Result<Self, MLError> {
        Ok(Self {
            ml_core: Arc::new(Mutex::new(MLCore::new()?)),
            bitcoin_layers_crawler: Arc::new(BitcoinLayersCrawler::new()),
            fl_researcher: Arc::new(FederatedLearningResearcher::new()),
            zk_system: Arc::new(ZKSnarkSystem::new()?),
            metrics: MLMetrics::new(),
        })
    }

    pub async fn train_with_research(&self) -> Result<(), MLError> {
        // Get research data
        let bitcoin_layers_data = self.bitcoin_layers_crawler.crawl_repositories().await
            .map_err(|e| MLError::ResearchError(e.to_string()))?;
        
        let fl_research = self.fl_researcher.analyze_research().await
            .map_err(|e| MLError::ResearchError(e.to_string()))?;

        // Process and combine research data
        let combined_data = self.process_research_data(bitcoin_layers_data, fl_research)?;

        // Create ZK proof of training data
        let proof = self.zk_system.create_proof(&combined_data)
            .map_err(|e| MLError::PrivacyError(e.to_string()))?;

        // Train ML core with verified data
        let mut ml_core = self.ml_core.lock().await;
        ml_core.train(&combined_data)?;

        self.metrics.record_training_success();
        info!("Successfully trained ML system with research data");
        
        Ok(())
    }

    pub async fn predict_with_verification(&self, input: &MLInput) -> Result<MLOutput, MLError> {
        // Create ZK proof of prediction input
        let input_proof = self.zk_system.create_proof(input)
            .map_err(|e| MLError::PrivacyError(e.to_string()))?;

        // Get prediction
        let ml_core = self.ml_core.lock().await;
        let prediction = ml_core.predict(input)?;

        // Create ZK proof of prediction output
        let output_proof = self.zk_system.create_proof(&prediction)
            .map_err(|e| MLError::PrivacyError(e.to_string()))?;

        self.metrics.record_prediction();
        Ok(prediction)
    }

    pub async fn update_from_network(&self, network_data: &[u8]) -> Result<(), MLError> {
        // Verify network data with ZK proof
        let data_proof = self.zk_system.verify_data(network_data)
            .map_err(|e| MLError::PrivacyError(e.to_string()))?;

        if data_proof {
            let mut ml_core = self.ml_core.lock().await;
            ml_core.update_from_network(network_data)?;
            self.metrics.record_network_update();
            Ok(())
        } else {
            Err(MLError::ValidationError("Network data verification failed".into()))
        }
    }
}

struct MLMetrics {
    training_successes: Counter,
    training_failures: Counter,
    predictions_made: Counter,
    network_updates: Counter,
    model_accuracy: Gauge,
}

impl MLMetrics {
    fn new() -> Self {
        Self {
            training_successes: counter!("ml_training_successes_total"),
            training_failures: counter!("ml_training_failures_total"),
            predictions_made: counter!("ml_predictions_total"),
            network_updates: counter!("ml_network_updates_total"),
            model_accuracy: gauge!("ml_model_accuracy"),
        }
    }

    fn record_training_success(&self) {
        self.training_successes.increment(1);
    }

    fn record_prediction(&self) {
        self.predictions_made.increment(1);
    }

    fn record_network_update(&self) {
        self.network_updates.increment(1);
    }

    fn update_accuracy(&self, accuracy: f64) {
        self.model_accuracy.set(accuracy);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_unified_ml_system() {
        let system = UnifiedMLSystem::new().await.unwrap();
        
        // Test training
        assert!(system.train_with_research().await.is_ok());
        
        // Test prediction
        let input = MLInput::default();
        let prediction = system.predict_with_verification(&input).await;
        assert!(prediction.is_ok());
        
        // Test network update
        let network_data = vec![1, 2, 3, 4];
        assert!(system.update_from_network(&network_data).await.is_ok());
    }
}
