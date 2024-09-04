//! This module provides a client interface for interacting with the Bitcoin network via RPC.

use bitcoincore_rpc::{Auth, Client, RpcApi};
use std::collections::HashMap;
use std::error::Error;

// Connect to Bitcoin Core RPC (replace with your actual connection details)
const RPC_USER: &str = "your_rpc_user";
const RPC_PASSWORD: &str = "your_rpc_password";
const RPC_HOST: &str = "localhost"; // Or your remote host
const RPC_PORT: u16 = 8332;

fn create_rpc_client() -> Result<Client, Box<dyn Error>> {
    let rpc_url = format!("http://{}:{}", RPC_HOST, RPC_PORT);
    let auth = Auth::UserPass(RPC_USER.to_string(), RPC_PASSWORD.to_string());
    Ok(Client::new(&rpc_url, auth)?)
}

/// Fetches unspent transaction outputs (UTXOs) for a given address.
pub fn get_utxos(address: &str) -> Result<Vec<HashMap<String, serde_json::Value>>, Box<dyn Error>> {
    let client = create_rpc_client()?;
    let utxos = client.list_unspent(None, None, Some(&[address]), None, None)?;
    
    Ok(utxos
        .into_iter()
        .map(|utxo| {
            let mut map = HashMap::new();
            map.insert("txid".to_string(), serde_json::Value::String(utxo.txid.to_string()));
            map.insert("vout".to_string(), serde_json::Value::Number(utxo.vout.into()));
            map.insert("value".to_string(), serde_json::Value::Number(utxo.amount.to_sat().into()));
            map
        })
        .collect())
}

/// Fetches the raw transaction data for a given transaction ID.
pub fn get_raw_transaction(txid: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let client = create_rpc_client()?;
    let tx = client.get_raw_transaction_verbose(&bitcoin::Txid::from_str(txid)?)?;
    Ok(serde_json::to_value(tx)?)
}

/// Broadcasts a raw transaction to the Bitcoin network
pub fn send_raw_transaction(tx_hex: &str) -> Result<String, Box<dyn Error>> {
    let client = create_rpc_client()?;
    let txid = client.send_raw_transaction(hex::decode(tx_hex)?)?;
    Ok(txid.to_string())
}

/// Estimates the fee rate for a transaction to be confirmed within the specified number of blocks.
pub fn estimate_fee(target_conf: u16) -> Result<f64, Box<dyn Error>> {
    let client = create_rpc_client()?;
    let fee_rate = client.estimate_smart_fee(target_conf, None)?;
    Ok(fee_rate.fee_rate.unwrap_or(0.0))
}

/// Fetches a block by its hash.
pub fn get_block(block_hash: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let client = create_rpc_client()?;
    let block_hash = bitcoin::BlockHash::from_str(block_hash)?;
    let block = client.get_block(&block_hash)?;
    Ok(serde_json::to_value(block)?)
}

/// Gets the current block count of the Bitcoin network.
pub fn get_block_count() -> Result<u64, Box<dyn Error>> {
    let client = create_rpc_client()?;
    Ok(client.get_block_count()?)
}

/// Fetches the balance of a given address.
pub fn get_address_balance(address: &str) -> Result<f64, Box<dyn Error>> {
    let client = create_rpc_client()?;
    let balance = client.get_received_by_address(
        &bitcoin::Address::from_str(address)?,
        Some(0)
    )?;
    Ok(balance.to_btc())
}

/// Validates a Bitcoin address.
pub fn validate_address(address: &str) -> Result<bool, Box<dyn Error>> {
    let client = create_rpc_client()?;
    let validation = client.validate_address(&bitcoin::Address::from_str(address)?)?;
    Ok(validation.is_valid)
}

pub fn select_rpc_config() -> RpcConfig {
    // Use Blockstream endpoint by default
    RpcConfig::new("https://bitcoin-mainnet.public.blastapi.io", None)
}

// ... existing code ...
