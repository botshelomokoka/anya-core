//! Mobile integration module
use std::sync::Arc;
use tokio::sync::RwLock;
use thiserror::Error;

mod bridge;
mod spv;
mod wallet;
mod security;

pub use bridge::FFIBridge;
pub use spv::SPVClient;
pub use wallet::MobileWallet;
pub use security::SecurityManager;

#[derive(Error, Debug)]
pub enum MobileError {
    #[error("Wallet error: {0}")]
    WalletError(String),
    #[error("SPV error: {0}")]
    SPVError(String),
    #[error("Security error: {0}")]
    SecurityError(String),
    #[error("Bridge error: {0}")]
    BridgeError(String),
}

pub struct MobileConfig {
    pub network: bitcoin::Network,
    pub spv_enabled: bool,
    pub secure_storage: bool,
    pub qr_enabled: bool,
}

pub struct MobileManager {
    config: MobileConfig,
    wallet: Arc<RwLock<MobileWallet>>,
    spv: Arc<RwLock<SPVClient>>,
    security: Arc<RwLock<SecurityManager>>,
    bridge: FFIBridge,
}

impl MobileManager {
    pub async fn new(config: MobileConfig) -> Result<Self, MobileError> {
        let wallet = Arc::new(RwLock::new(MobileWallet::new(&config)?));
        let spv = Arc::new(RwLock::new(SPVClient::new(&config)?));
        let security = Arc::new(RwLock::new(SecurityManager::new(&config)?));
        let bridge = FFIBridge::new(wallet.clone(), spv.clone(), security.clone())?;

        Ok(Self {
            config,
            wallet,
            spv,
            security,
            bridge,
        })
    }

    pub async fn initialize(&self) -> Result<(), MobileError> {
        // Initialize SPV client if enabled
        if self.config.spv_enabled {
            self.spv.write().await.start().await?;
        }

        // Initialize secure storage
        if self.config.secure_storage {
            self.security.write().await.initialize().await?;
        }

        Ok(())
    }

    pub async fn create_wallet(&self, seed: &[u8]) -> Result<String, MobileError> {
        let mut wallet = self.wallet.write().await;
        wallet.create_from_seed(seed).await
    }

    pub async fn sign_transaction(&self, tx_data: &[u8]) -> Result<Vec<u8>, MobileError> {
        let wallet = self.wallet.read().await;
        wallet.sign_transaction(tx_data).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mobile_manager() {
        let config = MobileConfig {
            network: bitcoin::Network::Testnet,
            spv_enabled: true,
            secure_storage: true,
            qr_enabled: true,
        };

        let manager = MobileManager::new(config).await.unwrap();
        assert!(manager.config.spv_enabled);
        assert!(manager.config.secure_storage);
    }
}
