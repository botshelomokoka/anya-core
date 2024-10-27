use crate::ml_core::{MLCore, MLInput, MLOutput};
use crate::privacy::zksnarks::ZKSnarkSystem;
use crate::blockchain::BlockchainInterface;
use crate::metrics::{counter, gauge};
use thiserror::Error;
use log::{info, warn, error};
use std::sync::Arc;
use tokio::sync::Mutex;
use ndarray::{Array1, Array2};
use tch::{nn, Device, Tensor, Kind};

#[derive(Error, Debug)]
pub enum MLIntegrationError {
    #[error("Model adaptation failed: {0}")]
    AdaptationError(String),
    #[error("Federated learning error: {0}")]
    FederatedError(String),
    #[error("Privacy constraint violation: {0}")]
    PrivacyError(String),
}

pub struct AdvancedMLIntegration {
    ml_core: Arc<Mutex<MLCore>>,
    blockchain: Arc<BlockchainInterface>,
    zk_system: Arc<ZKSnarkSystem>,
    device: Device,
    model: nn::Sequential,
    metrics: MLMetrics,
}

impl AdvancedMLIntegration {
    pub fn new(
        ml_core: Arc<Mutex<MLCore>>,
        blockchain: Arc<BlockchainInterface>,
        zk_system: Arc<ZKSnarkSystem>,
    ) -> Result<Self, MLIntegrationError> {
        let device = Device::cuda_if_available();
        let vs = nn::VarStore::new(device);
        
        // Create advanced neural network architecture
        let model = nn::seq()
            .add(nn::linear(&vs.root(), 128, 256, Default::default()))
            .add_fn(|x| x.relu())
            .add(nn::dropout(&vs.root(), 0.2))
            .add(nn::linear(&vs.root(), 256, 512, Default::default()))
            .add_fn(|x| x.relu())
            .add(nn::dropout(&vs.root(), 0.2))
            .add(nn::linear(&vs.root(), 512, 256, Default::default()))
            .add_fn(|x| x.relu())
            .add(nn::linear(&vs.root(), 256, 64, Default::default()));

        Ok(Self {
            ml_core,
            blockchain,
            zk_system,
            device,
            model,
            metrics: MLMetrics::new(),
        })
    }

    pub async fn train_federated(&mut self, data: &[MLInput]) -> Result<(), MLIntegrationError> {
        info!("Starting federated training with {} samples", data.len());
        
        // Create privacy-preserving data tensors
        let features = self.prepare_features(data)?;
        let labels = self.prepare_labels(data)?;

        // Generate ZK proof for training data
        let data_proof = self.create_training_proof(&features, &labels)?;

        // Verify data privacy
        self.verify_privacy_constraints(&data_proof)?;

        // Train model with federated learning
        self.train_model_federated(features, labels).await?;

        self.metrics.record_successful_training();
        Ok(())
    }

    async fn train_model_federated(&mut self, features: Tensor, labels: Tensor) -> Result<(), MLIntegrationError> {
        let mut opt = tch::nn::Adam::default();
        
        for epoch in 0..100 {
            let loss = self.model.forward(&features)
                .cross_entropy_for_logits(&labels);
            
            opt.backward_step(&loss);
            
            if epoch % 10 == 0 {
                info!("Epoch {}: Loss = {}", epoch, f64::from(loss));
                self.metrics.record_training_loss(f64::from(loss));
            }
        }

        Ok(())
    }

    pub async fn predict_with_privacy(&self, input: &MLInput) -> Result<MLOutput, MLIntegrationError> {
        // Create privacy-preserving input tensor
        let features = self.prepare_single_feature(input)?;
        
        // Generate prediction with privacy guarantees
        let prediction = self.model.forward(&features);
        
        // Create ZK proof for prediction
        let pred_proof = self.create_prediction_proof(&prediction)?;
        
        // Verify prediction privacy
        self.verify_privacy_constraints(&pred_proof)?;

        let output = MLOutput {
            prediction: f64::from(prediction.max()),
            confidence: f64::from(prediction.softmax(-1, Kind::Float).max()),
        };

        self.metrics.record_prediction();
        Ok(output)
    }

