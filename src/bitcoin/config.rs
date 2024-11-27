use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use std::error::Error;
use bitcoin::Network;
use thiserror::Error;
use directories::BaseDirs;

#[derive(Debug, Error)]
pub enum BitcoinConfigError {
    #[error("Failed to detect Bitcoin Core installation: {0}")]
    DetectionError(String),
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitcoinConfig {
    pub network: Network,
    pub connection_type: BitcoinConnectionType,
    pub custom_settings: Option<CustomBitcoinSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BitcoinConnectionType {
    LocalCore,
    CustomConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomBitcoinSettings {
    pub rpc_url: String,
    pub rpc_user: String,
    pub rpc_password: String,
    pub data_dir: PathBuf,
}

impl Default for BitcoinConfig {
    fn default() -> Self {
        Self {
            network: Network::Bitcoin,
            connection_type: BitcoinConnectionType::LocalCore,
            custom_settings: None,
        }
    }
}

impl BitcoinConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_network(mut self, network: Network) -> Self {
        self.network = network;
        self
    }

    pub fn with_custom_settings(mut self, settings: CustomBitcoinSettings) -> Self {
        self.connection_type = BitcoinConnectionType::CustomConfig;
        self.custom_settings = Some(settings);
        self
    }

    pub fn detect_bitcoin_core() -> Result<Option<PathBuf>, BitcoinConfigError> {
        if let Some(base_dirs) = BaseDirs::new() {
            let possible_paths = match std::env::consts::OS {
                "windows" => vec![
                    base_dirs.data_dir().join("Bitcoin"),
                    PathBuf::from("C:\\Program Files\\Bitcoin"),
                    PathBuf::from("C:\\Program Files (x86)\\Bitcoin"),
                ],
                "macos" => vec![
                    base_dirs.home_dir().join("Library/Application Support/Bitcoin"),
                ],
                "linux" => vec![
                    base_dirs.home_dir().join(".bitcoin"),
                    PathBuf::from("/usr/local/bin/bitcoin"),
                ],
                _ => vec![],
            };

            for path in possible_paths {
                if path.exists() && path.join("bitcoin.conf").exists() {
                    return Ok(Some(path));
                }
            }
        }

        Ok(None)
    }

    pub fn load_or_create() -> Result<Self, BitcoinConfigError> {
        let config_path = Self::get_config_path()?;
        
        if config_path.exists() {
            let config_str = fs::read_to_string(&config_path)?;
            Ok(serde_json::from_str(&config_str)
                .map_err(|e| BitcoinConfigError::InvalidConfig(e.to_string()))?)
        } else {
            let config = if let Some(bitcoin_path) = Self::detect_bitcoin_core()? {
                // Bitcoin Core detected, use local installation
                Self {
                    network: Network::Bitcoin,
                    connection_type: BitcoinConnectionType::LocalCore,
                    custom_settings: None,
                }
            } else {
                // No Bitcoin Core detected, use default custom settings
                Self {
                    network: Network::Bitcoin,
                    connection_type: BitcoinConnectionType::CustomConfig,
                    custom_settings: Some(CustomBitcoinSettings {
                        rpc_url: "http://localhost:8332".to_string(),
                        rpc_user: "user".to_string(),
                        rpc_password: "password".to_string(),
                        data_dir: PathBuf::from("./bitcoin_data"),
                    }),
                }
            };

            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<(), BitcoinConfigError> {
        let config_path = Self::get_config_path()?;
        
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let config_str = serde_json::to_string_pretty(self)
            .map_err(|e| BitcoinConfigError::InvalidConfig(e.to_string()))?;
        
        fs::write(&config_path, config_str)?;
        Ok(())
    }

    fn get_config_path() -> Result<PathBuf, BitcoinConfigError> {
        if let Some(proj_dirs) = directories::ProjectDirs::from("com", "anya", "anya-core") {
            Ok(proj_dirs.config_dir().join("bitcoin_config.json"))
        } else {
            Err(BitcoinConfigError::DetectionError(
                "Could not determine config directory".to_string(),
            ))
        }
    }

    pub fn get_connection_info(&self) -> Result<(String, String, String), BitcoinConfigError> {
        match &self.connection_type {
            BitcoinConnectionType::LocalCore => {
                let bitcoin_path = Self::detect_bitcoin_core()?
                    .ok_or_else(|| BitcoinConfigError::DetectionError(
                        "Bitcoin Core installation not found".to_string(),
                    ))?;

                let conf_path = bitcoin_path.join("bitcoin.conf");
                let conf_content = fs::read_to_string(conf_path)?;

                // Parse bitcoin.conf
                let mut rpc_user = String::new();
                let mut rpc_password = String::new();
                let mut rpc_port = "8332";

                for line in conf_content.lines() {
                    let line = line.trim();
                    if line.starts_with("rpcuser=") {
                        rpc_user = line["rpcuser=".len()..].to_string();
                    } else if line.starts_with("rpcpassword=") {
                        rpc_password = line["rpcpassword=".len()..].to_string();
                    } else if line.starts_with("rpcport=") {
                        rpc_port = &line["rpcport=".len()..];
                    }
                }

                if rpc_user.is_empty() || rpc_password.is_empty() {
                    return Err(BitcoinConfigError::InvalidConfig(
                        "Missing RPC credentials in bitcoin.conf".to_string(),
                    ));
                }

                Ok((
                    format!("http://localhost:{}", rpc_port),
                    rpc_user,
                    rpc_password,
                ))
            }
            BitcoinConnectionType::CustomConfig => {
                if let Some(settings) = &self.custom_settings {
                    Ok((
                        settings.rpc_url.clone(),
                        settings.rpc_user.clone(),
                        settings.rpc_password.clone(),
                    ))
                } else {
                    Err(BitcoinConfigError::InvalidConfig(
                        "Custom settings not provided".to_string(),
                    ))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_bitcoin_config() {
        let temp_dir = tempdir().unwrap();
        let custom_settings = CustomBitcoinSettings {
            rpc_url: "http://localhost:8332".to_string(),
            rpc_user: "testuser".to_string(),
            rpc_password: "testpass".to_string(),
            data_dir: temp_dir.path().to_path_buf(),
        };

        let config = BitcoinConfig::new()
            .with_network(Network::Testnet)
            .with_custom_settings(custom_settings.clone());

        assert_eq!(config.network, Network::Testnet);
        assert!(matches!(config.connection_type, BitcoinConnectionType::CustomConfig));
        assert_eq!(
            config.custom_settings.as_ref().unwrap().rpc_url,
            custom_settings.rpc_url
        );
    }

    #[test]
    fn test_save_and_load() -> Result<(), Box<dyn Error>> {
        let temp_dir = tempdir()?;
        let custom_settings = CustomBitcoinSettings {
            rpc_url: "http://localhost:8332".to_string(),
            rpc_user: "testuser".to_string(),
            rpc_password: "testpass".to_string(),
            data_dir: temp_dir.path().to_path_buf(),
        };

        let config = BitcoinConfig::new()
            .with_network(Network::Testnet)
            .with_custom_settings(custom_settings);

        config.save()?;
        let loaded_config = BitcoinConfig::load_or_create()?;

        assert_eq!(loaded_config.network, Network::Testnet);
        assert!(matches!(loaded_config.connection_type, BitcoinConnectionType::CustomConfig));
        
        Ok(())
    }
}
