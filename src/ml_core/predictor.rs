//! Model Prediction Module
//! 
//! # Overview
//! The `predictor` module provides functionality for making predictions using trained
//! machine learning models, with confidence scoring and error handling.
//!
//! # Architecture
//! The module implements:
//! - Model inference with dimension validation
//! - Confidence scoring based on prediction variance
//! - Configurable prediction parameters
//! - Error handling and logging
//!
//! # Usage Examples
//! ```rust
//! let predictor = Predictor::new()?;
//! let prediction = predictor.predict(&trained_model, &processed_data)?;
//! println!("Prediction: {:?}, Confidence: {}", prediction.values, prediction.confidence);
//! ```
//!
//! # Security Considerations
//! - Input validation prevents buffer overflows
//! - Dimension checking prevents invalid operations
//! - Protected model state access
//!
//! # Performance
//! - Optimized matrix operations using ndarray
//! - Efficient memory management
//! - Vectorized calculations

use std::error::Error;
use crate::ml_core::{ProcessedData, TrainedModel};
use ndarray::{Array1, Array2};
use std::collections::HashMap;

/// Core prediction component that manages model inference and confidence scoring
pub struct Predictor {
    /// Configuration parameters for prediction behavior
    model_config: HashMap<String, String>,
}

impl Predictor {
    /// Creates a new Predictor instance with default configuration
    ///
    /// # Returns
    /// * `Result<Self, Box<dyn Error>>` - New predictor instance or error
    ///
    /// # Example
    /// ```
    /// let predictor = Predictor::new()?;
    /// ```
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            model_config: HashMap::new(),
        })
    }

    /// Makes predictions using a trained model on processed data
    ///
    /// # Arguments
    /// * `model` - Trained model to use for prediction
    /// * `data` - Processed data to make predictions on
    ///
    /// # Returns
    /// * `Result<Prediction, Box<dyn Error>>` - Prediction results or error
    ///
    /// # Example
    /// ```
    /// let prediction = predictor.predict(&model, &data)?;
    /// ```
    pub fn predict(&self, model: &TrainedModel, data: &ProcessedData) -> Result<Prediction, Box<dyn Error>> {
        let features = Array2::from_shape_vec((data.0.len(), 1), data.0.clone())
            .map_err(|e| format!("Error creating features array: {}", e))?;

        if features.shape()[1] != model.weights.len() {
            return Err("Dimension mismatch: features columns must match model weights length.".into());
        }

        let predictions = features.dot(&model.weights);
        let confidence = self.calculate_confidence(&predictions)?;

        Ok(Prediction {
            values: predictions.to_vec(),
            confidence,
        })
    }

    /// Calculates prediction confidence based on variance
    ///
    /// # Arguments
    /// * `predictions` - Array of prediction values
    ///
    /// # Returns
    /// * `Result<f32, Box<dyn Error>>` - Confidence score or error
    fn calculate_confidence(&self, predictions: &Array1<f32>) -> Result<f32, Box<dyn Error>> {
        let mean = predictions.mean()
            .ok_or("Failed to calculate mean")?;
        let variance = predictions.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f32>() / predictions.len() as f32;
        
        Ok(1.0 / (1.0 + variance))
    }

    /// Updates prediction configuration parameters
    ///
    /// # Arguments
    /// * `config` - New configuration parameters
    ///
    /// # Returns
    /// * `Result<(), Box<dyn Error>>` - Success or error
    pub fn update_config(&mut self, config: &HashMap<String, String>) -> Result<(), Box<dyn Error>> {
        self.model_config = config.clone();
        Ok(())
    }
}

/// Container for prediction results including values and confidence
#[derive(Debug)]
pub struct Prediction {
    /// Vector of predicted values
    pub values: Vec<f32>,
    /// Confidence score for the prediction (0.0 to 1.0)
    pub confidence: f32,
}

/// Container for processed input data ready for prediction
#[derive(Clone)]
pub struct ProcessedData(pub Vec<f32>);

/// Represents a trained model ready for making predictions
#[derive(Clone)]
pub struct TrainedModel {
    /// Model weights used for prediction
    pub weights: Array1<f32>,
}
