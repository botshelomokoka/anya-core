use ndarray::{Array1, Array2};
use ndarray_rand::RandomExt;
use rand::SeedableRng;
use rand::rngs::StdRng;
use ndarray_rand::rand_distr::Uniform;
use std::collections::HashMap;
pub struct TrainingData(pub Vec<f32>);

pub struct ProcessedData(pub Vec<f32>);

pub struct ModelTrainer {
    // Holds the trained model after the training process is complete
    model: Option<TrainedModel>,
    // Configuration parameters for the model training process
    config: HashMap<String, String>,
}

impl ModelTrainer {
    pub fn new() -> Self {
        Self {
            model: None,
            config: HashMap::new(),
        }
    }

    pub fn train(&mut self, data: &TrainingData) -> TrainedModel {
        let learning_rate: f32 = self.config.get("learning_rate")
            .expect("learning_rate not found in config")
            .parse()
            .expect("Failed to parse learning_rate");

        let num_iterations: usize = self.config.get("num_iterations")
            .expect("num_iterations not found in config")
            .parse()
            .expect("Failed to parse num_iterations");

        let features = Array2::from_shape_vec((data.0.len(), 1), data.0.clone())
            .expect("Failed to create features array");

        let mut rng = StdRng::seed_from_u64(42); // Seed the RNG for reproducibility
        let mut weights = Array1::random_using(features.ncols(), Uniform::new(0., 1.), &mut rng);
        // Assuming targets are the same length as data and initialized here for demonstration
        let targets = Array1::from_vec(vec![0.0; data.0.len()]);

        for _ in 0..num_iterations {
            let predictions = features.dot(&weights);
            let errors = &predictions - &targets;
            let gradient = features.t().dot(&errors) / features.nrows() as f32;
            weights = &weights - learning_rate * &gradient;
        }   weights = &weights - learning_rate * &gradient;
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