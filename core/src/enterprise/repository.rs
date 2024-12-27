use std::sync::Arc;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::repository::{Repository, GenericRepository};
use crate::metrics::{UnifiedMetrics, ComponentHealth};
use crate::validation::{ValidationResult, Validator};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseOperation {
    pub id: String,
    pub operation_type: OperationType,
    pub status: OperationStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: OperationMetadata,
    pub validation_result: Option<ValidationResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    AtomicSwap,
    LiquidTransfer,
    StateChainTransfer,
    DLCContract,
    MultiPartyComputation,
    PortfolioRebalancing,
    ComplianceCheck,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationStatus {
    Initiated,
    InProgress,
    Completed,
    Failed,
    Cancelled,
    RequiresApproval,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationMetadata {
    pub user_id: String,
    pub organization_id: String,
    pub operation_details: OperationDetails,
    pub compliance_info: ComplianceInfo,
    pub risk_assessment: RiskAssessment,
    pub audit_trail: Vec<AuditEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationDetails {
    pub transaction_ids: Vec<String>,
    pub asset_ids: Vec<String>,
    pub amount: Option<u64>,
    pub counterparties: Vec<String>,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceInfo {
    pub compliance_level: ComplianceLevel,
    pub kyc_status: KYCStatus,
    pub risk_score: f64,
    pub jurisdiction: String,
    pub restrictions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceLevel {
    Basic,
    Enhanced,
    Premium,
    Institutional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KYCStatus {
    NotStarted,
    InProgress,
    Approved,
    Rejected,
    RequiresUpdate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub risk_level: RiskLevel,
    pub risk_factors: Vec<RiskFactor>,
    pub mitigation_measures: Vec<String>,
    pub approval_chain: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub factor_type: String,
    pub severity: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: DateTime<Utc>,
    pub action: String,
    pub actor: String,
    pub details: String,
}

pub struct EnterpriseOperationValidator;

#[async_trait]
impl Validator<EnterpriseOperation> for EnterpriseOperationValidator {
    async fn validate(&self, operation: &EnterpriseOperation) -> Result<ValidationResult, ValidationError> {
        // Validate timestamps
        if operation.created_at > Utc::now() || operation.updated_at > Utc::now() {
            return Ok(ValidationResult::Invalid("Operation timestamps cannot be in the future".to_string()));
        }

        // Validate metadata
        if operation.metadata.user_id.is_empty() {
            return Ok(ValidationResult::Invalid("User ID cannot be empty".to_string()));
        }

        if operation.metadata.organization_id.is_empty() {
            return Ok(ValidationResult::Invalid("Organization ID cannot be empty".to_string()));
        }

        // Validate compliance info
        if operation.metadata.compliance_info.risk_score < 0.0 || 
           operation.metadata.compliance_info.risk_score > 1.0 {
            return Ok(ValidationResult::Invalid("Risk score must be between 0 and 1".to_string()));
        }

        // Validate operation details
        match operation.operation_type {
            OperationType::AtomicSwap | OperationType::LiquidTransfer => {
                if operation.metadata.operation_details.transaction_ids.is_empty() {
                    return Ok(ValidationResult::Invalid("Transaction IDs cannot be empty for transfer operations".to_string()));
                }
            },
            OperationType::DLCContract => {
                if operation.metadata.operation_details.counterparties.is_empty() {
                    return Ok(ValidationResult::Invalid("Counterparties cannot be empty for DLC contracts".to_string()));
                }
            },
            _ => {}
        }

        Ok(ValidationResult::Valid)
    }
}

pub type EnterpriseOperationRepository = GenericRepository<EnterpriseOperation, EnterpriseError>;

impl EnterpriseOperationRepository {
    pub fn new(metrics: Arc<RwLock<UnifiedMetrics>>) -> Self {
        Self::new(
            metrics,
            Arc::new(EnterpriseOperationValidator),
        )
    }

    pub async fn get_operations_by_status(&self, status: OperationStatus) -> Result<Vec<EnterpriseOperation>, EnterpriseError> {
        let ops = self.list().await?;
        Ok(ops.into_iter()
            .filter(|op| op.status == status)
            .collect())
    }

    pub async fn get_operations_by_type(&self, op_type: OperationType) -> Result<Vec<EnterpriseOperation>, EnterpriseError> {
        let ops = self.list().await?;
        Ok(ops.into_iter()
            .filter(|op| op.operation_type == op_type)
            .collect())
    }

    pub async fn get_operations_by_organization(&self, org_id: &str) -> Result<Vec<EnterpriseOperation>, EnterpriseError> {
        let ops = self.list().await?;
        Ok(ops.into_iter()
            .filter(|op| op.metadata.organization_id == org_id)
            .collect())
    }

    pub async fn get_high_risk_operations(&self) -> Result<Vec<EnterpriseOperation>, EnterpriseError> {
        let ops = self.list().await?;
        Ok(ops.into_iter()
            .filter(|op| matches!(op.metadata.compliance_info.risk_score >= 0.7))
            .collect())
    }

    pub async fn cleanup_stale_operations(&self) -> Result<usize, EnterpriseError> {
        let mut count = 0;
        let ops = self.list().await?;
        let now = Utc::now();
        
        for op in ops {
            if op.status == OperationStatus::InProgress {
                let age = now.signed_duration_since(op.updated_at).num_hours();
                if age > 24 { // Mark as failed after 24 hours
                    let mut failed_op = op.clone();
                    failed_op.status = OperationStatus::Failed;
                    self.update(&op.id, failed_op).await?;
                    count += 1;
                }
            }
        }
        
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_enterprise_operation_repository() {
        let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
        let repo = EnterpriseOperationRepository::new(metrics);

        // Create test operation
        let operation = EnterpriseOperation {
            id: "test-1".to_string(),
            operation_type: OperationType::AtomicSwap,
            status: OperationStatus::Initiated,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: OperationMetadata {
                user_id: "user-1".to_string(),
                organization_id: "org-1".to_string(),
                operation_details: OperationDetails {
                    transaction_ids: vec!["tx-1".to_string()],
                    asset_ids: vec!["asset-1".to_string()],
                    amount: Some(100000),
                    counterparties: vec!["party-1".to_string()],
                    parameters: serde_json::json!({}),
                },
                compliance_info: ComplianceInfo {
                    compliance_level: ComplianceLevel::Enhanced,
                    kyc_status: KYCStatus::Approved,
                    risk_score: 0.3,
                    jurisdiction: "US".to_string(),
                    restrictions: vec![],
                },
                risk_assessment: RiskAssessment {
                    risk_level: RiskLevel::Low,
                    risk_factors: vec![],
                    mitigation_measures: vec![],
                    approval_chain: vec![],
                },
                audit_trail: vec![],
            },
            validation_result: None,
        };

        // Test create
        let created = repo.create(operation.clone()).await.unwrap();
        assert_eq!(created.id, operation.id);

        // Test get by status
        let initiated = repo.get_operations_by_status(OperationStatus::Initiated).await.unwrap();
        assert_eq!(initiated.len(), 1);
        assert_eq!(initiated[0].id, operation.id);

        // Test get by type
        let swaps = repo.get_operations_by_type(OperationType::AtomicSwap).await.unwrap();
        assert_eq!(swaps.len(), 1);
        assert_eq!(swaps[0].id, operation.id);

        // Test get by organization
        let org_ops = repo.get_operations_by_organization("org-1").await.unwrap();
        assert_eq!(org_ops.len(), 1);
        assert_eq!(org_ops[0].id, operation.id);

        // Test get high risk
        let high_risk = repo.get_high_risk_operations().await.unwrap();
        assert_eq!(high_risk.len(), 0); // Our test operation has low risk

        // Test cleanup stale
        let stale = repo.cleanup_stale_operations().await.unwrap();
        assert_eq!(stale, 0); // Our test operation is not stale
    }
}
