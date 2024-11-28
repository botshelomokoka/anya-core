use secrecy::{Secret, ExposeSecret};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use thiserror::Error;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::{thread_rng, RngCore};

#[derive(Error, Debug)]
pub enum SecureStorageError {
    #[error("Key not found")]
    KeyNotFound,
    #[error("Encryption error: {0}")]
    EncryptionError(String),
    #[error("Storage error: {0}")]
    StorageError(String),
}

pub struct SecureStorage {
    encrypted_store: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    encryption_key: Secret<Vec<u8>>,
    cipher: Aes256Gcm,
}

impl SecureStorage {
    pub fn new(encryption_key: Secret<Vec<u8>>) -> Result<Self, SecureStorageError> {
        let key = encryption_key.expose_secret();
        if key.len() != 32 {
            return Err(SecureStorageError::EncryptionError(
                "Encryption key must be 32 bytes".to_string(),
            ));
        }

        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| SecureStorageError::EncryptionError(e.to_string()))?;

        Ok(Self {
            encrypted_store: Arc::new(RwLock::new(HashMap::new())),
            encryption_key,
            cipher,
        })
    }

    pub async fn store(&self, key: &str, value: &[u8]) -> Result<(), SecureStorageError> {
        let encrypted = self.encrypt(value)?;
        let mut store = self.encrypted_store.write().await;
        store.insert(key.to_string(), encrypted);
        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<Vec<u8>, SecureStorageError> {
        let store = self.encrypted_store.read().await;
        let encrypted = store
            .get(key)
            .ok_or(SecureStorageError::KeyNotFound)?
            .clone();
        self.decrypt(&encrypted)
    }

    pub async fn remove(&self, key: &str) -> Result<(), SecureStorageError> {
        let mut store = self.encrypted_store.write().await;
        store.remove(key);
        Ok(())
    }

    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, SecureStorageError> {
        let mut nonce_bytes = [0u8; 12];
        thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = self.cipher
            .encrypt(nonce, data)
            .map_err(|e| SecureStorageError::EncryptionError(e.to_string()))?;

        // Combine nonce and ciphertext
        let mut encrypted = Vec::with_capacity(nonce_bytes.len() + ciphertext.len());
        encrypted.extend_from_slice(&nonce_bytes);
        encrypted.extend_from_slice(&ciphertext);

        Ok(encrypted)
    }

    fn decrypt(&self, encrypted: &[u8]) -> Result<Vec<u8>, SecureStorageError> {
        if encrypted.len() < 12 {
            return Err(SecureStorageError::EncryptionError(
                "Invalid encrypted data".to_string(),
            ));
        }

        let (nonce_bytes, ciphertext) = encrypted.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        self.cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| SecureStorageError::EncryptionError(e.to_string()))
    }
}

// Credential Manager for handling sensitive configuration
pub struct CredentialManager {
    secure_storage: Arc<SecureStorage>,
}

impl CredentialManager {
    pub fn new(encryption_key: Secret<Vec<u8>>) -> Result<Self, SecureStorageError> {
        Ok(Self {
            secure_storage: Arc::new(SecureStorage::new(encryption_key)?),
        })
    }

    pub async fn store_credentials(&self, service: &str, credentials: &HashMap<String, String>) -> Result<(), SecureStorageError> {
        let serialized = serde_json::to_vec(credentials)
            .map_err(|e| SecureStorageError::StorageError(e.to_string()))?;
        self.secure_storage.store(service, &serialized).await
    }

    pub async fn get_credentials(&self, service: &str) -> Result<HashMap<String, String>, SecureStorageError> {
        let data = self.secure_storage.get(service).await?;
        serde_json::from_slice(&data)
            .map_err(|e| SecureStorageError::StorageError(e.to_string()))
    }

    pub async fn remove_credentials(&self, service: &str) -> Result<(), SecureStorageError> {
        self.secure_storage.remove(service).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_secure_storage() -> Result<(), SecureStorageError> {
        let mut key = vec![0u8; 32];
        thread_rng().fill_bytes(&mut key);
        let secret_key = Secret::new(key);

        let storage = SecureStorage::new(secret_key)?;
        let test_data = b"Hello, World!";
        
        // Test store and retrieve
        storage.store("test", test_data).await?;
        let retrieved = storage.get("test").await?;
        assert_eq!(retrieved, test_data);

        // Test remove
        storage.remove("test").await?;
        assert!(storage.get("test").await.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_credential_manager() -> Result<(), SecureStorageError> {
        let mut key = vec![0u8; 32];
        thread_rng().fill_bytes(&mut key);
        let secret_key = Secret::new(key);

        let manager = CredentialManager::new(secret_key)?;
        let mut creds = HashMap::new();
        creds.insert("username".to_string(), "test_user".to_string());
        creds.insert("password".to_string(), "test_pass".to_string());

        // Test store and retrieve
        manager.store_credentials("test_service", &creds).await?;
        let retrieved = manager.get_credentials("test_service").await?;
        assert_eq!(retrieved, creds);

        // Test remove
        manager.remove_credentials("test_service").await?;
        assert!(manager.get_credentials("test_service").await.is_err());

        Ok(())
    }
}
