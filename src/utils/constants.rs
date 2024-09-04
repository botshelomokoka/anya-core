//! This module stores project-wide constants and configuration settings for Anya.

// Network constants
pub const BITCOIN_NETWORK: &str = "mainnet";  // or "testnet"
pub const RSK_NETWORK: &str = "mainnet";  // or "testnet"

// Derivation paths (BIP44)
pub const BIP44_COIN_TYPE: u32 = 0;  // Bitcoin
pub const BIP44_ACCOUNT: u32 = 0;   // First account (change this if needed)

// Address types
pub const DEFAULT_ADDRESS_TYPE: &str = "p2wpkh";  // Native SegWit (bech32)

// Fee estimation
pub const DEFAULT_FEE_RATE: u32 = 1;  // sat/byte (adjust as needed)
pub const FEE_ESTIMATION_SOURCE: &str = "bitcoin_core";  // or "external_api" (if using an external service)

// Other constants (add more as needed)
pub const MAX_TRANSACTION_INPUTS: usize = 100;  // Limit the number of inputs in a transaction
