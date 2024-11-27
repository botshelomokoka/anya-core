use std::sync::Arc;
use tokio::sync::RwLock;
use web5::{
    did::{Did, DidMethod},
    protocol::{Protocol, ProtocolDefinition},
    storage::{Storage, StorageOptions},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StateError {
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("Protocol error: {0}")]
    ProtocolError(String),
    #[error("DID error: {0}")]
    DidError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolState {
    pub config_parameters: Vec<ConfigParameter>,
    pub contract_info: ContractInfo,
    pub permission_info: PermissionInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigParameter {
    pub key: String,
    pub value: String,
    pub last_updated: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractInfo {
    pub address: String,
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionInfo {
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

pub struct ProtocolStateManager {
    storage: Arc<Storage>,
    protocol: Arc<Protocol>,
    did: Arc<Did>,
    state_cache: Arc<RwLock<Option<ProtocolState>>>,
}

impl ProtocolStateManager {
    pub async fn new() -> Result<Self, StateError> {
        let storage = Storage::new(StorageOptions::default())
            .map_err(|e| StateError::StorageError(e.to_string()))?;
            
        let protocol_def = ProtocolDefinition::from_file(".web5/protocols/anya.json")
            .map_err(|e| StateError::ProtocolError(e.to_string()))?;
            
        let protocol = Protocol::new(protocol_def)
            .map_err(|e| StateError::ProtocolError(e.to_string()))?;
            
        let did = Did::create(DidMethod::Key, None)
            .map_err(|e| StateError::DidError(e.to_string()))?;

        Ok(Self {
            storage: Arc::new(storage),
            protocol: Arc::new(protocol),
            did: Arc::new(did),
            state_cache: Arc::new(RwLock::new(None)),
        })
    }

    pub async fn get_protocol_state(&self) -> Result<ProtocolState, StateError> {
        if let Some(state) = self.state_cache.read().await.as_ref() {
            return Ok(state.clone());
        }

        let state = self.storage
            .query_records(self.protocol.as_ref(), "configuration", None)
            .await
            .map_err(|e| StateError::StorageError(e.to_string()))?
            .first()
            .and_then(|record| serde_json::from_value(record.data.clone()).ok())
            .unwrap_or_default();

        *self.state_cache.write().await = Some(state.clone());
        Ok(state)
    }

    pub async fn update_protocol_state(&self, state: ProtocolState) -> Result<(), StateError> {
        self.storage
            .create_record(
                self.protocol.as_ref(),
                "configuration",
                serde_json::to_value(state.clone()).unwrap(),
                None,
            )
            .await
            .map_err(|e| StateError::StorageError(e.to_string()))?;

        *self.state_cache.write().await = Some(state);
        Ok(())
    }

    pub async fn clear_cache(&self) {
        *self.state_cache.write().await = None;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalInfo {
    pub id: u64,
    pub proposer: String,
    pub title: String,
    pub description: String,
    pub start_block: u64,
    pub end_block: u64,
    pub execution_block: u64,
    pub votes_for: u64,
    pub votes_against: u64,
    pub executed: bool,
    pub canceled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteInfo {
    pub power: u64,
    pub support: bool,
    pub reason: Option<String>,
    pub timestamp: u64,
}

pub async fn get_proposal(&self, proposal_id: u64) -> Result<ProposalInfo, StateError> {
    let key = format!("proposal-{}", proposal_id);

    let proposal = self.storage
        .query_records(self.protocol.as_ref(), &key, None)
        .await
        .map_err(|e| StateError::StorageError(e.to_string()))?
        .first()
        .and_then(|record| serde_json::from_value(record.data.clone()).ok())
        .unwrap_or_default();

    Ok(proposal)
}

pub async fn get_vote(&self, proposal_id: u64, voter: &str) -> Result<VoteInfo, StateError> {
    let key = format!("vote-{}-{}", proposal_id, voter);

    let vote = self.storage
        .query_records(self.protocol.as_ref(), &key, None)
        .await
        .map_err(|e| StateError::StorageError(e.to_string()))?
        .first()
        .and_then(|record| serde_json::from_value(record.data.clone()).ok())
        .unwrap_or_default();

    Ok(vote)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_protocol_state_manager() {
        let manager = ProtocolStateManager::new().await.unwrap();
        
        let state = ProtocolState {
            config_parameters: vec![ConfigParameter {
                key: "test_key".to_string(),
                value: "test_value".to_string(),
                last_updated: 123,
            }],
            contract_info: ContractInfo {
                address: "test_address".to_string(),
                name: "test_contract".to_string(),
                version: "1.0.0".to_string(),
            },
            permission_info: PermissionInfo {
                roles: vec!["admin".to_string()],
                permissions: vec!["write".to_string()],
            },
        };

        manager.update_protocol_state(state.clone()).await.unwrap();
        let retrieved_state = manager.get_protocol_state().await.unwrap();
        assert_eq!(
            retrieved_state.config_parameters[0].key,
            state.config_parameters[0].key
        );
    }
}
