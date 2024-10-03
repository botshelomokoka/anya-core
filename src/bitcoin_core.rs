use bitcoin::Network;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BitcoinCoreError {
    #[error("RPC error: {0}")]
    RpcError(#[from] bitcoincore_rpc::Error),
    #[error("Bitcoin Core initialization failed: {0}")]
    InitializationError(String),
}

pub struct BitcoinCore {
    client: Client,
}

impl BitcoinCore {
    pub fn new(url: &str, auth: Auth, network: Network) -> Result<Self, BitcoinCoreError> {
        let client = Client::new(url, auth)
            .map_err(|e| BitcoinCoreError::InitializationError(e.to_string()))?;
        Ok(Self { client })
    }

    pub fn get_block_count(&self) -> Result<u64, BitcoinCoreError> {
        self.client.get_block_count().map_err(BitcoinCoreError::from)
    }

    // Add more Bitcoin Core related methods as needed
}