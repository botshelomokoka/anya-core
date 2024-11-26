use crate::ml_core::{MLCore, MLInput, MLOutput};
use crate::blockchain::BlockchainInterface;
use crate::dowe::DoweOracle;
use crate::privacy::zksnarks::ZKSnarkSystem;
use crate::metrics::{counter, gauge};
use thiserror::Error;
use log::{info, warn, error};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

#[derive(Error, Debug)]
pub enum UnifiedPipelineError {
    #[error("Data ingestion error: {0}")]
    IngestionError(String),
    #[error("ML processing error: {0}")]
    MLError(String),
    #[error("Business logic error: {0}")]
    BusinessError(String),
    #[error("API error: {0}")]
    APIError(String),
}

pub struct UnifiedDataPipeline {
    ml_core: Arc<Mutex<MLCore>>,
    blockchain: Arc<BlockchainInterface>,
    dowe_oracle: Arc<DoweOracle>,
    zk_system: Arc<ZKSnarkSystem>,
    business_logic: Arc<BusinessLogicProcessor>,
    api_gateway: Arc<APIGateway>,
    data_tx: mpsc::Sender<DataPacket>,
    data_rx: mpsc::Receiver<DataPacket>,
    metrics: PipelineMetrics,
}

#[derive(Debug, Clone)]
pub struct DataPacket {
    source: DataSource,
    data: Vec<u8>,
    metadata: DataMetadata,
    business_rules: Vec<BusinessRule>,
    ml_config: MLConfig,
}

#[derive(Debug, Clone)]
pub struct BusinessRule {
    rule_type: RuleType,
    parameters: serde_json::Value,
    priority: Priority,
}

#[derive(Debug, Clone)]
pub struct MLConfig {
    model_type: ModelType,
    training_params: TrainingParameters,
    inference_params: InferenceParameters,
}

impl UnifiedDataPipeline {
    pub fn new(
        ml_core: Arc<Mutex<MLCore>>,
        blockchain: Arc<BlockchainInterface>,
        dowe_oracle: Arc<DoweOracle>,
        zk_system: Arc<ZKSnarkSystem>,
        business_logic: Arc<BusinessLogicProcessor>,
        api_gateway: Arc<APIGateway>,
    ) -> Self {
        let (data_tx, data_rx) = mpsc::channel(1000);
        
        Self {
            ml_core,
            blockchain,
            dowe_oracle,
            zk_system,
            business_logic,
            api_gateway,
            data_tx,
            data_rx,
            metrics: PipelineMetrics::new(),
        }
    }

    pub async fn process_data_packet(&self, packet: DataPacket) -> Result<ProcessedOutput, UnifiedPipelineError> {
        // Apply business rules
        let processed_data = self.business_logic.apply_rules(&packet)
            .await
            .map_err(|e| UnifiedPipelineError::BusinessError(e.to_string()))?;

        // Process with ML
        let ml_result = self.process_with_ml(&processed_data, &packet.ml_config)
            .await
            .map_err(|e| UnifiedPipelineError::MLError(e.to_string()))?;

        // Create API response
        let api_response = self.api_gateway.create_response(&ml_result)
            .await
            .map_err(|e| UnifiedPipelineError::APIError(e.to_string()))?;

        // Update metrics
        self.metrics.record_successful_processing();

        Ok(ProcessedOutput {
            ml_result,
            api_response,
            metadata: packet.metadata,
        })
    }

    async fn process_with_ml(&self, data: &ProcessedData, config: &MLConfig) -> Result<MLOutput, UnifiedPipelineError> {
        let ml_input = self.prepare_ml_input(data, config)?;
        
        let mut ml_core = self.ml_core.lock().await;
        let output = ml_core.process(&ml_input)
            .map_err(|e| UnifiedPipelineError::MLError(e.to_string()))?;

        // Verify output with ZK proof
        self.verify_ml_output(&output)
            .await
            .map_err(|e| UnifiedPipelineError::MLError(e.to_string()))?;

        Ok(output)
    }

    async fn verify_ml_output(&self, output: &MLOutput) -> Result<(), UnifiedPipelineError> {
        let proof = self.zk_system.create_proof(&[
            &output.prediction.to_le_bytes(),
            &output.confidence.to_le_bytes(),
        ]).map_err(|e| UnifiedPipelineError::MLError(e.to_string()))?;

        if !self.zk_system.verify_proof(&proof, &[&output.prediction.to_le_bytes()])? {
            return Err(UnifiedPipelineError::MLError("Invalid ML output proof".into()));
        }

        Ok(())
    }
}

struct PipelineMetrics {
    successful_processing: Counter,
    failed_processing: Counter,
    ml_processing_time: Gauge,
    business_rules_applied: Counter,
}

impl PipelineMetrics {
    fn new() -> Self {
        Self {
            successful_processing: counter!("pipeline_successful_processing_total"),
            failed_processing: counter!("pipeline_failed_processing_total"),
            ml_processing_time: gauge!("pipeline_ml_processing_time_seconds"),
            business_rules_applied: counter!("pipeline_business_rules_applied_total"),
        }
    }

    fn record_successful_processing(&self) {
        self.successful_processing.increment(1);
    }

    fn record_failed_processing(&self) {
        self.failed_processing.increment(1);
    }

    fn record_ml_processing_time(&self, duration: f64) {
        self.ml_processing_time.set(duration);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_unified_pipeline() {
        let pipeline = setup_test_pipeline().await;
        
        let test_packet = create_test_packet();
        let result = pipeline.process_data_packet(test_packet).await;
        
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.ml_result.confidence > 0.5);
    }
}
