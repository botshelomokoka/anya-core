//! Bitcoin module provides core Bitcoin functionality and Layer 2 integrations.

pub mod core;
pub mod lightning;
pub mod dlc;
pub mod privacy;
pub mod taproot;

pub use core::BitcoinCore;
pub use lightning::Lightning;
pub use dlc::DLCManager;
pub use privacy::PrivacyModule;
pub use taproot::TaprootModule;

// Re-export common Bitcoin types
pub use bitcoin::{
    Network,
    Transaction,
    Address,
    OutPoint,
    Script,
    ScriptBuf,
    util::psbt::PartiallySignedTransaction,
};
