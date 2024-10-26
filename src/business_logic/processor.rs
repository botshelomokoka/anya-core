use crate::ml_core::{MLCore, MLInput, MLOutput};
use crate::data_pipeline::DataPacket;
use crate::metrics::{counter, gauge};
use thiserror::Error;
use log::{info, warn, error};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Error, Debug)]
pub enum BusinessLogicError {
    #[error("Rule validation failed: {0}")]
    RuleValidationError(String),
    #[error("Processing error: {0}")]
    ProcessingError(String),
    #[error("ML integration error: {0}")]
    MLError(String),
}

pub struct BusinessLogicProcessor {
    ml_core: Arc<Mutex<MLCore>>,
    rules_engine: RulesEngine,
    metrics: BusinessMetrics,
    validation_layer: DataValidationLayer,
}

impl BusinessLogicProcessor {
    pub fn new(ml_core: Arc<Mutex<MLCore>>) -> Self {
        Self {
            ml_core,
            rules_engine: RulesEngine::new(),
            metrics: BusinessMetrics::new(),
            validation_layer: DataValidationLayer::new(),
        }
    }

    pub async fn process_packet(&self, packet: DataPacket) -> Result<ProcessedOutput, BusinessLogicError> {
        // Validate incoming data
        self.validation_layer.validate_packet(&packet)?;

        // Apply business rules
        let rule_result = self.rules_engine.apply_rules(&packet)?;

        // Process with ML if needed
        let ml_result = if rule_result.requires_ml {
            self.process_with_ml(&packet).await?
        } else {
            None
        };

        // Combine results
        let output = self.combine_results(rule_result, ml_result)?;

        self.metrics.record_successful_processing();
        Ok(output)
    }

    async fn process_with_ml(&self, packet: &DataPacket) -> Result<Option<MLOutput>, BusinessLogicError> {
        let ml_input = self.prepare_ml_input(packet)?;
        
        let mut ml_core = self.ml_core.lock().await;
        let output = ml_core.process(&ml_input)
            .map_err(|e| BusinessLogicError::MLError(e.to_string()))?;

        Ok(Some(output))
    }

    fn prepare_ml_input(&self, packet: &DataPacket) -> Result<MLInput, BusinessLogicError> {
        // Convert packet to ML input format
        Ok(MLInput {
            features: packet.data.clone(),
            timestamp: packet.metadata.timestamp,
            source: format!("{:?}", packet.source),
        })
    }

    fn combine_results(&self, rule_result: RuleResult, ml_result: Option<MLOutput>) -> Result<ProcessedOutput, BusinessLogicError> {
        Ok(ProcessedOutput {
            rule_result,
            ml_result,
            timestamp: chrono::Utc::now(),
        })
    }
}

struct RulesEngine {
    rules: Vec<Box<dyn BusinessRule>>,
}

impl RulesEngine {
    fn new() -> Self {
        Self {
            rules: Vec::new(),
        }
    }

    fn apply_rules(&self, packet: &DataPacket) -> Result<RuleResult, BusinessLogicError> {
        let mut requires_ml = false;
        let mut applied_rules = Vec::new();

        for rule in &self.rules {
            if rule.should_apply(packet) {
                let result = rule.apply(packet)?;
                requires_ml |= result.requires_ml;
                applied_rules.push(result);
            }
        }

        Ok(RuleResult {
            requires_ml,
            applied_rules,
        })
    }
}

#[async_trait]
trait BusinessRule: Send + Sync {
    fn should_apply(&self, packet: &DataPacket) -> bool;
    fn apply(&self, packet: &DataPacket) -> Result<RuleOutput, BusinessLogicError>;
}

struct RuleResult {
    requires_ml: bool,
    applied_rules: Vec<RuleOutput>,
}

struct RuleOutput {
    rule_id: String,
    result: serde_json::Value,
    requires_ml: bool,
}

struct ProcessedOutput {
    rule_result: RuleResult,
    ml_result: Option<MLOutput>,
    timestamp: chrono::DateTime<chrono::Utc>,
}

struct BusinessMetrics {
    successful_processing: Counter,
    failed_processing: Counter,
    ml_usage: Counter,
    rule_applications: Counter,
}

impl BusinessMetrics {
    fn new() -> Self {
        Self {
            successful_processing: counter!("business_processing_successful_total"),
            failed_processing: counter!("business_processing_failed_total"),
            ml_usage: counter!("business_ml_usage_total"),
            rule_applications: counter!("business_rule_applications_total"),
        }
    }

    fn record_successful_processing(&self) {
        self.successful_processing.increment(1);
    }

    fn record_failed_processing(&self) {
        self.failed_processing.increment(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_business_logic_processor() {
        let ml_core = Arc::new(Mutex::new(MLCore::new()));
        let processor = BusinessLogicProcessor::new(ml_core);

        let test_packet = DataPacket {
            source: DataSource::Blockchain,
            data: vec![1, 2, 3, 4],
            metadata: DataMetadata {
                timestamp: chrono::Utc::now(),
                priority: Priority::High,
                verification_level: VerificationLevel::Full,
            },
            privacy_proof: None,
        };

        let result = processor.process_packet(test_packet).await;
        assert!(result.is_ok());
    }
}
