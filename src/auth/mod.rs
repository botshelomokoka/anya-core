use async_trait::async_trait;

#[async_trait]
pub trait BlockchainAuth: Send + Sync {
    async fn verify(&self, credentials: &AuthCredentials) -> Result<bool, AuthError>;
}

pub struct AuthCredentials {
    pub api_key: String,
    pub endpoint: String,
}

// Auth implementations
pub mod stacks;
pub mod lightning;
pub mod web5;
pub mod default;

