use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use governor::{Quota, RateLimiter};
use nonzero_ext::nonzero;
use crate::stacks_client::{StacksContractClient, Error};

#[derive(Debug, Clone)]
pub struct Permission {
    pub name: String,
    pub description: String,
    pub rate_limit: Option<u32>,
}

#[derive(Debug)]
pub struct SecurityManager {
    contract_client: Arc<StacksContractClient>,
    permission_cache: Arc<RwLock<HashMap<String, Vec<Permission>>>>,
    rate_limiters: Arc<RwLock<HashMap<String, Arc<RateLimiter>>>>,
}

impl SecurityManager {
    pub fn new(contract_client: StacksContractClient) -> Self {
        Self {
            contract_client: Arc::new(contract_client),
            permission_cache: Arc::new(RwLock::new(HashMap::new())),
            rate_limiters: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn verify_action(
        &self,
        action: &ProtocolAction,
        caller: &str,
    ) -> Result<(), SecurityError> {
        // Check permissions
        self.verify_permissions(caller, action.required_permissions()).await?;
        
        // Rate limiting
        self.check_rate_limit(caller, action.rate_limit_key()).await?;
        
        // Validate action parameters
        self.validate_action_params(action).await?;
        
        Ok(())
    }

    async fn verify_permissions(
        &self,
        caller: &str,
        required: &[String],
    ) -> Result<(), SecurityError> {
        // Try cache first
        let permissions = {
            let cache = self.permission_cache.read().await;
            if let Some(perms) = cache.get(caller) {
                perms.clone()
            } else {
                drop(cache);
                self.load_permissions(caller).await?
            }
        };

        // Check each required permission
        for required_perm in required {
            if !permissions.iter().any(|p| p.name == *required_perm) {
                return Err(SecurityError::InsufficientPermissions(required_perm.clone()));
            }
        }

        Ok(())
    }

    async fn load_permissions(&self, address: &str) -> Result<Vec<Permission>, SecurityError> {
        // Load from blockchain
        let permissions: Vec<Permission> = self.contract_client
            .call_read_only(
                "protocol",
                "get-permissions",
                &[address],
            )
            .await
            .map_err(|e| SecurityError::ContractError(e.to_string()))?;

        // Update cache
        self.permission_cache.write().await
            .insert(address.to_string(), permissions.clone());

        Ok(permissions)
    }

    async fn check_rate_limit(&self, caller: &str, action_key: &str) -> Result<(), SecurityError> {
        let rate_limiter = {
            let limiters = self.rate_limiters.read().await;
            if let Some(limiter) = limiters.get(action_key) {
                limiter.clone()
            } else {
                drop(limiters);
                self.create_rate_limiter(action_key).await?
            }
        };

        if !rate_limiter.check().is_ok() {
            return Err(SecurityError::RateLimitExceeded);
        }

        Ok(())
    }

    async fn create_rate_limiter(&self, action_key: &str) -> Result<Arc<RateLimiter>, SecurityError> {
        let quota = Quota::per_second(nonzero!(10u32)); // Default rate limit
        let limiter = Arc::new(RateLimiter::direct(quota));
        
        self.rate_limiters.write().await
            .insert(action_key.to_string(), limiter.clone());
        
        Ok(limiter)
    }

    async fn validate_action_params(&self, action: &ProtocolAction) -> Result<(), SecurityError> {
        match action {
            ProtocolAction::UpdateConfig { key, value } => {
                if key.is_empty() || value.is_empty() {
                    return Err(SecurityError::InvalidParameters("Config key and value cannot be empty".into()));
                }
            }
            ProtocolAction::UpgradeContract { address, name, version } => {
                if address.is_empty() || name.is_empty() || version.is_empty() {
                    return Err(SecurityError::InvalidParameters("Contract upgrade parameters cannot be empty".into()));
                }
            }
            // Add validation for other action types
        }
        Ok(())
    }

    pub async fn invalidate_permissions(&self, address: &str) {
        self.permission_cache.write().await.remove(address);
    }

    pub async fn clear_cache(&self) {
        self.permission_cache.write().await.clear();
        self.rate_limiters.write().await.clear();
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("Insufficient permissions: {0}")]
    InsufficientPermissions(String),
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),
    
    #[error("Contract error: {0}")]
    ContractError(String),
}

#[derive(Debug, Clone)]
pub enum ProtocolAction {
    UpdateConfig {
        key: String,
        value: String,
    },
    UpgradeContract {
        address: String,
        name: String,
        version: String,
    },
    UpdatePermissions {
        address: String,
        role: String,
        permissions: Vec<String>,
    },
    TransferFunds {
        recipient: String,
        amount: u64,
    },
}

impl ProtocolAction {
    pub fn required_permissions(&self) -> Vec<String> {
        match self {
            Self::UpdateConfig { .. } => vec!["admin".into()],
            Self::UpgradeContract { .. } => vec!["upgrade".into()],
            Self::UpdatePermissions { .. } => vec!["admin".into()],
            Self::TransferFunds { .. } => vec!["treasury".into()],
        }
    }

    pub fn rate_limit_key(&self) -> String {
        match self {
            Self::UpdateConfig { .. } => "config_update",
            Self::UpgradeContract { .. } => "contract_upgrade",
            Self::UpdatePermissions { .. } => "permission_update",
            Self::TransferFunds { .. } => "fund_transfer",
        }.to_string()
    }
}
