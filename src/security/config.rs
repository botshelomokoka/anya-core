use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::auth::AuthManager;
use crate::metrics::{counter, gauge};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    // Access Control
    pub mfa_required: bool,
    pub rbac_enabled: bool,
    pub session_timeout_minutes: u32,
    pub max_failed_attempts: u32,
    
    // Encryption
    pub encryption_algorithm: String,  // Default: "AES-256-GCM"
    pub key_rotation_days: u32,
    pub secure_key_storage: bool,
    
    // Rate Limiting
    pub rate_limit_requests: u32,
    pub rate_limit_window_seconds: u32,
    
    // Audit Logging
    pub audit_log_enabled: bool,
    pub audit_log_retention_days: u32,
    
    // Security Headers
    pub hsts_enabled: bool,
    pub csp_enabled: bool,
    pub xss_protection: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            mfa_required: true,
            rbac_enabled: true,
            session_timeout_minutes: 30,
            max_failed_attempts: 5,
            encryption_algorithm: "AES-256-GCM".to_string(),
            key_rotation_days: 90,
            secure_key_storage: true,
            rate_limit_requests: 100,
            rate_limit_window_seconds: 60,
            audit_log_enabled: true,
            audit_log_retention_days: 90,
            hsts_enabled: true,
            csp_enabled: true,
            xss_protection: true,
        }
    }
}

pub struct SecurityManager {
    config: Arc<RwLock<SecurityConfig>>,
    auth_manager: Arc<AuthManager>,
}

impl SecurityManager {
    pub fn new(config: SecurityConfig, auth_manager: Arc<AuthManager>) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
            auth_manager,
        }
    }

    pub async fn update_config(&self, new_config: SecurityConfig) {
        let mut config = self.config.write().await;
        *config = new_config;
        
        // Update metrics
        gauge!("security.mfa_enabled", new_config.mfa_required as i64);
        gauge!("security.rbac_enabled", new_config.rbac_enabled as i64);
        counter!("security.config_updates").increment(1);
    }

    pub async fn verify_security_settings(&self) -> Result<SecurityAuditReport, SecurityError> {
        let config = self.config.read().await;
        
        let mut report = SecurityAuditReport::new();
        
        // Verify encryption settings
        if config.encryption_algorithm != "AES-256-GCM" {
            report.add_warning("Non-standard encryption algorithm in use");
        }
        
        // Verify access control
        if !config.mfa_required {
            report.add_warning("MFA is not required");
        }
        if !config.rbac_enabled {
            report.add_warning("RBAC is not enabled");
        }
        
        // Verify rate limiting
        if config.rate_limit_requests > 1000 {
            report.add_warning("High rate limit threshold");
        }
        
        // Verify audit logging
        if !config.audit_log_enabled {
            report.add_warning("Audit logging is disabled");
        }
        
        Ok(report)
    }
}

#[derive(Debug)]
pub struct SecurityAuditReport {
    pub warnings: Vec<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl SecurityAuditReport {
    pub fn new() -> Self {
        Self {
            warnings: Vec::new(),
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn add_warning(&mut self, warning: &str) {
        self.warnings.push(warning.to_string());
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Audit error: {0}")]
    AuditError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_security_config_validation() {
        let auth_manager = Arc::new(AuthManager::new());
        let config = SecurityConfig::default();
        let security_manager = SecurityManager::new(config, auth_manager);
        
        let report = security_manager.verify_security_settings().await.unwrap();
        assert!(report.warnings.is_empty(), "Default config should have no warnings");
    }
    
    #[tokio::test]
    async fn test_security_config_update() {
        let auth_manager = Arc::new(AuthManager::new());
        let config = SecurityConfig::default();
        let security_manager = SecurityManager::new(config.clone(), auth_manager);
        
        let mut new_config = config;
        new_config.mfa_required = false;
        security_manager.update_config(new_config).await;
        
        let report = security_manager.verify_security_settings().await.unwrap();
        assert_eq!(report.warnings.len(), 1, "Should warn about disabled MFA");
    }
}
