use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn, error};
use metrics::{counter, gauge};
use serde::{Serialize, Deserialize};

#[derive(Error, Debug)]
pub enum DataIntegrationError {
    #[error("Data encryption failed: {0}")]
    EncryptionError(String),
    #[error("Permission verification failed: {0}")]
    PermissionError(String),
    #[error("Protocol handling failed: {0}")]
    ProtocolError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Web5MLDataIntegration {
    dwn: Arc<DWN>,
    encryption_manager: Arc<EncryptionManager>,
    protocol_handler: Arc<ProtocolHandler>,
    metrics: DataMetrics,
}

impl Web5MLDataIntegration {
    pub async fn new(
        dwn: Arc<DWN>,
        did_manager: Arc<DIDManager>,
    ) -> Result<Self> {
        let encryption_manager = Arc::new(EncryptionManager::new(did_manager.clone()));
        let protocol_handler = Arc::new(ProtocolHandler::new(dwn.clone(), did_manager));

        Ok(Self {
            dwn,
            encryption_manager,
            protocol_handler,
            metrics: DataMetrics::new(),
        })
    }

    pub async fn process_ml_data(&self, data: MLDataRecord) -> Result<Vec<u8>> {
        // Verify data permissions
        self.verify_data_permissions(&data).await?;
        
        // Encrypt data for storage
        let encrypted_data = self.encrypt_ml_data(&data).await?;
        
        // Create DWN record
        let record = self.create_data_record(encrypted_data, &data).await?;
        
        // Store record
        self.store_record(record).await?;
        
        self.metrics.record_data_processed();
        Ok(encrypted_data)
    }

    async fn verify_data_permissions(&self, data: &MLDataRecord) -> Result<()> {
        self.encryption_manager.verify_permissions(&data.permissions)
            .map_err(|e| DataIntegrationError::PermissionError(e.to_string()))
    }

    async fn encrypt_ml_data(&self, data: &MLDataRecord) -> Result<Vec<u8>> {
        self.encryption_manager.encrypt(&data.data)
            .map_err(|e| DataIntegrationError::EncryptionError(e.to_string()))
    }

    async fn create_data_record(&self, encrypted_data: Vec<u8>, data: &MLDataRecord) -> Result<Record> {
        let record = Record::new()
            .with_protocol(&data.protocol)
            .with_schema(&data.schema)
            .with_data(encrypted_data)
            .with_owner_did(&data.owner_did)
            .build()?;
        
        Ok(record)
    }

    async fn store_record(&self, record: Record) -> Result<()> {
        self.dwn.create_record(record).await
            .map_err(|e| DataIntegrationError::ProtocolError(e.to_string()))
    }
}
