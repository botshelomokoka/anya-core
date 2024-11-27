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
use crate::ml::{MLCore, MLLogic, DAORules, FederatedLearningModel};
use crate::system::directory_manager::SystemChange;
use anyhow::Result;

pub struct MLChangeProcessor {
    registry: Arc<MLRegistry>,
    ml_core: Arc<MLCore>,
    ml_logic: Arc<MLLogic>,
    dao_rules: Arc<DAORules>,
}

impl MLChangeProcessor {
    pub async fn process_ml_changes(&self, changes: Vec<SystemChange>) -> Result<()> {
        // Group changes by ML component
        let grouped_changes = self.group_changes(changes);
        
        // Process core ML changes
        if let Some(core_changes) = grouped_changes.get("ml_core") {
            self.process_core_changes(core_changes).await?;
        }

        // Process ML logic changes
        if let Some(logic_changes) = grouped_changes.get("ml_logic") {
            self.process_logic_changes(logic_changes).await?;
        }

        // Process DAO rules changes
        if let Some(dao_changes) = grouped_changes.get("dao_rules") {
            self.process_dao_changes(dao_changes).await?;
        }

        Ok(())
    }

    async fn process_core_changes(&self, changes: &[SystemChange]) -> Result<()> {
        for change in changes {
            match change.file_type {
                "model_trainer" => self.ml_core.update_trainer(change).await?,
                "data_processor" => self.ml_core.update_processor(change).await?,
                "predictor" => self.ml_core.update_predictor(change).await?,
                _ => continue,
            }
        }
        Ok(())
    }
}



