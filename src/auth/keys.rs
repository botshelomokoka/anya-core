use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey, Message};
use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey};
use bitcoin::util::key::PrivateKey;
use bitcoin::Network;
use rand::RngCore;
use crate::auth::error::AuthError;
use zeroize::{Zeroize, ZeroizeOnDrop};

pub struct KeyManager {
    secp: Secp256k1<bitcoin::secp256k1::All>,
    network: Network,
}

impl KeyManager {
    pub fn new(network: Network) -> Self {
        Self {
            secp: Secp256k1::new(),
            network,
        }
    }
    
    pub fn generate_key_pair(&self) -> Result<(SecretKey, PublicKey), AuthError> {
        let mut rng = rand::thread_rng();
        let mut seed = [0u8; 32];
        rng.fill_bytes(&mut seed);
        
        let secret_key = SecretKey::from_slice(&seed)
            .map_err(|e| AuthError::KeyDerivation(e.to_string()))?;
            
        let public_key = PublicKey::from_secret_key(&self.secp, &secret_key);
        
        Ok((secret_key, public_key))
    }

    pub fn sign_message(&self, message: &[u8], secret_key: &SecretKey) -> Result<Vec<u8>, AuthError> {
        let message = Message::from_slice(message)
            .map_err(|e| AuthError::Signing(e.to_string()))?;
            
        let signature = self.secp.sign_ecdsa(&message, secret_key);
        Ok(signature.serialize_der().to_vec())
    }

    pub fn verify_signature(
        &self,
        message: &[u8],
        signature: &[u8],
        public_key: &PublicKey,
    ) -> Result<bool, AuthError> {
        let message = Message::from_slice(message)
            .map_err(|e| AuthError::Signing(e.to_string()))?;
            
        let signature = bitcoin::secp256k1::ecdsa::Signature::from_der(signature)
            .map_err(|e| AuthError::Signing(e.to_string()))?;
            
        Ok(self.secp.verify_ecdsa(&message, &signature, public_key).is_ok())
    }

    pub fn secure_generate_key_pair(&self) -> Result<(SecureKeyStorage, PublicKey), AuthError> {
        let mut seed = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut seed);
        
        let secret_key = SecretKey::from_slice(&seed)?;
        let public_key = PublicKey::from_secret_key(&self.secp, &secret_key);
        
        let storage = SecureKeyStorage {
            secret_key,
            seed,
        };
        
        Ok((storage, public_key))
    }
}

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SecureKeyStorage {
    secret_key: SecretKey,
    seed: [u8; 32],
}
