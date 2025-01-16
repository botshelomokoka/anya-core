use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use argon2::{Argon2, PasswordHasher};
use zeroize::Zeroize;

pub struct KeyEncryption {
    cipher: Aes256Gcm,
}

impl KeyEncryption {
    pub fn new(master_key: &[u8]) -> Result<Self, AuthError> {
        let key = Key::from_slice(master_key);
        let cipher = Aes256Gcm::new(key);
        Ok(Self { cipher })
    }

    pub fn encrypt_key(&self, secret_key: &SecretKey) -> Result<Vec<u8>, AuthError> {
        let nonce = Nonce::from_slice(b"unique nonce"); // Should be random in production
        let ciphertext = self.cipher
            .encrypt(nonce, secret_key.as_ref())
            .map_err(|e| AuthError::Encryption(e.to_string()))?;
        Ok(ciphertext)
    }

    pub fn decrypt_key(&self, ciphertext: &[u8]) -> Result<SecretKey, AuthError> {
        let nonce = Nonce::from_slice(b"unique nonce");
        let plaintext = self.cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| AuthError::Encryption(e.to_string()))?;
        SecretKey::from_slice(&plaintext)
            .map_err(|e| AuthError::KeyDerivation(e.to_string()))
    }
}
