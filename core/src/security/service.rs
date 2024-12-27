use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use serde::{Serialize, Deserialize};

use crate::service::{Service, GenericService};
use crate::metrics::{UnifiedMetrics, ComponentHealth};
use crate::security::{SecurityContext, SecurityManager};
use crate::validation::ValidationResult;
use crate::security::repository::{SecurityAudit, SecurityAuditRepository, AuditType, AuditSeverity, AuditStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRequest {
    pub action: SecurityAction,
    pub resource_id: String,
    pub metadata: Option<SecurityRequestMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityAction {
    Authenticate,
    Authorize,
    Audit,
    Encrypt,
    Decrypt,
    Sign,
    Verify,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRequestMetadata {
    pub user_id: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub additional_context: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityResponse {
    pub success: bool,
    pub audit_id: Option<String>,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: Option<SecurityResponseMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityResponseMetadata {
    pub severity: Option<AuditSeverity>,
    pub requires_action: bool,
    pub expiration: Option<DateTime<Utc>>,
}

pub struct SecurityService {
    repository: Arc<SecurityAuditRepository>,
    metrics: Arc<RwLock<UnifiedMetrics>>,
    security: Arc<dyn SecurityManager>,
    crypto_provider: Arc<dyn CryptoProvider>,
}

#[async_trait]
impl Service for SecurityService {
    type Item = SecurityRequest;
    type Response = SecurityResponse;
    type Error = SecurityError;

    async fn process(&self, context: &SecurityContext, request: Self::Item) -> Result<Self::Response, Self::Error> {
        let start_time = Utc::now();

        // Validate security context
        self.security.validate_context(context).await?;

        // Process based on action type
        let (success, message, severity) = match request.action {
            SecurityAction::Authenticate => {
                self.handle_authentication(&request).await?
            },
            SecurityAction::Authorize => {
                self.handle_authorization(context, &request).await?
            },
            SecurityAction::Audit => {
                self.handle_audit(&request).await?
            },
            SecurityAction::Encrypt => {
                self.handle_encryption(&request).await?
            },
            SecurityAction::Decrypt => {
                self.handle_decryption(context, &request).await?
            },
            SecurityAction::Sign => {
                self.handle_signing(context, &request).await?
            },
            SecurityAction::Verify => {
                self.handle_verification(&request).await?
            },
        };

        // Create audit record
        let audit = SecurityAudit {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: start_time,
            audit_type: match request.action {
                SecurityAction::Authenticate => AuditType::Authentication,
                SecurityAction::Authorize => AuditType::Authorization,
                SecurityAction::Audit => AuditType::SecurityEvent,
                _ => AuditType::SystemChange,
            },
            severity,
            description: message.clone(),
            metadata: SecurityMetadata {
                user_id: request.metadata.as_ref().and_then(|m| m.user_id.clone()),
                resource_id: Some(request.resource_id),
                ip_address: request.metadata.as_ref().and_then(|m| m.ip_address.clone()),
                user_agent: request.metadata.as_ref().and_then(|m| m.user_agent.clone()),
                location: None,
                tags: vec![request.action.to_string()],
            },
            status: if success { AuditStatus::Closed } else { AuditStatus::RequiresAttention },
        };

        let audit_id = audit.id.clone();
        self.repository.create(audit).await?;

        // Update metrics
        let mut metrics = self.metrics.write().await;
        metrics.security.as_mut().map(|sec| {
            if success {
                sec.successful_operations += 1;
            } else {
                sec.failed_operations += 1;
            }
            sec.last_operation_time = Some(start_time);
        });

        Ok(SecurityResponse {
            success,
            audit_id: Some(audit_id),
            message,
            timestamp: start_time,
            metadata: Some(SecurityResponseMetadata {
                severity: Some(severity),
                requires_action: !success,
                expiration: None,
            }),
        })
    }

    async fn validate(&self, request: &Self::Item) -> Result<ValidationResult, Self::Error> {
        // Validate resource_id
        if request.resource_id.is_empty() {
            return Ok(ValidationResult::Invalid("Resource ID cannot be empty".to_string()));
        }

        // Validate metadata if present
        if let Some(metadata) = &request.metadata {
            if let Some(ip) = &metadata.ip_address {
                if !is_valid_ip(ip) {
                    return Ok(ValidationResult::Invalid("Invalid IP address format".to_string()));
                }
            }
        }

        Ok(ValidationResult::Valid)
    }

    async fn get_health(&self) -> Result<ComponentHealth, Self::Error> {
        let audits = self.repository.list().await?;
        
        let open_critical = audits.iter()
            .filter(|a| a.status == AuditStatus::Open && a.severity == AuditSeverity::Critical)
            .count();
            
        let total_open = audits.iter()
            .filter(|a| a.status == AuditStatus::Open)
            .count();
            
        let recent_failures = audits.iter()
            .filter(|a| {
                a.timestamp > (Utc::now() - Duration::hours(24)) &&
                matches!(a.status, AuditStatus::Open | AuditStatus::RequiresAttention)
            })
            .count();

        Ok(ComponentHealth {
            operational: open_critical == 0,
            health_score: if total_open > 0 {
                100.0 * (1.0 - (open_critical as f64 / total_open as f64))
            } else {
                100.0
            },
            last_incident: audits.iter()
                .filter(|a| a.severity == AuditSeverity::Critical)
                .map(|a| a.timestamp)
                .max(),
            error_count: open_critical,
            warning_count: recent_failures,
        })
    }
}

impl SecurityService {
    async fn handle_authentication(&self, request: &SecurityRequest) -> Result<(bool, String, AuditSeverity), SecurityError> {
        // Implementation details...
        Ok((true, "Authentication successful".to_string(), AuditSeverity::Info))
    }

    async fn handle_authorization(&self, context: &SecurityContext, request: &SecurityRequest) 
        -> Result<(bool, String, AuditSeverity), SecurityError> {
        // Implementation details...
        Ok((true, "Authorization granted".to_string(), AuditSeverity::Info))
    }

    async fn handle_audit(&self, request: &SecurityRequest) 
        -> Result<(bool, String, AuditSeverity), SecurityError> {
        // Implementation details...
        Ok((true, "Audit recorded".to_string(), AuditSeverity::Info))
    }

    async fn handle_encryption(&self, request: &SecurityRequest) 
        -> Result<(bool, String, AuditSeverity), SecurityError> {
        // Implementation details...
        Ok((true, "Data encrypted".to_string(), AuditSeverity::Info))
    }

    async fn handle_decryption(&self, context: &SecurityContext, request: &SecurityRequest) 
        -> Result<(bool, String, AuditSeverity), SecurityError> {
        // Implementation details...
        Ok((true, "Data decrypted".to_string(), AuditSeverity::Info))
    }

    async fn handle_signing(&self, context: &SecurityContext, request: &SecurityRequest) 
        -> Result<(bool, String, AuditSeverity), SecurityError> {
        // Implementation details...
        Ok((true, "Data signed".to_string(), AuditSeverity::Info))
    }

    async fn handle_verification(&self, request: &SecurityRequest) 
        -> Result<(bool, String, AuditSeverity), SecurityError> {
        // Implementation details...
        Ok((true, "Signature verified".to_string(), AuditSeverity::Info))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_security_service() {
        let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
        let repository = Arc::new(SecurityAuditRepository::new(metrics.clone()));
        let security = Arc::new(MockSecurityManager);
        let crypto_provider = Arc::new(MockCryptoProvider);

        let service = SecurityService {
            repository,
            metrics,
            security,
            crypto_provider,
        };

        // Test authentication request
        let request = SecurityRequest {
            action: SecurityAction::Authenticate,
            resource_id: "user-123".to_string(),
            metadata: Some(SecurityRequestMetadata {
                user_id: Some("user-123".to_string()),
                ip_address: Some("192.168.1.1".to_string()),
                user_agent: Some("Mozilla/5.0".to_string()),
                additional_context: None,
            }),
        };

        let context = SecurityContext::default();
        let response = service.process(&context, request).await.unwrap();

        assert!(response.success);
        assert!(response.audit_id.is_some());
        assert_eq!(response.message, "Authentication successful");

        // Test health check
        let health = service.get_health().await.unwrap();
        assert!(health.operational);
        assert_eq!(health.health_score, 100.0);
    }
}
