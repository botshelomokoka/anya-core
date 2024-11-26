use crate::dowe::{DoweOracle, OracleData};
use crate::privacy::zksnarks::ZKSnarkSystem;
use crate::blockchain::BlockchainInterface;
use thiserror::Error;
use log::{info, warn, error};
use metrics::{counter, gauge};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Error, Debug)]
pub enum ConsensusError {
    #[error("Consensus failed: {0}")]
    ConsensusFailed(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Threshold not met: {0}")]
    ThresholdError(String),
}

pub struct DoweConsensus {
    oracles: Vec<Arc<DoweOracle>>,
    zk_system: Arc<ZKSnarkSystem>,
    blockchain: Arc<BlockchainInterface>,
    consensus_threshold: f64,
    metrics: ConsensusMetrics,
}

impl DoweConsensus {
    pub fn new(
        oracles: Vec<Arc<DoweOracle>>,
        zk_system: Arc<ZKSnarkSystem>,
        blockchain: Arc<BlockchainInterface>,
    ) -> Self {
        Self {
            oracles,
            zk_system,
            blockchain,
            consensus_threshold: 0.8,
            metrics: ConsensusMetrics::new(),
        }
    }

    pub async fn reach_consensus(&self, data: &[OracleData]) -> Result<OracleData, ConsensusError> {
        // Verify all data proofs
        for oracle_data in data {
            self.verify_data_proof(oracle_data).await?;
        }

        // Calculate consensus value
        let consensus_value = self.calculate_consensus_value(data)?;

        // Check if consensus meets threshold
        if self.check_consensus_threshold(&consensus_value, data).await? {
            // Create proof of consensus
            let consensus_proof = self.create_consensus_proof(&consensus_value).await?;

            // Submit consensus to blockchain
            self.submit_consensus(&consensus_value, &consensus_proof).await?;

            self.metrics.record_successful_consensus();
            Ok(consensus_value)
        } else {
            self.metrics.record_failed_consensus();
            Err(ConsensusError::ThresholdError("Consensus threshold not met".into()))
        }
    }

    async fn verify_data_proof(&self, data: &OracleData) -> Result<(), ConsensusError> {
        let proof_valid = self.zk_system.verify_proof(
            &data.proof,
            &[&data.value.to_string().as_bytes()],
        ).map_err(|e| ConsensusError::ValidationError(e.to_string()))?;

        if !proof_valid {
            return Err(ConsensusError::ValidationError("Invalid ZK proof".into()));
        }

        Ok(())
    }

    fn calculate_consensus_value(&self, data: &[OracleData]) -> Result<OracleData, ConsensusError> {
        // Implement consensus calculation logic (e.g., weighted average, median, etc.)
        // This is a simplified example
        if data.is_empty() {
            return Err(ConsensusError::ConsensusFailed("No data provided".into()));
        }

        // For this example, we'll take the median value
        let mut values: Vec<_> = data.to_vec();
        values.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());
        let median = values[values.len() / 2].clone();

        Ok(median)
    }

    async fn check_consensus_threshold(&self, consensus: &OracleData, data: &[OracleData]) -> Result<bool, ConsensusError> {
        let mut agreement_count = 0;
        let tolerance = 0.01; // 1% tolerance

        for oracle_data in data {
            if (oracle_data.value.as_f64().unwrap() - consensus.value.as_f64().unwrap()).abs() <= tolerance {
                agreement_count += 1;
            }
        }

        let agreement_ratio = agreement_count as f64 / data.len() as f64;
        Ok(agreement_ratio >= self.consensus_threshold)
    }

    async fn create_consensus_proof(&self, consensus: &OracleData) -> Result<Vec<u8>, ConsensusError> {
        self.zk_system.create_proof(&[
            consensus.value.to_string().as_bytes(),
            &consensus.timestamp.timestamp().to_le_bytes(),
        ]).map_err(|e| ConsensusError::ValidationError(e.to_string()))
    }

    async fn submit_consensus(&self, consensus: &OracleData, proof: &[u8]) -> Result<(), ConsensusError> {
        self.blockchain.submit_oracle_data(consensus, proof)
            .await
            .map_err(|e| ConsensusError::ConsensusFailed(e.to_string()))
    }
}

struct ConsensusMetrics {
    successful_consensus: Counter,
    failed_consensus: Counter,
    consensus_latency: Histogram,
    oracle_participation: Gauge,
}

impl ConsensusMetrics {
    fn new() -> Self {
        Self {
            successful_consensus: counter!("dowe_consensus_successful_total"),
            failed_consensus: counter!("dowe_consensus_failed_total"),
            consensus_latency: histogram!("dowe_consensus_latency_seconds"),
            oracle_participation: gauge!("dowe_oracle_participation_ratio"),
        }
    }

    fn record_successful_consensus(&self) {
        self.successful_consensus.increment(1);
    }

    fn record_failed_consensus(&self) {
        self.failed_consensus.increment(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_consensus_mechanism() {
        let oracles = vec![
            Arc::new(DoweOracle::new(
                Arc::new(BlockchainInterface::new()),
                Arc::new(ZKSnarkSystem::new().unwrap()),
            )),
        ];

        let consensus = DoweConsensus::new(
            oracles,
            Arc::new(ZKSnarkSystem::new().unwrap()),
            Arc::new(BlockchainInterface::new()),
        );

        let test_data = vec![
            OracleData {
                source: "test".into(),
                timestamp: Utc::now(),
                value: serde_json::json!(100),
                signature: vec![],
                proof: vec![],
            },
        ];

        let result = consensus.reach_consensus(&test_data).await;
        assert!(result.is_ok());
    }
}