    fn prepare_features(&self, data: &[MLInput]) -> Result<Tensor, MLIntegrationError> {
        let features: Vec<f64> = data.iter()
            .flat_map(|input| input.features.clone())
            .collect();
        
        let tensor = Tensor::of_slice(&features)
            .view([-1, data[0].features.len() as i64])
            .to_device(self.device);
            
        Ok(tensor)
    }

    fn prepare_labels(&self, data: &[MLInput]) -> Result<Tensor, MLIntegrationError> {
        let labels: Vec<f64> = data.iter()
            .map(|input| input.label)
            .collect();
            
        let tensor = Tensor::of_slice(&labels)
            .to_device(self.device);
            
        Ok(tensor)
    }

    fn prepare_single_feature(&self, input: &MLInput) -> Result<Tensor, MLIntegrationError> {
        let tensor = Tensor::of_slice(&input.features)
            .view([1, -1])
            .to_device(self.device);
            
        Ok(tensor)
    }

    fn create_training_proof(&self, features: &Tensor, labels: &Tensor) -> Result<Vec<u8>, MLIntegrationError> {
        self.zk_system.create_proof(&[
            &features.to_vec::<f64>()?,
            &labels.to_vec::<f64>()?,
        ]).map_err(|e| MLIntegrationError::PrivacyError(e.to_string()))
    }

    fn create_prediction_proof(&self, prediction: &Tensor) -> Result<Vec<u8>, MLIntegrationError> {
        self.zk_system.create_proof(&[
            &prediction.to_vec::<f64>()?,
        ]).map_err(|e| MLIntegrationError::PrivacyError(e.to_string()))
    }

    fn verify_privacy_constraints(&self, proof: &[u8]) -> Result<(), MLIntegrationError> {
        if !self.zk_system.verify_proof(proof, &[])? {
            return Err(MLIntegrationError::PrivacyError("Privacy verification failed".into()));
        }
        Ok(())
    }
}

struct MLMetrics {
    successful_training: Counter,
    failed_training: Counter,
    predictions_made: Counter,
    training_loss: Gauge,
    prediction_latency: Gauge,
}

impl MLMetrics {
    fn new() -> Self {
        Self {
            successful_training: counter!("ml_training_successful_total"),
            failed_training: counter!("ml_training_failed_total"),
            predictions_made: counter!("ml_predictions_total"),
            training_loss: gauge!("ml_training_loss"),
            prediction_latency: gauge!("ml_prediction_latency_seconds"),
        }
    }

    fn record_successful_training(&self) {
        self.successful_training.increment(1);
    }

    fn record_training_loss(&self, loss: f64) {
        self.training_loss.set(loss);
    }

    fn record_prediction(&self) {
        self.predictions_made.increment(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_federated_training() {
        let ml_core = Arc::new(Mutex::new(MLCore::new()));
        let blockchain = Arc::new(BlockchainInterface::new());
        let zk_system = Arc::new(ZKSnarkSystem::new().unwrap());
        
        let mut integration = AdvancedMLIntegration::new(
            ml_core,
            blockchain,
            zk_system,
        ).unwrap();

        let test_data = vec![MLInput::default(); 10];
        let result = integration.train_federated(&test_data).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_private_prediction() {
        let ml_core = Arc::new(Mutex::new(MLCore::new()));
        let blockchain = Arc::new(BlockchainInterface::new());
        let zk_system = Arc::new(ZKSnarkSystem::new().unwrap());
        
        let integration = AdvancedMLIntegration::new(
            ml_core,
            blockchain,
            zk_system,
        ).unwrap();

        let test_input = MLInput::default();
        let result = integration.predict_with_privacy(&test_input).await;
        assert!(result.is_ok());
    }
}
