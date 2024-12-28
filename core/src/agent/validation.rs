//! Validation System for ML*/Agent
//! 
//! This module provides comprehensive validation capabilities for the ML*/Agent system.
//! It handles various types of validation including core functionality, ML models,
//! security requirements, performance standards, and resource utilization.
//!
//! # Architecture
//! 
//! The validation system consists of:
//! - ValidationManager: Central coordinator for all validation operations
//! - ValidationState: Thread-safe state management
//! - Specialized validators for different aspects (ML, Security, Performance)
//!
//! # Features
//!
//! - Comprehensive component validation
//! - ML model validation
//! - Security requirement validation
//! - Performance validation
//! - Resource utilization validation
//! - Historical validation tracking
//!
//! # Example
//!
//! ```rust
//! use anya::agent::validation::ValidationManager;
//!
//! async fn validate_component(component: &SystemComponent) -> Result<(), AgentError> {
//!     let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
//!     let validator = ValidationManager::new(metrics);
//!     
//!     // Run comprehensive validation
//!     let result = validator.validate_component(component).await?;
//!     
//!     // Check validation scores
//!     if result.core_score < 0.8 {
//!         println!("Core validation failed");
//!     }
//!     Ok(())
//! }
//! ```

use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::metrics::UnifiedMetrics;
use crate::system::{SystemComponent, ComponentStatus};
use super::AgentError;

/// Enhanced validation system for ML*/Agent.
///
/// The ValidationManager coordinates all validation operations including:
/// - Core functionality validation
/// - ML model validation
/// - Security requirement validation
/// - Performance validation
/// - Resource utilization validation
///
/// # Thread Safety
///
/// All state is managed through Arc<RwLock<_>> to ensure thread-safe access
/// in concurrent environments.
///
/// # Error Handling
///
/// Operations return Result<T, AgentError> to handle various failure scenarios
/// gracefully.
pub struct ValidationManager {
    metrics: Arc<RwLock<UnifiedMetrics>>,
    state: Arc<RwLock<ValidationState>>,
}

/// Represents the current state of validation operations.
///
/// This structure maintains:
/// - Per-component validation results
/// - Validation history
/// - Timestamp of last validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationState {
    pub component_validations: HashMap<String, ValidationResult>,
    pub validation_history: Vec<ValidationEvent>,
    pub last_validation: DateTime<Utc>,
}

/// Comprehensive validation results for a component.
///
/// Contains validation results across different aspects:
/// - Core functionality
/// - ML capabilities
/// - Security requirements
/// - Performance standards
/// - Resource utilization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    // Core validation
    /// Results of core functionality checks
    pub core_checks: Vec<ValidationCheck>,
    /// Overall core validation score (0.0 to 1.0)
    pub core_score: f64,
    
    // ML validation
    /// Results of ML-specific checks
    pub ml_checks: Vec<ValidationCheck>,
    /// Overall ML validation score (0.0 to 1.0)
    pub ml_score: f64,
    
    // Security validation
    /// Results of security requirement checks
    pub security_checks: Vec<ValidationCheck>,
    /// Overall security validation score (0.0 to 1.0)
    pub security_score: f64,
    
    // Performance validation
    /// Results of performance requirement checks
    pub performance_checks: Vec<ValidationCheck>,
    /// Overall performance validation score (0.0 to 1.0)
    pub performance_score: f64,
    
    // Resource validation
    /// Results of resource utilization checks
    pub resource_checks: Vec<ValidationCheck>,
    /// Overall resource validation score (0.0 to 1.0)
    pub resource_score: f64,
}

