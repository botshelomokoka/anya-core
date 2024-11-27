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
use serde::{Serialize, Deserialize};
use dwn_sdk_rs::{DWN, Record, RecordQuery};
use did_key::{DIDKey, KeyMaterial};
use dwn_sdk_rs::encryption::{EncryptionManager, Permission};

#[derive(Debug, Serialize, Deserialize)]
pub struct MLDataRecord {
    protocol: String,
    schema: String,
    data: Vec<u8>,
    owner_did: String,
    permissions: Vec<Permission>,
}

pub struct Web5DataHandler {
    dwn: Arc<DWN>,
    encryption: Arc<EncryptionManager>,
}

impl Web5DataHandler {
    pub async fn store_training_data(&self, data: &[u8], owner_did: &str) -> Result<String> {
        let record = MLDataRecord {
            protocol: "ml.training.data".to_string(),
            schema: "training-data".to_string(),
            data: data.to_vec(),
            owner_did: owner_did.to_string(),
            permissions: vec![Permission::OwnerOnly],
        };

        // Encrypt data before storage
        let encrypted_data = self.encryption.encrypt(&record)?;
        
        // Store in DWN
        let record_id = self.dwn.create_record(&encrypted_data).await?;
        
        Ok(record_id)
    }

    pub async fn retrieve_training_data(&self, record_id: &str) -> Result<Vec<u8>> {
        let record = self.dwn.get_record(record_id).await?;
        
        // Verify permissions
        self.verify_access_permissions(&record).await?;
        
        // Decrypt data
        let decrypted_data = self.encryption.decrypt(&record.data)?;
        
        Ok(decrypted_data)
    }
}



