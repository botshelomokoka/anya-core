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
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use ring::aead::{self, BoundKey, OpeningKey, SealingKey, UnboundKey, AES_256_GCM};
use ring::rand::{SecureRandom, SystemRandom};
use data_encoding::BASE64;

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
    root_path: PathBuf,
    key: Arc<RwLock<[u8; 32]>>,
    rng: SystemRandom,
}

impl SecureStorage {
    pub fn new(encryption_key: Secret<Vec<u8>>, root_path: PathBuf) -> Result<Self, SecureStorageError> {
        fs::create_dir_all(&root_path)?;
        
        let rng = SystemRandom::new();
        let mut key = [0u8; 32];
        rng.fill(&mut key)
            .map_err(|e| SecureStorageError::StorageError(e.to_string()))?;
        
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
            root_path,
            key: Arc::new(RwLock::new(key)),
            rng,
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

    pub async fn store_file(&self, key: &str, data: &[u8]) -> Result<(), SecureStorageError> {
        let encrypted = self.encrypt_file(data).await?;
        let path = self.get_path(key);
        fs::write(path, encrypted)?;
        Ok(())
    }

    pub async fn load_file(&self, key: &str) -> Result<Vec<u8>, SecureStorageError> {
        let path = self.get_path(key);
        let encrypted = fs::read(path)?;
        self.decrypt_file(&encrypted).await
    }

    pub async fn delete_file(&self, key: &str) -> Result<(), SecureStorageError> {
        let path = self.get_path(key);
        if path.exists() {
            fs::remove_file(path)?;
        }
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

    async fn encrypt_file(&self, data: &[u8]) -> Result<Vec<u8>, SecureStorageError> {
        let key = self.key.read().await;
        let unbound_key = UnboundKey::new(&AES_256_GCM, &key)
            .map_err(|e| SecureStorageError::EncryptionError(e.to_string()))?;

        let mut nonce = [0u8; 12];
        self.rng.fill(&mut nonce)
            .map_err(|e| SecureStorageError::EncryptionError(e.to_string()))?;

        let mut sealing_key = SealingKey::new(unbound_key, aead::Nonce::assume_unique_for_key(nonce));
        let mut in_out = data.to_vec();
        sealing_key.seal_in_place_append_tag(aead::Aad::empty(), &mut in_out)
            .map_err(|e| SecureStorageError::EncryptionError(e.to_string()))?;

        let mut result = nonce.to_vec();
        result.extend(in_out);
        Ok(result)
    }

    async fn decrypt_file(&self, encrypted: &[u8]) -> Result<Vec<u8>, SecureStorageError> {
        if encrypted.len() < 12 {
            return Err(SecureStorageError::EncryptionError(
                "Invalid encrypted data".to_string(),
            ));
        }

        let key = self.key.read().await;
        let unbound_key = UnboundKey::new(&AES_256_GCM, &key)
            .map_err(|e| SecureStorageError::EncryptionError(e.to_string()))?;

        let nonce = &encrypted[..12];
        let ciphertext = &encrypted[12..];

        let mut opening_key = OpeningKey::new(
            unbound_key,
            aead::Nonce::try_assume_unique_for_key(nonce)
                .map_err(|e| SecureStorageError::EncryptionError(e.to_string()))?
        );

        let mut in_out = ciphertext.to_vec();
        let decrypted = opening_key.open_in_place(aead::Aad::empty(), &mut in_out)
            .map_err(|e| SecureStorageError::EncryptionError(e.to_string()))?;

        Ok(decrypted.to_vec())
    }

    fn get_path(&self, key: &str) -> PathBuf {
        let encoded = BASE64.encode(key.as_bytes());
        self.root_path.join(encoded)
    }
}

// Credential Manager for handling sensitive configuration
pub struct CredentialManager {
    secure_storage: Arc<SecureStorage>,
}

impl CredentialManager {
    pub fn new(encryption_key: Secret<Vec<u8>>, root_path: PathBuf) -> Result<Self, SecureStorageError> {
        Ok(Self {
            secure_storage: Arc::new(SecureStorage::new(encryption_key, root_path)?),
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
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_secure_storage() -> Result<(), SecureStorageError> {
        let temp_dir = tempdir().unwrap();
        let mut key = vec![0u8; 32];
        thread_rng().fill_bytes(&mut key);
        let secret_key = Secret::new(key);

        let storage = SecureStorage::new(secret_key, temp_dir.path()).await?;
        let test_data = b"Hello, World!";
        
        // Test store and retrieve
        storage.store("test", test_data).await?;
        let retrieved = storage.get("test").await?;
        assert_eq!(retrieved, test_data);

        // Test remove
        storage.remove("test").await?;
        assert!(storage.get("test").await.is_err());

        // Test file operations
        storage.store_file("test_file", test_data).await?;
        let loaded = storage.load_file("test_file").await?;
        assert_eq!(&loaded, test_data);

        storage.delete_file("test_file").await?;
        assert!(storage.load_file("test_file").await.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_credential_manager() -> Result<(), SecureStorageError> {
        let temp_dir = tempdir().unwrap();
        let mut key = vec![0u8; 32];
        thread_rng().fill_bytes(&mut key);
        let secret_key = Secret::new(key);

        let manager = CredentialManager::new(secret_key, temp_dir.path()).await?;
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