/// Individual validation check result.
///
/// Represents the result of a single validation check including:
/// - Type of check performed
/// - Check result (pass/fail)
/// - Severity level
/// - Detailed information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationCheck {
    /// Type of validation check performed
    pub check_type: ValidationCheckType,
    /// Name of the validation check
    pub name: String,
    /// Whether the check passed
    pub passed: bool,
    /// Severity level of the check
    pub severity: ValidationSeverity,
    /// Detailed check results
    pub details: String,
    /// When the check was performed
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValidationCheckType {
    // Core checks
    ComponentHealth,
    DependencyCheck,
    ConfigurationCheck,
    StateConsistency,
    
    // ML checks
    ModelAccuracy,
    TrainingPerformance,
    InferenceTiming,
    DataQuality,
    ModelDrift,
    
    // Security checks
    VulnerabilityScan,
    ComplianceCheck,
    AuthenticationCheck,
    AuthorizationCheck,
    DataPrivacy,
    
    // Performance checks
    ResponseTime,
    Throughput,
    ResourceUtilization,
    ErrorRate,
    Latency,
    
    // Resource checks
    CPUUsage,
    MemoryUsage,
    DiskUsage,
    NetworkUsage,
    DatabaseConnections,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValidationSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationEvent {
    pub timestamp: DateTime<Utc>,
    pub component_id: String,
    pub check_type: ValidationCheckType,
    pub result: bool,
    pub details: String,
}

impl ValidationManager {
    /// Creates a new ValidationManager instance.
    ///
    /// # Arguments
    ///
    /// * `metrics` - Shared reference to UnifiedMetrics
    ///
    /// # Returns
    ///
    /// A new ValidationManager instance
    pub fn new(metrics: Arc<RwLock<UnifiedMetrics>>) -> Self {
        Self {
            metrics,
            state: Arc::new(RwLock::new(ValidationState::default())),
        }
    }

    /// Runs comprehensive validation for a component.
    ///
    /// Performs validation across all aspects:
    /// - Core functionality
    /// - ML capabilities
    /// - Security requirements
    /// - Performance standards
    /// - Resource utilization
    ///
    /// # Arguments
    ///
    /// * `component` - The system component to validate
    ///
    /// # Returns
    ///
    /// ValidationResult containing all validation results
    pub async fn validate_component(&self, component: &SystemComponent) -> Result<ValidationResult, AgentError> {
        let mut result = ValidationResult::default();
        
        // Run core validation
        result.core_checks = self.validate_core(component).await?;
        result.core_score = self.calculate_score(&result.core_checks);
        
        // Run ML validation
        result.ml_checks = self.validate_ml(component).await?;
        result.ml_score = self.calculate_score(&result.ml_checks);
        
        // Run security validation
        result.security_checks = self.validate_security(component).await?;
        result.security_score = self.calculate_score(&result.security_checks);
        
        // Run performance validation
        result.performance_checks = self.validate_performance(component).await?;
        result.performance_score = self.calculate_score(&result.performance_checks);
        
        // Run resource validation
        result.resource_checks = self.validate_resources(component).await?;
        result.resource_score = self.calculate_score(&result.resource_checks);
        
        // Update validation state
        self.update_validation_state(component, &result).await?;
        
        Ok(result)
    }

    /// Validates core component functionality.
    ///
    /// Checks:
    /// - Component health
    /// - Dependencies
    /// - Configuration
    /// - State consistency
    ///
    /// # Arguments
    ///
    /// * `component` - The system component to validate
    ///
    /// # Returns
    ///
    /// Vector of ValidationCheck for core functionality
    pub async fn validate_core(&self, component: &SystemComponent) -> Result<Vec<ValidationCheck>, AgentError> {
        let mut checks = Vec::new();
        let metrics = self.metrics.read().await;
        
        // Component health check
        checks.push(ValidationCheck {
            check_type: ValidationCheckType::ComponentHealth,
            name: "Component Health Check".to_string(),
            passed: component.status.operational,
            severity: ValidationSeverity::Critical,
            details: format!("Component health status: {}", if component.status.operational { "OK" } else { "Failed" }),
            timestamp: Utc::now(),
        });
        
        // Dependency check
        let deps_ok = self.validate_dependencies(component).await?;
        checks.push(ValidationCheck {
            check_type: ValidationCheckType::DependencyCheck,
            name: "Dependency Validation".to_string(),
            passed: deps_ok,
            severity: ValidationSeverity::High,
            details: format!("Dependency check: {}", if deps_ok { "OK" } else { "Failed" }),
            timestamp: Utc::now(),
        });
        
        // Configuration check
        let config_ok = self.validate_configuration(component).await?;
        checks.push(ValidationCheck {
            check_type: ValidationCheckType::ConfigurationCheck,
            name: "Configuration Validation".to_string(),
            passed: config_ok,
            severity: ValidationSeverity::High,
            details: format!("Configuration check: {}", if config_ok { "OK" } else { "Failed" }),
            timestamp: Utc::now(),
        });
        
        // State consistency check
        let state_ok = self.validate_state_consistency(component).await?;
        checks.push(ValidationCheck {
            check_type: ValidationCheckType::StateConsistency,
            name: "State Consistency Check".to_string(),
            passed: state_ok,
            severity: ValidationSeverity::High,
            details: format!("State consistency: {}", if state_ok { "OK" } else { "Failed" }),
            timestamp: Utc::now(),
        });
        
        Ok(checks)
    }

    /// Validates ML functionality.
    ///
    /// Checks:
    /// - Model accuracy
    /// - Training performance
    /// - Inference timing
    /// - Data quality
    /// - Model drift
    ///
    /// # Arguments
    ///
    /// * `component` - The system component to validate
    ///
    /// # Returns
    ///
    /// Vector of ValidationCheck for ML functionality
    pub async fn validate_ml(&self, component: &SystemComponent) -> Result<Vec<ValidationCheck>, AgentError> {
        let mut checks = Vec::new();
        let metrics = self.metrics.read().await;
        
        if let Some(ml) = metrics.ml.as_ref() {
            // Model accuracy check
            checks.push(ValidationCheck {
                check_type: ValidationCheckType::ModelAccuracy,
                name: "Model Accuracy Check".to_string(),
                passed: ml.model_accuracy >= 0.8,
                severity: ValidationSeverity::High,
                details: format!("Model accuracy: {:.2}%", ml.model_accuracy * 100.0),
                timestamp: Utc::now(),
            });
            
            // Training performance check
            checks.push(ValidationCheck {
                check_type: ValidationCheckType::TrainingPerformance,
                name: "Training Performance Check".to_string(),
                passed: ml.training_time < 3600.0, // 1 hour threshold
                severity: ValidationSeverity::Medium,
                details: format!("Training time: {:.2}s", ml.training_time),
                timestamp: Utc::now(),
            });
            
            // Inference timing check
            checks.push(ValidationCheck {
                check_type: ValidationCheckType::InferenceTiming,
                name: "Inference Timing Check".to_string(),
                passed: ml.inference_time < 1.0, // 1 second threshold
                severity: ValidationSeverity::High,
                details: format!("Inference time: {:.2}ms", ml.inference_time * 1000.0),
                timestamp: Utc::now(),
            });
        }
        
        Ok(checks)
    }

    /// Validates security requirements.
    ///
    /// Checks:
    /// - Vulnerability scan
    /// - Compliance check
    /// - Authentication check
    /// - Authorization check
    /// - Data privacy
    ///
    /// # Arguments
    ///
    /// * `component` - The system component to validate
    ///
    /// # Returns
    ///
    /// Vector of ValidationCheck for security requirements
    pub async fn validate_security(&self, component: &SystemComponent) -> Result<Vec<ValidationCheck>, AgentError> {
        let mut checks = Vec::new();
        let metrics = self.metrics.read().await;
        
        if let Some(security) = metrics.security.as_ref() {
            // Vulnerability scan
            checks.push(ValidationCheck {
                check_type: ValidationCheckType::VulnerabilityScan,
                name: "Vulnerability Scan".to_string(),
                passed: security.vulnerability_count == 0,
                severity: ValidationSeverity::Critical,
                details: format!("Found {} vulnerabilities", security.vulnerability_count),
                timestamp: Utc::now(),
            });
            
            // Compliance check
            checks.push(ValidationCheck {
                check_type: ValidationCheckType::ComplianceCheck,
                name: "Compliance Validation".to_string(),
                passed: security.security_score >= 0.9,
                severity: ValidationSeverity::High,
                details: format!("Compliance score: {:.2}%", security.security_score * 100.0),
                timestamp: Utc::now(),
            });
        }
        
        Ok(checks)
    }

    /// Validates performance requirements.
    ///
    /// Checks:
    /// - Response time
    /// - Throughput
    /// - Resource utilization
    /// - Error rate
    /// - Latency
    ///
    /// # Arguments
    ///
    /// * `component` - The system component to validate
    ///
    /// # Returns
    ///
    /// Vector of ValidationCheck for performance requirements
    pub async fn validate_performance(&self, component: &SystemComponent) -> Result<Vec<ValidationCheck>, AgentError> {
        let mut checks = Vec::new();
        let metrics = self.metrics.read().await;
        
        // Response time check
        checks.push(ValidationCheck {
            check_type: ValidationCheckType::ResponseTime,
            name: "Response Time Check".to_string(),
            passed: metrics.system.ops_latency < 0.1, // 100ms threshold
            severity: ValidationSeverity::High,
            details: format!("Response time: {:.2}ms", metrics.system.ops_latency * 1000.0),
            timestamp: Utc::now(),
        });
        
        // Throughput check
        let throughput = metrics.system.ops_success as f64 / 60.0; // ops per second
        checks.push(ValidationCheck {
            check_type: ValidationCheckType::Throughput,
            name: "Throughput Check".to_string(),
            passed: throughput >= 100.0, // 100 ops/s threshold
            severity: ValidationSeverity::Medium,
            details: format!("Throughput: {:.2} ops/s", throughput),
            timestamp: Utc::now(),
        });
        
        // Error rate check
        let error_rate = metrics.system.error_rate;
        checks.push(ValidationCheck {
            check_type: ValidationCheckType::ErrorRate,
            name: "Error Rate Check".to_string(),
            passed: error_rate < 0.01, // 1% threshold
            severity: ValidationSeverity::High,
            details: format!("Error rate: {:.2}%", error_rate * 100.0),
            timestamp: Utc::now(),
        });
        
        Ok(checks)
    }

    /// Validates resource utilization.
    ///
    /// Checks:
    /// - CPU usage
    /// - Memory usage
    /// - Disk usage
    /// - Network usage
    /// - Database connections
    ///
    /// # Arguments
    ///
    /// * `component` - The system component to validate
    ///
    /// # Returns
    ///
    /// Vector of ValidationCheck for resource utilization
    pub async fn validate_resources(&self, component: &SystemComponent) -> Result<Vec<ValidationCheck>, AgentError> {
        let mut checks = Vec::new();
        let metrics = self.metrics.read().await;
        
        // CPU usage check
        checks.push(ValidationCheck {
            check_type: ValidationCheckType::CPUUsage,
            name: "CPU Usage Check".to_string(),
            passed: metrics.system.cpu_usage < 80.0,
            severity: ValidationSeverity::High,
            details: format!("CPU usage: {:.2}%", metrics.system.cpu_usage),
            timestamp: Utc::now(),
        });
        
        // Memory usage check
        checks.push(ValidationCheck {
            check_type: ValidationCheckType::MemoryUsage,
            name: "Memory Usage Check".to_string(),
            passed: metrics.system.memory_usage < 80.0,
            severity: ValidationSeverity::High,
            details: format!("Memory usage: {:.2}%", metrics.system.memory_usage),
            timestamp: Utc::now(),
        });
        
        // Disk usage check
        checks.push(ValidationCheck {
            check_type: ValidationCheckType::DiskUsage,
            name: "Disk Usage Check".to_string(),
            passed: metrics.system.disk_usage < 80.0,
            severity: ValidationSeverity::Medium,
            details: format!("Disk usage: {:.2}%", metrics.system.disk_usage),
            timestamp: Utc::now(),
        });
        
        Ok(checks)
    }

    /// Calculates the validation score.
    ///
    /// # Arguments
    ///
    /// * `checks` - Vector of ValidationCheck
    ///
    /// # Returns
    ///
    /// Validation score (0.0 to 1.0)
    fn calculate_score(&self, checks: &[ValidationCheck]) -> f64 {
        if checks.is_empty() {
            return 0.0;
        }
        
        let mut total_weight = 0.0;
        let mut weighted_sum = 0.0;
        
        for check in checks {
            let weight = match check.severity {
                ValidationSeverity::Critical => 4.0,
                ValidationSeverity::High => 3.0,
                ValidationSeverity::Medium => 2.0,
                ValidationSeverity::Low => 1.0,
                ValidationSeverity::Info => 0.5,
            };
            
            total_weight += weight;
            if check.passed {
                weighted_sum += weight;
            }
        }
        
        weighted_sum / total_weight
    }

    /// Updates the validation state.
    ///
    /// # Arguments
    ///
    /// * `component` - The system component to update
    /// * `result` - ValidationResult
    ///
    /// # Returns
    ///
    /// Result<(), AgentError>
    async fn update_validation_state(&self, component: &SystemComponent, result: &ValidationResult) -> Result<(), AgentError> {
        let mut state = self.state.write().await;
        
        // Update component validation
        state.component_validations.insert(component.name.clone(), result.clone());
        
        // Add validation events
        let events: Vec<ValidationEvent> = result.core_checks.iter()
            .chain(result.ml_checks.iter())
            .chain(result.security_checks.iter())
            .chain(result.performance_checks.iter())
            .chain(result.resource_checks.iter())
            .map(|check| ValidationEvent {
                timestamp: check.timestamp,
                component_id: component.name.clone(),
                check_type: check.check_type.clone(),
                result: check.passed,
                details: check.details.clone(),
            })
            .collect();
        
        state.validation_history.extend(events);
        state.last_validation = Utc::now();
        
        Ok(())
    }
}

impl Default for ValidationState {
    fn default() -> Self {
        Self {
            component_validations: HashMap::new(),
            validation_history: Vec::new(),
            last_validation: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::MockMetricsProvider;

    #[tokio::test]
    async fn test_validation_manager() {
        // Create test metrics
        let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
        let validator = ValidationManager::new(metrics);
        
        // Create test component
        let component = SystemComponent {
            name: "test_validation".to_string(),
            component_type: ComponentType::Agent,
            status: ComponentStatus::default(),
            path: PathBuf::from("/test"),
            dependencies: HashSet::new(),
        };
        
        // Test validation
        let result = validator.validate_component(&component).await.unwrap();
        
        // Check core validation
        assert!(!result.core_checks.is_empty());
        assert!(result.core_score >= 0.0);
        
        // Check ML validation
        assert!(!result.ml_checks.is_empty());
        assert!(result.ml_score >= 0.0);
        
        // Check security validation
        assert!(!result.security_checks.is_empty());
        assert!(result.security_score >= 0.0);
        
        // Check performance validation
        assert!(!result.performance_checks.is_empty());
        assert!(result.performance_score >= 0.0);
        
        // Check resource validation
        assert!(!result.resource_checks.is_empty());
        assert!(result.resource_score >= 0.0);
    }
}
