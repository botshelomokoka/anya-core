use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigManager {
    config: HashMap<String, ConfigValue>,
    environment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum ConfigValue {
    String(String),
    Number(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<ConfigValue>),
    Object(HashMap<String, ConfigValue>),
}

impl ConfigManager {
    pub fn new(environment: &str) -> Self {
        Self {
            config: HashMap::new(),
            environment: environment.to_string(),
        }
    }

    pub fn load_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Load default config
        let default_config = self.load_config_file("config/default.yaml")?;
        self.config.extend(default_config);

        // Load environment specific config
        let env_config = self.load_config_file(&format!("config/{}.yaml", self.environment))?;
        self.config.extend(env_config);

        // Load local overrides if they exist
        let local_config_path = "config/local.yaml";
        if Path::new(local_config_path).exists() {
            let local_config = self.load_config_file(local_config_path)?;
            self.config.extend(local_config);
        }

        Ok(())
    }

    fn load_config_file(&self, path: &str) -> Result<HashMap<String, ConfigValue>, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: HashMap<String, ConfigValue> = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        match self.config.get(key) {
            Some(ConfigValue::String(s)) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn get_number(&self, key: &str) -> Option<i64> {
        match self.config.get(key) {
            Some(ConfigValue::Number(n)) => Some(*n),
            _ => None,
        }
    }

    pub fn get_boolean(&self, key: &str) -> Option<bool> {
        match self.config.get(key) {
            Some(ConfigValue::Boolean(b)) => Some(*b),
            _ => None,
        }
    }

    pub fn validate_required_keys(&self, required_keys: &[&str]) -> Result<(), String> {
        for key in required_keys {
            if !self.config.contains_key(*key) {
                return Err(format!("Missing required configuration key: {}", key));
            }
        }
        Ok(())
    }
}

// Global configuration instance
lazy_static! {
    pub static ref CONFIG: Arc<RwLock<ConfigManager>> = Arc::new(RwLock::new(
        ConfigManager::new(&std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()))
    ));
}

// Helper function to get config instance
pub async fn get_config() -> Arc<RwLock<ConfigManager>> {
    CONFIG.clone()
}
