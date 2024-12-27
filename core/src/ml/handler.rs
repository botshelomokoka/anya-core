use std::sync::Arc;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

use crate::handler::{Handler, GenericHandler};
use crate::metrics::{UnifiedMetrics, ComponentHealth};
use crate::security::{SecurityContext, SecurityManager};
use crate::validation::ValidationResult;
use crate::ml::service::{MLService, MLRequest, MLResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLPredictionRequest {
    pub data: Vec<f64>,
    pub model_id: Option<String>,
    pub batch_size: Option<usize>,
    pub confidence_threshold: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLPredictionResponse {
    pub predictions: Vec<f64>,
    pub confidence: f64,
    pub model_info: MLModelInfo,
    pub execution_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLModelInfo {
    pub id: String,
    pub version: String,
    pub accuracy: f64,
}

pub struct MLHandler {
    inner: GenericHandler<MLRequest, MLResponse, MLError>,
}

impl MLHandler {
    pub fn new(
        service: Arc<MLService>,
        metrics: Arc<RwLock<UnifiedMetrics>>,
        security: Arc<dyn SecurityManager>,
    ) -> Self {
        Self {
            inner: GenericHandler::new(service, metrics, security),
        }
    }
}

#[async_trait]
impl Handler for MLHandler {
    type Request = MLPredictionRequest;
    type Response = MLPredictionResponse;
    type Error = MLError;

    async fn handle(&self, context: &SecurityContext, request: Self::Request) -> Result<Self::Response, Self::Error> {
        // Convert prediction request to ML request
        let ml_request = MLRequest {
            model_id: request.model_id,
            input_data: request.data,
            parameters: Some(MLRequestParameters {
                batch_size: request.batch_size,
                threshold: request.confidence_threshold,
                max_iterations: None,
            }),
        };

        // Process through inner handler
        let response = self.inner.handle(context, ml_request).await?;

        // Convert ML response to prediction response
        Ok(MLPredictionResponse {
            predictions: response.predictions,
            confidence: response.confidence,
            model_info: MLModelInfo {
                id: response.model_id,
                version: response.model_version,
                accuracy: response.confidence,
            },
            execution_time: response.processing_time,
        })
    }

    async fn validate(&self, request: &Self::Request) -> Result<ValidationResult, Self::Error> {
        // Validate input data
        if request.data.is_empty() {
            return Ok(ValidationResult::Invalid("Input data cannot be empty".to_string()));
        }

        // Validate confidence threshold if present
        if let Some(threshold) = request.confidence_threshold {
            if threshold < 0.0 || threshold > 1.0 {
                return Ok(ValidationResult::Invalid("Confidence threshold must be between 0 and 1".to_string()));
            }
        }

        // Validate batch size if present
        if let Some(batch_size) = request.batch_size {
            if batch_size == 0 {
                return Ok(ValidationResult::Invalid("Batch size must be greater than 0".to_string()));
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

    struct MockMLService;

    #[async_trait]
    impl Service for MockMLService {
        type Item = MLRequest;
        type Response = MLResponse;
        type Error = MLError;

        async fn process(&self, _context: &SecurityContext, request: Self::Item) -> Result<Self::Response, Self::Error> {
            Ok(MLResponse {
                model_id: "test-model".to_string(),
                predictions: request.input_data.iter().map(|x| x * 2.0).collect(),
                confidence: 0.95,
                processing_time: 10.0,
                model_version: "1.0.0".to_string(),
            })
        }

        async fn validate(&self, _request: &Self::Item) -> Result<ValidationResult, Self::Error> {
            Ok(ValidationResult::Valid)
        }

        async fn get_health(&self) -> Result<ComponentHealth, Self::Error> {
            Ok(ComponentHealth {
                operational: true,
                health_score: 100.0,
                last_incident: None,
                error_count: 0,
                warning_count: 0,
            })
        }
    }

    #[tokio::test]
    async fn test_ml_handler() {
        let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
        let service = Arc::new(MockMLService);
        let security = Arc::new(MockSecurityManager);

        let handler = MLHandler::new(service, metrics, security);

        // Test prediction request
        let request = MLPredictionRequest {
            data: vec![1.0, 2.0, 3.0],
            model_id: None,
            batch_size: Some(32),
            confidence_threshold: Some(0.8),
        };

        let context = SecurityContext::default();
        let response = handler.handle(&context, request).await.unwrap();

        assert_eq!(response.predictions, vec![2.0, 4.0, 6.0]);
        assert_eq!(response.confidence, 0.95);
        assert_eq!(response.model_info.version, "1.0.0");
        assert!(response.execution_time > 0.0);

        // Test health check
        let health = handler.get_health().await.unwrap();
        assert!(health.operational);
        assert_eq!(health.health_score, 100.0);
    }
}
