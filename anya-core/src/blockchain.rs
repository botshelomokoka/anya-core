use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error;

#[async_trait]
pub trait BlockchainInterface {
    async fn submit_transaction_async(&self, transaction: Transaction) -> Result<TransactionResult, BlockchainError>;
    async fn update_config(&mut self, config: &HashMap<String, String>) -> Result<(), BlockchainError>;
}

pub struct Transaction {
    // Define transaction fields
    pub id: String,
    pub amount: f64,
}

pub struct TransactionResult {
    pub fee: f64,
    // Add other relevant fields
    pub success: bool,
}

#[derive(Debug)]
pub enum BlockchainError {
    // Define blockchain-related errors
    InvalidTransaction,
    NetworkError,
    UnknownError,
}

impl std::fmt::Display for BlockchainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for BlockchainError {}