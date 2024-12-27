use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use serde::{Serialize, Deserialize};

use crate::service::{Service, GenericService};
use crate::metrics::{UnifiedMetrics, ComponentHealth};
use crate::security::{SecurityContext, SecurityManager};
use crate::validation::ValidationResult;
use crate::ml::repository::{MLModel, MLModelRepository, MLModelStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLRequest {
    pub model_id: Option<String>,
    pub input_data: Vec<f64>,
    pub parameters: Option<MLRequestParameters>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLRequestParameters {
    pub batch_size: Option<usize>,
    pub threshold: Option<f64>,
    pub max_iterations: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLResponse {
    pub model_id: String,
    pub predictions: Vec<f64>,
    pub confidence: f64,
    pub processing_time: f64,
    pub model_version: String,
}

pub struct MLService {
    repository: Arc<MLModelRepository>,
    metrics: Arc<RwLock<UnifiedMetrics>>,
    security: Arc<dyn SecurityManager>,
    model_executor: Arc<dyn ModelExecutor>,
}

#[async_trait]
impl Service for MLService {
    type Item = MLRequest;
    type Response = MLResponse;
    type Error = MLError;

    async fn process(&self, context: &SecurityContext, request: Self::Item) -> Result<Self::Response, Self::Error> {
        let start_time = Utc::now();

        // Validate security context
        self.security.validate_context(context).await?;

        // Get model (either specified or best available)
        let model = match request.model_id {
            Some(id) => self.repository.read(&id).await?.ok_or(MLError::ModelNotFound)?,
            None => self.repository.get_best_model().await?.ok_or(MLError::NoModelsAvailable)?,
        };

        // Check model status
        if model.status != MLModelStatus::Ready {
            return Err(MLError::ModelNotReady(model.id));
        }

        // Execute model
        let execution_result = self.model_executor.execute(
            &model,
            &request.input_data,
            request.parameters,
        ).await?;

        // Calculate processing time
        let processing_time = Utc::now().signed_duration_since(start_time).num_milliseconds() as f64;

        // Update metrics
        let mut metrics = self.metrics.write().await;
        metrics.ml.as_mut().map(|ml| {
            ml.inference_time = execution_result.inference_time;
            ml.model_accuracy = execution_result.confidence;
        });

        Ok(MLResponse {
            model_id: model.id,
            predictions: execution_result.predictions,
            confidence: execution_result.confidence,
            processing_time,
            model_version: model.version,
        })
    }

    async fn validate(&self, request: &Self::Item) -> Result<ValidationResult, Self::Error> {
        // Validate input data
        if request.input_data.is_empty() {
            return Ok(ValidationResult::Invalid("Input data cannot be empty".to_string()));
        }

        // Validate parameters if present
        if let Some(params) = &request.parameters {
            if let Some(batch_size) = params.batch_size {
                if batch_size == 0 {
                    return Ok(ValidationResult::Invalid("Batch size must be greater than 0".to_string()));
                }
            }

            if let Some(threshold) = params.threshold {
                if threshold < 0.0 || threshold > 1.0 {
                    return Ok(ValidationResult::Invalid("Threshold must be between 0 and 1".to_string()));
                }
            }

            if let Some(max_iterations) = params.max_iterations {
                if max_iterations == 0 {
                    return Ok(ValidationResult::Invalid("Max iterations must be greater than 0".to_string()));
                }
            }
        }

        Ok(ValidationResult::Valid)
    }

    async fn get_health(&self) -> Result<ComponentHealth, Self::Error> {
        // Get all models
        let models = self.repository.list().await?;
        
        // Calculate health metrics
        let ready_models = models.iter()
            .filter(|m| m.status == MLModelStatus::Ready)
            .count();
        
        let failed_models = models.iter()
            .filter(|m| m.status == MLModelStatus::Failed)
            .count();
            
        let avg_accuracy = models.iter()
            .filter(|m| m.status == MLModelStatus::Ready)
            .map(|m| m.metrics.accuracy)
            .sum::<f64>() / ready_models as f64;

        Ok(ComponentHealth {
            operational: ready_models > 0,
            health_score: if ready_models > 0 { avg_accuracy * 100.0 } else { 0.0 },
            last_incident: if failed_models > 0 {
                models.iter()
                    .filter(|m| m.status == MLModelStatus::Failed)
                    .map(|m| m.updated_at)
                    .max()
            } else {
                None
            },
            error_count: failed_models,
            warning_count: 0,
        })
    }
}

#[async_trait]
pub trait ModelExecutor: Send + Sync {
    async fn execute(
        &self,
        model: &MLModel,
        input_data: &[f64],
        parameters: Option<MLRequestParameters>,
    ) -> Result<ModelExecutionResult, MLError>;
}

#[derive(Debug)]
pub struct ModelExecutionResult {
    pub predictions: Vec<f64>,
    pub confidence: f64,
    pub inference_time: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockModelExecutor;

    #[async_trait]
    impl ModelExecutor for MockModelExecutor {
        async fn execute(
            &self,
            _model: &MLModel,
            input_data: &[f64],
            _parameters: Option<MLRequestParameters>,
        ) -> Result<ModelExecutionResult, MLError> {
            Ok(ModelExecutionResult {
                predictions: input_data.iter().map(|x| x * 2.0).collect(),
                confidence: 0.95,
                inference_time: 10.0,
            })
        }
    }

    #[tokio::test]
    async fn test_ml_service() {
        let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
        let repository = Arc::new(MLModelRepository::new(metrics.clone()));
        let security = Arc::new(MockSecurityManager);
        let model_executor = Arc::new(MockModelExecutor);

        let service = MLService {
            repository,
            metrics,
            security,
            model_executor,
        };

        // Create test model
        let model = MLModel {
            id: "test-1".to_string(),
            name: "Test Model".to_string(),
            version: "1.0.0".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metrics: MLModelMetrics::default(),
            parameters: MLModelParameters::default(),
            status: MLModelStatus::Ready,
        };

        service.repository.create(model).await.unwrap();

        // Test process
        let request = MLRequest {
            model_id: Some("test-1".to_string()),
            input_data: vec![1.0, 2.0, 3.0],
            parameters: None,
        };

        let context = SecurityContext::default();
        let response = service.process(&context, request).await.unwrap();

        assert_eq!(response.predictions, vec![2.0, 4.0, 6.0]);
        assert_eq!(response.confidence, 0.95);
        assert!(response.processing_time > 0.0);
    }
}
