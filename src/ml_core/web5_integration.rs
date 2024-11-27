use crate::{
    web5::{Web5Protocol, Web5Service, MLModelMetadata, protocols::*},
    ml_core::optimizer::OptimizerConfig,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;
use metrics::{counter, gauge};
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use metrics::{Counter, Gauge, Histogram, register_counter, register_gauge, register_histogram};
use lz4_flex::{compress_prepend_size, decompress_size_prepended};

#[derive(Debug, Serialize, Deserialize)]
pub struct MLWeb5Config {
    pub protocol_uri: String,
    pub storage_namespace: String,
    pub encryption_enabled: bool,
    pub ml_protocol: ProtocolDefinition,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelVersion {
    pub version: String,
    pub timestamp: i64,
    pub parameters: Vec<f32>,
    pub metrics: ModelMetrics,
    pub checksum: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelMetrics {
    pub accuracy: f32,
    pub loss: f32,
    pub training_time: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FederatedCheckpoint {
    pub round: u32,
    pub participants: Vec<String>,
    pub aggregated_model: ModelVersion,
    pub participant_metrics: Vec<ParticipantMetrics>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipantMetrics {
    pub participant_id: String,
    pub contribution_score: f32,
    pub training_duration: f32,
}

pub struct MLMetrics {
    pub models_stored: u64,
    pub data_processed: u64,
    pub last_operation: i64,
}

pub struct Web5MLMetrics {
    model_versions: Counter,
    storage_size: Gauge,
    compression_ratio: Histogram,
    checkpoint_size: Histogram,
    operation_latency: Histogram,
}

impl Web5MLMetrics {
    pub fn new() -> Self {
        Self {
            model_versions: register_counter!("web5_ml_model_versions"),
            storage_size: register_gauge!("web5_ml_storage_size_bytes"),
            compression_ratio: register_histogram!("web5_ml_compression_ratio"),
            checkpoint_size: register_histogram!("web5_ml_checkpoint_size_bytes"),
            operation_latency: register_histogram!("web5_ml_operation_latency"),
        }
    }
}

pub struct MLWeb5Integration {
    web5_service: Web5Service,
    config: MLWeb5Config,
    metrics: MLMetrics,
    web5_ml_metrics: Web5MLMetrics,
    cache: Arc<Mutex<ModelCache>>,
    retry_config: RetryConfig,
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
            web5_ml_metrics: Web5MLMetrics::new(),
            cache: Arc::new(Mutex::new(ModelCache::new())),
            retry_config: RetryConfig::default(),
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

    pub async fn store_model_version(&self, model: ModelVersion) -> Result<String> {
        let start = std::time::Instant::now();
        
        // Compress model parameters
        let compressed = self.compress_parameters(&model.parameters)?;
        let compression_ratio = model.parameters.len() as f32 / compressed.len() as f32;
        self.web5_ml_metrics.compression_ratio.record(compression_ratio);
        
        // Store with retries
        let record_id = self.retry_operation(|| async {
            self.web5_service.store_model(
                compressed,
                MLModelMetadata {
                    model_id: uuid::Uuid::new_v4().to_string(),
                    version: model.version.clone(),
                    architecture: "model_version".to_string(),
                    training_params: serde_json::to_value(&model)?,
                    performance_metrics: None,
                    timestamp: chrono::Utc::now().timestamp(),
                },
            ).await
        }).await?;
        
        // Update metrics
        self.web5_ml_metrics.model_versions.increment(1);
        self.web5_ml_metrics.storage_size.set(compressed.len() as f64);
        self.web5_ml_metrics.operation_latency.record(start.elapsed().as_secs_f64());
        
        // Update cache
        let mut cache = self.cache.lock().await;
        cache.add_model(record_id.clone(), model);
        
        Ok(record_id)
    }

    pub async fn store_federated_checkpoint(&self, checkpoint: FederatedCheckpoint) -> Result<String> {
        let start = std::time::Instant::now();
        
        // Compress checkpoint data
        let serialized = serde_json::to_vec(&checkpoint)?;
        let compressed = compress_prepend_size(&serialized);
        
        // Store with retries
        let record_id = self.retry_operation(|| async {
            self.web5_service.store_model(
                compressed,
                MLModelMetadata {
                    model_id: uuid::Uuid::new_v4().to_string(),
                    version: "1.0".to_string(),
                    architecture: "federated_checkpoint".to_string(),
                    training_params: serde_json::json!({
                        "round": checkpoint.round,
                        "participants": checkpoint.participants,
                    }),
                    performance_metrics: None,
                    timestamp: chrono::Utc::now().timestamp(),
                },
            ).await
        }).await?;
        
        // Update metrics
        self.web5_ml_metrics.checkpoint_size.record(compressed.len() as f64);
        self.web5_ml_metrics.operation_latency.record(start.elapsed().as_secs_f64());
        
        Ok(record_id)
    }

    pub async fn get_model_version(&self, record_id: &str) -> Result<ModelVersion> {
        // Check cache first
        let cache = self.cache.lock().await;
        if let Some(model) = cache.get_model(record_id) {
            return Ok(model.clone());
        }
        drop(cache);
        
        // Fetch from storage with retries
        let record = self.retry_operation(|| async {
            self.web5_service.get_record(record_id).await
        }).await?;
        
        // Decompress parameters
        let model = serde_json::from_value(record.metadata)?;
        let parameters = self.decompress_parameters(&record.data)?;
        
        Ok(ModelVersion {
            parameters,
            ..model
        })
    }

    async fn retry_operation<F, Fut, T>(&self, operation: F) -> Result<T>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T>>,
    {
        let mut attempts = 0;
        let mut last_error = None;
        
        while attempts < self.retry_config.max_attempts {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    last_error = Some(e);
                    attempts += 1;
                    if attempts < self.retry_config.max_attempts {
                        tokio::time::sleep(self.retry_config.backoff(attempts)).await;
                    }
                }
            }
        }
        
        Err(last_error.unwrap_or_else(|| anyhow!("Retry operation failed")))
    }

    fn compress_parameters(&self, parameters: &[f32]) -> Result<Vec<u8>> {
        let bytes = unsafe {
            std::slice::from_raw_parts(
                parameters.as_ptr() as *const u8,
                parameters.len() * std::mem::size_of::<f32>(),
            )
        };
        Ok(compress_prepend_size(bytes))
    }

    fn decompress_parameters(&self, compressed: &[u8]) -> Result<Vec<f32>> {
        let decompressed = decompress_size_prepended(compressed)?;
        let float_size = std::mem::size_of::<f32>();
        let float_count = decompressed.len() / float_size;
        
        let mut parameters = Vec::with_capacity(float_count);
        unsafe {
            let float_ptr = decompressed.as_ptr() as *const f32;
            parameters.extend_from_slice(std::slice::from_raw_parts(float_ptr, float_count));
        }
        Ok(parameters)
    }
}

struct ModelCache {
    models: HashMap<String, (ModelVersion, Instant)>,
    max_size: usize,
    ttl: Duration,
}

impl ModelCache {
    fn new() -> Self {
        Self {
            models: HashMap::new(),
            max_size: 100,
            ttl: Duration::from_secs(3600),
        }
    }

    fn add_model(&mut self, id: String, model: ModelVersion) {
        if self.models.len() >= self.max_size {
            self.evict_expired();
        }
        self.models.insert(id, (model, Instant::now()));
    }

    fn get_model(&self, id: &str) -> Option<&ModelVersion> {
        self.models.get(id).and_then(|(model, timestamp)| {
            if timestamp.elapsed() < self.ttl {
                Some(model)
            } else {
                None
            }
        })
    }

    fn evict_expired(&mut self) {
        self.models.retain(|_, (_, timestamp)| {
            timestamp.elapsed() < self.ttl
        });
    }
}

#[derive(Debug)]
struct RetryConfig {
    max_attempts: u32,
    base_delay: Duration,
    max_delay: Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(5),
        }
    }
}

impl RetryConfig {
    fn backoff(&self, attempt: u32) -> Duration {
        let delay = self.base_delay * 2_u32.pow(attempt - 1);
        std::cmp::min(delay, self.max_delay)
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
