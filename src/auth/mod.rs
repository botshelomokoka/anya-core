use async_trait::async_trait;
use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey};
use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey, DerivationPath};
use bitcoin::Network;
use bitcoin::util::bip32::ChildNumber;
use bitcoin::util::bip32::ExtendedPubKey;
use bitcoin::psbt::PartiallySignedTransaction;
use lightning::util::message_signing;
use bitcoin::taproot::{TapTweakHash, TaprootBuilder, LeafVersion};
use bitcoin::schnorr::{TapTweak, UntweakedPublicKey};

#[async_trait]
pub trait BlockchainAuth: Send + Sync {
    async fn verify(&self, credentials: &AuthCredentials) -> Result<bool, error::AuthError>;
    async fn sign_message(&self, message: &[u8]) -> Result<Vec<u8>, error::AuthError>;
    async fn sign_psbt(&self, psbt: PartiallySignedTransaction) -> Result<PartiallySignedTransaction, error::AuthError>;
    async fn sign_lightning_message(&self, message: &[u8]) -> Result<Vec<u8>, error::AuthError>;
}

pub struct AuthCredentials {
    pub api_key: String,
    pub endpoint: String,
    pub network: Option<Network>,
}

// Auth implementations
pub mod stacks;
pub mod lightning;
pub mod web5;
pub mod default;

pub mod error;
pub mod keys;
pub mod session;

const DERIVATION_PATH: &str = "m/84'/0'/0'/0/0"; // BIP84 for native SegWit
const NETWORK: Network = Network::Bitcoin;

pub struct AuthManager {
    secp: Secp256k1<bitcoin::secp256k1::All>,
    network: Network,
    derivation_path: DerivationPath,
}

impl AuthManager {
    pub fn new() -> Self {
        Self {
            secp: Secp256k1::new(),
            network: NETWORK,
            derivation_path: DERIVATION_PATH.parse().expect("Valid derivation path"),
        }
    }

    pub fn with_network(network: Network) -> Self {
        Self {
            secp: Secp256k1::new(),
            network,
            derivation_path: DERIVATION_PATH.parse().expect("Valid derivation path"),
        }
    }

    pub fn derive_keys(&self, seed: &[u8]) -> Result<(ExtendedPrivKey, ExtendedPubKey), error::AuthError> {
        let master = ExtendedPrivKey::new_master(self.network, seed)
            .map_err(|e| error::AuthError::KeyDerivation(e.to_string()))?;
            
        let derived_priv = master
            .derive_priv(&self.secp, &self.derivation_path)
            .map_err(|e| error::AuthError::KeyDerivation(e.to_string()))?;
            
        let derived_pub = ExtendedPubKey::from_priv(&self.secp, &derived_priv);
        
        Ok((derived_priv, derived_pub))
    }

    pub fn derive_taproot_keys(&self, seed: &[u8]) -> Result<(SecretKey, UntweakedPublicKey), error::AuthError> {
        let (priv_key, pub_key) = self.derive_keys(seed)?;
        let internal_key = UntweakedPublicKey::from_secret_key(&self.secp, &priv_key.to_priv_key());
        
        // Can add Tapscript trees here if needed
        let merkle_root = TaprootBuilder::new()
            .finalize(&self.secp, internal_key)
            .map_err(|e| error::AuthError::KeyDerivation(e.to_string()))?;
            
        Ok((priv_key.to_priv_key(), internal_key))
    }
}
