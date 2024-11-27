use crate::web5::{protocol::Protocol, validator::{SchemaValidator, ValidationError}};
use serde::{Deserialize, Serialize};
use web5::did::{KeyMethod, PrimaryDid};
use web5::dwn::{DataFormat, Message, RecordId};
use web5::Web5;
use std::collections::HashMap;
use async_trait::async_trait;
use thiserror::Error;
use crate::web5::cache::{Cache, Web5Cache, CacheConfig};
use crate::web5::batch::{BatchProcessor, BatchOptions};
use crate::web5::events::{EventBus, EventPublisher, EventType};
use crate::monitoring::health::{HealthMonitor, SystemStatus};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationError),
    #[error("Web5 error: {0}")]
    Web5Error(String),
    #[error("Record not found: {0}")]
    NotFound(String),
    #[error("Schema error: {0}")]
    SchemaError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataRecord {
    pub id: String,
    pub table_name: String,
    pub data: serde_json::Value,
    pub created_at: i64,
    pub updated_at: i64,
    pub version: u32,
    pub owner_did: String,
}

#[async_trait]
pub trait DataStore {
    async fn create_record(&self, table_name: &str, data: serde_json::Value) -> Result<String, StoreError>;
    async fn get_record(&self, record_id: &str) -> Result<DataRecord, StoreError>;
    async fn update_record(&self, record_id: &str, data: serde_json::Value) -> Result<(), StoreError>;
    async fn delete_record(&self, record_id: &str) -> Result<(), StoreError>;
    async fn query_records(&self, table_name: &str, filter: Option<serde_json::Value>) -> Result<Vec<DataRecord>, StoreError>;
}

pub struct Web5Store {
    web5: Web5,
    did: PrimaryDid,
    validator: SchemaValidator,
    protocol: Protocol,
    cache: Web5Cache,
    batch_processor: Option<BatchProcessor<Self>>,
    event_publisher: EventPublisher,
    health_monitor: Arc<HealthMonitor>,
}

impl Web5Store {
    pub async fn new() -> Result<Self, StoreError> {
        let (web5, did) = Web5::connect(None)
            .await
            .map_err(|e| StoreError::Web5Error(e.to_string()))?;

        // Initialize protocol
        let protocol = Protocol::new();
        let protocol_msg = protocol.to_dwn_protocol();
        
        // Configure protocol
        web5.dwn.protocols.configure(&protocol_msg)
            .await
            .map_err(|e| StoreError::Web5Error(e.to_string()))?;

        // Initialize validator
        let mut validator = SchemaValidator::new();
        for (name, schema_def) in &protocol.types {
            validator.register_schema(name, &schema_def.schema)
                .map_err(|e| StoreError::SchemaError(e.to_string()))?;
        }

        let cache = Web5Cache::new(CacheConfig::default());
        let event_bus = Arc::new(EventBus::new(1000));
        let event_publisher = EventPublisher::new(Arc::clone(&event_bus), "web5_store");
        let health_monitor = Arc::new(HealthMonitor::new(Arc::clone(&event_bus)));
        
        let store = Self { 
            web5, 
            did, 
            validator, 
            protocol, 
            cache, 
            batch_processor: None, 
            event_publisher,
            health_monitor,
        };
        
        // Set the batch processor after store construction
        store.batch_processor = Some(BatchProcessor::new(
            store.clone(),
            BatchOptions::default()
        ));
        
        // Start health monitoring
        health_monitor.start().await;
        
        Ok(store)
    }

    pub async fn create_table(&self, name: &str, schema: HashMap<String, String>) -> Result<(), StoreError> {
        let schema_data = serde_json::json!({
            "name": name,
            "fields": schema,
            "created_at": chrono::Utc::now().timestamp()
        });

        self.validator.validate("schema", &schema_data)
            .map_err(StoreError::ValidationError)?;

        let message = Message {
            data: serde_json::to_vec(&schema_data).unwrap(),
            data_format: DataFormat::Json,
            schema: "schema".to_string(),
            protocol: Some(self.protocol.protocol_url.clone()),
            ..Default::default()
        };

        self.web5.dwn.records.create(&message)
            .await
            .map_err(|e| StoreError::Web5Error(e.to_string()))?;

        Ok(())
    }

