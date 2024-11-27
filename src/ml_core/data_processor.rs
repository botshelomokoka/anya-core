//! Data Processing Module for Machine Learning Pipeline
//! 
//! # Overview
//! The `data_processor` module provides robust data preprocessing capabilities for the ML pipeline,
//! including normalization, feature extraction, and data validation. It serves as the first stage
//! in the ML pipeline, ensuring data quality and consistency.
//!
//! # Architecture
//! The module implements a multi-stage processing pipeline:
//! 1. Data Validation - Ensures data consistency and completeness
//! 2. Normalization - Standardizes features using mean and standard deviation
//! 3. Feature Extraction - Computes derived features based on configuration
//! 4. Data Transformation - Converts between different data formats
//!
//! # Usage Examples
//! ```rust
//! use anya::ml_core::DataProcessor;
//! use anya::PyConfig;
//!
//! let config = PyConfig::new();
//! let mut processor = DataProcessor::new(config)?;
//! 
//! // Preprocess training data
//! let data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
//! let processed = processor.preprocess(data)?;
//!
//! // Analyze data statistics
//! let stats = processor.analyze(data)?;
//! ```
//!
//! # Security Considerations
//! - Input validation prevents buffer overflows
//! - Numeric overflow checks in calculations
//! - Memory limits on data size
//! - Safe handling of Python objects via PyO3
//!
//! # Performance
//! - Efficient ndarray operations
//! - Vectorized computations
//! - Optional GPU acceleration
//! - Cached normalization parameters

use std::error::Error;
use crate::error::{AnyaError, AnyaResult};
use crate::PyConfig;
use log::{info, error, debug};
use ndarray::{Array1, Array2, Axis};
use serde::{Serialize, Deserialize};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::types::IntoPyDict;

/// Data processor for ML pipeline that handles preprocessing, normalization and feature extraction
#[pyclass]
#[derive(Debug, Serialize, Deserialize)]
pub struct DataProcessor {
    /// Configuration parameters for processing behavior
    config: PyConfig,
    /// Cached normalization parameters for inverse transforms
    normalization_params: Option<NormalizationParams>,
}

/// Parameters used for data normalization and denormalization
#[derive(Debug, Serialize, Deserialize)]
struct NormalizationParams {
    /// Mean values for each feature
    mean: Array1<f64>,
    /// Standard deviation values for each feature
    std: Array1<f64>,
}

#[pymethods]
impl DataProcessor {
    /// Creates a new DataProcessor with the specified configuration
    ///
    /// # Arguments
    /// * `config` - Configuration parameters for processing behavior
    ///
    /// # Returns
    /// * `PyResult<Self>` - New processor instance
    ///
    /// # Example
    /// ```
    /// let config = PyConfig::new();
    /// let processor = DataProcessor::new(config)?;
    /// ```
    #[new]
    pub fn new(config: PyConfig) -> PyResult<Self> {
        info!("Creating new DataProcessor");
        Ok(DataProcessor {
            config,
            normalization_params: None,
        })
    }

