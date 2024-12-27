use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use serde::{Serialize, Deserialize};

use crate::service::{Service, GenericService};
use crate::metrics::{UnifiedMetrics, ComponentHealth};
use crate::security::{SecurityContext, SecurityManager};
use crate::validation::ValidationResult;
use crate::protocol::repository::{
    ProtocolTransaction, ProtocolTransactionRepository,
    ProtocolType, TransactionStatus, TransactionMetadata,
    TransactionInput, TransactionOutput
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolRequest {
    pub operation: ProtocolOperation,
    pub protocol_type: ProtocolType,
    pub parameters: ProtocolParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProtocolOperation {
    CreateTransaction,
    SignTransaction,
    BroadcastTransaction,
    ValidateTransaction,
    QueryTransaction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolParameters {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
    pub fee_rate: Option<u64>,
    pub locktime: Option<u32>,
    pub rbf: Option<bool>,
    pub additional_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolResponse {
    pub success: bool,
    pub transaction_id: Option<String>,
    pub status: TransactionStatus,
    pub details: ProtocolResponseDetails,
    pub metadata: Option<TransactionMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolResponseDetails {
    pub message: String,
    pub confirmations: Option<u32>,
    pub block_height: Option<u32>,
    pub timestamp: DateTime<Utc>,
}

pub struct ProtocolService {
    repository: Arc<ProtocolTransactionRepository>,
    metrics: Arc<RwLock<UnifiedMetrics>>,
    security: Arc<dyn SecurityManager>,
    protocol_executor: Arc<dyn ProtocolExecutor>,
}

#[async_trait]
impl Service for ProtocolService {
    type Item = ProtocolRequest;
    type Response = ProtocolResponse;
    type Error = ProtocolError;

    async fn process(&self, context: &SecurityContext, request: Self::Item) -> Result<Self::Response, Self::Error> {
        let start_time = Utc::now();

        // Validate security context
        self.security.validate_context(context).await?;

        // Process based on operation type
        let result = match request.operation {
            ProtocolOperation::CreateTransaction => {
                self.create_transaction(&request).await?
            },
            ProtocolOperation::SignTransaction => {
                self.sign_transaction(context, &request).await?
            },
            ProtocolOperation::BroadcastTransaction => {
                self.broadcast_transaction(&request).await?
            },
            ProtocolOperation::ValidateTransaction => {
                self.validate_transaction(&request).await?
            },
            ProtocolOperation::QueryTransaction => {
                self.query_transaction(&request).await?
            },
        };

        // Create transaction record
        let tx = ProtocolTransaction {
            id: uuid::Uuid::new_v4().to_string(),
            protocol_type: request.protocol_type,
            status: result.status.clone(),
            created_at: start_time,
            updated_at: start_time,
            metadata: result.metadata.clone().unwrap_or_default(),
            validation_result: None,
        };

        self.repository.create(tx).await?;

        // Update metrics
        let mut metrics = self.metrics.write().await;
        metrics.protocol.as_mut().map(|p| {
            if result.success {
                p.successful_transactions += 1;
            } else {
                p.failed_transactions += 1;
            }
            p.last_transaction_time = Some(start_time);
        });

        Ok(result)
    }

    async fn validate(&self, request: &Self::Item) -> Result<ValidationResult, Self::Error> {
        // Validate inputs
        if request.parameters.inputs.is_empty() {
            return Ok(ValidationResult::Invalid("Transaction must have at least one input".to_string()));
        }

        // Validate outputs
        if request.parameters.outputs.is_empty() {
            return Ok(ValidationResult::Invalid("Transaction must have at least one output".to_string()));
        }

        // Validate fee rate if present
        if let Some(fee_rate) = request.parameters.fee_rate {
            if fee_rate == 0 {
                return Ok(ValidationResult::Invalid("Fee rate cannot be zero".to_string()));
            }
        }

        // Validate locktime if present
        if let Some(locktime) = request.parameters.locktime {
            let current_height = self.protocol_executor.get_current_height().await?;
            if locktime > current_height + 1000 {
                return Ok(ValidationResult::Invalid("Locktime too far in the future".to_string()));
            }
        }

        Ok(ValidationResult::Valid)
    }

    async fn get_health(&self) -> Result<ComponentHealth, Self::Error> {
        let txs = self.repository.list().await?;
        
        let recent_txs = txs.iter()
            .filter(|tx| {
                tx.created_at > (Utc::now() - Duration::hours(24))
            })
            .count();
            
        let failed_txs = txs.iter()
            .filter(|tx| tx.status == TransactionStatus::Failed)
            .count();
            
        let success_rate = if recent_txs > 0 {
            let successful = recent_txs - failed_txs;
            (successful as f64 / recent_txs as f64) * 100.0
        } else {
            100.0
        };

        Ok(ComponentHealth {
            operational: success_rate >= 95.0,
            health_score: success_rate,
            last_incident: txs.iter()
                .filter(|tx| tx.status == TransactionStatus::Failed)
                .map(|tx| tx.updated_at)
                .max(),
            error_count: failed_txs,
            warning_count: txs.iter()
                .filter(|tx| tx.status == TransactionStatus::Pending)
                .count(),
        })
    }
}

impl ProtocolService {
    async fn create_transaction(&self, request: &ProtocolRequest) -> Result<ProtocolResponse, ProtocolError> {
        // Implementation details...
        Ok(ProtocolResponse {
            success: true,
            transaction_id: Some(uuid::Uuid::new_v4().to_string()),
            status: TransactionStatus::Pending,
            details: ProtocolResponseDetails {
                message: "Transaction created successfully".to_string(),
                confirmations: None,
                block_height: None,
                timestamp: Utc::now(),
            },
            metadata: Some(TransactionMetadata::default()),
        })
    }

    async fn sign_transaction(&self, context: &SecurityContext, request: &ProtocolRequest) 
        -> Result<ProtocolResponse, ProtocolError> {
        // Implementation details...
        Ok(ProtocolResponse {
            success: true,
            transaction_id: Some(uuid::Uuid::new_v4().to_string()),
            status: TransactionStatus::Pending,
            details: ProtocolResponseDetails {
                message: "Transaction signed successfully".to_string(),
                confirmations: None,
                block_height: None,
                timestamp: Utc::now(),
            },
            metadata: Some(TransactionMetadata::default()),
        })
    }

    async fn broadcast_transaction(&self, request: &ProtocolRequest) 
        -> Result<ProtocolResponse, ProtocolError> {
        // Implementation details...
        Ok(ProtocolResponse {
            success: true,
            transaction_id: Some(uuid::Uuid::new_v4().to_string()),
            status: TransactionStatus::Pending,
            details: ProtocolResponseDetails {
                message: "Transaction broadcasted successfully".to_string(),
                confirmations: None,
                block_height: None,
                timestamp: Utc::now(),
            },
            metadata: Some(TransactionMetadata::default()),
        })
    }

    async fn validate_transaction(&self, request: &ProtocolRequest) 
        -> Result<ProtocolResponse, ProtocolError> {
        // Implementation details...
        Ok(ProtocolResponse {
            success: true,
            transaction_id: Some(uuid::Uuid::new_v4().to_string()),
            status: TransactionStatus::Pending,
            details: ProtocolResponseDetails {
                message: "Transaction validated successfully".to_string(),
                confirmations: None,
                block_height: None,
                timestamp: Utc::now(),
            },
            metadata: Some(TransactionMetadata::default()),
        })
    }

    async fn query_transaction(&self, request: &ProtocolRequest) 
        -> Result<ProtocolResponse, ProtocolError> {
        // Implementation details...
        Ok(ProtocolResponse {
            success: true,
            transaction_id: Some(uuid::Uuid::new_v4().to_string()),
            status: TransactionStatus::Confirmed,
            details: ProtocolResponseDetails {
                message: "Transaction found".to_string(),
                confirmations: Some(6),
                block_height: Some(700000),
                timestamp: Utc::now(),
            },
            metadata: Some(TransactionMetadata::default()),
        })
    }
}

#[async_trait]
pub trait ProtocolExecutor: Send + Sync {
    async fn get_current_height(&self) -> Result<u32, ProtocolError>;
    async fn execute_transaction(&self, tx: &ProtocolTransaction) -> Result<TransactionStatus, ProtocolError>;
    async fn get_transaction_status(&self, tx_id: &str) -> Result<TransactionStatus, ProtocolError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockProtocolExecutor;

    #[async_trait]
    impl ProtocolExecutor for MockProtocolExecutor {
        async fn get_current_height(&self) -> Result<u32, ProtocolError> {
            Ok(700000)
        }

        async fn execute_transaction(&self, _tx: &ProtocolTransaction) -> Result<TransactionStatus, ProtocolError> {
            Ok(TransactionStatus::Confirmed)
        }

        async fn get_transaction_status(&self, _tx_id: &str) -> Result<TransactionStatus, ProtocolError> {
            Ok(TransactionStatus::Confirmed)
        }
    }

    #[tokio::test]
    async fn test_protocol_service() {
        let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
        let repository = Arc::new(ProtocolTransactionRepository::new(metrics.clone()));
        let security = Arc::new(MockSecurityManager);
        let protocol_executor = Arc::new(MockProtocolExecutor);

        let service = ProtocolService {
            repository,
            metrics,
            security,
            protocol_executor,
        };

        // Test create transaction request
        let request = ProtocolRequest {
            operation: ProtocolOperation::CreateTransaction,
            protocol_type: ProtocolType::Bitcoin,
            parameters: ProtocolParameters {
                inputs: vec![TransactionInput {
                    txid: "a".repeat(64),
                    vout: 0,
                    sequence: 0xffffffff,
                    witness: None,
                }],
                outputs: vec![TransactionOutput {
                    value: 100000,
                    script_pubkey: "76a914...88ac".to_string(),
                    address: Some("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string()),
                }],
                fee_rate: Some(5),
                locktime: None,
                rbf: None,
                additional_data: None,
            },
        };

        let context = SecurityContext::default();
        let response = service.process(&context, request).await.unwrap();

        assert!(response.success);
        assert!(response.transaction_id.is_some());
        assert_eq!(response.status, TransactionStatus::Pending);

        // Test health check
        let health = service.get_health().await.unwrap();
        assert!(health.operational);
        assert_eq!(health.health_score, 100.0);
    }
}
