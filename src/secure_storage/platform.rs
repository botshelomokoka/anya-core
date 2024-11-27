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
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[cfg(target_os = "linux")]
pub use crate::secure_storage::linux::LinuxSecureStorage as SecureStorage;
#[cfg(target_os = "windows")]
pub use crate::secure_storage::windows::WindowsSecureStorage as SecureStorage;
#[cfg(target_os = "macos")]
pub use crate::secure_storage::macos::MacOSSecureStorage as SecureStorage;
#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub use crate::secure_storage::fallback::FallbackSecureStorage as SecureStorage;

pub trait SecureStorageBackend {
    fn store<T: Serialize>(&self, key: &str, value: &T) -> Result<()>;
    fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<T>;
    fn delete(&self, key: &str) -> Result<()>;
}


