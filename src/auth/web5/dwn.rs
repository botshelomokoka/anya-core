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
use super::data_manager::{Web5DataManager, DataRecord};
use web5::dwn::{DwnClient, SyncOptions, RecordFilter};
use std::time::Duration;

pub struct DwnSyncManager {
    client: DwnClient,
    sync_interval: Duration,
}

impl DwnSyncManager {
    pub fn new(sync_interval: Duration) -> Self {
        Self {
            client: DwnClient::new(),
            sync_interval,
        }
    }

    pub async fn sync_records(&self, data_manager: &Web5DataManager) -> Result<(), Web5Error> {
        // Get local records
        let local_records = data_manager.get_all_records().await?;
        
        // Get remote records
        let remote_records = self.client.get_records(RecordFilter::default()).await?;
        
        // Merge records
        for record in remote_records {
            if !local_records.contains(&record.id) {
                data_manager.store_data(record).await?;
            }
        }
        
        Ok(())
    }

    pub async fn start_sync_loop(self, data_manager: Web5DataManager) {
        tokio::spawn(async move {
            loop {
                if let Err(e) = self.sync_records(&data_manager).await {
                    log::error!("DWN sync error: {}", e);
                }
                tokio::time::sleep(self.sync_interval).await;
            }
        });
    }
}


