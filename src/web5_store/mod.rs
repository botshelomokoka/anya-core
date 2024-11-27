use serde::{Deserialize, Serialize};
use std::error::Error;
use web5::Web5;
use thiserror::Error;
use metrics::{Counter, Gauge, Histogram, register_counter, register_gauge, register_histogram};
use chrono::Utc;

#[derive(Error, Debug)]
pub enum Web5StoreError {
    #[error("Record not found: {0}")]
    RecordNotFound(String),
    #[error("DWN error: {0}")]
    DWNError(String),
    #[error("Protocol error: {0}")]
    ProtocolError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataRecord {
    pub id: String,
    pub data: Vec<u8>,
    pub timestamp: i64,
    pub metadata: Option<serde_json::Value>,
    pub schema: String,
    pub protocol_version: String,
    pub encryption_enabled: bool,
}

pub struct Web5Metrics {
    operations_total: Counter,
    operation_duration: Histogram,
    active_connections: Gauge,
    protocol_operations: Counter,
    storage_size: Gauge,
    error_count: Counter,
    record_size: Histogram,
}

impl Web5Metrics {
    pub fn new() -> Self {
        Self {
            operations_total: register_counter!("web5_operations_total"),
            operation_duration: register_histogram!("web5_operation_duration_seconds"),
            active_connections: register_gauge!("web5_active_connections"),
            protocol_operations: register_counter!("web5_protocol_operations_total"),
            storage_size: register_gauge!("web5_storage_size_bytes"),
            error_count: register_counter!("web5_error_count"),
            record_size: register_histogram!("web5_record_size_bytes"),
        }
    }

    pub fn record_operation(&self, duration: f64) {
        self.operations_total.increment(1);
        self.operation_duration.record(duration);
    }

    pub fn record_protocol_operation(&self, protocol: &str) {
        self.protocol_operations.increment(1);
        // Add protocol-specific label
    }

    pub fn update_storage_size(&self, size: i64) {
        self.storage_size.set(size as f64);
    }

    pub fn record_error(&self, error_type: &str) {
        self.error_count.increment(1);
        // Add error type label
    }

    pub fn record_record_size(&self, size: usize) {
        self.record_size.record(size as f64);
    }

    pub fn connection_started(&self) {
        self.active_connections.increment(1.0);
    }

    pub fn connection_ended(&self) {
        self.active_connections.decrement(1.0);
    }
}

pub struct Web5Store {
    web5: Web5,
    protocol_uri: String,
    metrics: Web5Metrics,
}

impl Web5Store {
    pub async fn new() -> Result<Self, Web5StoreError> {
        let web5 = Web5::connect()
            .await
            .map_err(|e| Web5StoreError::DWNError(e.to_string()))?;
            
        let protocol_uri = "https://anya.blockchain/ml-data".to_string();
        
        // Register protocol with enhanced schema
        let protocol_def = serde_json::json!({
            "protocol": protocol_uri,
            "types": {
                "data": {
                    "schema": "https://anya.ai/schemas/data",
                    "dataFormats": ["application/octet-stream", "application/json"]
                },
                "metadata": {
                    "schema": "https://anya.ai/schemas/metadata",
                    "dataFormats": ["application/json"]
                }
            },
            "structure": {
                "data": {
                    "$actions": [
                        { "who": "author", "can": "write" },
                        { "who": "recipient", "can": "read" }
                    ]
                }
            }
        });
        
        web5.did_manager()
            .register_protocol_with_schema(&protocol_uri, protocol_def)
            .await
            .map_err(|e| Web5StoreError::ProtocolError(e.to_string()))?;
        
        Ok(Self {
            web5,
            protocol_uri,
            metrics: Web5Metrics::new(),
        })
    }

    pub async fn store_data(&self, data: Vec<u8>, metadata: Option<serde_json::Value>) -> Result<String, Web5StoreError> {
        let start = std::time::Instant::now();
        self.metrics.connection_started();
        
        let record = DataRecord {
            id: uuid::Uuid::new_v4().to_string(),
            data: data.clone(),
            timestamp: Utc::now().timestamp(),
            metadata,
            schema: "https://anya.ai/schemas/data".to_string(),
            protocol_version: "1.0".to_string(),
            encryption_enabled: false,
        };

        self.metrics.record_record_size(data.len());
        
        let record_id = self.web5.dwn()
            .records()
            .create(&self.protocol_uri, &record)
            .await
            .map_err(|e| Web5StoreError::DWNError(e.to_string()))?;

        let duration = start.elapsed().as_secs_f64();
        self.metrics.record_operation(duration);
        self.metrics.connection_ended();
        
        Ok(record_id)
    }

