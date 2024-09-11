use ndarray::{Array1, Array2};
use ndarray_stats::QuantileExt;
use std::collections::HashMap;
use crate::ml_core::ProcessedData;

pub struct DataProcessor {
    config: HashMap<String, String>,
}

impl DataProcessor {
    pub fn new() -> Self {
        Self {
            config: HashMap::new(),
        }
    }

    pub fn process(&self, data: Vec<f32>) -> ProcessedData {
        let data = Array1::from(data);
        
        // Normalize the data
        let normalized = self.normalize(&data);
        
        // Handle missing values
        let imputed = self.impute_missing_values(&normalized);
        
        // Feature scaling
        let scaled = self.scale_features(&imputed);
        
        ProcessedData(scaled.to_vec())
    }

    fn normalize(&self, data: &Array1<f32>) -> Array1<f32> {
        let min = data.min().unwrap();
        let max = data.max().unwrap();
        (data - min) / (max - min)
    }

    fn impute_missing_values(&self, data: &Array1<f32>) -> Array1<f32> {
        let mean = data.mean().unwrap_or(0.0);
        data.map(|&x| if x.is_nan() { mean } else { x })
    }

    fn scale_features(&self, data: &Array1<f32>) -> Array1<f32> {
        let mean = data.mean().unwrap_or(0.0);
        let std = data.std(0.0);
        (data - mean) / std
    }

    pub fn update_config(&mut self, config: &HashMap<String, String>) {
        self.config = config.clone();
    }
}

pub struct ProcessedData(pub Vec<f32>);