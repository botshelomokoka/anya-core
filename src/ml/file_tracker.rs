use super::model::{MLModel, Feature, FeatureCategory};
use std::path::PathBuf;
use regex::Regex;
use std::collections::HashMap;
use tokio::fs;
use crate::model::ModelError;

pub struct FileAnalysisModel {
    model: MLModel,
    feature_extractor: FeatureExtractor,
    model_path: PathBuf,
}

impl FileAnalysisModel {
    pub async fn new(model_path: PathBuf) -> Result<Self, ModelError> {
        let model = MLModel::load(&model_path).await?;
        Ok(Self {
            model,
            feature_extractor: FeatureExtractor::new(),
            model_path,
        })
    }

    pub async fn update_model(&mut self) -> Result<(), ModelError> {
        let new_model = self.train_new_model().await?;
        if new_model.validation_score > self.model.validation_score {
            self.model = new_model;
            self.model.save(&self.model_path).await?;
        }
        Ok(())
    }

    fn predict_category(&self, content: &str) -> FileCategory {
        let features = self.feature_extractor.extract_features(content);
        let scores = self.calculate_category_scores(&features);
        self.get_highest_scoring_category(scores)
    }

    fn calculate_importance(&self, content: &str) -> f64 {
        let features = self.feature_extractor.extract_features(content);
        self.model.calculate_importance(&features)
    }
}

pub struct FeatureExtractor {
    patterns: HashMap<String, Regex>,
}

impl FeatureExtractor {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();
        patterns.insert(
            "bitcoin_imports".to_string(),
            Regex::new(r"use\s+bitcoin::").unwrap()
        );
        patterns.insert(
            "lightning_imports".to_string(),
            Regex::new(r"use\s+lightning::").unwrap()
        );
        // Add more patterns...
        Self { patterns }
    }

    pub fn extract_features(&self, content: &str) -> Vec<f64> {
        self.patterns.iter()
            .map(|(_, pattern)| {
                pattern.find_iter(content).count() as f64
            })
            .collect()
    }
}
