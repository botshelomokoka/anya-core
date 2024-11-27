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
// anya/src/watchtower.rs
use lightning::chain::Watch;
use lightning::chain::chainmonitor::ChainMonitor;
use lightning::chain::keysinterface::KeysInterface;
use lightning::chain::keysinterface::InMemorySigner;
use lightning::chain::transaction::OutPoint;
use lightning::chain::transaction::Transaction;
use lightning::util::logger::Logger;
use std::sync::Arc;

pub struct Watchtower {
	chain_monitor: ChainMonitor<InMemorySigner>,
}

impl Watchtower {
	pub fn new<Signer: KeysInterface<Signer = InMemorySigner>>(
		keys_interface: Arc<Signer>,
		logger: Arc<dyn Logger>,
	) -> Self  -> Result<(), Box<dyn Error>> {
		let chain_monitor = ChainMonitor::new(None, keys_interface, logger);
		Watchtower { chain_monitor }
	}

	pub fn watch_transaction(&self, tx: Transaction, outpoint: OutPoint)  -> Result<(), Box<dyn Error>> {
		self.chain_monitor.watch_transaction(tx, outpoint);
	}
}

