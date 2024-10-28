use super::super::IdentityError;
use ring::aead::{self, BoundKey, Nonce, NonceSequence, UnboundKey, AES_256_GCM};
use ring::rand::{SecureRandom, SystemRandom};
use std::sync::Arc;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct EncryptionKey {
    key: [u8; 32],
}

pub struct NonceGen {
    nonce: [u8; 12],
}

impl NonceSequence for NonceGen {
    fn advance(&mut self) -> Result<Nonce, ring::error::Unspecified> {
        Ok(Nonce::assume_unique_for_key(self.nonce))
    }
}

pub struct AdvancedEncryption {
    rng: SystemRandom,
    key_encryption: Arc<KeyEncryption>,
}

impl AdvancedEncryption {
    pub fn new(key_encryption: Arc<KeyEncryption>) -> Self {
        Self {
            rng: SystemRandom::new(),
            key_encryption,
        }
    }

    pub fn encrypt_with_aad(&self, data: &[u8], aad: &[u8]) -> Result<Vec<u8>, IdentityError> {
        let mut key_bytes = [0u8; 32];
        self.rng.fill(&mut key_bytes)
            .map_err(|e| IdentityError::EncryptionError(e.to_string()))?;

        let key = UnboundKey::new(&AES_256_GCM, &key_bytes)
            .map_err(|e| IdentityError::EncryptionError(e.to_string()))?;

        let mut nonce_bytes = [0u8; 12];
        self.rng.fill(&mut nonce_bytes)
            .map_err(|e| IdentityError::EncryptionError(e.to_string()))?;

        let nonce_gen = NonceGen { nonce: nonce_bytes };
        let mut sealing_key = aead::SealingKey::new(key, nonce_gen);

        let mut in_out = data.to_vec();
        sealing_key.seal_in_place_append_tag(aead::Aad::from(aad), &mut in_out)
            .map_err(|e| IdentityError::EncryptionError(e.to_string()))?;

        // Combine nonce and ciphertext
        let mut result = Vec::with_capacity(nonce_bytes.len() + in_out.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&in_out);

        Ok(result)
    }

    pub fn decrypt_with_aad(&self, encrypted: &[u8], aad: &[u8]) -> Result<Vec<u8>, IdentityError> {
        if encrypted.len() < 12 {
            return Err(IdentityError::DecryptionError("Invalid ciphertext".into()));
        }

        let (nonce_bytes, ciphertext) = encrypted.split_at(12);
        let nonce_gen = NonceGen { nonce: nonce_bytes.try_into().unwrap() };

        let key = self.key_encryption.get_opening_key()?;
        let mut opening_key = aead::OpeningKey::new(key, nonce_gen);

        let mut in_out = ciphertext.to_vec();
        let decrypted = opening_key.open_in_place(aead::Aad::from(aad), &mut in_out)
            .map_err(|e| IdentityError::DecryptionError(e.to_string()))?;

        Ok(decrypted.to_vec())
    }
}
