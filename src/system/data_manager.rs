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
use crate::storage::DWNStorage;
use crate::system::identity::DIDManager;
use crate::system::protocol_registry::ProtocolRegistry;
use anyhow::Result;

pub struct SystemDataManager {
    dwn_storage: Arc<DWNStorage>,
    protocol_registry: Arc<ProtocolRegistry>,
    identity_manager: Arc<DIDManager>,
}

impl SystemDataManager {
    pub async fn store_data(&self, data: &[u8], protocol: &str) -> Result<String> {
        // Get protocol definition
        let protocol_def = self.protocol_registry.get_protocol(protocol)?;
        
        // Create record
        let record = Record::new()
            .with_protocol(protocol_def.name())
            .with_schema(protocol_def.schema())
            .with_data(data)
            .with_owner(self.identity_manager.system_did())
            .build()?;
            
        // Store in DWN
        let record_id = self.dwn_storage.store_record(&record).await?;
        
        Ok(record_id)
    }

    pub async fn retrieve_data(&self, record_id: &str) -> Result<Vec<u8>> {
        // Retrieve record
        let record = self.dwn_storage.get_record(record_id).await?;
        
        // Verify permissions
        self.verify_access_permissions(&record).await?;
        
        Ok(record.data().to_vec())
    }
}



