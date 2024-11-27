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
use bitcoin::FeeRate;

pub struct FeeEstimator {
    rpc_client: BitcoinRpcClient,
}

impl FeeEstimator {
    pub async fn estimate_fee(&self, target_blocks: u16) -> Result<FeeRate, BitcoinError> {
        let fee_rate = self.rpc_client
            .estimate_smart_fee(target_blocks)
            .await?;
            
        Ok(FeeRate::from_sat_per_vb(fee_rate.fee_rate))
    }
}


