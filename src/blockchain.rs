use async_trait::async_trait;

#[async_trait]
pub trait BlockchainInterface {
    async fn submit_transaction(&self, transaction: Transaction) -> Result<TransactionResult, BlockchainError>;
    async fn update_config(&mut self, config: &HashMap<String, String>) -> Result<(), BlockchainError>;
}

pub struct Transaction {
    // Define transaction fields
}

pub struct TransactionResult {
    pub fee: f64,
    // Add other relevant fields
}

#[derive(Debug)]
pub enum BlockchainError {
    // Define blockchain-related errors
}