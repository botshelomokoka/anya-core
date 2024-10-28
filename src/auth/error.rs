use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Bitcoin key derivation error: {0}")]
    KeyDerivation(String),
    
    #[error("Signing error: {0}")]
    Signing(String),
    
    #[error("Invalid key format: {0}")]
    InvalidKey(String),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    
    #[error("Invalid credentials: {0}")]
    InvalidCredentials(String),
}
