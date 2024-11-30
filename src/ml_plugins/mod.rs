use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLMetrics {
    pub ml_accuracy: f32,
    pub ml_loss: f32,
    pub ml_training_time: f32,
    pub ml_inference_time: f32,
    pub ml_batch_size: usize,
    pub ml_model_size_mb: f32,
    pub ml_memory_usage_mb: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLConfig {
    pub ml_model_type: String,
    pub ml_batch_size: usize,
    pub ml_learning_rate: f32,
    pub ml_epochs: usize,
    pub ml_optimizer: String,
    pub ml_loss_function: String,
    pub ml_device: String,  // "cpu" or "gpu"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLDataStats {
    pub ml_samples_count: usize,
    pub ml_features_count: usize,
    pub ml_class_distribution: HashMap<String, usize>,
    pub ml_missing_values_ratio: f32,
    pub ml_data_skewness: f32,
}

#[async_trait]
pub trait MLPlugin: Send + Sync {
    /// Get ML model information
    fn ml_info(&self) -> MLPluginInfo;
    
    /// Initialize ML model with configuration
    async fn ml_initialize(&self, config: MLConfig) -> Result<(), MLPluginError>;
    
    /// Train ML model
    async fn ml_train(&self, data: Vec<u8>) -> Result<MLMetrics, MLPluginError>;
    
    /// Make predictions
    async fn ml_predict(&self, input: Vec<u8>) -> Result<Vec<u8>, MLPluginError>;
    
    /// Get model metrics
    async fn ml_metrics(&self) -> Result<MLMetrics, MLPluginError>;
    
    /// Analyze input data
    async fn ml_analyze_data(&self, data: Vec<u8>) -> Result<MLDataStats, MLPluginError>;
    
    /// Export model
    async fn ml_export_model(&self) -> Result<Vec<u8>, MLPluginError>;
    
    /// Import model
    async fn ml_import_model(&self, model_data: Vec<u8>) -> Result<(), MLPluginError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLPluginInfo {
    pub ml_name: String,
    pub ml_version: String,
    pub ml_description: String,
    pub ml_model_type: String,
    pub ml_supported_tasks: Vec<String>,
    pub ml_input_format: String,
    pub ml_output_format: String,
}

#[derive(Debug, thiserror::Error)]
pub enum MLPluginError {
    #[error("ML Plugin not found: {0}")]
    NotFound(String),
    
    #[error("ML Model initialization failed: {0}")]
    InitError(String),
    
    #[error("ML Training failed: {0}")]
    TrainingError(String),
    
    #[error("ML Prediction failed: {0}")]
    PredictionError(String),
    
    #[error("ML Invalid data format: {0}")]
    DataError(String),
    
    #[error("ML Model export/import failed: {0}")]
    ModelIOError(String),
}

pub struct MLPluginManager {
    plugins: HashMap<String, Box<dyn MLPlugin>>,
    configs: HashMap<String, MLConfig>,
}

impl MLPluginManager {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            configs: HashMap::new(),
        }
    }

    pub async fn register_plugin(
        &mut self,
        name: String,
        plugin: Box<dyn MLPlugin>,
        config: MLConfig,
    ) -> Result<(), MLPluginError> {
        // Initialize plugin with config
        plugin.ml_initialize(config.clone()).await?;
        
        // Store plugin and config
        self.plugins.insert(name.clone(), plugin);
        self.configs.insert(name, config);
        
        Ok(())
    }

    pub async fn train_model(
        &self,
        plugin_name: &str,
        training_data: Vec<u8>,
    ) -> Result<MLMetrics, MLPluginError> {
        let plugin = self.plugins.get(plugin_name)
            .ok_or_else(|| MLPluginError::NotFound(plugin_name.to_string()))?;
            
        plugin.ml_train(training_data).await
    }

    pub async fn predict(
        &self,
        plugin_name: &str,
        input_data: Vec<u8>,
    ) -> Result<Vec<u8>, MLPluginError> {
        let plugin = self.plugins.get(plugin_name)
            .ok_or_else(|| MLPluginError::NotFound(plugin_name.to_string()))?;
            
        plugin.ml_predict(input_data).await
    }

    pub async fn get_metrics(
        &self,
        plugin_name: &str,
    ) -> Result<MLMetrics, MLPluginError> {
        let plugin = self.plugins.get(plugin_name)
            .ok_or_else(|| MLPluginError::NotFound(plugin_name.to_string()))?;
            
        plugin.ml_metrics().await
    }

    pub async fn analyze_data(
        &self,
        plugin_name: &str,
        data: Vec<u8>,
    ) -> Result<MLDataStats, MLPluginError> {
        let plugin = self.plugins.get(plugin_name)
            .ok_or_else(|| MLPluginError::NotFound(plugin_name.to_string()))?;
            
        plugin.ml_analyze_data(data).await
    }

    pub async fn export_model(
        &self,
        plugin_name: &str,
    ) -> Result<Vec<u8>, MLPluginError> {
        let plugin = self.plugins.get(plugin_name)
            .ok_or_else(|| MLPluginError::NotFound(plugin_name.to_string()))?;
            
        plugin.ml_export_model().await
    }

    pub async fn import_model(
        &self,
        plugin_name: &str,
        model_data: Vec<u8>,
    ) -> Result<(), MLPluginError> {
        let plugin = self.plugins.get(plugin_name)
            .ok_or_else(|| MLPluginError::NotFound(plugin_name.to_string()))?;
            
        plugin.ml_import_model(model_data).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct TestMLPlugin {
        metrics: MLMetrics,
    }
    
    #[async_trait]
    impl MLPlugin for TestMLPlugin {
        fn ml_info(&self) -> MLPluginInfo {
            MLPluginInfo {
                ml_name: "test".to_string(),
                ml_version: "1.0".to_string(),
                ml_description: "Test ML plugin".to_string(),
                ml_model_type: "test_model".to_string(),
                ml_supported_tasks: vec!["test_task".to_string()],
                ml_input_format: "bytes".to_string(),
                ml_output_format: "bytes".to_string(),
            }
        }
        
        async fn ml_initialize(&self, _config: MLConfig) -> Result<(), MLPluginError> {
            Ok(())
        }
        
        async fn ml_train(&self, _data: Vec<u8>) -> Result<MLMetrics, MLPluginError> {
            Ok(self.metrics.clone())
        }
        
        async fn ml_predict(&self, input: Vec<u8>) -> Result<Vec<u8>, MLPluginError> {
            Ok(input)
        }
        
        async fn ml_metrics(&self) -> Result<MLMetrics, MLPluginError> {
            Ok(self.metrics.clone())
        }
        
        async fn ml_analyze_data(&self, _data: Vec<u8>) -> Result<MLDataStats, MLPluginError> {
            Ok(MLDataStats {
                ml_samples_count: 1000,
                ml_features_count: 10,
                ml_class_distribution: HashMap::new(),
                ml_missing_values_ratio: 0.0,
                ml_data_skewness: 0.0,
            })
        }
        
        async fn ml_export_model(&self) -> Result<Vec<u8>, MLPluginError> {
            Ok(vec![])
        }
        
        async fn ml_import_model(&self, _model_data: Vec<u8>) -> Result<(), MLPluginError> {
            Ok(())
        }
    }
    
    #[tokio::test]
    async fn test_ml_plugin_manager() {
        let mut manager = MLPluginManager::new();
        
        let test_metrics = MLMetrics {
            ml_accuracy: 0.95,
            ml_loss: 0.05,
            ml_training_time: 10.0,
            ml_inference_time: 0.1,
            ml_batch_size: 32,
            ml_model_size_mb: 10.0,
            ml_memory_usage_mb: 512.0,
        };
        
        let plugin = TestMLPlugin {
            metrics: test_metrics.clone(),
        };
        
        let config = MLConfig {
            ml_model_type: "test".to_string(),
            ml_batch_size: 32,
            ml_learning_rate: 0.001,
            ml_epochs: 10,
            ml_optimizer: "adam".to_string(),
            ml_loss_function: "cross_entropy".to_string(),
            ml_device: "cpu".to_string(),
        };
        
        // Test plugin registration
        manager.register_plugin("test".to_string(), Box::new(plugin), config).await.unwrap();
        
        // Test training
        let metrics = manager.train_model("test", vec![]).await.unwrap();
        assert_eq!(metrics.ml_accuracy, test_metrics.ml_accuracy);
        
        // Test prediction
        let input = vec![1, 2, 3];
        let output = manager.predict("test", input.clone()).await.unwrap();
        assert_eq!(output, input);
        
        // Test metrics
        let metrics = manager.get_metrics("test").await.unwrap();
        assert_eq!(metrics.ml_accuracy, test_metrics.ml_accuracy);
    }
}
