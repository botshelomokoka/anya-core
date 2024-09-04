//! This module contains helper functions for the Anya project

use bitcoin::consensus::encode::deserialize;
use bitcoin::Transaction;
use sha2::{Sha256, Digest};

/// Calculates the transaction ID (txid) from a raw transaction hex string
pub fn calculate_txid(tx_hex: &str) -> Result<String, Box<dyn std::error::Error>> {
    // 1. Deserialize the transaction hex
    let tx: Transaction = deserialize(&hex::decode(tx_hex)?)?;

    // 2. Calculate the double SHA-256 hash of the transaction
    let tx_bytes = bitcoin::consensus::encode::serialize(&tx);
    let tx_hash = Sha256::digest(&Sha256::digest(&tx_bytes));

    // 3. Reverse the bytes and convert to hex
    let txid = hex::encode(tx_hash.reverse());

    Ok(txid)
}

/// Converts a satoshi amount to Bitcoin
pub fn convert_satoshi_to_bitcoin(satoshi_amount: u64) -> f64 {
    satoshi_amount as f64 / 100_000_000.0 // 1 Bitcoin = 100,000,000 satoshis
}

/// Converts a Bitcoin amount to satoshis
pub fn convert_bitcoin_to_satoshi(bitcoin_amount: f64) -> u64 {
    (bitcoin_amount * 100_000_000.0) as u64
}

// ... (Other helper functions as needed)
