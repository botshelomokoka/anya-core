//! Module documentation for $moduleName
//!
//! # Overview
//! This module is part of the Anya Core project, located at $modulePath.
//!
//! # Architecture
//! [Add module-specific architecture details]
//!
//! # API Reference
//! [Document public functions and types]
//!
//! # Usage Examples
//! `ust
//! // Add usage examples
//! `
//!
//! # Error Handling
//! This module uses proper error handling with Result types.
//!
//! # Security Considerations
//! [Document security features and considerations]
//!
//! # Performance
//! [Document performance characteristics]

use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use bitcoin_rpc::{BitcoinRpc, Auth};
use anyhow::{Result, Context};
use sysinfo::{System, SystemExt};
use std::env;
use std::time::{Duration, Instant};

pub struct BitcoinNode {
    rpc: Arc<BitcoinRpc>,
    system_info: Arc<Mutex<System>>,
    max_mempool_size: u64,
    max_tps: f32,
}

impl BitcoinNode {
    pub async fn new() -> Result<Self> {
        let rpc_url = env::var("BITCOIN_RPC_URL").context("BITCOIN_RPC_URL not set")?;
        let rpc_user = env::var("BITCOIN_RPC_USER").context("BITCOIN_RPC_USER not set")?;
        let rpc_pass = env::var("BITCOIN_RPC_PASS").context("BITCOIN_RPC_PASS not set")?;

        let auth = Auth::UserPass(rpc_user, rpc_pass);
        let rpc = Arc::new(BitcoinRpc::new(rpc_url, auth).context("Failed to create Bitcoin RPC client")?);

        let mut system = System::new_all();
        system.refresh_all();

        let total_memory = system.total_memory();
        let max_mempool_size = (total_memory / 3) as u64; // Use 1/3 of total memory as max mempool size

        let num_cores = system.processors().len();
        let max_tps = num_cores as f32 * 7.0; // Estimate max TPS based on number of cores

        Ok(Self {
            rpc,
            system_info: Arc::new(Mutex::new(system)),
            max_mempool_size,
            max_tps,
        })
    }

    pub async fn get_mempool_size(&self) -> Result<u64> {
        let mempool_info = self.rpc.get_mempool_info().await.context("Failed to get mempool info")?;
        Ok(mempool_info.bytes as u64)
    }

    pub async fn get_sync_status(&self) -> Result<(u64, u64)> {
        let blockchain_info = self.rpc.get_blockchain_info().await.context("Failed to get blockchain info")?;
        let current_height = blockchain_info.blocks;
        let network_height = blockchain_info.headers;
        Ok((current_height as u64, network_height as u64))
    }

    pub async fn get_recent_tps(&self) -> Result<f32> {
        let start_time = Instant::now();
        let start_tx_count = self.get_tx_count().await?;
        tokio::time::sleep(Duration::from_secs(60)).await;
        let end_tx_count = self.get_tx_count().await?;

        let tx_diff = end_tx_count - start_tx_count;
        let time_diff = start_time.elapsed().as_secs_f32();

        Ok(tx_diff as f32 / time_diff)
    }

    async fn get_tx_count(&self) -> Result<u64> {
        let blockchain_info = self.rpc.get_blockchain_info().await.context("Failed to get blockchain info")?;
        Ok(blockchain_info.tx_count)
    }

    pub async fn auto_adjust(&mut self) -> Result<()> {
        let mut system = self.system_info.lock().await;
        system.refresh_all();

        let total_memory = system.total_memory();
        self.max_mempool_size = (total_memory / 3) as u64;

        let num_cores = system.processors().len();
        self.max_tps = num_cores as f32 * 7.0;

        Ok(())
    }

    pub fn get_max_mempool_size(&self) -> u64 {
        self.max_mempool_size
    }

    pub fn get_max_tps(&self) -> f32 {
        self.max_tps
    }
}

