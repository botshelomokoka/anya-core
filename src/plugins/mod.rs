use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PluginError {
    #[error("Plugin not found: {0}")]
    NotFound(String),
    #[error("Plugin initialization failed: {0}")]
    InitError(String),
    #[error("Plugin execution failed: {0}")]
    ExecutionError(String),
    #[error("Invalid configuration: {0}")]
    ConfigError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub name: String,
    pub version: String,
    pub enabled: bool,
    pub auto_scale: bool,
    pub rate_limits: RateLimitConfig,
    pub resource_limits: ResourceLimitConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_second: u32,
    pub burst_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimitConfig {
    pub max_memory_mb: u32,
    pub max_cpu_percent: u32,
    pub max_storage_gb: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetrics {
    pub requests_processed: u64,
    pub average_response_time_ms: f64,
    pub error_rate: f64,
    pub resource_utilization: ResourceUtilization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUtilization {
    pub memory_usage_mb: u32,
    pub cpu_usage_percent: f32,
    pub storage_usage_gb: f32,
}

#[async_trait]
pub trait Plugin: Send + Sync {
    /// Get plugin information
    fn info(&self) -> PluginInfo;
    
    /// Initialize the plugin
    async fn initialize(&self, config: PluginConfig) -> Result<(), PluginError>;
    
    /// Execute plugin functionality
    async fn execute(&self, input: serde_json::Value) -> Result<serde_json::Value, PluginError>;
    
    /// Get plugin metrics
    async fn metrics(&self) -> Result<PluginMetrics, PluginError>;
    
    /// Scale resources based on demand
    async fn scale(&self, demand: f32) -> Result<(), PluginError>;
    
    /// Cleanup plugin resources
    async fn cleanup(&self) -> Result<(), PluginError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub capabilities: Vec<String>,
}

pub struct PluginManager {
    plugins: Arc<RwLock<HashMap<String, Box<dyn Plugin>>>>,
    configs: Arc<RwLock<HashMap<String, PluginConfig>>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(RwLock::new(HashMap::new())),
            configs: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_plugin(
        &self,
        name: String,
        plugin: Box<dyn Plugin>,
        config: PluginConfig,
    ) -> Result<(), PluginError> {
        // Initialize plugin
        plugin.initialize(config.clone()).await?;
        
        // Store plugin and config
        let mut plugins = self.plugins.write().await;
        let mut configs = self.configs.write().await;
        
        plugins.insert(name.clone(), plugin);
        configs.insert(name, config);
        
        Ok(())
    }

    pub async fn execute_plugin(
        &self,
        name: &str,
        input: serde_json::Value,
    ) -> Result<serde_json::Value, PluginError> {
        let plugins = self.plugins.read().await;
        let plugin = plugins.get(name)
            .ok_or_else(|| PluginError::NotFound(name.to_string()))?;
            
        // Execute plugin
        let result = plugin.execute(input).await?;
        
        // Auto-scale if needed
        if let Some(config) = self.configs.read().await.get(name) {
            if config.auto_scale {
                let metrics = plugin.metrics().await?;
                let demand = calculate_demand(&metrics);
                plugin.scale(demand).await?;
            }
        }
        
        Ok(result)
    }

    pub async fn get_plugin_metrics(&self, name: &str) -> Result<PluginMetrics, PluginError> {
        let plugins = self.plugins.read().await;
        let plugin = plugins.get(name)
            .ok_or_else(|| PluginError::NotFound(name.to_string()))?;
            
        plugin.metrics().await
    }

    pub async fn update_plugin_config(
        &self,
        name: &str,
        config: PluginConfig,
    ) -> Result<(), PluginError> {
        let mut configs = self.configs.write().await;
        let plugin = self.plugins.read().await.get(name)
            .ok_or_else(|| PluginError::NotFound(name.to_string()))?;
            
        // Update plugin with new config
        plugin.initialize(config.clone()).await?;
        configs.insert(name.to_string(), config);
        
        Ok(())
    }
}

fn calculate_demand(metrics: &PluginMetrics) -> f32 {
    let cpu_factor = metrics.resource_utilization.cpu_usage_percent / 100.0;
    let memory_factor = metrics.resource_utilization.memory_usage_mb as f32 / 1024.0;
    let response_factor = if metrics.average_response_time_ms > 1000.0 { 1.5 } else { 1.0 };
    
    (cpu_factor + memory_factor) * response_factor
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    
    struct TestPlugin {
        metrics: Arc<RwLock<PluginMetrics>>,
    }
    
    #[async_trait]
    impl Plugin for TestPlugin {
        fn info(&self) -> PluginInfo {
            PluginInfo {
                name: "test".to_string(),
                version: "1.0".to_string(),
                description: "Test plugin".to_string(),
                capabilities: vec!["test".to_string()],
            }
        }
        
        async fn initialize(&self, _config: PluginConfig) -> Result<(), PluginError> {
            Ok(())
        }
        
        async fn execute(&self, input: serde_json::Value) -> Result<serde_json::Value, PluginError> {
            Ok(input)
        }
        
        async fn metrics(&self) -> Result<PluginMetrics, PluginError> {
            Ok(self.metrics.read().await.clone())
        }
        
        async fn scale(&self, _demand: f32) -> Result<(), PluginError> {
            Ok(())
        }
        
        async fn cleanup(&self) -> Result<(), PluginError> {
            Ok(())
        }
    }
    
    #[tokio::test]
    async fn test_plugin_manager() {
        let manager = PluginManager::new();
        
        let test_metrics = PluginMetrics {
            requests_processed: 100,
            average_response_time_ms: 50.0,
            error_rate: 0.01,
            resource_utilization: ResourceUtilization {
                memory_usage_mb: 512,
                cpu_usage_percent: 50.0,
                storage_usage_gb: 1.0,
            },
        };
        
        let plugin = TestPlugin {
            metrics: Arc::new(RwLock::new(test_metrics.clone())),
        };
        
        let config = PluginConfig {
            name: "test".to_string(),
            version: "1.0".to_string(),
            enabled: true,
            auto_scale: true,
            rate_limits: RateLimitConfig {
                requests_per_second: 100,
                burst_size: 10,
            },
            resource_limits: ResourceLimitConfig {
                max_memory_mb: 1024,
                max_cpu_percent: 80,
                max_storage_gb: 10,
            },
        };
        
        // Test plugin registration
        manager.register_plugin("test".to_string(), Box::new(plugin), config).await.unwrap();
        
        // Test plugin execution
        let input = serde_json::json!({"test": "data"});
        let result = manager.execute_plugin("test", input.clone()).await.unwrap();
        assert_eq!(result, input);
        
        // Test metrics
        let metrics = manager.get_plugin_metrics("test").await.unwrap();
        assert_eq!(metrics.requests_processed, test_metrics.requests_processed);
    }
}
