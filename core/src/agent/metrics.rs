//! Metrics Collection System for ML*/Agent
//! 
//! This module provides comprehensive metrics collection and analysis capabilities for the ML*/Agent system.
//! It handles various types of metrics including performance, resource utilization, ML-specific metrics,
//! and security metrics across different stages of development, production, and release.
//!
//! # Architecture
//! 
//! The metrics system is built around three main components:
//! - MetricsCollector: Central coordinator for metrics collection and analysis
//! - PerformanceTracker: Specialized component for performance monitoring
//! - MetricsState: Thread-safe state management for metrics data
//!
//! # Features
//!
//! - Real-time metrics collection and analysis
//! - Stage-specific metrics (development, production, release)
//! - ML-specific metrics tracking
//! - Resource utilization monitoring
//! - Performance bottleneck detection
//! - Security and compliance metrics
//!
//! # Example
//!
//! ```rust
//! use anya::agent::metrics::MetricsCollector;
//!
//! async fn collect_metrics(component: &SystemComponent) -> Result<(), AgentError> {
//!     let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
//!     let collector = MetricsCollector::new(metrics);
//!     
//!     // Collect development metrics
//!     let dev_metrics = collector.collect_development_metrics(component).await?;
//!     
//!     // Analyze performance
//!     let analysis = collector.analyze_performance(component, &dev_metrics).await?;
//!     Ok(())
//! }
//! ```

use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::metrics::UnifiedMetrics;
use crate::system::SystemComponent;
use super::AgentError;

/// Enhanced metrics collection for ML*/Agent system.
///
/// The MetricsCollector is responsible for gathering, analyzing, and managing various
/// types of metrics across different stages of component lifecycle. It provides:
///
/// - Comprehensive metrics collection across different stages
/// - Real-time performance analysis
/// - Resource utilization tracking
/// - ML-specific metrics monitoring
/// - Security and compliance metrics
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
pub struct MetricsCollector {
    metrics: Arc<RwLock<UnifiedMetrics>>,
    performance_tracker: Arc<PerformanceTracker>,
    state: Arc<RwLock<MetricsState>>,
}

/// Represents the current state of metrics collection and analysis.
///
/// This structure maintains:
/// - Per-component metrics
/// - Historical performance data
/// - Timestamp of last collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsState {
    pub component_metrics: HashMap<String, ComponentMetrics>,
    pub performance_history: Vec<PerformanceSnapshot>,
    pub last_collection: DateTime<Utc>,
}

