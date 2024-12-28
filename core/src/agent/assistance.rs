use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::metrics::UnifiedMetrics;
use crate::system::{SystemComponent, ComponentStatus};
use super::AgentError;

/// Agent Assistance System for workload management and efficiency
pub struct AgentAssistance {
    metrics: Arc<RwLock<UnifiedMetrics>>,
    workload_manager: Arc<WorkloadManager>,
    efficiency_analyzer: Arc<EfficiencyAnalyzer>,
    state: Arc<RwLock<AssistanceState>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistanceState {
    pub workload_distribution: HashMap<String, WorkloadMetrics>,
    pub efficiency_scores: HashMap<String, EfficiencyMetrics>,
    pub last_optimization: DateTime<Utc>,
    pub optimization_history: Vec<OptimizationEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadMetrics {
    pub cpu_allocation: f64,
    pub memory_allocation: f64,
    pub task_queue_size: usize,
    pub active_tasks: usize,
    pub completed_tasks: usize,
    pub failed_tasks: usize,
    pub average_completion_time: f64,
    pub task_priority_distribution: HashMap<Priority, usize>,
    pub resource_pressure: ResourcePressure,
    pub bottleneck_analysis: BottleneckAnalysis,
    pub scaling_metrics: ScalingMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePressure {
    pub cpu_pressure: f64,
    pub memory_pressure: f64,
    pub io_pressure: f64,
    pub network_pressure: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottleneckAnalysis {
    pub primary_bottleneck: BottleneckType,
    pub secondary_bottlenecks: Vec<BottleneckType>,
    pub impact_score: f64,
    pub recommendations: Vec<String>,
    pub bottleneck_chain: Vec<BottleneckDependency>,
    pub resource_contention: ResourceContention,
    pub performance_impact: PerformanceImpact,
    pub mitigation_strategies: Vec<MitigationStrategy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingMetrics {
    pub current_scale: usize,
    pub recommended_scale: usize,
    pub scale_factor: f64,
    pub scaling_trend: TrendType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EfficiencyMetrics {
    pub resource_utilization: f64,
    pub task_throughput: f64,
    pub error_rate: f64,
    pub response_time: f64,
    pub optimization_score: f64,
    pub resource_efficiency: ResourceEfficiency,
    pub performance_metrics: PerformanceMetrics,
    pub quality_metrics: QualityMetrics,
    pub cost_metrics: CostMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceEfficiency {
    pub cpu_efficiency: f64,
    pub memory_efficiency: f64,
    pub io_efficiency: f64,
    pub network_efficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub throughput_per_resource: f64,
    pub latency_distribution: Vec<f64>,
    pub resource_saturation: f64,
    pub performance_stability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub accuracy: f64,
    pub reliability: f64,
    pub consistency: f64,
    pub predictability: f64,
    pub stability_score: f64,
    pub resilience_score: f64,
    pub adaptability: AdaptabilityMetrics,
    pub robustness: RobustnessMetrics,
    pub security: SecurityMetrics,
    pub compliance: ComplianceMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptabilityMetrics {
    pub learning_rate: f64,
    pub adaptation_speed: f64,
    pub recovery_time: f64,
    pub flexibility_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobustnessMetrics {
    pub fault_tolerance: f64,
    pub error_recovery: f64,
    pub stability_under_load: f64,
    pub degradation_resistance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMetrics {
    pub threat_resistance: f64,
    pub vulnerability_score: f64,
    pub encryption_strength: f64,
    pub audit_compliance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceMetrics {
    pub standard_compliance: HashMap<String, f64>,
    pub policy_adherence: f64,
    pub certification_status: Vec<String>,
    pub risk_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BottleneckType {
    CPU,
    Memory,
    IO,
    Network,
    Database,
    External,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TrendType {
    Increasing,
    Decreasing,
    Stable,
    Fluctuating,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationEvent {
    pub timestamp: DateTime<Utc>,
    pub component_id: String,
    pub action_taken: String,
    pub improvement: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottleneckDependency {
    pub source: BottleneckType,
    pub target: BottleneckType,
    pub impact_weight: f64,
    pub propagation_path: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceContention {
    pub contention_points: Vec<ContentionPoint>,
    pub resource_dependencies: HashMap<String, Vec<String>>,
    pub contention_severity: f64,
    pub resolution_priority: Priority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentionPoint {
    pub resource: String,
    pub contention_type: ContentionType,
    pub waiting_tasks: usize,
    pub blocking_tasks: usize,
    pub average_wait_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceImpact {
    pub latency_increase: f64,
    pub throughput_decrease: f64,
    pub resource_waste: f64,
    pub user_experience: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationStrategy {
    pub strategy_type: MitigationType,
    pub estimated_impact: f64,
    pub implementation_cost: f64,
    pub time_to_implement: f64,
    pub prerequisites: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContentionType {
    Lock,
    Resource,
    Connection,
    Bandwidth,
    Processing,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MitigationType {
    Scale,
    Optimize,
    Redistribute,
    Cache,
    Throttle,
    Upgrade,
}

impl AgentAssistance {
    /// Create new agent assistance system
    pub fn new(metrics: Arc<RwLock<UnifiedMetrics>>) -> Self {
        Self {
            metrics,
            workload_manager: Arc::new(WorkloadManager::new()),
            efficiency_analyzer: Arc::new(EfficiencyAnalyzer::new()),
            state: Arc::new(RwLock::new(AssistanceState::default())),
        }
    }

    /// Analyze and optimize component workload with enhanced metrics
    pub async fn optimize_workload(&self, component: &SystemComponent) -> Result<WorkloadMetrics, AgentError> {
        // Get current metrics
        let metrics = self.metrics.read().await;
        
        // Analyze current workload with enhanced metrics
        let mut workload = self.workload_manager.analyze_workload(component, &metrics).await?;
        
        // Enhanced workload analysis
        workload.task_priority_distribution = self.analyze_task_priorities(&metrics).await?;
        workload.resource_pressure = self.analyze_resource_pressure(&metrics).await?;
        workload.bottleneck_analysis = self.analyze_bottlenecks(&metrics).await?;
        workload.scaling_metrics = self.analyze_scaling_needs(&metrics).await?;
        
        // Update workload distribution
        let mut state = self.state.write().await;
        state.workload_distribution.insert(component.name.clone(), workload.clone());
        
        // Enhanced optimization triggers
        if self.needs_optimization(&workload) {
            self.optimize_component(component, &workload).await?;
        }
        
        Ok(workload)
    }

    /// Analyze component efficiency with enhanced metrics
    pub async fn analyze_efficiency(&self, component: &SystemComponent) -> Result<EfficiencyMetrics, AgentError> {
        // Get current metrics
        let metrics = self.metrics.read().await;
        
        // Analyze efficiency with enhanced metrics
        let mut efficiency = self.efficiency_analyzer.analyze_efficiency(component, &metrics).await?;
        
        // Enhanced efficiency analysis
        efficiency.resource_efficiency = self.analyze_resource_efficiency(&metrics).await?;
        efficiency.performance_metrics = self.analyze_performance_metrics(&metrics).await?;
        efficiency.quality_metrics = self.analyze_quality_metrics(&metrics).await?;
        efficiency.cost_metrics = self.analyze_cost_metrics(&metrics).await?;
        
        // Update efficiency scores
        let mut state = self.state.write().await;
        state.efficiency_scores.insert(component.name.clone(), efficiency.clone());
        
        Ok(efficiency)
    }

    /// Analyze task priorities
    async fn analyze_task_priorities(&self, metrics: &UnifiedMetrics) -> Result<HashMap<Priority, usize>, AgentError> {
        let mut distribution = HashMap::new();
        
        // Analyze task queue and categorize by priority
        distribution.insert(Priority::Critical, metrics.system.critical_tasks);
        distribution.insert(Priority::High, metrics.system.high_priority_tasks);
        distribution.insert(Priority::Medium, metrics.system.medium_priority_tasks);
        distribution.insert(Priority::Low, metrics.system.low_priority_tasks);
        
        Ok(distribution)
    }

    /// Analyze resource pressure
    async fn analyze_resource_pressure(&self, metrics: &UnifiedMetrics) -> Result<ResourcePressure, AgentError> {
        Ok(ResourcePressure {
            cpu_pressure: self.calculate_cpu_pressure(metrics),
            memory_pressure: self.calculate_memory_pressure(metrics),
            io_pressure: self.calculate_io_pressure(metrics),
            network_pressure: self.calculate_network_pressure(metrics),
        })
    }

    /// Analyze bottlenecks
    async fn analyze_bottlenecks(&self, metrics: &UnifiedMetrics) -> Result<BottleneckAnalysis, AgentError> {
        let mut bottlenecks = Vec::new();
        let mut impact_score = 0.0;
        
        // Identify primary and secondary bottlenecks
        let primary = self.identify_primary_bottleneck(metrics);
        bottlenecks = self.identify_secondary_bottlenecks(metrics);
        impact_score = self.calculate_bottleneck_impact(metrics, &primary, &bottlenecks);
        
        // Enhanced bottleneck analysis
        let bottleneck_chain = self.analyze_bottleneck_chain(&primary, &bottlenecks, metrics).await?;
        let resource_contention = self.analyze_resource_contention(metrics).await?;
        let performance_impact = self.calculate_performance_impact(
            &primary,
            &bottlenecks,
            &bottleneck_chain,
            &resource_contention,
            metrics,
        ).await?;
        let mitigation_strategies = self.generate_mitigation_strategies(
            &primary,
            &bottlenecks,
            &bottleneck_chain,
            &resource_contention,
            &performance_impact,
        ).await?;
        
        Ok(BottleneckAnalysis {
            primary_bottleneck: primary,
            secondary_bottlenecks: bottlenecks,
            impact_score,
            recommendations: self.generate_bottleneck_recommendations(&primary, &bottlenecks),
            bottleneck_chain,
            resource_contention,
            performance_impact,
            mitigation_strategies,
        })
    }

    /// Analyze scaling needs
    async fn analyze_scaling_needs(&self, metrics: &UnifiedMetrics) -> Result<ScalingMetrics, AgentError> {
        Ok(ScalingMetrics {
            current_scale: metrics.system.instance_count,
            recommended_scale: self.calculate_recommended_scale(metrics),
            scale_factor: self.calculate_scale_factor(metrics),
            scaling_trend: self.analyze_scaling_trend(metrics),
        })
    }

    /// Analyze resource efficiency
    async fn analyze_resource_efficiency(&self, metrics: &UnifiedMetrics) -> Result<ResourceEfficiency, AgentError> {
        Ok(ResourceEfficiency {
            cpu_efficiency: self.calculate_cpu_efficiency(metrics),
            memory_efficiency: self.calculate_memory_efficiency(metrics),
            io_efficiency: self.calculate_io_efficiency(metrics),
            network_efficiency: self.calculate_network_efficiency(metrics),
        })
    }

    /// Analyze performance metrics
    async fn analyze_performance_metrics(&self, metrics: &UnifiedMetrics) -> Result<PerformanceMetrics, AgentError> {
        Ok(PerformanceMetrics {
            throughput_per_resource: self.calculate_throughput_per_resource(metrics),
            latency_distribution: self.calculate_latency_distribution(metrics),
            resource_saturation: self.calculate_resource_saturation(metrics),
            performance_stability: self.calculate_performance_stability(metrics),
        })
    }

    /// Analyze quality metrics
    async fn analyze_quality_metrics(&self, metrics: &UnifiedMetrics) -> Result<QualityMetrics, AgentError> {
        // Basic quality metrics
        let accuracy = self.calculate_accuracy(metrics);
        let reliability = self.calculate_reliability(metrics);
        let consistency = self.calculate_consistency(metrics);
        let predictability = self.calculate_predictability(metrics);
        
        // Enhanced quality metrics
        let stability_score = self.calculate_stability_score(metrics);
        let resilience_score = self.calculate_resilience_score(metrics);
        
        // Adaptability metrics
        let adaptability = self.analyze_adaptability_metrics(metrics).await?;
        
        // Robustness metrics
        let robustness = self.analyze_robustness_metrics(metrics).await?;
        
        // Security metrics
        let security = self.analyze_security_metrics(metrics).await?;
        
        // Compliance metrics
        let compliance = self.analyze_compliance_metrics(metrics).await?;
        
        Ok(QualityMetrics {
            accuracy,
            reliability,
            consistency,
            predictability,
            stability_score,
            resilience_score,
            adaptability,
            robustness,
            security,
            compliance,
        })
    }

    /// Analyze cost metrics
    async fn analyze_cost_metrics(&self, metrics: &UnifiedMetrics) -> Result<CostMetrics, AgentError> {
        Ok(CostMetrics {
            cost_per_request: self.calculate_cost_per_request(metrics),
            resource_cost: self.calculate_resource_cost(metrics),
            optimization_savings: self.calculate_optimization_savings(metrics),
            efficiency_ratio: self.calculate_efficiency_ratio(metrics),
        })
    }

    /// Analyze bottleneck dependencies and propagation
    async fn analyze_bottleneck_chain(
        &self,
        primary: &BottleneckType,
        secondary: &[BottleneckType],
        metrics: &UnifiedMetrics,
    ) -> Result<Vec<BottleneckDependency>, AgentError> {
        let mut chain = Vec::new();
        
        // Analyze primary to secondary dependencies
        for sec in secondary {
            if let Some(dependency) = self.analyze_dependency(primary, sec, metrics).await? {
                chain.push(dependency);
            }
        }
        
        // Analyze secondary to secondary dependencies
        for i in 0..secondary.len() {
            for j in i+1..secondary.len() {
                if let Some(dependency) = self.analyze_dependency(&secondary[i], &secondary[j], metrics).await? {
                    chain.push(dependency);
                }
            }
        }
        
        Ok(chain)
    }

    /// Analyze resource contention
    async fn analyze_resource_contention(&self, metrics: &UnifiedMetrics) -> Result<ResourceContention, AgentError> {
        let mut contention_points = Vec::new();
        let mut resource_dependencies = HashMap::new();
        
        // Analyze CPU contention
        if metrics.system.cpu_usage > 80.0 {
            contention_points.push(ContentionPoint {
                resource: "CPU".to_string(),
                contention_type: ContentionType::Processing,
                waiting_tasks: metrics.system.waiting_tasks,
                blocking_tasks: metrics.system.blocking_tasks,
                average_wait_time: metrics.system.average_wait_time,
            });
        }
        
        // Analyze memory contention
        if metrics.system.memory_usage > 80.0 {
            contention_points.push(ContentionPoint {
                resource: "Memory".to_string(),
                contention_type: ContentionType::Resource,
                waiting_tasks: metrics.system.memory_waiting_tasks,
                blocking_tasks: metrics.system.memory_blocking_tasks,
                average_wait_time: metrics.system.memory_wait_time,
            });
        }
        
        // Calculate overall contention severity
        let contention_severity = self.calculate_contention_severity(&contention_points);
        
        // Determine resolution priority
        let resolution_priority = if contention_severity > 0.8 {
            Priority::Critical
        } else if contention_severity > 0.6 {
            Priority::High
        } else if contention_severity > 0.4 {
            Priority::Medium
        } else {
            Priority::Low
        };
        
        Ok(ResourceContention {
            contention_points,
            resource_dependencies,
            contention_severity,
            resolution_priority,
        })
    }

    /// Generate optimization strategies
    async fn generate_mitigation_strategies(
        &self,
        primary: &BottleneckType,
        secondary: &[BottleneckType],
        chain: &[BottleneckDependency],
        contention: &ResourceContention,
        impact: &PerformanceImpact,
    ) -> Result<Vec<MitigationStrategy>, AgentError> {
        let mut strategies = Vec::new();
        
        // Scaling strategies
        if impact.throughput_decrease > 0.2 {
            strategies.push(MitigationStrategy {
                strategy_type: MitigationType::Scale,
                estimated_impact: 0.4,
                implementation_cost: 0.6,
                time_to_implement: 3600.0, // 1 hour
                prerequisites: vec!["Available capacity".to_string()],
            });
        }
        
        // Optimization strategies
        if impact.resource_waste > 0.3 {
            strategies.push(MitigationStrategy {
                strategy_type: MitigationType::Optimize,
                estimated_impact: 0.3,
                implementation_cost: 0.4,
                time_to_implement: 7200.0, // 2 hours
                prerequisites: vec!["Code analysis".to_string()],
            });
        }
        
        // Caching strategies
        if impact.latency_increase > 0.25 {
            strategies.push(MitigationStrategy {
                strategy_type: MitigationType::Cache,
                estimated_impact: 0.35,
                implementation_cost: 0.3,
                time_to_implement: 1800.0, // 30 minutes
                prerequisites: vec!["Cache storage".to_string()],
            });
        }
        
        Ok(strategies)
    }

    /// Analyze adaptability metrics
    async fn analyze_adaptability_metrics(&self, metrics: &UnifiedMetrics) -> Result<AdaptabilityMetrics, AgentError> {
        Ok(AdaptabilityMetrics {
            learning_rate: self.calculate_learning_rate(metrics),
            adaptation_speed: self.calculate_adaptation_speed(metrics),
            recovery_time: self.calculate_recovery_time(metrics),
            flexibility_score: self.calculate_flexibility_score(metrics),
        })
    }

    /// Analyze robustness metrics
    async fn analyze_robustness_metrics(&self, metrics: &UnifiedMetrics) -> Result<RobustnessMetrics, AgentError> {
        Ok(RobustnessMetrics {
            fault_tolerance: self.calculate_fault_tolerance(metrics),
            error_recovery: self.calculate_error_recovery(metrics),
            stability_under_load: self.calculate_stability_under_load(metrics),
            degradation_resistance: self.calculate_degradation_resistance(metrics),
        })
    }

    /// Analyze security metrics
    async fn analyze_security_metrics(&self, metrics: &UnifiedMetrics) -> Result<SecurityMetrics, AgentError> {
        Ok(SecurityMetrics {
            threat_resistance: self.calculate_threat_resistance(metrics),
            vulnerability_score: self.calculate_vulnerability_score(metrics),
            encryption_strength: self.calculate_encryption_strength(metrics),
            audit_compliance: self.calculate_audit_compliance(metrics),
        })
    }

    /// Analyze compliance metrics
    async fn analyze_compliance_metrics(&self, metrics: &UnifiedMetrics) -> Result<ComplianceMetrics, AgentError> {
        let mut standard_compliance = HashMap::new();
        standard_compliance.insert("ISO27001".to_string(), self.calculate_iso_compliance(metrics));
        standard_compliance.insert("GDPR".to_string(), self.calculate_gdpr_compliance(metrics));
        standard_compliance.insert("SOC2".to_string(), self.calculate_soc2_compliance(metrics));
        
        Ok(ComplianceMetrics {
            standard_compliance,
            policy_adherence: self.calculate_policy_adherence(metrics),
            certification_status: self.get_certification_status(metrics),
            risk_score: self.calculate_risk_score(metrics),
        })
    }

    /// Check if component needs optimization
    fn needs_optimization(&self, workload: &WorkloadMetrics) -> bool {
        // Check CPU utilization
        if workload.cpu_allocation > 80.0 {
            return true;
        }
        
        // Check memory utilization
        if workload.memory_allocation > 80.0 {
            return true;
        }
        
        // Check task queue
        if workload.task_queue_size > 100 {
            return true;
        }
        
        // Check error rate
        if (workload.failed_tasks as f64 / workload.completed_tasks as f64) > 0.1 {
            return true;
        }
        
        false
    }

    /// Optimize component resources and configuration
    async fn optimize_component(&self, component: &SystemComponent, workload: &WorkloadMetrics) -> Result<(), AgentError> {
        // Calculate optimal resource allocation
        let optimal_cpu = self.calculate_optimal_cpu(workload);
        let optimal_memory = self.calculate_optimal_memory(workload);
        
        // Apply optimizations
        self.workload_manager.adjust_resources(
            component,
            optimal_cpu,
            optimal_memory,
        ).await?;
        
        // Record optimization event
        let mut state = self.state.write().await;
        state.optimization_history.push(OptimizationEvent {
            timestamp: Utc::now(),
            component_id: component.name.clone(),
            action_taken: format!("Resource optimization: CPU={}, Memory={}", optimal_cpu, optimal_memory),
            improvement: 0.0, // Will be updated after measuring impact
        });
        
        Ok(())
    }

    /// Calculate optimal CPU allocation
    fn calculate_optimal_cpu(&self, workload: &WorkloadMetrics) -> f64 {
        let base_allocation = 50.0;
        let task_factor = workload.active_tasks as f64 * 0.1;
        let queue_factor = workload.task_queue_size as f64 * 0.05;
        
        (base_allocation + task_factor + queue_factor).min(95.0)
    }

    /// Calculate optimal memory allocation
    fn calculate_optimal_memory(&self, workload: &WorkloadMetrics) -> f64 {
        let base_allocation = 40.0;
        let task_factor = workload.active_tasks as f64 * 0.15;
        let completion_factor = workload.average_completion_time * 0.1;
        
        (base_allocation + task_factor + completion_factor).min(90.0)
    }
}

impl Default for AssistanceState {
    fn default() -> Self {
        Self {
            workload_distribution: HashMap::new(),
            efficiency_scores: HashMap::new(),
            last_optimization: Utc::now(),
            optimization_history: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::MockMetricsProvider;

    #[tokio::test]
    async fn test_workload_optimization() {
        // Create test metrics
        let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
        let assistance = AgentAssistance::new(metrics);
        
        // Create test component
        let component = SystemComponent {
            name: "test_component".to_string(),
            component_type: ComponentType::Agent,
            status: ComponentStatus::default(),
            path: PathBuf::from("/test"),
            dependencies: HashSet::new(),
        };
        
        // Test workload optimization
        let workload = assistance.optimize_workload(&component).await.unwrap();
        assert!(workload.cpu_allocation > 0.0);
        assert!(workload.memory_allocation > 0.0);
    }

    #[tokio::test]
    async fn test_efficiency_analysis() {
        // Create test metrics
        let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
        let assistance = AgentAssistance::new(metrics);
        
        // Create test component
        let component = SystemComponent {
            name: "test_component".to_string(),
            component_type: ComponentType::Agent,
            status: ComponentStatus::default(),
            path: PathBuf::from("/test"),
            dependencies: HashSet::new(),
        };
        
        // Test efficiency analysis
        let efficiency = assistance.analyze_efficiency(&component).await.unwrap();
        assert!(efficiency.resource_utilization >= 0.0);
        assert!(efficiency.optimization_score >= 0.0);
    }

    #[tokio::test]
    async fn test_optimization_triggers() {
        // Create test metrics
        let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
        let assistance = AgentAssistance::new(metrics);
        
        // Test optimization triggers
        let workload = WorkloadMetrics {
            cpu_allocation: 85.0,
            memory_allocation: 75.0,
            task_queue_size: 150,
            active_tasks: 10,
            completed_tasks: 100,
            failed_tasks: 15,
            average_completion_time: 0.5,
            task_priority_distribution: HashMap::new(),
            resource_pressure: ResourcePressure::default(),
            bottleneck_analysis: BottleneckAnalysis::default(),
            scaling_metrics: ScalingMetrics::default(),
        };
        
        assert!(assistance.needs_optimization(&workload));
    }
}
