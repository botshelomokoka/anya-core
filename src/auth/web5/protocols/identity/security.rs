use super::{IdentityError, Credential};
use crate::auth::encryption::KeyEncryption;
use ring::aead::{self, SealingKey, OpeningKey, Nonce, NONCE_LEN};
use ring::rand::SystemRandom;
use std::sync::Arc;

pub struct SecurityManager {
    key_encryption: Arc<KeyEncryption>,
    rng: SystemRandom,
}

impl SecurityManager {
    pub fn new(key_encryption: Arc<KeyEncryption>) -> Self {
        Self {
            key_encryption,
            rng: SystemRandom::new(),
        }
    }

    pub fn encrypt_credential(&self, credential: &Credential) -> Result<Vec<u8>, IdentityError> {
        let credential_bytes = serde_json::to_vec(credential)?;
        
        let mut in_out = credential_bytes.clone();
        let mut nonce_bytes = [0u8; NONCE_LEN];
        self.rng.fill(&mut nonce_bytes)
            .map_err(|e| IdentityError::EncryptionError(e.to_string()))?;
        let nonce = Nonce::assume_unique_for_key(nonce_bytes);
        
        let sealing_key = self.key_encryption.get_sealing_key()?;
        sealing_key.seal_in_place_append_tag(nonce, aead::Aad::empty(), &mut in_out)
            .map_err(|e| IdentityError::EncryptionError(e.to_string()))?;
            
        Ok(in_out)
    }

    pub fn decrypt_credential(&self, encrypted: &[u8]) -> Result<Credential, IdentityError> {
        let opening_key = self.key_encryption.get_opening_key()?;
        let mut in_out = encrypted.to_vec();
        let nonce_bytes = &encrypted[..NONCE_LEN];
        let nonce = Nonce::assume_unique_for_key(*array_ref!(nonce_bytes, 0, NONCE_LEN));
        
        let decrypted = opening_key.open_in_place(nonce, aead::Aad::empty(), &mut in_out)
            .map_err(|e| IdentityError::DecryptionError(e.to_string()))?;
            
        let credential = serde_json::from_slice(decrypted)?;
        Ok(credential)
    }

    pub fn secure_compare(&self, a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        
        let mut result = 0u8;
        for (x, y) in a.iter().zip(b.iter()) {
            result |= x ^ y;
        }
        result == 0
    }
}
