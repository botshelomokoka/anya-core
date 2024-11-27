use super::protocol::Protocol;
use crate::storage::web5_store::{DataRecord, DataStore, StoreError};
use async_trait::async_trait;
use futures::future::try_join_all;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use tokio::sync::Semaphore;

#[derive(Debug, Error)]
pub enum BatchError {
    #[error("Store error: {0}")]
    Store(#[from] StoreError),
    #[error("Batch operation failed: {0}")]
    Operation(String),
    #[error("Transaction error: {0}")]
    Transaction(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchOperation {
    pub operation_type: BatchOperationType,
    pub table_name: String,
    pub record_id: Option<String>,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BatchOperationType {
    Create,
    Update,
    Delete,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchResult {
    pub success: bool,
    pub operation_index: usize,
    pub record_id: Option<String>,
    pub error: Option<String>,
}

pub struct BatchOptions {
    pub max_concurrent: usize,
    pub stop_on_error: bool,
    pub timeout: std::time::Duration,
}

impl Default for BatchOptions {
    fn default() -> Self {
        Self {
            max_concurrent: 10,
            stop_on_error: false,
            timeout: std::time::Duration::from_secs(30),
        }
    }
}

pub struct BatchProcessor<S: DataStore> {
    store: S,
    options: BatchOptions,
}

impl<S: DataStore> BatchProcessor<S> {
    pub fn new(store: S, options: BatchOptions) -> Self {
        Self { store, options }
    }

    pub async fn process_batch(&self, operations: Vec<BatchOperation>) -> Result<Vec<BatchResult>, BatchError> {
        let semaphore = Arc::new(Semaphore::new(self.options.max_concurrent));
        let mut results = Vec::with_capacity(operations.len());
        let mut handles = Vec::new();

        for (index, operation) in operations.into_iter().enumerate() {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let store = self.store.clone();
            let stop_on_error = self.options.stop_on_error;

            let handle = tokio::spawn(async move {
                let result = match operation.operation_type {
                    BatchOperationType::Create => {
                        if let Some(data) = operation.data {
                            match store.create_record(&operation.table_name, data).await {
                                Ok(record_id) => BatchResult {
                                    success: true,
                                    operation_index: index,
                                    record_id: Some(record_id),
                                    error: None,
                                },
                                Err(e) => BatchResult {
                                    success: false,
                                    operation_index: index,
                                    record_id: None,
                                    error: Some(e.to_string()),
                                },
                            }
                        } else {
                            BatchResult {
                                success: false,
                                operation_index: index,
                                record_id: None,
                                error: Some("Missing data for create operation".to_string()),
                            }
                        }
                    },
                    BatchOperationType::Update => {
                        if let (Some(record_id), Some(data)) = (operation.record_id, operation.data) {
                            match store.update_record(&record_id, data).await {
                                Ok(_) => BatchResult {
                                    success: true,
                                    operation_index: index,
                                    record_id: Some(record_id),
                                    error: None,
                                },
                                Err(e) => BatchResult {
                                    success: false,
                                    operation_index: index,
                                    record_id: Some(record_id),
                                    error: Some(e.to_string()),
                                },
                            }
                        } else {
                            BatchResult {
                                success: false,
                                operation_index: index,
                                record_id: None,
                                error: Some("Missing record_id or data for update operation".to_string()),
                            }
                        }
                    },
                    BatchOperationType::Delete => {
                        if let Some(record_id) = operation.record_id {
                            match store.delete_record(&record_id).await {
                                Ok(_) => BatchResult {
                                    success: true,
                                    operation_index: index,
                                    record_id: Some(record_id),
                                    error: None,
                                },
                                Err(e) => BatchResult {
                                    success: false,
                                    operation_index: index,
                                    record_id: Some(record_id),
                                    error: Some(e.to_string()),
                                },
                            }
                        } else {
                            BatchResult {
                                success: false,
                                operation_index: index,
                                record_id: None,
                                error: Some("Missing record_id for delete operation".to_string()),
                            }
                        }
                    },
                };

                drop(permit);
                result
            });

            handles.push(handle);

            if stop_on_error {
                let result = handles.last().unwrap().await.unwrap();
                if !result.success {
                    return Err(BatchError::Operation(format!(
                        "Operation {} failed: {}",
                        result.operation_index,
                        result.error.unwrap_or_default()
                    )));
                }
            }
        }

        for handle in handles {
            results.push(handle.await.unwrap());
        }

        Ok(results)
    }

    pub async fn bulk_insert<T: Serialize>(
        &self,
        table_name: &str,
        records: Vec<T>,
    ) -> Result<Vec<String>, BatchError> {
        let operations: Vec<BatchOperation> = records
            .into_iter()
            .map(|record| BatchOperation {
                operation_type: BatchOperationType::Create,
                table_name: table_name.to_string(),
                record_id: None,
                data: Some(serde_json::to_value(record).unwrap()),
            })
            .collect();

        let results = self.process_batch(operations).await?;
        Ok(results
            .into_iter()
            .filter_map(|r| r.record_id)
            .collect())
    }

    pub async fn bulk_update(
        &self,
        table_name: &str,
        updates: HashMap<String, serde_json::Value>,
    ) -> Result<Vec<BatchResult>, BatchError> {
        let operations: Vec<BatchOperation> = updates
            .into_iter()
            .map(|(record_id, data)| BatchOperation {
                operation_type: BatchOperationType::Update,
                table_name: table_name.to_string(),
                record_id: Some(record_id),
                data: Some(data),
            })
            .collect();

        self.process_batch(operations).await
    }

    pub async fn bulk_delete(
        &self,
        table_name: &str,
        record_ids: Vec<String>,
    ) -> Result<Vec<BatchResult>, BatchError> {
        let operations: Vec<BatchOperation> = record_ids
            .into_iter()
            .map(|record_id| BatchOperation {
                operation_type: BatchOperationType::Delete,
                table_name: table_name.to_string(),
                record_id: Some(record_id),
                data: None,
            })
            .collect();

        self.process_batch(operations).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_batch_operations() {
        // Create test store and processor
        let store = TestStore::new();
        let options = BatchOptions {
            max_concurrent: 5,
            stop_on_error: false,
            timeout: std::time::Duration::from_secs(10),
        };
        let processor = BatchProcessor::new(store, options);

        // Test bulk insert
        let records = vec![
            json!({"name": "Test 1", "value": 1}),
            json!({"name": "Test 2", "value": 2}),
        ];
        let record_ids = processor.bulk_insert("test_table", records).await.unwrap();
        assert_eq!(record_ids.len(), 2);

        // Test bulk update
        let mut updates = HashMap::new();
        updates.insert(record_ids[0].clone(), json!({"name": "Updated 1", "value": 10}));
        let update_results = processor.bulk_update("test_table", updates).await.unwrap();
        assert!(update_results[0].success);

        // Test bulk delete
        let delete_results = processor.bulk_delete("test_table", record_ids).await.unwrap();
        assert!(delete_results.iter().all(|r| r.success));
    }
}
