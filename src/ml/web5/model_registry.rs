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
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use log::{info, warn, error};

#[derive(Error, Debug)]
pub enum ModelRegistryError {
    #[error("Model validation failed: {0}")]
    ValidationError(String),
    #[error("Storage operation failed: {0}")]
    StorageError(String),
    #[error("DID verification failed: {0}")]
    DIDError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelMetadata {
    model_id: String,
    version: String,
    owner_did: String,
    created_at: u64,
    updated_at: u64,
    permissions: Vec<Permission>,
}

pub struct Web5ModelRegistry {
    dwn: Arc<DWN>,
    did_manager: Arc<DIDManager>,
    data_handler: Arc<Web5DataHandler>,
    models: RwLock<HashMap<String, ModelMetadata>>,
    metrics: RegistryMetrics,
}

impl Web5ModelRegistry {
    pub async fn new(
        dwn: Arc<DWN>,
        did_manager: Arc<DIDManager>,
        data_handler: Arc<Web5DataHandler>,
    ) -> Result<Self> {
        Ok(Self {
            dwn,
            did_manager,
            data_handler,
            models: RwLock::new(HashMap::new()),
            metrics: RegistryMetrics::new(),
        })
    }

    pub async fn register_model(&self, model: Web5MLModel, owner_did: &str) -> Result<String> {
        // Validate model and owner
        self.validate_model(&model).await?;
        self.did_manager.verify_did(owner_did).await?;

        // Create metadata
        let model_id = self.generate_model_id();
        let metadata = ModelMetadata {
            model_id: model_id.clone(),
            version: "1.0.0".to_string(),
            owner_did: owner_did.to_string(),
            created_at: chrono::Utc::now().timestamp() as u64,
            updated_at: chrono::Utc::now().timestamp() as u64,
            permissions: vec![Permission::OwnerOnly],
        };

        // Store model and metadata
        self.store_model(&model, &metadata).await?;
        
        // Update registry
        self.models.write().await.insert(model_id.clone(), metadata);
        
        self.metrics.record_model_registered();
        Ok(model_id)
    }

    pub async fn update_model(&self, model_id: &str, model: Web5MLModel) -> Result<()> {
        let mut models = self.models.write().await;
        let metadata = models.get_mut(model_id)
            .ok_or_else(|| ModelRegistryError::ValidationError("Model not found".into()))?;

        // Update metadata
        metadata.version = increment_version(&metadata.version);
        metadata.updated_at = chrono::Utc::now().timestamp() as u64;

        // Store updated model
        self.store_model(&model, metadata).await?;
        
        self.metrics.record_model_updated();
        Ok(())
    }

    async fn store_model(&self, model: &Web5MLModel, metadata: &ModelMetadata) -> Result<()> {
        let record = MLDataRecord {
            protocol: "ml.model.registry".to_string(),
            schema: "model-metadata".to_string(),
            data: serde_json::to_vec(&metadata)?,
            owner_did: metadata.owner_did.clone(),
            permissions: metadata.permissions.clone(),
        };

        self.data_handler.store_training_data(&record.data, &record.owner_did).await?;
        Ok(())
    }
}


