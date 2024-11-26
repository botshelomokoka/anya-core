use crate::privacy::zksnarks::ZKSnarkSystem;
use crate::blockchain::BlockchainInterface;
use crate::metrics::{counter, gauge};
use crate::security::enhanced_security::EnhancedSecurity;
use thiserror::Error;
use log::{info, warn, error};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Error, Debug)]
pub enum UnifiedSecurityError {
    #[error("Quantum resistance error: {0}")]
    QuantumResistanceError(String),
    #[error("Privacy violation: {0}")]
    PrivacyViolation(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
}

pub struct UnifiedSecuritySystem {
    enhanced_security: Arc<EnhancedSecurity>,
    blockchain: Arc<BlockchainInterface>,
    zk_system: Arc<ZKSnarkSystem>,
    metrics: SecurityMetrics,
}

impl UnifiedSecuritySystem {
    pub fn new(
        blockchain: Arc<BlockchainInterface>,
        zk_system: Arc<ZKSnarkSystem>,
    ) -> Result<Self, UnifiedSecurityError> {
        let enhanced_security = Arc::new(EnhancedSecurity::new(
            Arc::clone(&blockchain),
            Arc::clone(&zk_system),
        )?);

        Ok(Self {
            enhanced_security,
            blockchain,
            zk_system,
            metrics: SecurityMetrics::new(),
        })
    }

    pub async fn validate_transaction(&self, tx: &Transaction) -> Result<(), UnifiedSecurityError> {
        // Apply quantum resistance checks
        self.enhanced_security.validate_transaction(tx).await?;

        // Generate and verify ZK proof
        let proof = self.zk_system.create_proof(&[
            &tx.amount.to_le_bytes(),
            &tx.timestamp.timestamp().to_le_bytes(),
        ]).map_err(|e| UnifiedSecurityError::ValidationError(e.to_string()))?;

        if !self.zk_system.verify_proof(&proof, &[&tx.amount.to_le_bytes()])? {
            return Err(UnifiedSecurityError::ValidationError("Invalid ZK proof".into()));
        }

        // Check blockchain state
        self.validate_blockchain_state().await?;

        self.metrics.record_successful_validation();
        Ok(())
    }

    pub async fn audit_system(&self) -> Result<AuditReport, UnifiedSecurityError> {
        info!("Starting unified system security audit");
        
        let mut report = AuditReport::new();

        // Get enhanced security audit
        let enhanced_audit = self.enhanced_security.audit_system().await?;
        report.quantum_resistance_score = enhanced_audit.quantum_resistance_score;
        report.privacy_score = enhanced_audit.privacy_score;
        report.network_security_score = enhanced_audit.network_security_score;

        // Add additional security metrics
        report.zk_proof_success_rate = self.metrics.get_zk_proof_success_rate();
        report.validation_success_rate = self.metrics.get_validation_success_rate();

        self.metrics.record_audit_completion(&report);
        Ok(report)
    }

    async fn validate_blockchain_state(&self) -> Result<(), UnifiedSecurityError> {
        let network_state = self.blockchain.get_network_state().await
            .map_err(|e| UnifiedSecurityError::ValidationError(e.to_string()))?;

        if network_state.anomaly_score > 0.8 {
            warn!("High anomaly score detected in blockchain state: {}", network_state.anomaly_score);
            return Err(UnifiedSecurityError::ValidationError("Suspicious blockchain state".into()));
        }

        Ok(())
    }
}

struct SecurityMetrics {
    successful_validations: Counter,
    failed_validations: Counter,
    quantum_resistance_score: Gauge,
    privacy_score: Gauge,
    network_security_score: Gauge,
    zk_proof_success_rate: Gauge,
    validation_success_rate: Gauge,
}

impl SecurityMetrics {
    fn new() -> Self {
        Self {
            successful_validations: counter!("security_validations_successful_total"),
            failed_validations: counter!("security_validations_failed_total"),
            quantum_resistance_score: gauge!("security_quantum_resistance_score"),
            privacy_score: gauge!("security_privacy_score"),
            network_security_score: gauge!("security_network_score"),
            zk_proof_success_rate: gauge!("security_zk_proof_success_rate"),
            validation_success_rate: gauge!("security_validation_success_rate"),
        }
    }

    fn record_successful_validation(&self) {
        self.successful_validations.increment(1);
    }

    fn record_failed_validation(&self) {
        self.failed_validations.increment(1);
    }

    fn record_audit_completion(&self, report: &AuditReport) {
        self.quantum_resistance_score.set(report.quantum_resistance_score);
        self.privacy_score.set(report.privacy_score);
        self.network_security_score.set(report.network_security_score);
        self.zk_proof_success_rate.set(report.zk_proof_success_rate);
        self.validation_success_rate.set(report.validation_success_rate);
    }

    fn get_zk_proof_success_rate(&self) -> f64 {
        let total = self.successful_validations.get() + self.failed_validations.get();
        if total > 0 {
            self.successful_validations.get() as f64 / total as f64
        } else {
            0.0
        }
    }

    fn get_validation_success_rate(&self) -> f64 {
        let total = self.successful_validations.get() + self.failed_validations.get();
        if total > 0 {
            self.successful_validations.get() as f64 / total as f64
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_unified_security_system() {
        let blockchain = Arc::new(BlockchainInterface::new());
        let zk_system = Arc::new(ZKSnarkSystem::new().unwrap());
        let security = UnifiedSecuritySystem::new(blockchain, zk_system).unwrap();

        let tx = Transaction {
            amount: 100.0,
            timestamp: chrono::Utc::now(),
        };

        let result = security.validate_transaction(&tx).await;
        assert!(result.is_ok());

        let audit_report = security.audit_system().await.unwrap();
        assert!(audit_report.quantum_resistance_score >= 0.0);
        assert!(audit_report.quantum_resistance_score <= 1.0);
    }
}
