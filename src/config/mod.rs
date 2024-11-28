use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use config::{Config, ConfigError, Environment, File};
use lazy_static::lazy_static;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkConfig {
    pub capacity: usize,
    pub node_connection_limit: usize,
    pub performance_threshold: f64,
    pub false_positive_threshold: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DAOConfig {
    pub contract_name: String,
    pub governance_token: String,
    pub proposal_threshold: u128,
    pub voting_period_blocks: u64,
    pub timelock_period_blocks: u64,
    pub voting_threshold: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NPUConfig {
    pub capacity_gb: f64,
    pub pipeline_depth: usize,
    pub optimization_level: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentConfig {
    pub resource_allocation: f64,
    pub maintenance_frequency: f64,
    pub update_aggressiveness: f64,
    pub security_level: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FeatureFlags {
    pub experimental_ml: bool,
    pub advanced_optimization: bool,
    pub quantum_resistant: bool,
    pub enhanced_security: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetricsConfig {
    pub max_history_length: usize,
    pub collection_interval_ms: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub network: NetworkConfig,
    pub dao: DAOConfig,
    pub npu: NPUConfig,
    pub agent: AgentConfig,
    pub features: FeatureFlags,
    pub metrics: MetricsConfig,
}

lazy_static! {
    static ref CONFIG: Arc<RwLock<AppConfig>> = Arc::new(RwLock::new(AppConfig::default()));
}

pub mod validator;
use validator::*;

impl AppConfig {
    pub async fn global() -> Arc<RwLock<AppConfig>> {
        CONFIG.clone()
    }

    pub fn load() -> Result<Self, ConfigError> {
        // Validate environment variables first
        ConfigValidator::validate_environment_variables()
            .map_err(|e| ConfigError::Message(e.to_string()))?;

        let mut config = Config::new();

        // Start with default configuration
        config.merge(File::with_name("config/default"))?;

        // Layer with environment specific config
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "development".into());
        config.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        // Add environment variables (prefixed with 'ANYA_')
        config.merge(Environment::with_prefix("ANYA"))?;

        // Convert to AppConfig
        let app_config: AppConfig = config.try_into()?;

        // Validate the final configuration
        ConfigValidator::validate_app_config(&app_config)
            .map_err(|e| ConfigError::Message(e.to_string()))?;

        Ok(app_config)
    }

    pub async fn reload() -> Result<(), ConfigError> {
        let new_config = Self::load()?;
        let mut config = CONFIG.write().await;
        *config = new_config;
        Ok(())
    }

    pub fn default() -> Self {
        Self {
            network: NetworkConfig {
                capacity: 1000,
                node_connection_limit: 100,
                performance_threshold: 0.6,
                false_positive_threshold: 0.7,
            },
            dao: DAOConfig {
                contract_name: "anya-dao".to_string(),
                governance_token: "anya-governance-token".to_string(),
                proposal_threshold: 100_000_000,
                voting_period_blocks: 1008,
                timelock_period_blocks: 288,
                voting_threshold: 0.7,
            },
            npu: NPUConfig {
                capacity_gb: 4.5,
                pipeline_depth: 24,
                optimization_level: 0.5,
            },
            agent: AgentConfig {
                resource_allocation: 0.5,
                maintenance_frequency: 0.3,
                update_aggressiveness: 0.4,
                security_level: 0.7,
            },
            features: FeatureFlags {
                experimental_ml: false,
                advanced_optimization: false,
                quantum_resistant: false,
                enhanced_security: true,
            },
            metrics: MetricsConfig {
                max_history_length: 100,
                collection_interval_ms: 5000,
            },
        }
    }
}

// Dynamic configuration calculations
impl AppConfig {
    pub fn calculate_timelock_period(&self, network_activity: f64) -> u64 {
        let base_period = self.dao.timelock_period_blocks;
        let activity_factor = (network_activity * 2.0).min(2.0).max(0.5);
        (base_period as f64 * activity_factor) as u64
    }

    pub fn calculate_network_limits(&self, system_resources: SystemResources) -> NetworkConfig {
        let memory_factor = (system_resources.available_memory_gb / 8.0).min(2.0).max(0.5);
        let cpu_factor = (system_resources.cpu_usage / 50.0).min(2.0).max(0.5);
        let scaling_factor = memory_factor.min(cpu_factor);

        NetworkConfig {
            capacity: (self.network.capacity as f64 * scaling_factor) as usize,
            node_connection_limit: (self.network.node_connection_limit as f64 * scaling_factor) as usize,
            performance_threshold: self.network.performance_threshold,
            false_positive_threshold: self.network.false_positive_threshold,
        }
    }
}

#[derive(Debug)]
pub struct SystemResources {
    pub available_memory_gb: f64,
    pub cpu_usage: f64,
}

// Secure credential management
pub mod credentials {
    use tokio::sync::RwLock;
    use std::collections::HashMap;
    use std::sync::Arc;
    use secrecy::{Secret, ExposeSecret};

    #[derive(Clone)]
    pub struct Credentials {
        secrets: Arc<RwLock<HashMap<String, Secret<String>>>>,
    }

    impl Credentials {
        pub fn new() -> Self {
            Self {
                secrets: Arc::new(RwLock::new(HashMap::new())),
            }
        }

        pub async fn store(&self, key: &str, value: Secret<String>) {
            let mut secrets = self.secrets.write().await;
            secrets.insert(key.to_string(), value);
        }

        pub async fn get(&self, key: &str) -> Option<Secret<String>> {
            let secrets = self.secrets.read().await;
            secrets.get(key).cloned()
        }

        pub async fn remove(&self, key: &str) {
            let mut secrets = self.secrets.write().await;
            secrets.remove(key);
        }
    }
}
