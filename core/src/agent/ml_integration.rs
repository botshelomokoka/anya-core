//! ML Model Integration System
//! 
//! This module provides integration capabilities for ML models within the ML*/Agent system.
//! It handles model lifecycle management, inference coordination, training management,
//! and model performance monitoring.
//!
//! # Architecture
//!
//! The ML integration system consists of:
//! - MLIntegration: Core coordinator for ML operations
//! - ModelManager: Handles model lifecycle and versioning
//! - InferenceEngine: Manages model inference operations
//! - TrainingCoordinator: Coordinates model training
//!
//! # Features
//!
//! - Model lifecycle management
//! - Inference coordination
//! - Training management
//! - Performance monitoring
//! - Version control
//! - Resource optimization
//!
//! # Example
//!
//! ```rust
//! use anya::agent::ml_integration::{MLIntegration, ModelConfig};
//!
//! async fn setup_ml_integration() -> Result<(), AgentError> {
//!     let config = ModelConfig::new()
//!         .with_model_path("models/classifier")
//!         .with_batch_size(32)
//!         .with_device("cuda");
//!
//!     let ml = MLIntegration::new(config);
//!     ml.load_model().await?;
//!
//!     Ok(())
//! }
//! ```

use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::ml::{
    service::{MLService, MLRequest, MLResponse},
    repository::{MLModelRepository, MLModel, MLModelStatus},
    handler::{MLHandler, MLPredictionRequest},
};
use crate::metrics::UnifiedMetrics;
use crate::security::SecurityManager;
use crate::system::SystemComponent;
use super::{AgentError, ComponentMetrics};

/// ML model configuration.
///
/// Provides configuration options for:
/// - Model paths and versions
/// - Hardware acceleration
/// - Batch processing
/// - Resource limits
/// - Performance targets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Path to model files
    pub model_path: String,
    /// Model version identifier
    pub model_version: String,
    /// Device to run model on (cpu/cuda)
    pub device: String,
    /// Batch size for inference
    pub batch_size: usize,
    /// Maximum memory usage
    pub max_memory: usize,
    /// Target latency in milliseconds
    pub target_latency: f64,
}

/// ML integration system for ML*/Agent.
///
/// Coordinates all ML-related operations:
/// - Model loading and initialization
/// - Inference requests
/// - Training coordination
/// - Performance monitoring
/// - Resource management
pub struct MLIntegration {
    ml_service: Arc<MLService>,
    ml_handler: Arc<MLHandler>,
    model_repository: Arc<MLModelRepository>,
    metrics: Arc<RwLock<UnifiedMetrics>>,
    security: Arc<dyn SecurityManager>,
}

impl MLIntegration {
    /// Create new ML integration
    pub fn new(
        ml_service: Arc<MLService>,
        model_repository: Arc<MLModelRepository>,
        metrics: Arc<RwLock<UnifiedMetrics>>,
        security: Arc<dyn SecurityManager>,
    ) -> Self {
        let ml_handler = Arc::new(MLHandler::new(
            ml_service.clone(),
            metrics.clone(),
            security.clone(),
        ));

        Self {
            ml_service,
            ml_handler,
            model_repository,
            metrics,
            security,
        }
    }

    /// Analyze component readiness using ML
    pub async fn analyze_component_readiness(&self, component: &SystemComponent) -> Result<f64, AgentError> {
        // Prepare component data for analysis
        let input_data = self.prepare_component_data(component).await?;
        
        // Create prediction request
        let request = MLPredictionRequest {
            data: input_data,
            model_id: None, // Use best available model
            batch_size: Some(1),
            confidence_threshold: Some(0.8),
        };
        
        // Get security context
        let context = self.security.get_system_context().await?;
        
        // Get prediction
        let response = self.ml_handler.handle(&context, request).await
            .map_err(|e| AgentError::MLError(format!("Failed to analyze component: {}", e)))?;
        
        // First prediction is the readiness score
        let readiness_score = response.predictions.first()
            .ok_or_else(|| AgentError::MLError("No prediction available".to_string()))?;
            
        Ok(*readiness_score)
    }

    /// Prepare component data for ML analysis
    async fn prepare_component_data(&self, component: &SystemComponent) -> Result<Vec<f64>, AgentError> {
        let metrics = self.metrics.read().await;
        
        // Extract relevant metrics
        let mut data = Vec::new();
        
        // Add performance metrics
        if let Some(perf) = metrics.get_performance_metrics(&component.name) {
            data.push(perf.cpu_usage);
            data.push(perf.memory_usage);
            data.push(perf.response_time);
        }
        
        // Add test metrics
        if let Some(test) = metrics.get_test_metrics(&component.name) {
            data.push(test.coverage as f64);
            data.push(test.pass_rate);
        }
        
        // Add security metrics
        if let Some(sec) = metrics.get_security_metrics(&component.name) {
            data.push(sec.vulnerability_score);
            data.push(sec.compliance_score);
        }
        
        Ok(data)
    }

    /// Validate component using ML models
    pub async fn validate_component(&self, component: &SystemComponent) -> Result<ComponentMetrics, AgentError> {
        // Get all relevant models
        let models = self.model_repository.get_models_by_status(MLModelStatus::Ready).await
            .map_err(|e| AgentError::MLError(format!("Failed to get models: {}", e)))?;
            
        let mut total_score = 0.0;
        let mut count = 0;
        
        for model in models {
            if let Ok(score) = self.validate_with_model(component, &model).await {
                total_score += score;
                count += 1;
            }
        }
        
        if count == 0 {
            return Err(AgentError::MLError("No valid models available".to_string()));
        }
        
        let average_score = total_score / count as f64;
        
        Ok(ComponentMetrics {
            performance_score: average_score,
            reliability_score: average_score,
            security_score: average_score,
            test_coverage: average_score,
            error_rate: 1.0 - average_score,
        })
    }

    /// Validate component with specific model
    async fn validate_with_model(&self, component: &SystemComponent, model: &MLModel) -> Result<f64, AgentError> {
        let input_data = self.prepare_component_data(component).await?;
        
        let request = MLRequest {
            model_id: Some(model.id.clone()),
            input_data,
            parameters: None,
        };
        
        let context = self.security.get_system_context().await?;
        
        let response = self.ml_service.process(&context, request).await
            .map_err(|e| AgentError::MLError(format!("Model validation failed: {}", e)))?;
            
        Ok(response.confidence)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::service::MockMLService;
    use crate::security::MockSecurityManager;

    #[tokio::test]
    async fn test_component_analysis() {
        // Test implementation
    }

    #[tokio::test]
    async fn test_component_validation() {
        // Test implementation
    }

    #[tokio::test]
    async fn test_model_validation() {
        // Test implementation
    }
}