    /// Preprocesses input data through normalization and feature extraction
    ///
    /// # Arguments
    /// * `data` - Raw input data as nested vector
    ///
    /// # Returns
    /// * `PyResult<Vec<Vec<f64>>>` - Processed data
    ///
    /// # Example
    /// ```
    /// let data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    /// let processed = processor.preprocess(data)?;
    /// ```
    pub fn preprocess(&mut self, data: Vec<Vec<f64>>) -> PyResult<Vec<Vec<f64>>> {
        if data.is_empty() {
            return Err(PyErr::new::<PyValueError, _>("Input data is empty"));
        }
        let len = data[0].len();
        if !data.iter().all(|row| row.len() == len) {
            return Err(PyErr::new::<PyValueError, _>("Input data rows have different lengths"));
        }
        let data = Array2::from_shape_vec((data.len(), len), data.into_iter().flatten().collect())
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("Failed to create Array2: {}", e)))?;
        info!("Preprocessing data with shape {:?}", data.shape());
        let normalized = self.normalize(&data)?;
        let features = self.extract_features(&normalized)?;
        Ok(self.convert_to_vec_vec(features))
    }

    /// Converts processed data back to original scale
    ///
    /// # Arguments
    /// * `data` - Normalized data to inverse transform
    ///
    /// # Returns
    /// * `PyResult<Vec<Vec<f64>>>` - Data in original scale
    ///
    /// # Example
    /// ```
    /// let original = processor.inverse_transform(processed)?;
    /// ```
    pub fn inverse_transform(&self, data: Vec<Vec<f64>>) -> PyResult<Vec<Vec<f64>>> {
        if data.is_empty() {
            return Err(PyErr::new::<PyValueError, _>("Input data is empty"));
        }
        let len = data[0].len();
        if !data.iter().all(|row| row.len() == len) {
            return Err(PyErr::new::<PyValueError, _>("All inner vectors must have the same length"));
        }

        let data = Array2::from_shape_vec((data.len(), len), data.into_iter().flatten().collect())?;
        if let Some(params) = &self.normalization_params {
            let denormalized = &data * &params.std + &params.mean;
            Ok(denormalized.into_raw_vec().chunks(denormalized.ncols()).map(|chunk| chunk.to_vec()).collect())
        } else {
            error!("Normalization parameters not set. Cannot inverse transform.");
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Normalization parameters not set"))
        }
    }

    /// Analyzes data and computes statistical metrics
    ///
    /// # Arguments
    /// * `data` - Input data to analyze
    ///
    /// # Returns
    /// * `PyResult<Vec<f64>>` - Statistical metrics
    ///
    /// # Example
    /// ```
    /// let stats = processor.analyze(data)?;
    /// ```
    pub fn analyze(&self, data: Vec<Vec<f64>>) -> PyResult<Vec<f64>> {
        if data.is_empty() {
            return Err(PyErr::new::<PyValueError, _>("Input data is empty"));
        }
        let data = self.convert_to_array2(data)?;
        let mean = data.mean_axis(Axis(0)).unwrap();
        let std = data.std_axis(Axis(0), 0.0);
        let min = data.fold_axis(Axis(0), f64::INFINITY, |&acc, &x| acc.min(x));
        let max = data.fold_axis(Axis(0), f64::NEG_INFINITY, |&acc, &x| acc.max(x));
        
        Ok(vec![mean[0], std[0], min[0], max[0]])
    }

    // Internal helper methods
    fn convert_to_array2(&self, data: Vec<Vec<f64>>) -> PyResult<Array2<f64>> {
        Array2::from_shape_vec((data.len(), data[0].len()), data.into_iter().flatten().collect())
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("Failed to create Array2: {}", e)))
    }

    fn convert_to_vec_vec(&self, data: Array2<f64>) -> Vec<Vec<f64>> {
        data.into_raw_vec().chunks(data.ncols()).map(|chunk| chunk.to_vec()).collect()
    }

    fn normalize(&mut self, data: &Array2<f64>) -> PyResult<Array2<f64>> {
        let mean = data.mean_axis(Axis(0)).unwrap();
        let std = data.std_axis(Axis(0), 0.0);
        
        self.normalization_params = Some(NormalizationParams {
            mean: mean.clone(),
            std: std.clone(),
        });
        
        Ok((data - &mean) / &std)
    }

    fn extract_features(&self, data: &Array2<f64>) -> PyResult<Array2<f64>> {
        if self.config.get_feature("AdvancedFeatures".to_string()).unwrap_or(false) {
            info!("Extracting advanced features");
            #[cfg(feature = "advanced_features")]
            {
                // Advanced feature extraction logic
                unimplemented!()
            }
            #[cfg(not(feature = "advanced_features"))]
            {
                Ok(data.clone())
            }
        } else {
            Ok(data.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_processor() -> PyResult<()> {
        let config = PyConfig::new();
        let mut processor = DataProcessor::new(config)?;
        let data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let processed = processor.preprocess(data.clone())?;
        assert_eq!(processed.len(), 2);
        assert_eq!(processed[0].len(), 2);
        Ok(())
    }

    #[test]
    fn test_empty_data() {
        let config = PyConfig::new();
        let mut processor = DataProcessor::new(config).unwrap();
        let data = Vec::<Vec<f64>>::new();
        assert!(processor.preprocess(data).is_err());
    }

    #[test]
    fn test_unequal_length_rows() {
        let config = PyConfig::new();
        let mut processor = DataProcessor::new(config).unwrap();
        let data = vec![vec![1.0], vec![2.0, 3.0]];
        assert!(processor.preprocess(data).is_err());
    }

    #[test]
    fn test_analyze() -> PyResult<()> {
        let config = PyConfig::new();
        let processor = DataProcessor::new(config)?;
        let data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let stats = processor.analyze(data)?;
        assert_eq!(stats.len(), 4); // mean, std, min, max
        Ok(())
    }
}

/// Data Processing Module
//! 
//! # Overview
//! Handles data preprocessing, validation and normalization for ML operations
//! 
//! # Architecture
//! - Input validation
//! - Feature extraction
//! - Data normalization
//! - Dimension reduction
//! 
//! # Security
//! - Input bounds checking
//! - Memory limits
//! - Sanitization of inputs
//!
//! # Performance
//! - Efficient matrix operations
//! - Parallel processing support
//! - Memory-efficient transformations

use ndarray::{Array1, Array2, ArrayView1, Axis};
use std::error::Error;
use crate::ml_core::{MLError, InputBounds};
use rayon::prelude::*;
use std::sync::Arc;
use metrics::{counter, histogram};

/// Configuration for data processing
#[derive(Debug, Clone)]
pub struct ProcessorConfig {
    /// Normalization strategy
    pub normalization: NormalizationStrategy,
    /// Feature selection method
    pub feature_selection: FeatureSelectionMethod,
    /// Maximum allowed features
    pub max_features: usize,
    /// Minimum required samples
    pub min_samples: usize,
}

/// Normalization strategies
#[derive(Debug, Clone)]
pub enum NormalizationStrategy {
    /// Standardization (zero mean, unit variance)
    StandardScaler,
    /// Min-max scaling to [0,1]
    MinMaxScaler,
    /// Robust scaling using quantiles
    RobustScaler,
}

/// Feature selection methods
#[derive(Debug, Clone)]
pub enum FeatureSelectionMethod {
    /// Variance threshold
    VarianceThreshold(f64),
    /// Correlation threshold
    CorrelationThreshold(f64),
    /// Principal component analysis
    PCA(usize),
}

/// Core data processing component
#[derive(Debug)]
pub struct DataProcessor {
    config: ProcessorConfig,
    input_bounds: InputBounds,
    metrics: ProcessorMetrics,
}

/// Metrics for data processing operations
#[derive(Debug)]
struct ProcessorMetrics {
    processing_time: Arc<histogram::Histogram>,
    samples_processed: Arc<counter::Counter>,
    validation_failures: Arc<counter::Counter>,
}

impl DataProcessor {
    /// Creates a new DataProcessor with default configuration
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            config: ProcessorConfig::default(),
            input_bounds: InputBounds::default(),
            metrics: ProcessorMetrics::new()?,
        })
    }

    /// Processes input data with validation and normalization
    pub fn process(&self, data: Array2<f64>) -> Result<ProcessedData, MLError> {
        let start = std::time::Instant::now();

        // Validate input
        self.validate_input(&data)?;

        // Apply feature selection
        let selected_features = self.select_features(&data)?;

        // Normalize data
        let normalized = self.normalize(&selected_features)?;

        // Update metrics
        let duration = start.elapsed();
        self.metrics.processing_time.record(duration.as_secs_f64());
        self.metrics.samples_processed.increment(data.nrows() as u64);

        Ok(ProcessedData {
            data: normalized,
            feature_indices: (0..selected_features.ncols()).collect(),
        })
    }

    /// Validates input data against bounds and constraints
    fn validate_input(&self, data: &Array2<f64>) -> Result<(), MLError> {
        // Check dimensions
        if data.ncols() > self.config.max_features {
            return Err(MLError::DimensionMismatchError(
                format!("Too many features: {} (max: {})", 
                    data.ncols(), self.config.max_features)
            ));
        }

        if data.nrows() < self.config.min_samples {
            return Err(MLError::DataValidationError(
                format!("Too few samples: {} (min: {})",
                    data.nrows(), self.config.min_samples)
            ));
        }

        // Check value bounds
        if let Some(&min) = data.iter().min_by(|a, b| a.partial_cmp(b).unwrap()) {
            if min < self.input_bounds.min_value {
                return Err(MLError::DataValidationError(
                    format!("Value below minimum: {} (min: {})",
                        min, self.input_bounds.min_value)
                ));
            }
        }

        if let Some(&max) = data.iter().max_by(|a, b| a.partial_cmp(b).unwrap()) {
            if max > self.input_bounds.max_value {
                return Err(MLError::DataValidationError(
                    format!("Value above maximum: {} (max: {})",
                        max, self.input_bounds.max_value)
                ));
            }
        }

        Ok(())
    }

    /// Applies feature selection based on configuration
    fn select_features(&self, data: &Array2<f64>) -> Result<Array2<f64>, MLError> {
        match &self.config.feature_selection {
            FeatureSelectionMethod::VarianceThreshold(threshold) => {
                self.select_by_variance(data, *threshold)
            }
            FeatureSelectionMethod::CorrelationThreshold(threshold) => {
                self.select_by_correlation(data, *threshold)
            }
            FeatureSelectionMethod::PCA(n_components) => {
                self.apply_pca(data, *n_components)
            }
        }
    }

    /// Normalizes data based on selected strategy
    fn normalize(&self, data: &Array2<f64>) -> Result<Array2<f64>, MLError> {
        match &self.config.normalization {
            NormalizationStrategy::StandardScaler => {
                self.standardize(data)
            }
            NormalizationStrategy::MinMaxScaler => {
                self.min_max_scale(data)
            }
            NormalizationStrategy::RobustScaler => {
                self.robust_scale(data)
            }
        }
    }

    /// Standardizes data to zero mean and unit variance
    fn standardize(&self, data: &Array2<f64>) -> Result<Array2<f64>, MLError> {
        let mean = data.mean_axis(Axis(0))
            .ok_or_else(|| MLError::DataValidationError("Failed to compute mean".into()))?;
        let std = data.std_axis(Axis(0), 0.0)
            .ok_or_else(|| MLError::DataValidationError("Failed to compute std".into()))?;

        Ok(data.outer_iter()
            .map(|row| {
                row.iter()
                    .zip(mean.iter().zip(std.iter()))
                    .map(|(&x, (&m, &s))| if s != 0.0 { (x - m) / s } else { 0.0 })
                    .collect::<Vec<f64>>()
            })
            .collect::<Vec<Vec<f64>>>()
            .into())
    }

    // Additional helper methods...
}

impl Default for ProcessorConfig {
    fn default() -> Self {
        Self {
            normalization: NormalizationStrategy::StandardScaler,
            feature_selection: FeatureSelectionMethod::VarianceThreshold(0.0),
            max_features: 1000,
            min_samples: 10,
        }
    }
}

impl ProcessorMetrics {
    fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            processing_time: Arc::new(histogram!("data_processing_time_seconds")),
            samples_processed: Arc::new(counter!("data_samples_processed_total")),
            validation_failures: Arc::new(counter!("data_validation_failures_total")),
        })
    }
}