/// Comprehensive metrics for a single component.
///
/// Tracks various aspects of component health and performance:
/// - Core performance metrics
/// - Resource utilization
/// - Task processing statistics
/// - ML-specific metrics
/// - Security metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentMetrics {
    // Core metrics
    /// Overall performance score (0.0 to 1.0)
    pub performance_score: f64,
    /// System reliability score (0.0 to 1.0)
    pub reliability_score: f64,
    /// Security assessment score (0.0 to 1.0)
    pub security_score: f64,
    /// Code test coverage percentage (0.0 to 1.0)
    pub test_coverage: f64,
    /// System error rate (0.0 to 1.0)
    pub error_rate: f64,
    
    // Resource metrics
    /// CPU utilization percentage (0.0 to 100.0)
    pub cpu_usage: f64,
    /// Memory utilization percentage (0.0 to 100.0)
    pub memory_usage: f64,
    /// Disk space utilization percentage (0.0 to 100.0)
    pub disk_usage: f64,
    /// Network bandwidth utilization percentage (0.0 to 100.0)
    pub network_usage: f64,
    
    // Task metrics
    /// Number of successfully completed tasks
    pub tasks_completed: u64,
    /// Number of failed tasks
    pub tasks_failed: u64,
    /// Average time to complete a task (in seconds)
    pub average_task_time: f64,
    /// Current size of task queue
    pub task_queue_size: u64,
    
    // ML metrics
    /// Model prediction accuracy (0.0 to 1.0)
    pub model_accuracy: Option<f64>,
    /// Time spent in model training (in seconds)
    pub training_time: Option<f64>,
    /// Average inference time per request (in seconds)
    pub inference_time: Option<f64>,
    
    // Security metrics
    /// Number of detected vulnerabilities
    pub vulnerability_count: Option<u32>,
    /// Security audit score (0.0 to 1.0)
    pub audit_score: Option<f64>,
    /// Compliance score (0.0 to 1.0)
    pub compliance_score: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSnapshot {
    pub timestamp: DateTime<Utc>,
    pub component_id: String,
    pub metrics: ComponentMetrics,
    pub analysis: PerformanceAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAnalysis {
    pub health_score: f64,
    pub bottlenecks: Vec<PerformanceBottleneck>,
    pub recommendations: Vec<String>,
    pub trend: PerformanceTrend,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBottleneck {
    pub resource_type: ResourceType,
    pub severity: f64,
    pub description: String,
    pub potential_impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceType {
    CPU,
    Memory,
    Disk,
    Network,
    Database,
    Cache,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceTrend {
    Improving,
    Stable,
    Degrading,
    Critical,
}

impl MetricsCollector {
    /// Creates a new MetricsCollector instance.
    ///
    /// # Arguments
    ///
    /// * `metrics` - Shared reference to UnifiedMetrics
    ///
    /// # Returns
    ///
    /// A new MetricsCollector instance
    pub fn new(metrics: Arc<RwLock<UnifiedMetrics>>) -> Self {
        Self {
            metrics,
            performance_tracker: Arc::new(PerformanceTracker::new()),
            state: Arc::new(RwLock::new(MetricsState::default())),
        }
    }

    /// Collects metrics specific to development environment.
    ///
    /// Focuses on metrics relevant during development:
    /// - Test coverage
    /// - Code quality metrics
    /// - Development environment performance
    /// - ML model training metrics
    ///
    /// # Arguments
    ///
    /// * `component` - The system component to collect metrics for
    ///
    /// # Returns
    ///
    /// ComponentMetrics containing development-specific metrics
    pub async fn collect_development_metrics(&self, component: &SystemComponent) -> Result<ComponentMetrics, AgentError> {
        let mut metrics = self.collect_base_metrics(component).await?;
        
        // Add development-specific metrics
        metrics.test_coverage *= 0.6; // Development stage factor
        metrics.security_score *= 0.6;
        metrics.performance_score *= 0.6;
        
        self.update_metrics_state(component, &metrics).await?;
        Ok(metrics)
    }

    /// Collects metrics specific to production environment.
    ///
    /// Focuses on metrics relevant in production:
    /// - System performance
    /// - Resource utilization
    /// - Error rates
    /// - User-facing metrics
    ///
    /// # Arguments
    ///
    /// * `component` - The system component to collect metrics for
    ///
    /// # Returns
    ///
    /// ComponentMetrics containing production-specific metrics
    pub async fn collect_production_metrics(&self, component: &SystemComponent) -> Result<ComponentMetrics, AgentError> {
        let mut metrics = self.collect_base_metrics(component).await?;
        
        // Add production-specific metrics
        metrics.test_coverage *= 0.9; // Production stage factor
        metrics.security_score *= 0.9;
        metrics.performance_score *= 0.9;
        
        self.update_metrics_state(component, &metrics).await?;
        Ok(metrics)
    }

    /// Collects metrics specific to release environment.
    ///
    /// Focuses on metrics relevant during release:
    /// - System performance
    /// - Resource utilization
    /// - Error rates
    /// - User-facing metrics
    ///
    /// # Arguments
    ///
    /// * `component` - The system component to collect metrics for
    ///
    /// # Returns
    ///
    /// ComponentMetrics containing release-specific metrics
    pub async fn collect_release_metrics(&self, component: &SystemComponent) -> Result<ComponentMetrics, AgentError> {
        let mut metrics = self.collect_base_metrics(component).await?;
        
        // Add release-specific metrics
        metrics.test_coverage *= 0.99; // Release stage factor
        metrics.security_score *= 0.99;
        metrics.performance_score *= 0.99;
        
        self.update_metrics_state(component, &metrics).await?;
        Ok(metrics)
    }

    /// Collects base metrics common to all stages.
    ///
    /// # Arguments
    ///
    /// * `component` - The system component to collect metrics for
    ///
    /// # Returns
    ///
    /// ComponentMetrics containing base metrics
    async fn collect_base_metrics(&self, component: &SystemComponent) -> Result<ComponentMetrics, AgentError> {
        // Get current unified metrics
        let unified_metrics = self.metrics.read().await;
        
        // Get performance snapshot
        let performance = self.performance_tracker
            .collect_performance_metrics(component)
            .await?;
        
        // Create component metrics
        let metrics = ComponentMetrics {
            performance_score: performance.health_score,
            reliability_score: self.calculate_reliability_score(&unified_metrics, component),
            security_score: self.calculate_security_score(&unified_metrics, component),
            test_coverage: self.calculate_test_coverage(&unified_metrics, component),
            error_rate: self.calculate_error_rate(&unified_metrics, component),
            
            cpu_usage: performance.cpu_usage,
            memory_usage: performance.memory_usage,
            disk_usage: performance.disk_usage,
            network_usage: performance.network_usage,
            
            tasks_completed: performance.tasks_completed,
            tasks_failed: performance.tasks_failed,
            average_task_time: performance.average_task_time,
            task_queue_size: performance.task_queue_size,
            
            model_accuracy: unified_metrics.ml.as_ref().map(|m| m.model_accuracy),
            training_time: unified_metrics.ml.as_ref().map(|m| m.training_time),
            inference_time: unified_metrics.ml.as_ref().map(|m| m.inference_time),
            
            vulnerability_count: unified_metrics.security.as_ref().map(|s| s.vulnerability_count as u32),
            audit_score: unified_metrics.security.as_ref().map(|s| s.security_score),
            compliance_score: unified_metrics.validation.as_ref().map(|v| v.validation_score),
        };
        
        Ok(metrics)
    }

    /// Updates the metrics state with new component metrics.
    ///
    /// # Arguments
    ///
    /// * `component` - The system component to update metrics for
    /// * `metrics` - The new component metrics
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    async fn update_metrics_state(&self, component: &SystemComponent, metrics: &ComponentMetrics) -> Result<(), AgentError> {
        let mut state = self.state.write().await;
        
        // Update component metrics
        state.component_metrics.insert(component.name.clone(), metrics.clone());
        
        // Create performance snapshot
        let snapshot = PerformanceSnapshot {
            timestamp: Utc::now(),
            component_id: component.name.clone(),
            metrics: metrics.clone(),
            analysis: self.analyze_performance(component, metrics).await?,
        };
        
        // Add to history
        state.performance_history.push(snapshot);
        state.last_collection = Utc::now();
        
        Ok(())
    }

    /// Analyzes performance metrics to detect bottlenecks and trends.
    ///
    /// # Arguments
    ///
    /// * `component` - The system component to analyze performance for
    /// * `metrics` - The component metrics to analyze
    ///
    /// # Returns
    ///
    /// PerformanceAnalysis containing bottlenecks and trend
    async fn analyze_performance(&self, component: &SystemComponent, metrics: &ComponentMetrics) -> Result<PerformanceAnalysis, AgentError> {
        let mut bottlenecks = Vec::new();
        let mut recommendations = Vec::new();
        
        // Check CPU bottleneck
        if metrics.cpu_usage > 80.0 {
            bottlenecks.push(PerformanceBottleneck {
                resource_type: ResourceType::CPU,
                severity: (metrics.cpu_usage - 80.0) / 20.0,
                description: "High CPU utilization".to_string(),
                potential_impact: "May cause increased response times".to_string(),
            });
            recommendations.push("Consider scaling CPU resources".to_string());
        }
        
        // Check memory bottleneck
        if metrics.memory_usage > 80.0 {
            bottlenecks.push(PerformanceBottleneck {
                resource_type: ResourceType::Memory,
                severity: (metrics.memory_usage - 80.0) / 20.0,
                description: "High memory usage".to_string(),
                potential_impact: "May cause OOM errors".to_string(),
            });
            recommendations.push("Optimize memory usage or increase allocation".to_string());
        }
        
        // Determine performance trend
        let trend = if metrics.performance_score > 0.9 {
            PerformanceTrend::Improving
        } else if metrics.performance_score > 0.7 {
            PerformanceTrend::Stable
        } else if metrics.performance_score > 0.5 {
            PerformanceTrend::Degrading
        } else {
            PerformanceTrend::Critical
        };
        
        Ok(PerformanceAnalysis {
            health_score: metrics.performance_score,
            bottlenecks,
            recommendations,
            trend,
        })
    }
}

impl Default for MetricsState {
    fn default() -> Self {
        Self {
            component_metrics: HashMap::new(),
            performance_history: Vec::new(),
            last_collection: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::MockMetricsProvider;

    #[tokio::test]
    async fn test_metrics_collection() {
        // Create test metrics
        let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
        let collector = MetricsCollector::new(metrics);
        
        // Create test component
        let component = SystemComponent {
            name: "test_component".to_string(),
            component_type: ComponentType::Agent,
            status: ComponentStatus::default(),
            path: PathBuf::from("/test"),
            dependencies: HashSet::new(),
        };
        
        // Test development metrics
        let dev_metrics = collector.collect_development_metrics(&component).await.unwrap();
        assert!(dev_metrics.performance_score <= 0.6);
        
        // Test production metrics
        let prod_metrics = collector.collect_production_metrics(&component).await.unwrap();
        assert!(prod_metrics.performance_score <= 0.9);
        
        // Test release metrics
        let rel_metrics = collector.collect_release_metrics(&component).await.unwrap();
        assert!(rel_metrics.performance_score <= 0.99);
    }

    #[tokio::test]
    async fn test_performance_analysis() {
        // Create test metrics
        let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
        let collector = MetricsCollector::new(metrics);
        
        // Create test component metrics
        let component_metrics = ComponentMetrics {
            cpu_usage: 85.0,
            memory_usage: 90.0,
            performance_score: 0.6,
            ..ComponentMetrics::default()
        };
        
        // Create test component
        let component = SystemComponent::default();
        
        // Test performance analysis
        let analysis = collector.analyze_performance(&component, &component_metrics).await.unwrap();
        
        assert_eq!(analysis.bottlenecks.len(), 2); // CPU and Memory bottlenecks
        assert_eq!(analysis.trend, PerformanceTrend::Degrading);
        assert!(!analysis.recommendations.is_empty());
    }
}
