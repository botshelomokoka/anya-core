//! Module documentation for $moduleName
//!
//! # Overview
//! This module is part of the Anya Core project, located at $modulePath.
//!
//! # Architecture
//! [Add module-specific architecture details]
//!
//! # API Reference
//! [Document public functions and types]
//!
//! # Usage Examples
//! `ust
//! // Add usage examples
//! `
//!
//! # Error Handling
//! This module uses proper error handling with Result types.
//!
//! # Security Considerations
//! [Document security features and considerations]
//!
//! # Performance
//! [Document performance characteristics]

use std::error::Error;
use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Debug)]
pub struct SecureStorage {
    namespace: String,
}

impl SecureStorage {
    pub fn new(namespace: &str) -> Self {
        Self {
            namespace: namespace.to_string(),
        }
    }

    pub fn store<T: Serialize>(&self, key: &str, value: &T) -> Result<()> {
        let full_key = format!("{}_{}", self.namespace, key);
        
        #[cfg(target_os = "linux")]
        {
            use secret_service::{EncryptionType, SecretService};
            
            let ss = SecretService::new(EncryptionType::Dh)?;
            let collection = ss.get_default_collection()?;
            
            let value_str = serde_json::to_string(value)?;
            collection.create_item(
                &full_key,
                vec![("application", "anya")],
                value_str.as_bytes(),
                true,
                "text/plain"
            )?;
            Ok(())
        }

        #[cfg(target_os = "windows")]
        {
            use winapi::um::wincred::{CredentialW, CredWriteW};
            use std::ptr;
            
            let value_str = serde_json::to_string(value)?;
            let value_wide: Vec<u16> = value_str.encode_utf16().collect();
            
            let cred = CredentialW {
                Flags: 0,
                Type: 1, // CRED_TYPE_GENERIC
                TargetName: full_key.as_ptr() as *mut _,
                Comment: ptr::null_mut(),
                LastWritten: unsafe { std::mem::zeroed() },
                CredentialBlobSize: (value_wide.len() * 2) as u32,
                CredentialBlob: value_wide.as_ptr() as *mut _,
                Persist: 2, // CRED_PERSIST_LOCAL_MACHINE
                AttributeCount: 0,
                Attributes: ptr::null_mut(),
                TargetAlias: ptr::null_mut(),
                UserName: ptr::null_mut(),
            };
            
            let result = unsafe { CredWriteW(&cred, 0) };
            if result == 0 {
                anyhow::bail!("Failed to write to Windows Credential Manager");
            }
            Ok(())
        }

        #[cfg(target_os = "macos")]
        {
            use security_framework::os::macos::keychain::{SecKeychain, SecKeychainItem};
            use security_framework::os::macos::keychain_item::SecKeychainItemRef;
            
            let keychain = SecKeychain::default()?;
            let value_str = serde_json::to_string(value)?;
            
            keychain.create_generic_password(
                "anya",
                &full_key,
                value_str.as_bytes(),
            )?;
            Ok(())
        }

        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        {
            // Fallback to encrypted file storage
            let config_dir = dirs::config_dir()
                .context("Could not find config directory")?
                .join("anya")
                .join("secrets");
            
            std::fs::create_dir_all(&config_dir)?;
            
            let file_path = config_dir.join(format!("{}.enc", full_key));
            let value_str = serde_json::to_string(value)?;
            
            // Use age encryption
            let encryptor = age::Encryptor::with_user_passphrase(
                age::SecretString::new(self.get_master_key()?.to_string())
            );
            
            let mut encrypted = vec![];
            let mut writer = encryptor.wrap_output(&mut encrypted)?;
            writer.write_all(value_str.as_bytes())?;
            writer.finish()?;
            
            std::fs::write(file_path, encrypted)?;
            Ok(())
        }
    }

    pub fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<T> {
        let full_key = format!("{}_{}", self.namespace, key);

        #[cfg(target_os = "linux")]
        {
            use secret_service::SecretService;
            
            let ss = SecretService::new(EncryptionType::Dh)?;
            let collection = ss.get_default_collection()?;
            
            let items = collection.search_items(vec![("application", "anya")])?;
            let item = items.iter()
                .find(|i| i.get_label()? == full_key)
                .context("Secret not found")?;
                
            let secret = item.get_secret()?;
            let value_str = String::from_utf8(secret)?;
            Ok(serde_json::from_str(&value_str)?)
        }

        #[cfg(target_os = "windows")]
        {
            use winapi::um::wincred::{CredReadW, CredFree};
            
            let mut pcred: *mut CredentialW = ptr::null_mut();
            let result = unsafe {
                CredReadW(
                    full_key.as_ptr() as *const _,
                    1, // CRED_TYPE_GENERIC
                    0,
                    &mut pcred
                )
            };
            
            if result == 0 {
                anyhow::bail!("Secret not found in Windows Credential Manager");
            }
            
            let cred = unsafe { &*pcred };
            let blob = unsafe {
                std::slice::from_raw_parts(
                    cred.CredentialBlob as *const u16,
                    cred.CredentialBlobSize as usize / 2
                )
            };
            
            let value_str = String::from_utf16(blob)?;
            unsafe { CredFree(pcred as *mut _) };
            
            Ok(serde_json::from_str(&value_str)?)
        }

        #[cfg(target_os = "macos")]
        {
            use security_framework::os::macos::keychain::SecKeychain;
            
            let keychain = SecKeychain::default()?;
            let (_, data) = keychain.find_generic_password("anya", &full_key)?;
            let value_str = String::from_utf8(data)?;
            
            Ok(serde_json::from_str(&value_str)?)
        }

        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        {
            let config_dir = dirs::config_dir()
                .context("Could not find config directory")?
                .join("anya")
                .join("secrets");
                
            let file_path = config_dir.join(format!("{}.enc", full_key));
            let encrypted = std::fs::read(file_path)?;
            
            let decryptor = match age::Decryptor::new(&encrypted[..])? {
                age::Decryptor::Passphrase(d) => d,
                _ => anyhow::bail!("Invalid encryption type"),
            };
            
            let mut decrypted = vec![];
            let mut reader = decryptor.decrypt(&age::SecretString::new(
                self.get_master_key()?.to_string()
            ))?;
            
            reader.read_to_end(&mut decrypted)?;
            let value_str = String::from_utf8(decrypted)?;
            
            Ok(serde_json::from_str(&value_str)?)
        }
    }

    fn get_master_key(&self) -> Result<String> {
        // In a real implementation, this would use a more secure method
        // of deriving/storing the master key
        Ok(std::env::var("ANYA_MASTER_KEY")
            .context("ANYA_MASTER_KEY environment variable not set")?)
    }
}


