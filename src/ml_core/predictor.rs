use ndarray::{Array1, Array2};
use std::collections::HashMap;
use crate::ml_core::{ProcessedData, TrainedModel};

pub struct Predictor {
    config: HashMap<String, String>,
}

impl Predictor {
    pub fn new() -> Self {
        Self {
            config: HashMap::new(),
        }
    }

    pub fn predict(&self, model: &TrainedModel, data: &ProcessedData) -> Prediction {
        let features = Array2::from_shape_vec((data.0.len(), 1), data.0.clone()).unwrap();
        let predictions = features.dot(&model.weights);
        
        Prediction {
            values: predictions.to_vec(),
            confidence: self.calculate_confidence(&predictions),
        }
    }

    fn calculate_confidence(&self, predictions: &Array1<f32>) -> f32 {
        // Simple confidence calculation based on prediction variance
        let mean = predictions.mean().unwrap_or(0.0);
        let variance = predictions.iter().map(|&x| (x - mean).powi(2)).sum::<f32>() / predictions.len() as f32;
        1.0 / (1.0 + variance)
    }

    pub fn update_config(&mut self, config: &HashMap<String, String>) {
        self.config = config.clone();
    }
}

pub struct ProcessedData(pub Vec<f32>);

pub struct TrainedModel {
    pub weights: Array1<f32>,
}

pub struct Prediction {
    pub values: Vec<f32>,
    pub confidence: f32,
}