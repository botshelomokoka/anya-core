use std::sync::Arc;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

use crate::handler::{Handler, GenericHandler};
use crate::metrics::{UnifiedMetrics, ComponentHealth};
use crate::security::{SecurityContext, SecurityManager};
use crate::validation::ValidationResult;
use crate::protocol::service::{
    ProtocolService, ProtocolRequest, ProtocolResponse,
    ProtocolOperation, ProtocolParameters
};
use crate::protocol::repository::{
    TransactionInput, TransactionOutput,
    ProtocolType, TransactionStatus
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRequest {
    pub operation: TransactionOperationType,
    pub protocol: ProtocolType,
    pub transaction_details: TransactionDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionOperationType {
    Create,
    Sign,
    Broadcast,
    Validate,
    Query,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionDetails {
    pub inputs: Vec<InputDetails>,
    pub outputs: Vec<OutputDetails>,
    pub transaction_options: TransactionOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputDetails {
    pub previous_tx: String,
    pub output_index: u32,
    pub sequence: Option<u32>,
    pub witness_script: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputDetails {
    pub amount: u64,
    pub address: String,
    pub script_type: Option<ScriptType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionOptions {
    pub fee_rate: Option<u64>,
    pub locktime: Option<u32>,
    pub replace_by_fee: Option<bool>,
    pub custom_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScriptType {
    P2PKH,
    P2SH,
    P2WPKH,
    P2WSH,
    P2TR,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub success: bool,
    pub transaction_info: TransactionInfo,
    pub execution_details: ExecutionDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInfo {
    pub transaction_id: Option<String>,
    pub status: TransactionStatus,
    pub confirmations: Option<u32>,
    pub block_height: Option<u32>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionDetails {
    pub message: String,
    pub warnings: Vec<String>,
    pub fee_paid: Option<u64>,
    pub execution_time: f64,
}

pub struct ProtocolHandler {
    inner: GenericHandler<ProtocolRequest, ProtocolResponse, ProtocolError>,
}

impl ProtocolHandler {
    pub fn new(
        service: Arc<ProtocolService>,
        metrics: Arc<RwLock<UnifiedMetrics>>,
        security: Arc<dyn SecurityManager>,
    ) -> Self {
        Self {
            inner: GenericHandler::new(service, metrics, security),
        }
    }

    fn map_operation_type(op_type: TransactionOperationType) -> ProtocolOperation {
        match op_type {
            TransactionOperationType::Create => ProtocolOperation::CreateTransaction,
            TransactionOperationType::Sign => ProtocolOperation::SignTransaction,
            TransactionOperationType::Broadcast => ProtocolOperation::BroadcastTransaction,
            TransactionOperationType::Validate => ProtocolOperation::ValidateTransaction,
            TransactionOperationType::Query => ProtocolOperation::QueryTransaction,
        }
    }

    fn convert_input_details(input: &InputDetails) -> TransactionInput {
        TransactionInput {
            txid: input.previous_tx.clone(),
            vout: input.output_index,
            sequence: input.sequence.unwrap_or(0xffffffff),
            witness: input.witness_script.clone().map(|s| vec![s]),
        }
    }

    fn convert_output_details(output: &OutputDetails) -> TransactionOutput {
        TransactionOutput {
            value: output.amount,
            script_pubkey: "".to_string(), // Would be generated based on address and script_type
            address: Some(output.address.clone()),
        }
    }
}

#[async_trait]
impl Handler for ProtocolHandler {
    type Request = TransactionRequest;
    type Response = TransactionResponse;
    type Error = ProtocolError;

    async fn handle(&self, context: &SecurityContext, request: Self::Request) -> Result<Self::Response, Self::Error> {
        let start_time = Utc::now();

        // Convert transaction request to protocol request
        let protocol_request = ProtocolRequest {
            operation: Self::map_operation_type(request.operation),
            protocol_type: request.protocol,
            parameters: ProtocolParameters {
                inputs: request.transaction_details.inputs.iter()
                    .map(|i| Self::convert_input_details(i))
                    .collect(),
                outputs: request.transaction_details.outputs.iter()
                    .map(|o| Self::convert_output_details(o))
                    .collect(),
                fee_rate: request.transaction_details.transaction_options.fee_rate,
                locktime: request.transaction_details.transaction_options.locktime,
                rbf: request.transaction_details.transaction_options.replace_by_fee,
                additional_data: request.transaction_details.transaction_options.custom_data,
            },
        };

        // Process through inner handler
        let response = self.inner.handle(context, protocol_request).await?;

        // Calculate execution time
        let execution_time = Utc::now()
            .signed_duration_since(start_time)
            .num_milliseconds() as f64;

        // Convert protocol response to transaction response
        Ok(TransactionResponse {
            success: response.success,
            transaction_info: TransactionInfo {
                transaction_id: response.transaction_id,
                status: response.status,
                confirmations: response.details.confirmations,
                block_height: response.details.block_height,
                timestamp: response.details.timestamp,
            },
            execution_details: ExecutionDetails {
                message: response.details.message,
                warnings: vec![],
                fee_paid: response.metadata.and_then(|m| m.fee_rate),
                execution_time,
            },
        })
    }

    async fn validate(&self, request: &Self::Request) -> Result<ValidationResult, Self::Error> {
        // Validate inputs
        if request.transaction_details.inputs.is_empty() {
            return Ok(ValidationResult::Invalid("Transaction must have at least one input".to_string()));
        }

        for input in &request.transaction_details.inputs {
            if input.previous_tx.len() != 64 || !input.previous_tx.chars().all(|c| c.is_ascii_hexdigit()) {
                return Ok(ValidationResult::Invalid("Invalid transaction ID format".to_string()));
            }
        }

        // Validate outputs
        if request.transaction_details.outputs.is_empty() {
            return Ok(ValidationResult::Invalid("Transaction must have at least one output".to_string()));
        }

        for output in &request.transaction_details.outputs {
            if output.amount == 0 {
                return Ok(ValidationResult::Invalid("Output amount cannot be zero".to_string()));
            }
            if output.address.is_empty() {
                return Ok(ValidationResult::Invalid("Output address cannot be empty".to_string()));
            }
        }

        // Validate options
        if let Some(fee_rate) = request.transaction_details.transaction_options.fee_rate {
            if fee_rate == 0 {
                return Ok(ValidationResult::Invalid("Fee rate cannot be zero".to_string()));
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
    async fn test_protocol_handler() {
        let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
        let service = Arc::new(MockProtocolService);
        let security = Arc::new(MockSecurityManager);

        let handler = ProtocolHandler::new(service, metrics, security);

        // Test transaction request
        let request = TransactionRequest {
            operation: TransactionOperationType::Create,
            protocol: ProtocolType::Bitcoin,
            transaction_details: TransactionDetails {
                inputs: vec![InputDetails {
                    previous_tx: "a".repeat(64),
                    output_index: 0,
                    sequence: None,
                    witness_script: None,
                }],
                outputs: vec![OutputDetails {
                    amount: 100000,
                    address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
                    script_type: Some(ScriptType::P2PKH),
                }],
                transaction_options: TransactionOptions {
                    fee_rate: Some(5),
                    locktime: None,
                    replace_by_fee: None,
                    custom_data: None,
                },
            },
        };

        let context = SecurityContext::default();
        let response = handler.handle(&context, request).await.unwrap();

        assert!(response.success);
        assert!(response.transaction_info.transaction_id.is_some());
        assert!(response.execution_details.execution_time > 0.0);

        // Test health check
        let health = handler.get_health().await.unwrap();
        assert!(health.operational);
        assert_eq!(health.health_score, 100.0);
    }
}
