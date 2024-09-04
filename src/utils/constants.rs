//! This module stores project-wide constants and configuration settings for Anya.

use bitcoin::Network;
use bitcoin::util::address::AddressType;
use std::time::Duration;

// Network constants
pub const BITCOIN_NETWORK: Network = Network::Bitcoin;
pub const RSK_NETWORK: &str = "mainnet";

// Derivation paths (BIP44)
pub const BIP44_COIN_TYPE: u32 = 0;  // Bitcoin
pub const BIP44_ACCOUNT: u32 = 0;   // First account

// Address types
pub const DEFAULT_ADDRESS_TYPE: AddressType = AddressType::P2wpkh;

// Fee estimation
pub const DEFAULT_FEE_RATE: u32 = 1;  // sat/byte
pub const FEE_ESTIMATION_SOURCE: &str = "bitcoin_core";

// Transaction limits
pub const MAX_TRANSACTION_INPUTS: usize = 100;

// Timeouts
pub const RPC_TIMEOUT: Duration = Duration::from_secs(30);

// UTXO selection
pub const DUST_THRESHOLD: u64 = 546;  // in satoshis

// Mnemonic settings
pub const DEFAULT_MNEMONIC_LANGUAGE: &str = "english";
pub const DEFAULT_MNEMONIC_WORD_COUNT: usize = 24;

// Blockchain sync
pub const MAX_HEADERS_TO_SYNC: u32 = 2000;
pub const SYNC_INTERVAL: Duration = Duration::from_secs(600);  // 10 minutes

// Wallet backup
pub const BACKUP_INTERVAL: Duration = Duration::from_secs(86400);  // 24 hours
pub const MAX_BACKUP_FILES: usize = 5;

// Logging
pub const LOG_ROTATION_DAYS: u64 = 7;
pub const MAX_LOG_SIZE: u64 = 10 * 1024 * 1024;  // 10 MB
