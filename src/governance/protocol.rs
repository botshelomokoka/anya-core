use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use crate::stacks_client::{StacksContractClient, Error as ClientError};
use crate::state_manager::{ProtocolStateManager, ProtocolState};
use crate::security::{SecurityManager, ProtocolAction, SecurityError};

#[derive(Debug)]
pub struct ProtocolManager {
    contract_client: Arc<StacksContractClient>,
    state_manager: Arc<ProtocolStateManager>,
    security_manager: Arc<SecurityManager>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolConfig {
    pub min_proposal_threshold: u64,
    pub min_quorum: u64,
    pub voting_period: u64,
    pub timelock_period: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractUpgrade {
    pub name: String,
    pub address: String,
    pub version: String,
    pub implementation: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ProtocolError {
    #[error("Client error: {0}")]
    Client(#[from] ClientError),
    
    #[error("Security error: {0}")]
    Security(#[from] SecurityError),
    
    #[error("Invalid state: {0}")]
    InvalidState(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
}

impl ProtocolManager {
    pub fn new(
        contract_client: StacksContractClient,
        state_manager: ProtocolStateManager,
        security_manager: SecurityManager,
    ) -> Self {
        Self {
            contract_client: Arc::new(contract_client),
            state_manager: Arc::new(state_manager),
            security_manager: Arc::new(security_manager),
        }
    }

    pub async fn update_config(
        &self,
        caller: &str,
        key: &str,
        value: &str,
    ) -> Result<(), ProtocolError> {
        // Verify action
        let action = ProtocolAction::UpdateConfig {
            key: key.to_string(),
            value: value.to_string(),
        };
        self.security_manager.verify_action(&action, caller).await?;

        // Update config on chain
        self.contract_client
            .call_contract(
                "protocol",
                "update-config",
                &[key, value],
            )
            .await?;

        // Invalidate state cache
        self.state_manager.invalidate_cache(crate::state_manager::StateKey::Protocol);

        Ok(())
    }

    pub async fn upgrade_contract(
        &self,
        caller: &str,
        upgrade: ContractUpgrade,
    ) -> Result<(), ProtocolError> {
        // Verify action
        let action = ProtocolAction::UpgradeContract {
            address: upgrade.address.clone(),
            name: upgrade.name.clone(),
            version: upgrade.version.clone(),
        };
        self.security_manager.verify_action(&action, caller).await?;

        // Perform upgrade on chain
        self.contract_client
            .call_contract(
                "protocol",
                "upgrade-contract",
                &[&upgrade.name, &upgrade.address, &upgrade.version, &upgrade.implementation],
            )
            .await?;

        // Invalidate caches
        self.state_manager.clear_cache();
        self.security_manager.clear_cache();

        Ok(())
    }

    pub async fn update_permissions(
        &self,
        caller: &str,
        address: &str,
        role: &str,
        permissions: Vec<String>,
    ) -> Result<(), ProtocolError> {
        // Verify action
        let action = ProtocolAction::UpdatePermissions {
            address: address.to_string(),
            role: role.to_string(),
            permissions: permissions.clone(),
        };
        self.security_manager.verify_action(&action, caller).await?;

        // Update permissions on chain
        self.contract_client
            .call_contract(
                "protocol",
                "update-permissions",
                &[address, role, &permissions.join(",")],
            )
            .await?;

        // Invalidate security cache for the address
        self.security_manager.invalidate_permissions(address);

        Ok(())
    }

    pub async fn transfer_funds(
        &self,
        caller: &str,
        recipient: &str,
        amount: u64,
    ) -> Result<(), ProtocolError> {
        // Verify action
        let action = ProtocolAction::TransferFunds {
            recipient: recipient.to_string(),
            amount,
        };
        self.security_manager.verify_action(&action, caller).await?;

        // Check treasury balance
        let state = self.state_manager.get_protocol_state().await?;
        if state.treasury_balance < amount {
            return Err(ProtocolError::InvalidState("Insufficient treasury balance".into()));
        }

        // Transfer funds on chain
        self.contract_client
            .call_contract(
                "protocol",
                "transfer-funds",
                &[recipient, &amount.to_string()],
            )
            .await?;

        // Invalidate state cache
        self.state_manager.invalidate_cache(crate::state_manager::StateKey::Protocol);

        Ok(())
    }

    pub async fn get_config(&self) -> Result<ProtocolConfig, ProtocolError> {
        let state = self.state_manager.get_protocol_state().await?;
        
        let mut config = ProtocolConfig {
            min_proposal_threshold: 100,
            min_quorum: 500,
            voting_period: 50400, // ~1 week in blocks
            timelock_period: 14400, // ~2 days in blocks
        };

        // Update with values from state
        for param in state.config_parameters {
            match param.key.as_str() {
                "min_proposal_threshold" => {
                    config.min_proposal_threshold = param.value.parse().map_err(|_| {
                        ProtocolError::Config("Invalid min_proposal_threshold".into())
                    })?;
                }
                "min_quorum" => {
                    config.min_quorum = param.value.parse().map_err(|_| {
                        ProtocolError::Config("Invalid min_quorum".into())
                    })?;
                }
                "voting_period" => {
                    config.voting_period = param.value.parse().map_err(|_| {
                        ProtocolError::Config("Invalid voting_period".into())
                    })?;
                }
                "timelock_period" => {
                    config.timelock_period = param.value.parse().map_err(|_| {
                        ProtocolError::Config("Invalid timelock_period".into())
                    })?;
                }
                _ => {}
            }
        }

        Ok(config)
    }

    pub async fn get_state(&self) -> Result<ProtocolState, ProtocolError> {
        Ok(self.state_manager.get_protocol_state().await?)
    }
}
