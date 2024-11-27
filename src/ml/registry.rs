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
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;

#[derive(Debug)]
pub struct MLRegistry {
    core_models: HashMap<String, Arc<MLCore>>,
    logic_modules: HashMap<String, Arc<MLLogic>>,
    dao_rules: HashMap<String, Arc<DAORules>>,
    federated_models: HashMap<String, Arc<FederatedLearningModel>>,
    active_processors: HashMap<String, Arc<RwLock<MLProcessor>>>,
}

impl MLRegistry {
    pub async fn register_changes(&self, changes: Vec<SystemChange>) -> Result<()> {
        for change in changes {
            match change.module_type {
                ModuleType::Core => self.update_core_model(change).await?,
                ModuleType::Logic => self.update_logic_module(change).await?,
                ModuleType::DAORules => self.update_dao_rules(change).await?,
                ModuleType::FederatedLearning => self.update_federated_model(change).await?,
            }
        }
        Ok(())
    }

    async fn update_core_model(&self, change: SystemChange) -> Result<()> {
        let model = self.core_models.get(&change.name).ok_or_else(|| {
            anyhow::anyhow!("Core model not found: {}", change.name)
        })?;
        
        model.apply_changes(change.content).await?;
        Ok(())
    }
}



