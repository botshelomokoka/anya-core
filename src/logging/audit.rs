use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AuditLog {
    timestamp: DateTime<Utc>,
    action: AuditAction,
    user_id: String,
    details: String,
}

#[derive(Debug, Serialize)]
pub enum AuditAction {
    KeyGeneration,
    KeyUsage,
    SignatureCreation,
    UTXOSpent,
    AuthenticationAttempt,
}

impl AuditLog {
    pub async fn log_action(
        &self,
        action: AuditAction,
        user_id: &str,
        details: &str,
    ) -> Result<(), LogError> {
        let log = AuditLog {
            timestamp: Utc::now(),
            action,
            user_id: user_id.to_string(),
            details: details.to_string(),
        };
        
        // Log to secure storage
        self.db.insert_audit_log(&log).await?;
        
        Ok(())
    }
}
