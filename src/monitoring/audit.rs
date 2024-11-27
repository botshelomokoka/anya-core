use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio::sync::RwLock;
use web5::did::Did;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub timestamp: DateTime<Utc>,
    pub event_type: AuditEventType,
    pub actor: Did,
    pub action: String,
    pub details: serde_json::Value,
    pub status: AuditEventStatus,
    pub metadata: AuditMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEventType {
    Governance,
    Security,
    Financial,
    System,
    CrossChain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEventStatus {
    Success,
    Failure,
    Pending,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditMetadata {
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub session_id: Option<String>,
    pub chain_id: Option<String>,
    pub block_number: Option<u64>,
    pub transaction_hash: Option<String>,
}

pub struct AuditLogger {
    store: Arc<RwLock<Vec<AuditEvent>>>,
    web5_protocol: web5::protocol::Protocol,
}

impl AuditLogger {
    pub async fn new(protocol: web5::protocol::Protocol) -> Self {
        Self {
            store: Arc::new(RwLock::new(Vec::new())),
            web5_protocol: protocol,
        }
    }

    pub async fn log_event(
        &self,
        event_type: AuditEventType,
        actor: Did,
        action: String,
        details: serde_json::Value,
        status: AuditEventStatus,
        metadata: AuditMetadata,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let event = AuditEvent {
            timestamp: Utc::now(),
            event_type,
            actor,
            action,
            details,
            status,
            metadata,
        };

        // Store locally
        self.store.write().await.push(event.clone());

        // Store in Web5
        self.web5_protocol
            .records()
            .create(
                &serde_json::to_value(&event)?,
                Some(&serde_json::json!({
                    "schema": "https://anya.ai/schemas/audit",
                    "dataFormat": "application/json"
                })),
            )
            .await?;

        Ok(())
    }

    pub async fn query_events(
        &self,
        filter: Option<AuditEventFilter>,
    ) -> Result<Vec<AuditEvent>, Box<dyn std::error::Error>> {
        let events = self.store.read().await;
        
        if let Some(filter) = filter {
            Ok(events
                .iter()
                .filter(|event| filter.matches(event))
                .cloned()
                .collect())
        } else {
            Ok(events.to_vec())
        }
    }

    pub async fn get_event_summary(&self) -> Result<AuditSummary, Box<dyn std::error::Error>> {
        let events = self.store.read().await;
        let mut summary = AuditSummary::default();

        for event in events.iter() {
            match event.status {
                AuditEventStatus::Success => summary.success_count += 1,
                AuditEventStatus::Failure => summary.failure_count += 1,
                AuditEventStatus::Rejected => summary.rejection_count += 1,
                AuditEventStatus::Pending => summary.pending_count += 1,
            }

            match event.event_type {
                AuditEventType::Governance => summary.governance_events += 1,
                AuditEventType::Security => summary.security_events += 1,
                AuditEventType::Financial => summary.financial_events += 1,
                AuditEventType::System => summary.system_events += 1,
                AuditEventType::CrossChain => summary.cross_chain_events += 1,
            }
        }

        Ok(summary)
    }
}

#[derive(Debug, Default)]
pub struct AuditSummary {
    pub success_count: u64,
    pub failure_count: u64,
    pub rejection_count: u64,
    pub pending_count: u64,
    pub governance_events: u64,
    pub security_events: u64,
    pub financial_events: u64,
    pub system_events: u64,
    pub cross_chain_events: u64,
}

pub struct AuditEventFilter {
    pub event_type: Option<AuditEventType>,
    pub actor: Option<Did>,
    pub status: Option<AuditEventStatus>,
    pub from_timestamp: Option<DateTime<Utc>>,
    pub to_timestamp: Option<DateTime<Utc>>,
}

impl AuditEventFilter {
    fn matches(&self, event: &AuditEvent) -> bool {
        if let Some(event_type) = &self.event_type {
            if !std::mem::discriminant(event_type).eq(&std::mem::discriminant(&event.event_type)) {
                return false;
            }
        }

        if let Some(actor) = &self.actor {
            if actor != &event.actor {
                return false;
            }
        }

        if let Some(status) = &self.status {
            if !std::mem::discriminant(status).eq(&std::mem::discriminant(&event.status)) {
                return false;
            }
        }

        if let Some(from) = self.from_timestamp {
            if event.timestamp < from {
                return false;
            }
        }

        if let Some(to) = self.to_timestamp {
            if event.timestamp > to {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_audit_logging() {
        let protocol = web5::protocol::Protocol::default();
        let logger = AuditLogger::new(protocol).await;

        // Log a test event
        logger.log_event(
            AuditEventType::Governance,
            Did::new("did:example:123"),
            "create_proposal".to_string(),
            serde_json::json!({"proposal_id": "123"}),
            AuditEventStatus::Success,
            AuditMetadata {
                ip_address: Some("127.0.0.1".to_string()),
                user_agent: None,
                session_id: None,
                chain_id: Some("1".to_string()),
                block_number: Some(12345),
                transaction_hash: Some("0x123...".to_string()),
            },
        ).await.unwrap();

        // Query events
        let events = logger.query_events(None).await.unwrap();
        assert_eq!(events.len(), 1);

        // Get summary
        let summary = logger.get_event_summary().await.unwrap();
        assert_eq!(summary.success_count, 1);
        assert_eq!(summary.governance_events, 1);
    }
}
