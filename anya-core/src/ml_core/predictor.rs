use crate::ml_core::{ProcessedData, TrainedModel};
use ndarray::{Array1, Array2};
use std::collections::HashMap;

pub struct Predictor {
    model_config: HashMap<String, String>,
}

impl Predictor {
    pub fn new() -> Self {
        Self {
            model_config: HashMap::new(),
        }
    }

    pub fn predict(&self, model: &TrainedModel, data: &ProcessedData) -> Prediction {
        let features = match Array2::from_shape_vec((data.features.len(), 1), data.features.clone()) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error creating features array: {}", e);
                return Prediction {
                    values: vec![],
                    confidence: 0.0,
                };
            }
        };
        if features.shape()[1] != model.weights.len() {
            panic!("Dimension mismatch: features columns must match model weights length.");
        }
        let predictions = features.dot(&model.weights);
        
        Prediction {
            values: predictions.to_vec(),
            confidence: self.calculate_confidence(&predictions),
        }
    }

    fn calculate_confidence(&self, predictions: &Array1<f32>) -> f32 {
        let mean = predictions.mean().unwrap_or(0.0);
        let variance = predictions.iter().map(|&x| (x - mean).powi(2)).sum::<f32>() / predictions.len() as f32;
        1.0 / (1.0 + variance)
    }

    pub fn update_config(&mut self, config: &HashMap<String, String>) {
        for (key, value) in config {
            self.model_config.insert(key.clone(), value.clone());
        }
    }
}

/// Struct to hold prediction results
pub struct Prediction {
    pub values: Vec<f32>,      // Predicted values
    pub confidence: f32,       // Confidence of the prediction
}

pub struct ProcessedData(pub Vec<f32>);

pub struct TrainedModel {
    pub weights: Array1<f32>,
}
    pub confidence: f32,
}        self.model_config = config.clone();