//! Performance Analysis System for ML*/Agent
//! 
//! This module provides comprehensive performance analysis capabilities for the ML*/Agent system.
//! It handles various aspects of performance including resource utilization, bottleneck detection,
//! optimization suggestions, and performance impact analysis.
//!
//! # Architecture
//! 
//! The performance analysis system consists of:
//! - PerformanceAnalyzer: Central coordinator for performance analysis
//! - PerformanceState: Thread-safe state management
//! - Specialized analyzers for different aspects (Resource, Task, System)
//!
//! # Features
//!
//! - Real-time performance analysis
//! - Bottleneck detection and analysis
//! - Resource contention analysis
//! - Performance impact assessment
//! - Optimization strategy generation
//! - Historical performance tracking
//!
//! # Example
//!
//! ```rust
//! use anya::agent::performance::PerformanceAnalyzer;
//!
//! async fn analyze_performance(component: &SystemComponent) -> Result<(), AgentError> {
//!     let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
//!     let analyzer = PerformanceAnalyzer::new(metrics);
//!     
//!     // Run performance analysis
//!     let profile = analyzer.analyze_performance(component).await?;
//!     
//!     // Check for bottlenecks
//!     if !profile.bottlenecks.is_empty() {
//!         println!("Performance bottlenecks detected");
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

/// Enhanced performance analysis system for ML*/Agent.
///
/// The PerformanceAnalyzer is responsible for analyzing and optimizing system
/// performance across various dimensions:
/// - Resource utilization
/// - Task processing efficiency
/// - System stability
/// - Performance bottlenecks
/// - Optimization opportunities
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
pub struct PerformanceAnalyzer {
    metrics: Arc<RwLock<UnifiedMetrics>>,
    state: Arc<RwLock<PerformanceState>>,
}

/// Current state of performance analysis.
///
/// Maintains:
/// - Per-component performance profiles
/// - Performance history
/// - Last analysis timestamp
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceState {
    pub component_performance: HashMap<String, PerformanceProfile>,
    pub performance_history: Vec<PerformanceEvent>,
    pub last_analysis: DateTime<Utc>,
}

/// Comprehensive performance profile for a component.
///
/// Contains:
/// - Core performance metrics
/// - Resource metrics and trends
/// - Task metrics and trends
/// - System metrics and trends
/// - Bottleneck analysis
/// - Optimization suggestions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceProfile {
    // Core performance
    /// Overall health score (0.0 to 1.0)
    pub health_score: f64,
    /// System stability score (0.0 to 1.0)
    pub stability_score: f64,
    /// System reliability score (0.0 to 1.0)
    pub reliability_score: f64,
    
    // Resource metrics
    /// Current resource utilization metrics
    pub resource_metrics: ResourceMetrics,
    /// Resource utilization trends
    pub resource_trends: ResourceTrends,
    
    // Task metrics
    /// Current task processing metrics
    pub task_metrics: TaskMetrics,
    /// Task processing trends
    pub task_trends: TaskTrends,
    
    // System metrics
    /// Current system-level metrics
    pub system_metrics: SystemMetrics,
    /// System-level trends
    pub system_trends: SystemTrends,
    
    // Analysis results
    /// Detected performance bottlenecks
    pub bottlenecks: Vec<PerformanceBottleneck>,
    /// Suggested optimizations
    pub optimizations: Vec<OptimizationSuggestion>,
    /// Active performance alerts
    pub alerts: Vec<PerformanceAlert>,
}

