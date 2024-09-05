//! This module stores project-wide constants and configuration settings for Anya.

use bitcoin::{Network, util::address::AddressType};
use std::time::Duration;
use stacks_common::types::StacksEpochId;
use web5::{
    did::DidMethod,
    credentials::{VerifiableCredential, VerifiablePresentation},
    api::{Web5Api, DwnApi},
};
use dlc::{Oracle, Contract, OracleInfo, Announcement};
use lightning::{
    ln::channelmanager::ChannelManager,
    util::config::UserConfig,
};
use libp2p::{PeerId, Multiaddr, identity::Keypair};

// Network constants
pub const BITCOIN_NETWORK: Network = Network::Bitcoin;
pub const RSK_NETWORK: &str = "mainnet";
pub const STACKS_NETWORK: &str = "mainnet";

// Derivation paths (BIP44)
pub const BIP44_COIN_TYPE_BITCOIN: u32 = 0;
pub const BIP44_COIN_TYPE_STACKS: u32 = 5757;
pub const BIP44_ACCOUNT: u32 = 0;   // First account

// Address types
pub const DEFAULT_BITCOIN_ADDRESS_TYPE: AddressType = AddressType::P2wpkh;
pub const DEFAULT_STACKS_ADDRESS_TYPE: &str = "P2PKH";

// Fee estimation
pub const DEFAULT_BITCOIN_FEE_RATE: u32 = 1;  // sat/byte
pub const DEFAULT_STACKS_FEE_RATE: u64 = 1000;  // microSTX
pub const FEE_ESTIMATION_SOURCE: &str = "bitcoin_core";

// Transaction limits
pub const MAX_TRANSACTION_INPUTS: usize = 100;

// Timeouts
pub const RPC_TIMEOUT: Duration = Duration::from_secs(30);

// UTXO selection
pub const BITCOIN_DUST_THRESHOLD: u64 = 546;  // in satoshis
pub const STACKS_DUST_THRESHOLD: u64 = 1;  // in microSTX

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

// Stacks-specific constants
pub const STACKS_EPOCH_2_05: StacksEpochId = StacksEpochId::Epoch20;
pub const STACKS_EPOCH_2_1: StacksEpochId = StacksEpochId::Epoch21;
pub const STACKS_TRANSACTION_VERSION: u8 = 0x00;
pub const STACKS_CHAIN_ID: u32 = 0x00000001;

// Web5-specific constants
pub const DEFAULT_DID_METHOD: DidMethod = DidMethod::Key;
pub const WEB5_CREDENTIAL_CONTEXT: &str = "https://www.w3.org/2018/credentials/v1";
pub const WEB5_DWN_ENDPOINT: &str = "https://dwn.tbddev.org/";

// DLC-specific constants
pub const DLC_ORACLE_ANNOUNCEMENT_TIMELOCK: u32 = 144; // ~1 day in blocks
pub const DLC_LOCKTIME: u32 = 500000; // Default locktime for DLC contracts

// Lightning Network-specific constants
pub const LN_CHANNEL_RESERVE_SATOSHIS: u64 = 10000;
pub const LN_HTLC_MINIMUM_MSAT: u64 = 1000;
pub const LN_DEFAULT_CLTV_EXPIRY_DELTA: u16 = 144;
pub const LN_DEFAULT_MAX_TOTAL_HTLC_MSAT: u64 = 21_000_000_000_000; // 21 BTC in msat

// libp2p-specific constants
pub const LIBP2P_LISTEN_ADDRESS: &str = "/ip4/0.0.0.0/tcp/0";
pub const LIBP2P_PROTOCOL_VERSION: &str = "/anya/1.0.0";
pub const LIBP2P_MAX_CONNECTIONS: usize = 25;
pub const LIBP2P_CONNECTION_TIMEOUT: Duration = Duration::from_secs(10);

// Rust library version constants
pub const RUST_BITCOIN_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const RUST_LIGHTNING_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const RUST_DLC_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const RUST_LIBP2P_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const RUST_STACKS_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const RUST_WEB5_VERSION: &str = env!("CARGO_PKG_VERSION");

// Default configurations
pub const DEFAULT_LIGHTNING_CONFIG: UserConfig = UserConfig {
    channel_handshake_limits: Default::default(),
    channel_handshake_config: Default::default(),
    accept_inbound_channels: true,
    manually_accept_inbound_channels: false,
    accept_htlc_maximum_value_msat: LN_DEFAULT_MAX_TOTAL_HTLC_MSAT,
};

pub const DEFAULT_LIBP2P_CONFIG: libp2p::core::config::Config = libp2p::core::config::Config {
    transport: libp2p::core::config::TransportConfig::default(),
    protocol_config: libp2p::core::config::ProtocolConfig::default(),
    ping_config: libp2p::core::config::PingConfig::default(),
    identify_config: libp2p::core::config::IdentifyConfig::default(),
    kademlia_config: None,
    mdns_config: None,
    relay_config: None,
    gossipsub_config: None,
};
