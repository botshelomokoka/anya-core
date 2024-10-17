use ndarray::{Array1, Array2};
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;
use std::collections::HashMap;
use crate::ml_core::TrainedModel;

pub struct ProcessedData(pub Vec<f32>);

pub struct ModelTrainer {
    model: Option<TrainedModel>,
    config: HashMap<String, String>,
}

impl ModelTrainer {
    pub fn new() -> Self {
        Self {
            model: None,
            config: HashMap::new(),
        }
    }

    pub fn train(&mut self, data: &ProcessedData) -> TrainedModel {
        let learning_rate: f32 = self.config.get("learning_rate")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.01);

        let num_iterations: usize = self.config.get("num_iterations")
            .and_then(|s| s.parse().ok())
        let features = Array2::from_shape_vec((data.0.len(), 1), data.0.clone())
            .expect("Failed to create features array");

        let features = Array2::from_shape_vec((data.0.len(), 1), data.0.clone()).unwrap();
        let targets = Array1::from_vec(data.0.clone());

        let mut weights = Array1::random(features.ncols(), Uniform::new(0., 1.));

        for _ in 0..num_iterations {
            let predictions = features.dot(&weights);
            let errors = &predictions - &targets;
            let gradient = features.t().dot(&errors) / features.nrows() as f32;
            weights = &weights - learning_rate * &gradient;
        }

        let model = TrainedModel { weights };
        self.model = Some(model.clone());
        model
    }

    pub fn update_model(&mut self, model: TrainedModel) {
        self.model = Some(model);
    }

    pub fn update_config(&mut self, config: &HashMap<String, String>) {
        self.config = config.clone();
    }
}

#[derive(Clone)]
pub struct TrainedModel {
    weights: Array1<f32>,
}