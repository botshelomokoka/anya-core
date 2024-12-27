use std::sync::Arc;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::repository::{Repository, GenericRepository};
use crate::metrics::{UnifiedMetrics, ComponentHealth};
use crate::validation::{ValidationResult, Validator};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolTransaction {
    pub id: String,
    pub protocol_type: ProtocolType,
    pub status: TransactionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: TransactionMetadata,
    pub validation_result: Option<ValidationResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProtocolType {
    Bitcoin,
    Lightning,
    Liquid,
    DLC,
    Taproot,
    RSK,
    StateChain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
    Expired,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionMetadata {
    pub tx_hash: Option<String>,
    pub block_height: Option<u32>,
    pub confirmations: Option<u32>,
    pub fee_rate: Option<u64>,
    pub value: Option<u64>,
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInput {
    pub txid: String,
    pub vout: u32,
    pub sequence: u32,
    pub witness: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionOutput {
    pub value: u64,
    pub script_pubkey: String,
    pub address: Option<String>,
}

pub struct ProtocolTransactionValidator;

#[async_trait]
impl Validator<ProtocolTransaction> for ProtocolTransactionValidator {
    async fn validate(&self, tx: &ProtocolTransaction) -> Result<ValidationResult, ValidationError> {
        // Validate timestamps
        if tx.created_at > Utc::now() || tx.updated_at > Utc::now() {
            return Ok(ValidationResult::Invalid("Transaction timestamps cannot be in the future".to_string()));
        }

        // Validate metadata
        if let Some(hash) = &tx.metadata.tx_hash {
            if !is_valid_txid(hash) {
                return Ok(ValidationResult::Invalid("Invalid transaction hash format".to_string()));
            }
        }

        // Validate inputs
        for input in &tx.metadata.inputs {
            if !is_valid_txid(&input.txid) {
                return Ok(ValidationResult::Invalid("Invalid input txid format".to_string()));
            }
        }

        // Validate outputs
        for output in &tx.metadata.outputs {
            if output.value == 0 {
                return Ok(ValidationResult::Invalid("Output value cannot be zero".to_string()));
            }
            if output.script_pubkey.is_empty() {
                return Ok(ValidationResult::Invalid("Script pubkey cannot be empty".to_string()));
            }
            if let Some(addr) = &output.address {
                if !is_valid_address(addr) {
                    return Ok(ValidationResult::Invalid("Invalid address format".to_string()));
                }
            }
        }

        Ok(ValidationResult::Valid)
    }
}

fn is_valid_txid(txid: &str) -> bool {
    txid.len() == 64 && txid.chars().all(|c| c.is_ascii_hexdigit())
}

fn is_valid_address(address: &str) -> bool {
    // Basic Bitcoin address validation - could be more sophisticated
    address.len() >= 26 && address.len() <= 35 &&
    (address.starts_with('1') || address.starts_with('3') || address.starts_with('b'))
}

pub type ProtocolTransactionRepository = GenericRepository<ProtocolTransaction, ProtocolError>;

impl ProtocolTransactionRepository {
    pub fn new(metrics: Arc<RwLock<UnifiedMetrics>>) -> Self {
        Self::new(
            metrics,
            Arc::new(ProtocolTransactionValidator),
        )
    }

    pub async fn get_transactions_by_status(&self, status: TransactionStatus) -> Result<Vec<ProtocolTransaction>, ProtocolError> {
        let txs = self.list().await?;
        Ok(txs.into_iter()
            .filter(|tx| tx.status == status)
            .collect())
    }

    pub async fn get_transactions_by_protocol(&self, protocol: ProtocolType) -> Result<Vec<ProtocolTransaction>, ProtocolError> {
        let txs = self.list().await?;
        Ok(txs.into_iter()
            .filter(|tx| tx.protocol_type == protocol)
            .collect())
    }

    pub async fn get_confirmed_transactions(&self) -> Result<Vec<ProtocolTransaction>, ProtocolError> {
        let txs = self.list().await?;
        Ok(txs.into_iter()
            .filter(|tx| {
                tx.status == TransactionStatus::Confirmed && 
                tx.metadata.confirmations.unwrap_or(0) > 0
            })
            .collect())
    }

    pub async fn cleanup_expired_transactions(&self) -> Result<usize, ProtocolError> {
        let mut count = 0;
        let txs = self.list().await?;
        let now = Utc::now();
        
        for tx in txs {
            if tx.status == TransactionStatus::Pending {
                let age = now.signed_duration_since(tx.created_at).num_hours();
                if age > 24 { // Expire after 24 hours
                    let mut expired_tx = tx.clone();
                    expired_tx.status = TransactionStatus::Expired;
                    self.update(&tx.id, expired_tx).await?;
                    count += 1;
                }
            }
        }
        
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_protocol_transaction_repository() {
        let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
        let repo = ProtocolTransactionRepository::new(metrics);

        // Create test transaction
        let tx = ProtocolTransaction {
            id: "test-1".to_string(),
            protocol_type: ProtocolType::Bitcoin,
            status: TransactionStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: TransactionMetadata {
                tx_hash: Some("a".repeat(64)),
                block_height: Some(700000),
                confirmations: Some(1),
                fee_rate: Some(5),
                value: Some(100000),
                inputs: vec![TransactionInput {
                    txid: "b".repeat(64),
                    vout: 0,
                    sequence: 0xffffffff,
                    witness: None,
                }],
                outputs: vec![TransactionOutput {
                    value: 100000,
                    script_pubkey: "76a914...88ac".to_string(),
                    address: Some("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string()),
                }],
                tags: vec!["test".to_string()],
            },
            validation_result: None,
        };

        // Test create
        let created = repo.create(tx.clone()).await.unwrap();
        assert_eq!(created.id, tx.id);

        // Test get by status
        let pending = repo.get_transactions_by_status(TransactionStatus::Pending).await.unwrap();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].id, tx.id);

        // Test get by protocol
        let bitcoin_txs = repo.get_transactions_by_protocol(ProtocolType::Bitcoin).await.unwrap();
        assert_eq!(bitcoin_txs.len(), 1);
        assert_eq!(bitcoin_txs[0].id, tx.id);

        // Test get confirmed
        let confirmed = repo.get_confirmed_transactions().await.unwrap();
        assert_eq!(confirmed.len(), 0); // Our test tx is pending

        // Test cleanup expired
        let expired = repo.cleanup_expired_transactions().await.unwrap();
        assert_eq!(expired, 0); // Our test tx is not old enough
    }
}
