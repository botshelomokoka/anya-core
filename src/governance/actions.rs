use serde::{Deserialize, Serialize};
use web5::{
    did::Did,
    protocol::{Protocol, Record},
    storage::Storage,
};
use thiserror::Error;
use std::sync::Arc;

#[derive(Debug, Error)]
pub enum ActionError {
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("Protocol error: {0}")]
    ProtocolError(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("Invalid action: {0}")]
    InvalidAction(String),
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProtocolAction {
    // Configuration Actions
    UpdateConfig {
        key: String,
        value: String,
    },
    AddContract {
        name: String,
        address: String,
        version: String,
    },
    RemoveContract {
        name: String,
    },

    // Permission Actions
    GrantRole {
        role: String,
        address: String,
    },
    RevokeRole {
        role: String,
        address: String,
    },
    AddPermission {
        role: String,
        permission: String,
    },
    RemovePermission {
        role: String,
        permission: String,
    },

    // Proposal Actions
    CreateProposal {
        title: String,
        description: String,
        actions: Vec<ProtocolAction>,
    },
    CastVote {
        proposal_id: u64,
        support: bool,
        reason: Option<String>,
    },
    ExecuteProposal {
        proposal_id: u64,
    },
    CancelProposal {
        proposal_id: u64,
    },

    // Treasury Actions
    TransferFunds {
        recipient: String,
        amount: u64,
    },
}

pub struct ActionManager {
    storage: Arc<Storage>,
    protocol: Arc<Protocol>,
    did: Arc<Did>,
}

impl ActionManager {
    pub async fn new(storage: Storage, protocol: Protocol, did: Did) -> Self {
        Self {
            storage: Arc::new(storage),
            protocol: Arc::new(protocol),
            did: Arc::new(did),
        }
    }

    pub async fn execute_action(&self, action: ProtocolAction) -> Result<Record, ActionError> {
        // Check rate limits
        self.check_rate_limits(&action).await?;

        // Check permissions
        self.check_permissions(&action).await?;

        // Create action record
        let record = self.storage
            .create_record(
                self.protocol.as_ref(),
                "action",
                serde_json::to_value(action.clone()).unwrap(),
                None,
            )
            .await
            .map_err(|e| ActionError::StorageError(e.to_string()))?;

        // Process action based on type
        match action {
            ProtocolAction::UpdateConfig { key, value } => {
                self.process_config_update(key, value).await?;
            }
            ProtocolAction::AddContract { name, address, version } => {
                self.process_add_contract(name, address, version).await?;
            }
            ProtocolAction::CreateProposal { title, description, actions } => {
                self.process_create_proposal(title, description, actions).await?;
            }
            // Add other action processing...
            _ => {}
        }

        Ok(record)
    }

    async fn check_rate_limits(&self, action: &ProtocolAction) -> Result<(), ActionError> {
        let action_type = serde_json::to_value(action).unwrap()["type"].as_str().unwrap();
        let recent_actions = self.storage
            .query_records(
                self.protocol.as_ref(),
                "action",
                Some(&format!("type = '{}'", action_type)),
            )
            .await
            .map_err(|e| ActionError::StorageError(e.to_string()))?;

        // Simple rate limiting: max 10 actions per minute
        let one_minute_ago = chrono::Utc::now().timestamp() - 60;
        let recent_count = recent_actions
            .iter()
            .filter(|r| r.created_at > one_minute_ago)
            .count();

        if recent_count >= 10 {
            return Err(ActionError::RateLimitExceeded);
        }

        Ok(())
    }

    async fn check_permissions(&self, action: &ProtocolAction) -> Result<(), ActionError> {
        let required_permission = match action {
            ProtocolAction::UpdateConfig { .. } => "CONFIG_UPDATE",
            ProtocolAction::AddContract { .. } => "CONTRACT_MANAGEMENT",
            ProtocolAction::GrantRole { .. } => "ROLE_MANAGEMENT",
            ProtocolAction::CreateProposal { .. } => "PROPOSAL_CREATE",
            // Add other permission mappings...
            _ => "BASE_PERMISSION",
        };

        let user_permissions = self.storage
            .query_records(
                self.protocol.as_ref(),
                "permission",
                Some(&format!("did = '{}'", self.did.to_string())),
            )
            .await
            .map_err(|e| ActionError::StorageError(e.to_string()))?;

        if !user_permissions.iter().any(|p| {
            p.data["permissions"].as_array()
                .unwrap()
                .contains(&serde_json::Value::String(required_permission.to_string()))
        }) {
            return Err(ActionError::PermissionDenied(format!(
                "Missing required permission: {}",
                required_permission
            )));
        }

        Ok(())
    }

