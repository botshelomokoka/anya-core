use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use tokio::fs;
use chrono::Utc;
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct MLModel {
    version: String,
    features: Vec<Feature>,
    weights: Vec<f64>,
    last_updated: chrono::DateTime<chrono::Utc>,
    validation_score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feature {
    name: String,
    importance: f64,
    category: FeatureCategory,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FeatureCategory {
    Bitcoin,
    Lightning,
    Security,
    Code,
    Documentation,
}

impl MLModel {
    pub async fn load(path: &PathBuf) -> Result<Self, ModelError> {
        let content = fs::read_to_string(path).await?;
        let model: MLModel = serde_json::from_str(&content)?;
        model.validate()?;
        Ok(model)
    }

    pub async fn save(&self, path: &PathBuf) -> Result<(), ModelError> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content).await?;
        Ok(())
    }

    pub fn validate(&self) -> Result<(), ModelError> {
        if self.features.is_empty() {
            return Err(ModelError::InvalidModel("No features defined".into()));
        }
        if self.weights.len() != self.features.len() {
            return Err(ModelError::InvalidModel("Feature/weight mismatch".into()));
        }
        Ok(())
    }
}
