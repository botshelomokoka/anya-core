//! This module provides a client interface for interacting with the RSK network.

use web3::Web3;
use web3::transports::Http;
use web3::types::{Address, H256, TransactionReceipt, U256, Transaction, BlockNumber};
use std::str::FromStr;
use std::time::Duration;
use tokio::time::sleep;

// Connect to an RSK node
const RSK_NODE_URL: &str = "https://public-node.rsk.co";  // Or your preferred RSK node URL
const MAX_RETRIES: u32 = 5;
const RETRY_DELAY: Duration = Duration::from_secs(2);

pub struct RskClient {
    web3: Web3<Http>,
}

impl RskClient {
    pub fn new() -> Result<Self, web3::Error> {
        let transport = Http::new(RSK_NODE_URL)?;
        let web3 = Web3::new(transport);
        Ok(RskClient { web3 })
    }

    /// Gets the RBTC balance of an address on the RSK network
    ///
    /// # Arguments
    ///
    /// * `address` - The address to check
    ///
    /// # Returns
    ///
    /// The balance in wei
    pub async fn get_balance(&self, address: &str) -> Result<U256, web3::Error> {
        let address = Address::from_str(address).map_err(|_| web3::Error::InvalidAddress)?;
        self.web3.eth().balance(address, None).await
    }

    /// Sends a signed transaction to the RSK network
    ///
    /// # Arguments
    ///
    /// * `transaction` - The signed transaction bytes
    ///
    /// # Returns
    ///
    /// The transaction hash if successful
    pub async fn send_transaction(&self, transaction: &[u8]) -> Result<H256, web3::Error> {
        self.web3.eth().send_raw_transaction(transaction.into()).await
    }

    /// Gets the details of a transaction on the RSK network
    ///
    /// # Arguments
    ///
    /// * `tx_hash` - The transaction hash
    ///
    /// # Returns
    ///
    /// The transaction receipt
    pub async fn get_transaction(&self, tx_hash: H256) -> Result<Option<TransactionReceipt>, web3::Error> {
        self.web3.eth().transaction_receipt(tx_hash).await
    }

    /// Gets the latest block number on the RSK network
    ///
    /// # Returns
    ///
    /// The latest block number
    pub async fn get_latest_block_number(&self) -> Result<U256, web3::Error> {
        self.web3.eth().block_number().await
    }

    /// Gets the transaction count for an address
    ///
    /// # Arguments
    ///
    /// * `address` - The address to check
    ///
    /// # Returns
    ///
    /// The transaction count (nonce)
    pub async fn get_transaction_count(&self, address: &str) -> Result<U256, web3::Error> {
        let address = Address::from_str(address).map_err(|_| web3::Error::InvalidAddress)?;
        self.web3.eth().transaction_count(address, None).await
    }

    /// Estimates gas for a transaction
    ///
    /// # Arguments
    ///
    /// * `transaction` - The transaction to estimate gas for
    ///
    /// # Returns
    ///
    /// The estimated gas
    pub async fn estimate_gas(&self, transaction: Transaction) -> Result<U256, web3::Error> {
        self.web3.eth().estimate_gas(transaction, None).await
    }

    /// Waits for a transaction to be mined
    ///
    /// # Arguments
    ///
    /// * `tx_hash` - The transaction hash to wait for
    ///
    /// # Returns
    ///
    /// The transaction receipt once mined
    pub async fn wait_for_transaction_receipt(&self, tx_hash: H256) -> Result<TransactionReceipt, web3::Error> {
        for _ in 0..MAX_RETRIES {
            if let Some(receipt) = self.get_transaction(tx_hash).await? {
                return Ok(receipt);
            }
            sleep(RETRY_DELAY).await;
        }
        Err(web3::Error::Unreachable)
    }

    /// Gets the current gas price on the RSK network
    ///
    /// # Returns
    ///
    /// The current gas price in wei
    pub async fn get_gas_price(&self) -> Result<U256, web3::Error> {
        self.web3.eth().gas_price().await
    }

    /// Gets a block by its number
    ///
    /// # Arguments
    ///
    /// * `block_number` - The block number to retrieve
    ///
    /// # Returns
    ///
    /// The block details
    pub async fn get_block(&self, block_number: u64) -> Result<Option<web3::types::Block<H256>>, web3::Error> {
        self.web3.eth().block(BlockNumber::Number(block_number.into())).await
    }

    // Additional methods can be added here as needed
}