    async fn process_config_update(&self, key: String, value: String) -> Result<(), ActionError> {
        let config_record = self.storage
            .create_record(
                self.protocol.as_ref(),
                "config",
                serde_json::json!({
                    "key": key,
                    "value": value,
                    "updated_at": chrono::Utc::now().timestamp(),
                }),
                None,
            )
            .await
            .map_err(|e| ActionError::StorageError(e.to_string()))?;

        // Emit event for config update
        self.storage
            .create_record(
                self.protocol.as_ref(),
                "event",
                serde_json::json!({
                    "type": "CONFIG_UPDATED",
                    "data": {
                        "key": key,
                        "value": value,
                        "record_id": config_record.id,
                    },
                }),
                None,
            )
            .await
            .map_err(|e| ActionError::StorageError(e.to_string()))?;

        Ok(())
    }

    async fn process_add_contract(
        &self,
        name: String,
        address: String,
        version: String,
    ) -> Result<(), ActionError> {
        let contract_record = self.storage
            .create_record(
                self.protocol.as_ref(),
                "contract",
                serde_json::json!({
                    "name": name,
                    "address": address,
                    "version": version,
                    "status": "ACTIVE",
                }),
                None,
            )
            .await
            .map_err(|e| ActionError::StorageError(e.to_string()))?;

        // Emit event for contract addition
        self.storage
            .create_record(
                self.protocol.as_ref(),
                "event",
                serde_json::json!({
                    "type": "CONTRACT_ADDED",
                    "data": {
                        "name": name,
                        "address": address,
                        "version": version,
                        "record_id": contract_record.id,
                    },
                }),
                None,
            )
            .await
            .map_err(|e| ActionError::StorageError(e.to_string()))?;

        Ok(())
    }

    async fn process_create_proposal(
        &self,
        title: String,
        description: String,
        actions: Vec<ProtocolAction>,
    ) -> Result<(), ActionError> {
        let proposal_record = self.storage
            .create_record(
                self.protocol.as_ref(),
                "proposal",
                serde_json::json!({
                    "title": title,
                    "description": description,
                    "actions": actions,
                    "status": "ACTIVE",
                    "votes_for": 0,
                    "votes_against": 0,
                    "created_at": chrono::Utc::now().timestamp(),
                }),
                None,
            )
            .await
            .map_err(|e| ActionError::StorageError(e.to_string()))?;

        // Emit event for proposal creation
        self.storage
            .create_record(
                self.protocol.as_ref(),
                "event",
                serde_json::json!({
                    "type": "PROPOSAL_CREATED",
                    "data": {
                        "title": title,
                        "description": description,
                        "record_id": proposal_record.id,
                    },
                }),
                None,
            )
            .await
            .map_err(|e| ActionError::StorageError(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_action_execution() {
        // Initialize test environment
        let storage = Storage::new(Default::default()).unwrap();
        let protocol = Protocol::new(Default::default()).unwrap();
        let did = Did::create(Default::default(), None).unwrap();
        
        let manager = ActionManager::new(storage, protocol, did).await;

        // Test config update action
        let action = ProtocolAction::UpdateConfig {
            key: "test_key".to_string(),
            value: "test_value".to_string(),
        };

        let result = manager.execute_action(action).await;
        assert!(result.is_ok());

        // Test proposal creation
        let action = ProtocolAction::CreateProposal {
            title: "Test Proposal".to_string(),
            description: "Test Description".to_string(),
            actions: vec![],
        };

        let result = manager.execute_action(action).await;
        assert!(result.is_ok());
    }
}
