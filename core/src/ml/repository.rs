use std::sync::Arc;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::repository::{Repository, GenericRepository};
use crate::metrics::{UnifiedMetrics, ComponentHealth};
use crate::validation::{ValidationResult, Validator};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLModel {
    pub id: String,
    pub name: String,
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metrics: MLModelMetrics,
    pub parameters: MLModelParameters,
    pub status: MLModelStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLModelMetrics {
    pub accuracy: f64,
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub training_time: f64,
    pub inference_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLModelParameters {
    pub learning_rate: f64,
    pub batch_size: usize,
    pub epochs: usize,
    pub optimizer: String,
    pub architecture: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MLModelStatus {
    Training,
    Ready,
    Failed,
    Archived,
}

pub struct MLModelValidator;

#[async_trait]
impl Validator<MLModel> for MLModelValidator {
    async fn validate(&self, model: &MLModel) -> Result<ValidationResult, ValidationError> {
        // Validate model parameters
        if model.parameters.learning_rate <= 0.0 || model.parameters.learning_rate >= 1.0 {
            return Ok(ValidationResult::Invalid("Learning rate must be between 0 and 1".to_string()));
        }

        if model.parameters.batch_size == 0 {
            return Ok(ValidationResult::Invalid("Batch size must be greater than 0".to_string()));
        }

        if model.parameters.epochs == 0 {
            return Ok(ValidationResult::Invalid("Epochs must be greater than 0".to_string()));
        }

        // Validate metrics
        if model.metrics.accuracy < 0.0 || model.metrics.accuracy > 1.0 {
            return Ok(ValidationResult::Invalid("Accuracy must be between 0 and 1".to_string()));
        }

        if model.metrics.precision < 0.0 || model.metrics.precision > 1.0 {
            return Ok(ValidationResult::Invalid("Precision must be between 0 and 1".to_string()));
        }

        if model.metrics.recall < 0.0 || model.metrics.recall > 1.0 {
            return Ok(ValidationResult::Invalid("Recall must be between 0 and 1".to_string()));
        }

        Ok(ValidationResult::Valid)
    }
}

pub type MLModelRepository = GenericRepository<MLModel, MLError>;

impl MLModelRepository {
    pub fn new(metrics: Arc<RwLock<UnifiedMetrics>>) -> Self {
        Self::new(
            metrics,
            Arc::new(MLModelValidator),
        )
    }

    pub async fn get_best_model(&self) -> Result<Option<MLModel>, MLError> {
        let models = self.list().await?;
        Ok(models.into_iter()
            .filter(|m| matches!(m.status, MLModelStatus::Ready))
            .max_by(|a, b| a.metrics.f1_score.partial_cmp(&b.metrics.f1_score).unwrap()))
    }

    pub async fn get_models_by_status(&self, status: MLModelStatus) -> Result<Vec<MLModel>, MLError> {
        let models = self.list().await?;
        Ok(models.into_iter()
            .filter(|m| m.status == status)
            .collect())
    }

    pub async fn archive_old_models(&self, threshold_days: i64) -> Result<usize, MLError> {
        let mut count = 0;
        let models = self.list().await?;
        let now = Utc::now();
        
        for model in models {
            let age = now.signed_duration_since(model.updated_at).num_days();
            if age > threshold_days && model.status != MLModelStatus::Archived {
                let mut archived_model = model.clone();
                archived_model.status = MLModelStatus::Archived;
                self.update(&model.id, archived_model).await?;
                count += 1;
            }
        }
        
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ml_model_repository() {
        let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
        let repo = MLModelRepository::new(metrics);

        // Create test model
        let model = MLModel {
            id: "test-1".to_string(),
            name: "Test Model".to_string(),
            version: "1.0.0".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metrics: MLModelMetrics {
                accuracy: 0.95,
                precision: 0.94,
                recall: 0.93,
                f1_score: 0.935,
                training_time: 100.0,
                inference_time: 10.0,
            },
            parameters: MLModelParameters {
                learning_rate: 0.01,
                batch_size: 32,
                epochs: 100,
                optimizer: "Adam".to_string(),
                architecture: "Transformer".to_string(),
            },
            status: MLModelStatus::Ready,
        };

        // Test create
        let created = repo.create(model.clone()).await.unwrap();
        assert_eq!(created.name, model.name);

        // Test get best model
        let best = repo.get_best_model().await.unwrap().unwrap();
        assert_eq!(best.id, model.id);

        // Test get by status
        let ready_models = repo.get_models_by_status(MLModelStatus::Ready).await.unwrap();
        assert_eq!(ready_models.len(), 1);
        assert_eq!(ready_models[0].id, model.id);

        // Test archive
        let archived = repo.archive_old_models(0).await.unwrap();
        assert_eq!(archived, 1);
        
        let archived_models = repo.get_models_by_status(MLModelStatus::Archived).await.unwrap();
        assert_eq!(archived_models.len(), 1);
    }
}
