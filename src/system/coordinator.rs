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
use crate::ml::web5::Web5MLIntegration;
use crate::system::identity::DIDManager;
use crate::storage::DWNStorage;
use anyhow::Result;

pub struct SystemCoordinator {
    web5_integration: Arc<Web5MLIntegration>,
    did_manager: Arc<DIDManager>,
    dwn_storage: Arc<DWNStorage>,
    ml_manager: Arc<Mutex<MLManager>>,
    protocol_registry: Arc<ProtocolRegistry>,
}

impl SystemCoordinator {
    pub async fn initialize_system(&self) -> Result<()> {
        // Initialize DID system
        self.did_manager.initialize().await?;
        
        // Initialize DWN storage
        self.dwn_storage.initialize().await?;
        
        // Register system protocols
        self.register_system_protocols().await?;
        
        // Initialize ML with Web5 integration
        self.initialize_ml_system().await?;
        
        Ok(())
    }

    async fn register_system_protocols(&self) -> Result<()> {
        let protocols = vec![
            ProtocolDefinition::new("ml.training")
                .with_schema(include_str!("../schemas/ml_training.json"))
                .with_permissions(vec!["read", "write"])
                .build()?,
                
            ProtocolDefinition::new("ml.model")
                .with_schema(include_str!("../schemas/ml_model.json"))
                .with_permissions(vec!["read", "execute"])
                .build()?,
                
            ProtocolDefinition::new("system.identity")
                .with_schema(include_str!("../schemas/identity.json"))
                .with_permissions(vec!["read"])
                .build()?,
        ];

        self.protocol_registry.register_batch(protocols).await?;
        Ok(())
    }
}


