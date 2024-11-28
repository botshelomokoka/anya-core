use super::*;
use thiserror::Error;
use std::collections::HashSet;

#[derive(Error, Debug)]
pub enum ConfigValidationError {
    #[error("Missing required configuration: {0}")]
    MissingRequired(String),
    #[error("Invalid value for {0}: {1}")]
    InvalidValue(String, String),
    #[error("Security configuration error: {0}")]
    SecurityError(String),
}

pub struct ConfigValidator;

impl ConfigValidator {
    pub fn validate_app_config(config: &AppConfig) -> Result<(), ConfigValidationError> {
        // Validate Network Configuration
        Self::validate_network_config(&config.network)?;
        
        // Validate DAO Configuration
        Self::validate_dao_config(&config.dao)?;
        
        // Validate NPU Configuration
        Self::validate_npu_config(&config.npu)?;
        
        // Validate Agent Configuration
        Self::validate_agent_config(&config.agent)?;
        
        // Validate Feature Flags
        Self::validate_feature_flags(&config.features)?;
        
        Ok(())
    }

    fn validate_network_config(config: &NetworkConfig) -> Result<(), ConfigValidationError> {
        if config.capacity == 0 {
            return Err(ConfigValidationError::InvalidValue(
                "network.capacity".to_string(),
                "must be greater than 0".to_string(),
            ));
        }

        if config.node_connection_limit == 0 {
            return Err(ConfigValidationError::InvalidValue(
                "network.node_connection_limit".to_string(),
                "must be greater than 0".to_string(),
            ));
        }

        if !(0.0..=1.0).contains(&config.performance_threshold) {
            return Err(ConfigValidationError::InvalidValue(
                "network.performance_threshold".to_string(),
                "must be between 0 and 1".to_string(),
            ));
        }

        Ok(())
    }

    fn validate_dao_config(config: &DAOConfig) -> Result<(), ConfigValidationError> {
        if config.contract_name.is_empty() {
            return Err(ConfigValidationError::MissingRequired(
                "dao.contract_name".to_string(),
            ));
        }

        if config.proposal_threshold == 0 {
            return Err(ConfigValidationError::InvalidValue(
                "dao.proposal_threshold".to_string(),
                "must be greater than 0".to_string(),
            ));
        }

        if config.voting_period_blocks == 0 {
            return Err(ConfigValidationError::InvalidValue(
                "dao.voting_period_blocks".to_string(),
                "must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }

    fn validate_npu_config(config: &NPUConfig) -> Result<(), ConfigValidationError> {
        if config.capacity_gb <= 0.0 {
            return Err(ConfigValidationError::InvalidValue(
                "npu.capacity_gb".to_string(),
                "must be greater than 0".to_string(),
            ));
        }

        if config.pipeline_depth == 0 {
            return Err(ConfigValidationError::InvalidValue(
                "npu.pipeline_depth".to_string(),
                "must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }

    fn validate_agent_config(config: &AgentConfig) -> Result<(), ConfigValidationError> {
        let fields = [
            ("agent.resource_allocation", config.resource_allocation),
            ("agent.maintenance_frequency", config.maintenance_frequency),
            ("agent.update_aggressiveness", config.update_aggressiveness),
            ("agent.security_level", config.security_level),
        ];

        for (field, value) in fields.iter() {
            if !(0.0..=1.0).contains(value) {
                return Err(ConfigValidationError::InvalidValue(
                    field.to_string(),
                    "must be between 0 and 1".to_string(),
                ));
            }
        }

        Ok(())
    }

    fn validate_feature_flags(config: &FeatureFlags) -> Result<(), ConfigValidationError> {
        // Validate feature flag combinations
        if config.experimental_ml && !config.advanced_optimization {
            return Err(ConfigValidationError::InvalidValue(
                "features".to_string(),
                "experimental_ml requires advanced_optimization".to_string(),
            ));
        }

        if config.quantum_resistant && !config.enhanced_security {
            return Err(ConfigValidationError::InvalidValue(
                "features".to_string(),
                "quantum_resistant requires enhanced_security".to_string(),
            ));
        }

        Ok(())
    }

    pub fn validate_environment_variables() -> Result<(), ConfigValidationError> {
        let required_vars = [
            "ANYA_BITCOIN_RPC_URL",
            "ANYA_WEB5_DWN_URL",
            "ANYA_WEB5_STORAGE_PATH",
        ];

        let mut missing = Vec::new();
        for var in required_vars.iter() {
            if std::env::var(var).is_err() {
                missing.push(*var);
            }
        }

        if !missing.is_empty() {
            return Err(ConfigValidationError::MissingRequired(
                format!("Missing environment variables: {}", missing.join(", "))
            ));
        }

        Ok(())
    }
}