/// Resource utilization metrics.
///
/// Tracks:
/// - CPU metrics (usage, steal, iowait)
/// - Memory metrics (usage, available, swap)
/// - Disk metrics (usage, IOPS, latency)
/// - Network metrics (throughput, latency)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetrics {
    // CPU metrics
    /// CPU utilization percentage (0.0 to 100.0)
    pub cpu_usage: f64,
    /// CPU steal time percentage (0.0 to 100.0)
    pub cpu_steal: f64,
    /// CPU I/O wait percentage (0.0 to 100.0)
    pub cpu_iowait: f64,
    /// CPU IRQ time percentage (0.0 to 100.0)
    pub cpu_irq: f64,
    
    // Memory metrics
    /// Memory utilization percentage (0.0 to 100.0)
    pub memory_usage: f64,
    /// Available memory in bytes
    pub memory_available: f64,
    /// Swap usage percentage (0.0 to 100.0)
    pub swap_usage: f64,
    /// Page faults per second
    pub page_faults: u64,
    
    // Disk metrics
    /// Disk utilization percentage (0.0 to 100.0)
    pub disk_usage: f64,
    /// Disk IOPS
    pub disk_iops: u64,
    /// Disk latency in milliseconds
    pub disk_latency: f64,
    /// Disk bandwidth in bytes per second
    pub disk_bandwidth: f64,
    
    // Network metrics
    /// Network throughput in bytes per second
    pub network_throughput: f64,
    /// Network latency in milliseconds
    pub network_latency: f64,
    /// Network errors per second
    pub network_errors: u64,
    /// Network packet retransmits per second
    pub network_retransmits: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceTrends {
    pub cpu_trend: TrendAnalysis,
    pub memory_trend: TrendAnalysis,
    pub disk_trend: TrendAnalysis,
    pub network_trend: TrendAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskMetrics {
    pub active_tasks: u64,
    pub queued_tasks: u64,
    pub completed_tasks: u64,
    pub failed_tasks: u64,
    pub average_response_time: f64,
    pub average_processing_time: f64,
    pub average_wait_time: f64,
    pub task_success_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTrends {
    pub throughput_trend: TrendAnalysis,
    pub latency_trend: TrendAnalysis,
    pub error_trend: TrendAnalysis,
    pub queue_trend: TrendAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub uptime: f64,
    pub load_average: Vec<f64>,
    pub context_switches: u64,
    pub interrupts: u64,
    pub system_calls: u64,
    pub process_count: u64,
    pub thread_count: u64,
    pub handle_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemTrends {
    pub load_trend: TrendAnalysis,
    pub stability_trend: TrendAnalysis,
    pub resource_trend: TrendAnalysis,
    pub scaling_trend: TrendAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysis {
    pub trend_type: TrendType,
    pub change_rate: f64,
    pub prediction: f64,
    pub confidence: f64,
    pub seasonality: Option<Seasonality>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Seasonality {
    pub period: f64,
    pub amplitude: f64,
    pub phase: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TrendType {
    Increasing,
    Decreasing,
    Stable,
    Fluctuating,
    Seasonal,
    Anomalous,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBottleneck {
    pub resource_type: ResourceType,
    pub severity: f64,
    pub impact: String,
    pub root_cause: String,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSuggestion {
    pub category: OptimizationCategory,
    pub priority: Priority,
    pub description: String,
    pub estimated_impact: f64,
    pub implementation_complexity: Complexity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    pub alert_type: AlertType,
    pub severity: Priority,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub context: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OptimizationCategory {
    ResourceAllocation,
    Caching,
    LoadBalancing,
    DatabaseOptimization,
    NetworkOptimization,
    CodeOptimization,
    ConfigurationTuning,
    ScalingStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Complexity {
    Simple,
    Moderate,
    Complex,
    VeryComplex,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertType {
    ResourceExhaustion,
    PerformanceDegradation,
    SystemInstability,
    SecurityThreat,
    DataIntegrityIssue,
}

impl PerformanceAnalyzer {
    /// Creates a new PerformanceAnalyzer instance.
    ///
    /// # Arguments
    ///
    /// * `metrics` - Shared reference to UnifiedMetrics
    ///
    /// # Returns
    ///
    /// A new PerformanceAnalyzer instance
    pub fn new(metrics: Arc<RwLock<UnifiedMetrics>>) -> Self {
        Self {
            metrics,
            state: Arc::new(RwLock::new(PerformanceState::default())),
        }
    }

    /// Analyzes component performance.
    ///
    /// Performs comprehensive performance analysis:
    /// - Resource utilization analysis
    /// - Task processing analysis
    /// - System stability analysis
    /// - Bottleneck detection
    /// - Optimization suggestions
    ///
    /// # Arguments
    ///
    /// * `component` - The system component to analyze
    ///
    /// # Returns
    ///
    /// PerformanceProfile containing analysis results
    pub async fn analyze_performance(&self, component: &SystemComponent) -> Result<PerformanceProfile, AgentError> {
        let metrics = self.metrics.read().await;
        
        // Collect resource metrics
        let resource_metrics = self.collect_resource_metrics(&metrics).await?;
        let resource_trends = self.analyze_resource_trends(&resource_metrics).await?;
        
        // Collect task metrics
        let task_metrics = self.collect_task_metrics(&metrics).await?;
        let task_trends = self.analyze_task_trends(&task_metrics).await?;
        
        // Collect system metrics
        let system_metrics = self.collect_system_metrics(&metrics).await?;
        let system_trends = self.analyze_system_trends(&system_metrics).await?;
        
        // Analyze bottlenecks
        let bottlenecks = self.identify_bottlenecks(
            &resource_metrics,
            &task_metrics,
            &system_metrics,
        ).await?;
        
        // Generate optimization suggestions
        let optimizations = self.generate_optimizations(
            &bottlenecks,
            &resource_trends,
            &task_trends,
            &system_trends,
        ).await?;
        
        // Generate alerts
        let alerts = self.generate_alerts(
            &bottlenecks,
            &resource_metrics,
            &task_metrics,
            &system_metrics,
        ).await?;
        
        // Calculate scores
        let health_score = self.calculate_health_score(
            &resource_metrics,
            &task_metrics,
            &system_metrics,
        );
        
        let stability_score = self.calculate_stability_score(
            &resource_trends,
            &task_trends,
            &system_trends,
        );
        
        let reliability_score = self.calculate_reliability_score(
            &task_metrics,
            &alerts,
        );
        
        let profile = PerformanceProfile {
            health_score,
            stability_score,
            reliability_score,
            resource_metrics,
            resource_trends,
            task_metrics,
            task_trends,
            system_metrics,
            system_trends,
            bottlenecks,
            optimizations,
            alerts,
        };
        
        // Update performance state
        self.update_performance_state(component, &profile).await?;
        
        Ok(profile)
    }

    /// Identify performance bottlenecks.
    ///
    /// Analyzes resource, task, and system metrics to detect performance bottlenecks.
    ///
    /// # Arguments
    ///
    /// * `resource_metrics` - Resource utilization metrics
    /// * `task_metrics` - Task processing metrics
    /// * `system_metrics` - System-level metrics
    ///
    /// # Returns
    ///
    /// A vector of PerformanceBottleneck instances
    async fn identify_bottlenecks(
        &self,
        resource_metrics: &ResourceMetrics,
        task_metrics: &TaskMetrics,
        system_metrics: &SystemMetrics,
    ) -> Result<Vec<PerformanceBottleneck>, AgentError> {
        let mut bottlenecks = Vec::new();
        
        // Check CPU bottlenecks
        if resource_metrics.cpu_usage > 80.0 {
            bottlenecks.push(PerformanceBottleneck {
                resource_type: ResourceType::CPU,
                severity: (resource_metrics.cpu_usage - 80.0) / 20.0,
                impact: "High CPU utilization affecting response times".to_string(),
                root_cause: "Compute-intensive operations or insufficient CPU resources".to_string(),
                recommendations: vec![
                    "Scale up CPU resources".to_string(),
                    "Optimize compute-intensive operations".to_string(),
                    "Implement caching for frequently computed results".to_string(),
                ],
            });
        }
        
        // Check memory bottlenecks
        if resource_metrics.memory_usage > 80.0 {
            bottlenecks.push(PerformanceBottleneck {
                resource_type: ResourceType::Memory,
                severity: (resource_metrics.memory_usage - 80.0) / 20.0,
                impact: "High memory usage risking OOM errors".to_string(),
                root_cause: "Memory leaks or inefficient memory usage".to_string(),
                recommendations: vec![
                    "Increase memory allocation".to_string(),
                    "Optimize memory usage patterns".to_string(),
                    "Implement memory pooling".to_string(),
                ],
            });
        }
        
        // Check disk bottlenecks
        if resource_metrics.disk_usage > 80.0 || resource_metrics.disk_latency > 100.0 {
            bottlenecks.push(PerformanceBottleneck {
                resource_type: ResourceType::Disk,
                severity: (resource_metrics.disk_usage - 80.0) / 20.0,
                impact: "High disk usage or latency affecting I/O performance".to_string(),
                root_cause: "Inefficient I/O patterns or insufficient disk resources".to_string(),
                recommendations: vec![
                    "Optimize disk I/O patterns".to_string(),
                    "Implement disk caching".to_string(),
                    "Consider SSD storage".to_string(),
                ],
            });
        }
        
        Ok(bottlenecks)
    }

    /// Generate optimization suggestions.
    ///
    /// Analyzes bottlenecks, trends, and metrics to generate optimization suggestions.
    ///
    /// # Arguments
    ///
    /// * `bottlenecks` - Performance bottlenecks
    /// * `resource_trends` - Resource utilization trends
    /// * `task_trends` - Task processing trends
    /// * `system_trends` - System-level trends
    ///
    /// # Returns
    ///
    /// A vector of OptimizationSuggestion instances
    async fn generate_optimizations(
        &self,
        bottlenecks: &[PerformanceBottleneck],
        resource_trends: &ResourceTrends,
        task_trends: &TaskTrends,
        system_trends: &SystemTrends,
    ) -> Result<Vec<OptimizationSuggestion>, AgentError> {
        let mut optimizations = Vec::new();
        
        // Resource allocation optimizations
        if bottlenecks.iter().any(|b| b.severity > 0.7) {
            optimizations.push(OptimizationSuggestion {
                category: OptimizationCategory::ResourceAllocation,
                priority: Priority::High,
                description: "Increase resource allocation for critical components".to_string(),
                estimated_impact: 0.8,
                implementation_complexity: Complexity::Moderate,
            });
        }
        
        // Caching optimizations
        if task_trends.latency_trend.trend_type == TrendType::Increasing {
            optimizations.push(OptimizationSuggestion {
                category: OptimizationCategory::Caching,
                priority: Priority::Medium,
                description: "Implement caching for frequently accessed data".to_string(),
                estimated_impact: 0.6,
                implementation_complexity: Complexity::Moderate,
            });
        }
        
        // Load balancing optimizations
        if system_trends.load_trend.trend_type == TrendType::Increasing {
            optimizations.push(OptimizationSuggestion {
                category: OptimizationCategory::LoadBalancing,
                priority: Priority::High,
                description: "Implement load balancing across multiple instances".to_string(),
                estimated_impact: 0.7,
                implementation_complexity: Complexity::Complex,
            });
        }
        
        Ok(optimizations)
    }

    /// Generate performance alerts.
    ///
    /// Analyzes bottlenecks, metrics, and trends to generate performance alerts.
    ///
    /// # Arguments
    ///
    /// * `bottlenecks` - Performance bottlenecks
    /// * `resource_metrics` - Resource utilization metrics
    /// * `task_metrics` - Task processing metrics
    /// * `system_metrics` - System-level metrics
    ///
    /// # Returns
    ///
    /// A vector of PerformanceAlert instances
    async fn generate_alerts(
        &self,
        bottlenecks: &[PerformanceBottleneck],
        resource_metrics: &ResourceMetrics,
        task_metrics: &TaskMetrics,
        system_metrics: &SystemMetrics,
    ) -> Result<Vec<PerformanceAlert>, AgentError> {
        let mut alerts = Vec::new();
        
        // Resource exhaustion alerts
        if bottlenecks.iter().any(|b| b.severity > 0.9) {
            alerts.push(PerformanceAlert {
                alert_type: AlertType::ResourceExhaustion,
                severity: Priority::Critical,
                message: "Critical resource exhaustion detected".to_string(),
                timestamp: Utc::now(),
                context: HashMap::new(),
            });
        }
        
        // Performance degradation alerts
        if task_metrics.task_success_rate < 0.95 {
            alerts.push(PerformanceAlert {
                alert_type: AlertType::PerformanceDegradation,
                severity: Priority::High,
                message: "Task success rate below threshold".to_string(),
                timestamp: Utc::now(),
                context: HashMap::new(),
            });
        }
        
        Ok(alerts)
    }

    /// Update performance state.
    ///
    /// Updates the performance state with the latest analysis results.
    ///
    /// # Arguments
    ///
    /// * `component` - The system component analyzed
    /// * `profile` - The performance profile
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    async fn update_performance_state(&self, component: &SystemComponent, profile: &PerformanceProfile) -> Result<(), AgentError> {
        let mut state = self.state.write().await;
        
        // Update component performance
        state.component_performance.insert(component.name.clone(), profile.clone());
        
        // Add performance event
        state.performance_history.push(PerformanceEvent {
            timestamp: Utc::now(),
            component_id: component.name.clone(),
            profile: profile.clone(),
        });
        
        state.last_analysis = Utc::now();
        
        Ok(())
    }
}

impl Default for PerformanceState {
    fn default() -> Self {
        Self {
            component_performance: HashMap::new(),
            performance_history: Vec::new(),
            last_analysis: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::MockMetricsProvider;

    #[tokio::test]
    async fn test_performance_analyzer() {
        // Create test metrics
        let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
        let analyzer = PerformanceAnalyzer::new(metrics);
        
        // Create test component
        let component = SystemComponent {
            name: "test_performance".to_string(),
            component_type: ComponentType::Agent,
            status: ComponentStatus::default(),
            path: PathBuf::from("/test"),
            dependencies: HashSet::new(),
        };
        
        // Test performance analysis
        let profile = analyzer.analyze_performance(&component).await.unwrap();
        
        // Check scores
        assert!(profile.health_score >= 0.0);
        assert!(profile.stability_score >= 0.0);
        assert!(profile.reliability_score >= 0.0);
        
        // Check bottlenecks
        assert!(!profile.bottlenecks.is_empty());
        
        // Check optimizations
        assert!(!profile.optimizations.is_empty());
        
        // Check alerts
        assert!(!profile.alerts.is_empty());
    }
}
