use crate::bitcoin::config::{BitcoinConfig, BitcoinConfigError};
use bitcoin::{
    Network,
    Transaction,
    Block,
    BlockHash,
    Address,
    Amount,
    Txid,
};
use bitcoincore_rpc::{
    Auth,
    Client as RpcClient,
    RpcApi,
    Error as RpcError
};
use thiserror::Error;
use std::sync::Arc;
use tokio::sync::RwLock;
use log::{info, warn, error};

#[derive(Debug, Error)]
pub enum BitcoinCoreError {
    #[error("RPC error: {0}")]
    RpcError(#[from] RpcError),
    #[error("Config error: {0}")]
    ConfigError(#[from] BitcoinConfigError),
    #[error("Connection error: {0}")]
    ConnectionError(String),
    #[error("Invalid state: {0}")]
    InvalidState(String),
}

pub struct BitcoinCore {
    config: BitcoinConfig,
    client: Arc<RpcClient>,
    network_info: RwLock<Option<NetworkInfo>>,
}

#[derive(Debug, Clone)]
pub struct NetworkInfo {
    pub version: i32,
    pub subversion: String,
    pub connections: u32,
    pub blocks: u64,
    pub difficulty: f64,
    pub network: Network,
    pub warnings: String,
}

impl BitcoinCore {
    pub async fn new(config: BitcoinConfig) -> Result<Self, BitcoinCoreError> {
        let (url, user, pass) = config.get_connection_info()?;
        
        let auth = Auth::UserPass(user, pass);
        let client = RpcClient::new(&url, auth)
            .map_err(|e| BitcoinCoreError::ConnectionError(e.to_string()))?;

        let core = Self {
            config,
            client: Arc::new(client),
            network_info: RwLock::new(None),
        };

        // Initialize network info
        core.update_network_info().await?;

        Ok(core)
    }

    pub async fn update_network_info(&self) -> Result<(), BitcoinCoreError> {
        let network_info = self.client.get_network_info()?;
        let blockchain_info = self.client.get_blockchain_info()?;

        let info = NetworkInfo {
            version: network_info.version,
            subversion: network_info.subversion,
            connections: network_info.connections,
            blocks: blockchain_info.blocks,
            difficulty: blockchain_info.difficulty,
            network: self.config.network,
            warnings: network_info.warnings,
        };

        *self.network_info.write().await = Some(info);
        Ok(())
    }

    pub async fn get_network_info(&self) -> Result<NetworkInfo, BitcoinCoreError> {
        if let Some(info) = self.network_info.read().await.clone() {
            Ok(info)
        } else {
            self.update_network_info().await?;
            Ok(self.network_info.read().await.clone()
                .ok_or_else(|| BitcoinCoreError::InvalidState("Network info not initialized".to_string()))?)
        }
    }

    pub async fn get_balance(&self, min_conf: Option<u32>) -> Result<Amount, BitcoinCoreError> {
        Ok(self.client.get_balance(None, min_conf)?)
    }

    pub async fn send_to_address(
        &self,
        address: &Address,
        amount: Amount,
        comment: Option<&str>,
        subtract_fee: bool,
    ) -> Result<Txid, BitcoinCoreError> {
        Ok(self.client.send_to_address(
            address,
            amount,
            comment,
            None,
            Some(subtract_fee),
            None,
            None,
            None,
        )?)
    }

    pub async fn get_transaction(&self, txid: &Txid) -> Result<Transaction, BitcoinCoreError> {
        Ok(self.client.get_transaction(txid, None)?.transaction()?)
    }

    pub async fn get_block(&self, hash: &BlockHash) -> Result<Block, BitcoinCoreError> {
        Ok(self.client.get_block(hash)?)
    }

    pub async fn get_block_count(&self) -> Result<u64, BitcoinCoreError> {
        Ok(self.client.get_block_count()?)
    }

    pub async fn get_new_address(&self, label: Option<&str>) -> Result<Address, BitcoinCoreError> {
        Ok(self.client.get_new_address(label, None)?)
    }

    pub async fn validate_address(&self, address: &Address) -> Result<bool, BitcoinCoreError> {
        let validation = self.client.validate_address(address)?;
        Ok(validation.is_valid)
    }

    pub async fn estimate_smart_fee(&self, conf_target: u16) -> Result<Amount, BitcoinCoreError> {
        let estimate = self.client.estimate_smart_fee(conf_target, None)?;
        Ok(estimate.fee_rate)
    }

    pub async fn scan_tx_out_set(
        &self,
        descriptors: &[String],
    ) -> Result<(u32, Amount), BitcoinCoreError> {
        let result = self.client.scan_tx_out_set(descriptors)?;
        Ok((result.unspents.len() as u32, result.total_amount))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::bitcoin::config::CustomBitcoinSettings;

    #[tokio::test]
    async fn test_bitcoin_core() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempdir()?;
        let custom_settings = CustomBitcoinSettings {
            rpc_url: "http://localhost:18332".to_string(),
            rpc_user: "testuser".to_string(),
            rpc_password: "testpass".to_string(),
            data_dir: temp_dir.path().to_path_buf(),
        };

        let config = BitcoinConfig::new()
            .with_network(Network::Testnet)
            .with_custom_settings(custom_settings);

        let core = BitcoinCore::new(config).await?;
        let network_info = core.get_network_info().await?;

        assert_eq!(network_info.network, Network::Testnet);
        Ok(())
    }
}
