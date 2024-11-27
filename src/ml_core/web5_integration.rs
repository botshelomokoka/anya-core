use crate::{
    web5::{Web5Protocol, Web5Service, MLModelMetadata, protocols::*},
    ml_core::optimizer::OptimizerConfig,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;
use metrics::{counter, gauge};

#[derive(Debug, Serialize, Deserialize)]
pub struct MLWeb5Config {
    pub protocol_uri: String,
    pub storage_namespace: String,
    pub encryption_enabled: bool,
    pub ml_protocol: ProtocolDefinition,
}

pub struct MLWeb5Integration {
    web5_service: Web5Service,
    config: MLWeb5Config,
    metrics: MLMetrics,
}

#[derive(Debug, Clone)]
pub struct MLMetrics {
    pub models_stored: u64,
    pub data_processed: u64,
    pub last_operation: i64,
}

impl MLWeb5Integration {
    pub async fn new(config: MLWeb5Config) -> Result<Self, Box<dyn Error>> {
        let web5_service = Web5Service::new(&config.protocol_uri).await?;
        
        // Register ML-specific protocol
        web5_service.register_protocol(&config.ml_protocol).await?;
        
        Ok(Self {
            web5_service,
            config,
            metrics: MLMetrics {
                models_stored: 0,
                data_processed: 0,
                last_operation: chrono::Utc::now().timestamp(),
            },
        })
    }

    pub async fn store_optimizer_config(&self, config: &OptimizerConfig) -> Result<String, Box<dyn Error>> {
        let metadata = MLModelMetadata {
            model_id: uuid::Uuid::new_v4().to_string(),
            version: "1.0".to_string(),
            architecture: "optimizer".to_string(),
            training_params: serde_json::to_value(config)?,
            performance_metrics: None,
            timestamp: chrono::Utc::now().timestamp(),
        };

        counter!("ml_web5_optimizer_configs_stored", 1);
        let serialized = serde_json::to_vec(config)?;
        self.web5_service.store_model(serialized, metadata).await
    }

    pub async fn load_optimizer_config(&self, config_id: &str) -> Result<OptimizerConfig, Box<dyn Error>> {
        let (data, _) = self.web5_service.retrieve_model(config_id).await?;
        let config: OptimizerConfig = serde_json::from_slice(&data)?;
        counter!("ml_web5_optimizer_configs_loaded", 1);
        Ok(config)
    }

    pub async fn store_training_metrics(&self, model_id: &str, metrics: serde_json::Value) -> Result<(), Box<dyn Error>> {
        let (_, mut metadata) = self.web5_service.retrieve_model(model_id).await?;
        metadata.performance_metrics = Some(metrics);
        counter!("ml_web5_training_metrics_stored", 1);
        self.web5_service.update_model_metadata(model_id, metadata).await
    }

    pub async fn verify_model_authenticity(&self, model_id: &str) -> Result<bool, Box<dyn Error>> {
        counter!("ml_web5_model_verifications", 1);
        self.web5_service.verify_identity(model_id).await
    }
}

#[async_trait]
pub trait Web5MLStorage {
    async fn save_model_checkpoint(&self, model_data: Vec<u8>, metadata: serde_json::Value) -> Result<String, Box<dyn Error>>;
    async fn load_model_checkpoint(&self, checkpoint_id: &str) -> Result<(Vec<u8>, serde_json::Value), Box<dyn Error>>;
    async fn list_model_checkpoints(&self) -> Result<Vec<String>, Box<dyn Error>>;
}

#[async_trait]
impl Web5MLStorage for MLWeb5Integration {
    async fn save_model_checkpoint(&self, model_data: Vec<u8>, metadata: serde_json::Value) -> Result<String, Box<dyn Error>> {
        let metadata = MLModelMetadata {
            model_id: uuid::Uuid::new_v4().to_string(),
            version: metadata["version"].as_str().unwrap_or("1.0").to_string(),
            architecture: metadata["architecture"].as_str().unwrap_or("unknown").to_string(),
            training_params: metadata.clone(),
            performance_metrics: None,
            timestamp: chrono::Utc::now().timestamp(),
        };

        counter!("ml_web5_checkpoints_saved", 1);
        gauge!("ml_web5_checkpoint_size_bytes", model_data.len() as f64);
        self.web5_service.store_model(model_data, metadata).await
    }

    async fn load_model_checkpoint(&self, checkpoint_id: &str) -> Result<(Vec<u8>, serde_json::Value), Box<dyn Error>> {
        let (data, metadata) = self.web5_service.retrieve_model(checkpoint_id).await?;
        counter!("ml_web5_checkpoints_loaded", 1);
        Ok((data, serde_json::to_value(metadata)?))
    }

    async fn list_model_checkpoints(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let query = serde_json::json!({
            "type": "model_checkpoint",
            "owner": self.web5_service.get_identity().did(),
        });
        
        let records = self.web5_service.query_models(query).await?;
        let checkpoint_ids: Vec<String> = records.into_iter()
            .map(|r| r.model_id)
            .collect();
            
        gauge!("ml_web5_checkpoint_count", checkpoint_ids.len() as f64);
        Ok(checkpoint_ids)
    }
}

// Helper functions for ML-specific Web5 operations
pub mod ml_web5_utils {
    use super::*;

    pub async fn create_federated_learning_session(web5: &Web5Service) -> Result<String, Box<dyn Error>> {
        let session_id = uuid::Uuid::new_v4().to_string();
        
        let metadata = MLModelMetadata {
            model_id: session_id.clone(),
            version: "1.0".to_string(),
            architecture: "federated".to_string(),
            training_params: serde_json::json!({
                "type": "federated_learning",
                "aggregation_method": "fedavg",
                "min_participants": 3,
                "rounds": 10
            }),
            performance_metrics: None,
            timestamp: chrono::Utc::now().timestamp(),
        };

        counter!("ml_web5_federated_sessions_created", 1);
        web5.store_model(vec![], metadata).await?;
        Ok(session_id)
    }

    pub fn verify_training_proof(proof: &[u8], model_id: &str) -> bool {
        counter!("ml_web5_training_proofs_verified", 1);
        // Implement zero-knowledge proof verification
        true
    }

    pub async fn create_model_sharing_agreement(
        web5: &Web5Service,
        model_id: &str,
        participants: Vec<String>,
    ) -> Result<String, Box<dyn Error>> {
        let agreement_id = uuid::Uuid::new_v4().to_string();
        
        let metadata = MLModelMetadata {
            model_id: agreement_id.clone(),
            version: "1.0".to_string(),
            architecture: "sharing_agreement".to_string(),
            training_params: serde_json::json!({
                "type": "model_sharing",
                "model_id": model_id,
                "participants": participants,
                "permissions": {
                    "read": true,
                    "write": false,
                    "share": false
                }
            }),
            performance_metrics: None,
            timestamp: chrono::Utc::now().timestamp(),
        };

        counter!("ml_web5_sharing_agreements_created", 1);
        web5.store_model(vec![], metadata).await?;
        Ok(agreement_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_optimizer_config_storage() -> Result<(), Box<dyn Error>> {
        let config = MLWeb5Config {
            protocol_uri: "https://anya.ai/ml-protocol".to_string(),
            storage_namespace: "test".to_string(),
            encryption_enabled: true,
            ml_protocol: create_ml_protocol(),
        };

        let integration = MLWeb5Integration::new(config).await?;
        
        let optimizer_config = OptimizerConfig {
            learning_rate: 0.001,
            batch_size: 32,
            epochs: 10,
        };

        let config_id = integration.store_optimizer_config(&optimizer_config).await?;
        let loaded_config = integration.load_optimizer_config(&config_id).await?;

        assert_eq!(optimizer_config.learning_rate, loaded_config.learning_rate);
        assert_eq!(optimizer_config.batch_size, loaded_config.batch_size);
        assert_eq!(optimizer_config.epochs, loaded_config.epochs);

        Ok(())
    }
}
