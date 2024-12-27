use std::sync::Arc;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

use crate::handler::{Handler, GenericHandler};
use crate::metrics::{UnifiedMetrics, ComponentHealth};
use crate::security::{SecurityContext, SecurityManager};
use crate::validation::ValidationResult;
use crate::security::service::{SecurityService, SecurityRequest, SecurityResponse, SecurityAction};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityOperationRequest {
    pub operation_type: SecurityOperationType,
    pub target_resource: String,
    pub parameters: SecurityParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityOperationType {
    UserAuthentication,
    ResourceAuthorization,
    DataEncryption,
    DataDecryption,
    DigitalSignature,
    SignatureVerification,
    SecurityAudit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityParameters {
    pub user_context: Option<UserContext>,
    pub encryption_context: Option<EncryptionContext>,
    pub signature_context: Option<SignatureContext>,
    pub audit_context: Option<AuditContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserContext {
    pub user_id: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionContext {
    pub algorithm: String,
    pub key_id: Option<String>,
    pub additional_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureContext {
    pub algorithm: String,
    pub key_id: Option<String>,
    pub signature_format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditContext {
    pub event_type: String,
    pub severity: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityOperationResponse {
    pub success: bool,
    pub operation_id: String,
    pub result: SecurityOperationResult,
    pub audit_details: Option<AuditDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityOperationResult {
    pub status: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditDetails {
    pub audit_id: String,
    pub severity: String,
    pub requires_attention: bool,
    pub recommendations: Vec<String>,
}

pub struct SecurityHandler {
    inner: GenericHandler<SecurityRequest, SecurityResponse, SecurityError>,
}

impl SecurityHandler {
    pub fn new(
        service: Arc<SecurityService>,
        metrics: Arc<RwLock<UnifiedMetrics>>,
        security: Arc<dyn SecurityManager>,
    ) -> Self {
        Self {
            inner: GenericHandler::new(service, metrics, security),
        }
    }

    fn map_operation_type(op_type: SecurityOperationType) -> SecurityAction {
        match op_type {
            SecurityOperationType::UserAuthentication => SecurityAction::Authenticate,
            SecurityOperationType::ResourceAuthorization => SecurityAction::Authorize,
            SecurityOperationType::DataEncryption => SecurityAction::Encrypt,
            SecurityOperationType::DataDecryption => SecurityAction::Decrypt,
            SecurityOperationType::DigitalSignature => SecurityAction::Sign,
            SecurityOperationType::SignatureVerification => SecurityAction::Verify,
            SecurityOperationType::SecurityAudit => SecurityAction::Audit,
        }
    }
}

#[async_trait]
impl Handler for SecurityHandler {
    type Request = SecurityOperationRequest;
    type Response = SecurityOperationResponse;
    type Error = SecurityError;

    async fn handle(&self, context: &SecurityContext, request: Self::Request) -> Result<Self::Response, Self::Error> {
        // Convert operation request to security request
        let security_request = SecurityRequest {
            action: Self::map_operation_type(request.operation_type),
            resource_id: request.target_resource,
            metadata: Some(SecurityRequestMetadata {
                user_id: request.parameters.user_context.map(|uc| uc.user_id),
                ip_address: request.parameters.user_context.and_then(|uc| uc.ip_address),
                user_agent: request.parameters.user_context.and_then(|uc| uc.user_agent),
                additional_context: Some(serde_json::to_value(&request.parameters)?),
            }),
        };

        // Process through inner handler
        let response = self.inner.handle(context, security_request).await?;

        // Convert security response to operation response
        Ok(SecurityOperationResponse {
            success: response.success,
            operation_id: uuid::Uuid::new_v4().to_string(),
            result: SecurityOperationResult {
                status: if response.success { "SUCCESS" } else { "FAILED" }.to_string(),
                message: response.message,
                timestamp: response.timestamp,
                expires_at: response.metadata.and_then(|m| m.expiration),
            },
            audit_details: response.audit_id.map(|audit_id| AuditDetails {
                audit_id,
                severity: response.metadata
                    .and_then(|m| m.severity)
                    .map_or("INFO".to_string(), |s| format!("{:?}", s)),
                requires_attention: response.metadata.map_or(false, |m| m.requires_action),
                recommendations: vec![],
            }),
        })
    }

    async fn validate(&self, request: &Self::Request) -> Result<ValidationResult, Self::Error> {
        // Validate target resource
        if request.target_resource.is_empty() {
            return Ok(ValidationResult::Invalid("Target resource cannot be empty".to_string()));
        }

        // Validate user context if present
        if let Some(user_context) = &request.parameters.user_context {
            if user_context.user_id.is_empty() {
                return Ok(ValidationResult::Invalid("User ID cannot be empty".to_string()));
            }

            if let Some(ip) = &user_context.ip_address {
                if !is_valid_ip(ip) {
                    return Ok(ValidationResult::Invalid("Invalid IP address format".to_string()));
                }
            }
        }

        // Validate encryption context if present
        if let Some(encryption_context) = &request.parameters.encryption_context {
            if encryption_context.algorithm.is_empty() {
                return Ok(ValidationResult::Invalid("Encryption algorithm cannot be empty".to_string()));
            }
        }

        // Validate signature context if present
        if let Some(signature_context) = &request.parameters.signature_context {
            if signature_context.algorithm.is_empty() {
                return Ok(ValidationResult::Invalid("Signature algorithm cannot be empty".to_string()));
            }
        }

        Ok(ValidationResult::Valid)
    }

    async fn get_health(&self) -> Result<ComponentHealth, Self::Error> {
        self.inner.get_health().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_security_handler() {
        let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
        let service = Arc::new(MockSecurityService);
        let security = Arc::new(MockSecurityManager);

        let handler = SecurityHandler::new(service, metrics, security);

        // Test authentication request
        let request = SecurityOperationRequest {
            operation_type: SecurityOperationType::UserAuthentication,
            target_resource: "user-123".to_string(),
            parameters: SecurityParameters {
                user_context: Some(UserContext {
                    user_id: "user-123".to_string(),
                    ip_address: Some("192.168.1.1".to_string()),
                    user_agent: Some("Mozilla/5.0".to_string()),
                    session_id: None,
                }),
                encryption_context: None,
                signature_context: None,
                audit_context: None,
            },
        };

        let context = SecurityContext::default();
        let response = handler.handle(&context, request).await.unwrap();

        assert!(response.success);
        assert_eq!(response.result.status, "SUCCESS");
        assert!(response.audit_details.is_some());

        // Test health check
        let health = handler.get_health().await.unwrap();
        assert!(health.operational);
        assert_eq!(health.health_score, 100.0);
    }
}
