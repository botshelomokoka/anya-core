use std::sync::Arc;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::repository::{Repository, GenericRepository};
use crate::metrics::{UnifiedMetrics, ComponentHealth};
use crate::validation::{ValidationResult, Validator};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAudit {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub audit_type: AuditType,
    pub severity: AuditSeverity,
    pub description: String,
    pub metadata: SecurityMetadata,
    pub status: AuditStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditType {
    Access,
    Authentication,
    Authorization,
    DataAccess,
    SystemChange,
    SecurityEvent,
    Compliance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditStatus {
    Open,
    InProgress,
    Resolved,
    Closed,
    RequiresAttention,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMetadata {
    pub user_id: Option<String>,
    pub resource_id: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub location: Option<String>,
    pub tags: Vec<String>,
}

pub struct SecurityAuditValidator;

#[async_trait]
impl Validator<SecurityAudit> for SecurityAuditValidator {
    async fn validate(&self, audit: &SecurityAudit) -> Result<ValidationResult, ValidationError> {
        // Validate timestamp
        if audit.timestamp > Utc::now() {
            return Ok(ValidationResult::Invalid("Timestamp cannot be in the future".to_string()));
        }

        // Validate description
        if audit.description.is_empty() {
            return Ok(ValidationResult::Invalid("Description cannot be empty".to_string()));
        }

        // Validate metadata
        if let Some(ip) = &audit.metadata.ip_address {
            if !is_valid_ip(ip) {
                return Ok(ValidationResult::Invalid("Invalid IP address format".to_string()));
            }
        }

        Ok(ValidationResult::Valid)
    }
}

fn is_valid_ip(ip: &str) -> bool {
    // Simple IP validation - could be more sophisticated
    ip.split('.').count() == 4 && ip.split('.').all(|x| {
        if let Ok(num) = x.parse::<u8>() {
            true
        } else {
            false
        }
    })
}

pub type SecurityAuditRepository = GenericRepository<SecurityAudit, SecurityError>;

impl SecurityAuditRepository {
    pub fn new(metrics: Arc<RwLock<UnifiedMetrics>>) -> Self {
        Self::new(
            metrics,
            Arc::new(SecurityAuditValidator),
        )
    }

    pub async fn get_audits_by_severity(&self, severity: AuditSeverity) -> Result<Vec<SecurityAudit>, SecurityError> {
        let audits = self.list().await?;
        Ok(audits.into_iter()
            .filter(|a| a.severity == severity)
            .collect())
    }

    pub async fn get_open_audits(&self) -> Result<Vec<SecurityAudit>, SecurityError> {
        let audits = self.list().await?;
        Ok(audits.into_iter()
            .filter(|a| matches!(a.status, AuditStatus::Open | AuditStatus::RequiresAttention))
            .collect())
    }

    pub async fn get_audits_by_type(&self, audit_type: AuditType) -> Result<Vec<SecurityAudit>, SecurityError> {
        let audits = self.list().await?;
        Ok(audits.into_iter()
            .filter(|a| a.audit_type == audit_type)
            .collect())
    }

    pub async fn get_audits_by_user(&self, user_id: &str) -> Result<Vec<SecurityAudit>, SecurityError> {
        let audits = self.list().await?;
        Ok(audits.into_iter()
            .filter(|a| a.metadata.user_id.as_ref().map_or(false, |id| id == user_id))
            .collect())
    }

    pub async fn close_resolved_audits(&self) -> Result<usize, SecurityError> {
        let mut count = 0;
        let audits = self.list().await?;
        
        for audit in audits {
            if matches!(audit.status, AuditStatus::Resolved) {
                let mut closed_audit = audit.clone();
                closed_audit.status = AuditStatus::Closed;
                self.update(&audit.id, closed_audit).await?;
                count += 1;
            }
        }
        
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_security_audit_repository() {
        let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
        let repo = SecurityAuditRepository::new(metrics);

        // Create test audit
        let audit = SecurityAudit {
            id: "test-1".to_string(),
            timestamp: Utc::now(),
            audit_type: AuditType::Authentication,
            severity: AuditSeverity::High,
            description: "Failed login attempt".to_string(),
            metadata: SecurityMetadata {
                user_id: Some("user-1".to_string()),
                resource_id: None,
                ip_address: Some("192.168.1.1".to_string()),
                user_agent: Some("Mozilla/5.0".to_string()),
                location: None,
                tags: vec!["login".to_string(), "failed".to_string()],
            },
            status: AuditStatus::Open,
        };

        // Test create
        let created = repo.create(audit.clone()).await.unwrap();
        assert_eq!(created.description, audit.description);

        // Test get by severity
        let high_severity = repo.get_audits_by_severity(AuditSeverity::High).await.unwrap();
        assert_eq!(high_severity.len(), 1);
        assert_eq!(high_severity[0].id, audit.id);

        // Test get open audits
        let open_audits = repo.get_open_audits().await.unwrap();
        assert_eq!(open_audits.len(), 1);
        assert_eq!(open_audits[0].id, audit.id);

        // Test get by type
        let auth_audits = repo.get_audits_by_type(AuditType::Authentication).await.unwrap();
        assert_eq!(auth_audits.len(), 1);
        assert_eq!(auth_audits[0].id, audit.id);

        // Test get by user
        let user_audits = repo.get_audits_by_user("user-1").await.unwrap();
        assert_eq!(user_audits.len(), 1);
        assert_eq!(user_audits[0].id, audit.id);

        // Test close resolved
        let mut resolved_audit = audit.clone();
        resolved_audit.status = AuditStatus::Resolved;
        repo.update(&audit.id, resolved_audit).await.unwrap();

        let closed = repo.close_resolved_audits().await.unwrap();
        assert_eq!(closed, 1);

        let closed_audit = repo.read(&audit.id).await.unwrap().unwrap();
        assert!(matches!(closed_audit.status, AuditStatus::Closed));
    }
}