    async fn validate_record(&self, table_name: &str, data: &serde_json::Value) -> Result<(), StoreError> {
        // Get table schema
        let query = Message {
            filter: Some(format!("{{\"name\": \"{}\"}}", table_name)),
            schema: Some("schema".to_string()),
            protocol: Some(self.protocol.protocol_url.clone()),
            ..Default::default()
        };

        let schemas = self.web5.dwn.records.query(&query)
            .await
            .map_err(|e| StoreError::Web5Error(e.to_string()))?;

        if schemas.is_empty() {
            return Err(StoreError::NotFound(format!("Table '{}' not found", table_name)));
        }

        let schema: serde_json::Value = serde_json::from_slice(&schemas[0].data)
            .map_err(|e| StoreError::SchemaError(e.to_string()))?;

        self.validator.validate("record", data)
            .map_err(StoreError::ValidationError)
    }

    pub async fn get_cached<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>, StoreError> {
        // Try cache first
        if let Ok(Some(value)) = self.cache.get(key).await {
            return Ok(Some(value));
        }
        
        // Cache miss, try store
        if let Some(value) = self.get_record(key).await? {
            // Cache the value for future use
            let _ = self.cache.set(key, &value, None).await;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    pub async fn bulk_create<T: Serialize>(&self, table: &str, records: Vec<T>) -> Result<Vec<String>, StoreError> {
        self.batch_processor
            .as_ref()
            .unwrap()
            .bulk_insert(table, records)
            .await
            .map_err(StoreError::from)
    }
    
    pub async fn bulk_update(&self, table: &str, updates: HashMap<String, serde_json::Value>) -> Result<Vec<BatchResult>, StoreError> {
        self.batch_processor
            .as_ref()
            .unwrap()
            .bulk_update(table, updates)
            .await
            .map_err(StoreError::from)
    }
    
    pub async fn bulk_delete(&self, table: &str, ids: Vec<String>) -> Result<Vec<BatchResult>, StoreError> {
        self.batch_processor
            .as_ref()
            .unwrap()
            .bulk_delete(table, ids)
            .await
            .map_err(StoreError::from)
    }

    pub async fn create_record(&self, table_name: &str, data: serde_json::Value) -> Result<String, StoreError> {
        let start_time = Instant::now();
        let result = self.create_record_internal(table_name, data).await;
        
        // Record metrics and events
        let duration = start_time.elapsed().as_secs_f64();
        // self.metrics_collector.record_performance_metric("storage", "create", duration).await;
        
        if let Ok(ref record_id) = result {
            self.event_publisher.publish_event(
                EventType::RecordCreated,
                json!({
                    "table": table_name,
                    "record_id": record_id,
                    "duration": duration
                }),
                None,
                None,
                vec!["storage".to_string()],
            )?;
        }
        
        result
    }

    async fn create_record_internal(&self, table_name: &str, data: serde_json::Value) -> Result<String, StoreError> {
        self.validate_record(table_name, &data).await?;

        let now = chrono::Utc::now().timestamp();
        let record = DataRecord {
            id: uuid::Uuid::new_v4().to_string(),
            table_name: table_name.to_string(),
            data,
            created_at: now,
            updated_at: now,
            version: 1,
            owner_did: self.did.to_string(),
        };

        let message = Message {
            data: serde_json::to_vec(&record).unwrap(),
            data_format: DataFormat::Json,
            schema: "record".to_string(),
            protocol: Some(self.protocol.protocol_url.clone()),
            ..Default::default()
        };

        let response = self.web5.dwn.records.create(&message)
            .await
            .map_err(|e| StoreError::Web5Error(e.to_string()))?;

        Ok(response.record_id.to_string())
    }

    pub async fn get_record(&self, record_id: &str) -> Result<DataRecord, StoreError> {
        let record = self.web5.dwn.records.read(&RecordId::from(record_id))
            .await
            .map_err(|e| StoreError::Web5Error(e.to_string()))?
            .ok_or_else(|| StoreError::NotFound(record_id.to_string()))?;

        let data: DataRecord = serde_json::from_slice(&record.data)
            .map_err(|e| StoreError::SchemaError(e.to_string()))?;

        Ok(data)
    }

    pub async fn update_record(&self, record_id: &str, data: serde_json::Value) -> Result<(), StoreError> {
        let mut record = self.get_record(record_id).await?;
        
        self.validate_record(&record.table_name, &data).await?;
        
        record.data = data;
        record.updated_at = chrono::Utc::now().timestamp();
        record.version += 1;

        let updated_data = serde_json::to_vec(&record)
            .map_err(|e| StoreError::SchemaError(e.to_string()))?;

        self.web5.dwn.records.update(record_id, &updated_data)
            .await
            .map_err(|e| StoreError::Web5Error(e.to_string()))?;

        Ok(())
    }

    pub async fn delete_record(&self, record_id: &str) -> Result<(), StoreError> {
        self.web5.dwn.records.delete(record_id)
            .await
            .map_err(|e| StoreError::Web5Error(e.to_string()))?;
        Ok(())
    }

    pub async fn query_records(&self, table_name: &str, filter: Option<serde_json::Value>) 
        -> Result<Vec<DataRecord>, StoreError> {
        let mut query_filter = serde_json::json!({
            "table_name": table_name
        });

        if let Some(f) = filter {
            if let Some(obj) = query_filter.as_object_mut() {
                obj.extend(f.as_object().unwrap().clone());
            }
        }

        let query = Message {
            filter: Some(serde_json::to_string(&query_filter).unwrap()),
            schema: Some("record".to_string()),
            protocol: Some(self.protocol.protocol_url.clone()),
            ..Default::default()
        };

        let records = self.web5.dwn.records.query(&query)
            .await
            .map_err(|e| StoreError::Web5Error(e.to_string()))?;

        let mut results = Vec::new();
        for record in records {
            if let Ok(data) = serde_json::from_slice::<DataRecord>(&record.data) {
                results.push(data);
            }
        }

        Ok(results)
    }

    pub async fn get_health_status(&self) -> SystemStatus {
        self.health_monitor.get_health_status().await
    }
}

#[async_trait]
impl DataStore for Web5Store {
    async fn create_record(&self, table_name: &str, data: serde_json::Value) -> Result<String, StoreError> {
        self.create_record(table_name, data).await
    }

    async fn get_record(&self, record_id: &str) -> Result<DataRecord, StoreError> {
        self.get_record(record_id).await
    }

    async fn update_record(&self, record_id: &str, data: serde_json::Value) -> Result<(), StoreError> {
        self.update_record(record_id, data).await
    }

    async fn delete_record(&self, record_id: &str) -> Result<(), StoreError> {
        self.delete_record(record_id).await
    }

    async fn query_records(&self, table_name: &str, filter: Option<serde_json::Value>) -> Result<Vec<DataRecord>, StoreError> {
        self.query_records(table_name, filter).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_web5_store() -> Result<(), StoreError> {
        let store = Web5Store::new().await?;

        // Create a table
        let mut schema = HashMap::new();
        schema.insert("name".to_string(), "string".to_string());
        schema.insert("age".to_string(), "number".to_string());
        store.create_table("users", schema).await?;

        // Create a record
        let user_data = json!({
            "name": "Alice",
            "age": 30
        });
        let record_id = store.create_record("users", user_data).await?;

        // Get record
        let record = store.get_record(&record_id).await?;
        assert_eq!(record.data["name"], "Alice");

        // Update record
        let updated_data = json!({
            "name": "Alice",
            "age": 31
        });
        store.update_record(&record_id, updated_data).await?;

        // Query records
        let filter = Some(json!({
            "age": { "$gt": 25 }
        }));
        let results = store.query_records("users", filter).await?;
        assert_eq!(results.len(), 1);

        // Delete record
        store.delete_record(&record_id).await?;

        Ok(())
    }
}