    pub async fn get_data(&self, record_id: &str) -> Result<Option<DataRecord>, Web5StoreError> {
        let start = std::time::Instant::now();
        self.metrics.connection_started();
        
        let record = self.web5.dwn()
            .records()
            .read(record_id)
            .await
            .map_err(|e| Web5StoreError::DWNError(e.to_string()))?;

        let duration = start.elapsed().as_secs_f64();
        self.metrics.record_operation(duration);
        self.metrics.connection_ended();
        
        match record {
            Some(r) => {
                let data: DataRecord = r.data()
                    .map_err(|e| Web5StoreError::SerializationError(e.to_string()))?;
                Ok(Some(data))
            }
            None => Ok(None),
        }
    }

    pub async fn update_data(&self, record_id: &str, data: Vec<u8>, metadata: Option<serde_json::Value>) -> Result<(), Web5StoreError> {
        let start = std::time::Instant::now();
        self.metrics.connection_started();
        
        let mut record = match self.get_data(record_id).await? {
            Some(r) => r,
            None => return Err(Web5StoreError::RecordNotFound(record_id.to_string())),
        };

        record.data = data.clone();
        record.timestamp = Utc::now().timestamp();
        if let Some(meta) = metadata {
            record.metadata = Some(meta);
        }

        self.metrics.record_record_size(data.len());
        
        self.web5.dwn()
            .records()
            .update(record_id, &record)
            .await
            .map_err(|e| Web5StoreError::DWNError(e.to_string()))?;

        let duration = start.elapsed().as_secs_f64();
        self.metrics.record_operation(duration);
        self.metrics.connection_ended();
        
        Ok(())
    }

    pub async fn delete_data(&self, record_id: &str) -> Result<(), Web5StoreError> {
        let start = std::time::Instant::now();
        self.metrics.connection_started();
        
        self.web5.dwn()
            .records()
            .delete(record_id)
            .await
            .map_err(|e| Web5StoreError::DWNError(e.to_string()))?;

        let duration = start.elapsed().as_secs_f64();
        self.metrics.record_operation(duration);
        self.metrics.connection_ended();
        
        Ok(())
    }

    pub async fn query_data(&self, query: serde_json::Value) -> Result<Vec<DataRecord>, Web5StoreError> {
        let start = std::time::Instant::now();
        self.metrics.connection_started();
        
        let records = self.web5.dwn()
            .records()
            .query(&self.protocol_uri)
            .filter(query)
            .execute()
            .await
            .map_err(|e| Web5StoreError::DWNError(e.to_string()))?;

        let duration = start.elapsed().as_secs_f64();
        self.metrics.record_operation(duration);
        self.metrics.connection_ended();
        
        let mut results = Vec::new();
        for record in records {
            let data: DataRecord = record.data()
                .map_err(|e| Web5StoreError::SerializationError(e.to_string()))?;
            results.push(data);
        }

        Ok(results)
    }

    pub fn get_metrics(&self) -> &Web5Metrics {
        &self.metrics
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_store_and_retrieve() -> Result<(), Web5StoreError> {
        let store = Web5Store::new().await?;
        
        // Test data
        let data = b"test data".to_vec();
        let metadata = Some(serde_json::json!({
            "test": "metadata"
        }));
        
        // Store
        let record_id = store.store_data(data.clone(), metadata.clone()).await?;
        
        // Retrieve
        let retrieved = store.get_data(&record_id).await?
            .expect("Data should exist");
            
        assert_eq!(retrieved.data, data);
        assert_eq!(retrieved.metadata, metadata);
        
        Ok(())
    }

    #[test]
    async fn test_metrics_recording() {
        let store = Web5Store::new().await.unwrap();
        let data = b"test data".to_vec();
        let metadata = Some(serde_json::json!({
            "test": "metadata"
        }));
        
        let result = store.store_data(data.clone(), metadata.clone()).await;
        assert!(result.is_ok());
        
        // Verify metrics were recorded
        // Note: In a real test we'd need a way to read metric values
    }

    #[test]
    async fn test_error_metrics() {
        let store = Web5Store::new().await.unwrap();
        let invalid_data = b"".to_vec(); // Invalid data
        
        let result = store.store_data(invalid_data, None).await;
        assert!(result.is_err());
        
        // Verify error was recorded in metrics
    }
}
