use crate::error::{AnyaError, AnyaResult};
use crate::PyConfig;
use log::{info, error, debug};
use ndarray::{Array1, Array2, Axis};
use serde::{Serialize, Deserialize};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::types::IntoPyDict;

#[pyclass]
#[derive(Debug, Serialize, Deserialize)]
pub struct DataProcessor {
    config: PyConfig,
    normalization_params: Option<NormalizationParams>,
}

#[derive(Debug, Serialize, Deserialize)]
struct NormalizationParams {
    mean: Array1<f64>,
    std: Array1<f64>,
}

#[pymethods]
impl DataProcessor {
    #[new]
    pub fn new(config: PyConfig) -> Self {
        info!("Creating new DataProcessor");
        DataProcessor {
            config,
            normalization_params: None,
        }
    }
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

    fn convert_to_array2(&self, data: Vec<Vec<f64>>) -> PyResult<Array2<f64>> {
        Array2::from_shape_vec((data.len(), data[0].len()), data.into_iter().flatten().collect())
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("Failed to create Array2: {}", e)))
    }

    fn convert_to_vec_vec(&self, data: Array2<f64>) -> Vec<Vec<f64>> {
        data.into_raw_vec().chunks(data.ncols()).map(|chunk| chunk.to_vec()).collect()
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

    pub fn analyze(&self, data: Vec<Vec<f64>>) -> PyResult<Vec<f64>> {
        info!("Analyzing data with {} points", data.len());
        // Implement analysis logic here
        // This is a placeholder implementation
        Ok(data.into_iter().map(|row| row.iter().sum()).collect())
    }
}

impl DataProcessor {
            fn normalize(&mut self, data: &Array2<f64>) -> AnyaResult<Array2<f64>> {
                if self.normalization_params.is_none() {
                    let mean = data.mean_axis(Axis(0))
                        .ok_or_else(|| AnyaError::DataProcessing("Failed to compute mean".into()))?;
                    let std = data.std_axis(Axis(0), 0.)
                        .ok_or_else(|| AnyaError::DataProcessing("Failed to compute standard deviation".into()))?;
                    self.normalization_params = Some(NormalizationParams { mean, std });
                }
        
                let params = self.normalization_params.as_ref().unwrap();
                debug!("Normalizing data");
                let normalized = (data - &params.mean) / &params.std;
                Ok(normalized)
            }
        }
        
        impl DataProcessor {
            fn normalize(&mut self, data: &Array2<f64>) -> AnyaResult<Array2<f64>> {
                if self.normalization_params.is_none() {
                    let mean = data.mean_axis(Axis(0))
                        .ok_or_else(|| AnyaError::DataProcessing("Failed to compute mean".into()))?;
                    let std = data.std_axis(Axis(0), 0.)
                        .ok_or_else(|| AnyaError::DataProcessing("Failed to compute standard deviation".into()))?;
                    self.normalization_params = Some(NormalizationParams { mean, std });
                }
        
                let params = self.normalization_params.as_ref().unwrap();
                debug!("Normalizing data");
                let normalized = (data - &params.mean) / &params.std;
                Ok(normalized)
            }
        
            fn extract_features(&self, data: &Array2<f64>) -> AnyaResult<Array2<f64>> {
                if self.config.get_feature("AdvancedFeatures".to_string()).unwrap_or(false) {
                    info!("Extracting advanced features");
                    #[cfg(feature = "advanced_features")]
                    {
                        // Implement advanced feature extraction
                        unimplemented!("Advanced feature extraction not yet implemented");
                    }
                    #[cfg(not(feature = "advanced_features"))]
                    {
                        error!("Advanced features are not enabled in this build");
                        Err(AnyaError::FeatureNotEnabled("AdvancedFeatures".into()))
                    }
                } else {
                    debug!("Using basic features");
                    Ok(data.to_owned())
                }
            }
        }
            if self.config.get_feature("AdvancedFeatures".to_string()).unwrap_or(false) {
                info!("Extracting advanced features");
                #[cfg(feature = "advanced_features")]
                {
                    // Implement advanced feature extraction
                    unimplemented!("Advanced feature extraction not yet implemented");
                }
                #[cfg(not(feature = "advanced_features"))]
                {
                    error!("Advanced features are not enabled in this build");
                    Err(AnyaError::FeatureNotEnabled("AdvancedFeatures".into()))
                }
            } else {
                debug!("Using basic features");
                Ok(data.to_owned())
            }
        }
    }

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    #[test]
    fn test_data_processor() {
        let config = PyConfig::new();
        let mut processor = DataProcessor::new(config);

        let data = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0], vec![7.0, 8.0, 9.0]];
        let processed = processor.preprocess(data.clone()).unwrap();

        assert_eq!(processed.len(), data.len());
        assert_eq!(processed[0].len(), data[0].len());

        let reconstructed = processor.inverse_transform(processed).unwrap();
        for (original, reconstructed) in data.iter().zip(reconstructed.iter()) {
            for (o, r) in original.iter().zip(reconstructed.iter()) {
                assert!((o - r).abs() < 1e-8);
            }
        }
    }

    #[test]
    fn test_empty_data() {
        let config = PyConfig::new();
        let mut processor = DataProcessor::new(config);

        let data: Vec<Vec<f64>> = vec![];
        assert!(processor.preprocess(data).is_err());
    }

    #[test]
    fn test_unequal_length_rows() {
        let config = PyConfig::new();
        let mut processor = DataProcessor::new(config);

        let data = vec![vec![1.0, 2.0], vec![3.0, 4.0, 5.0]];
        assert!(processor.preprocess(data).is_err());
    }

    #[test]
    fn test_analyze() {
        let config = PyConfig::new();
        let processor = DataProcessor::new(config);

        let data = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let analysis = processor.analyze(data).unwrap();

        assert_eq!(analysis, vec![6.0, 15.0]);
    }
}