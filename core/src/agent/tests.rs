use std::sync::Arc;
use tokio::sync::RwLock;
use std::path::PathBuf;
use std::collections::HashSet;

use crate::metrics::UnifiedMetrics;
use crate::system::{SystemComponent, ComponentType, ComponentStatus};
use crate::ml::service::{MLService, MLRequest, MLResponse};
use crate::security::SecurityManager;

use super::{
    MLAgentChecker,
    AgentError,
    StageStatus,
    metrics::{MetricsCollector, ComponentMetrics},
    ml_integration::{MLIntegration, WorkloadMetrics},
    assistance::{AgentAssistance, EfficiencyMetrics},
};

/// Create test component
fn create_test_component(name: &str) -> SystemComponent {
    SystemComponent {
        name: name.to_string(),
        component_type: ComponentType::Agent,
        status: ComponentStatus::default(),
        path: PathBuf::from("/test"),
        dependencies: HashSet::new(),
    }
}

/// Mock ML Service for testing
struct MockMLService;

#[async_trait::async_trait]
impl MLService for MockMLService {
    async fn process(&self, request: MLRequest) -> Result<MLResponse, MLError> {
        Ok(MLResponse {
            model_id: "test_model".to_string(),
            predictions: vec![0.8],
            confidence: 0.9,
            processing_time: 0.1,
            model_version: "1.0.0".to_string(),
        })
    }
}

/// Mock Security Manager for testing
struct MockSecurityManager;

#[async_trait::async_trait]
impl SecurityManager for MockSecurityManager {
    async fn validate_context(&self, context: &SecurityContext) -> Result<(), SecurityError> {
        Ok(())
    }
}

#[tokio::test]
async fn test_ml_agent_checker() {
    // Setup test environment
    let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
    let ml_service = Arc::new(MockMLService);
    let security_manager = Arc::new(MockSecurityManager);
    
    let checker = MLAgentChecker::new(
        Arc::new(SystemManager::default()),
        ml_service,
        security_manager,
        metrics.clone(),
    );
    
    // Test component
    let component = create_test_component("test_agent");
    
    // Test component readiness
    let stage = checker.check_component_readiness(&component).await.unwrap();
    assert!(matches!(stage, StageStatus::Development));
    
    // Test metrics collection
    let metrics = checker.metrics_collector.collect_development_metrics(&component).await.unwrap();
    assert!(metrics.performance_score > 0.0);
    assert!(metrics.test_coverage > 0.0);
}

#[tokio::test]
async fn test_ml_integration() {
    // Setup test environment
    let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
    let ml_service = Arc::new(MockMLService);
    let security_manager = Arc::new(MockSecurityManager);
    
    let integration = MLIntegration::new(
        ml_service,
        Arc::new(MLModelRepository::default()),
        metrics.clone(),
        security_manager,
    );
    
    // Test component
    let component = create_test_component("test_ml");
    
    // Test readiness analysis
    let readiness = integration.analyze_component_readiness(&component).await.unwrap();
    assert!(readiness > 0.0);
    
    // Test component validation
    let validation = integration.validate_component(&component).await.unwrap();
    assert!(validation.performance_score > 0.0);
}

#[tokio::test]
async fn test_metrics_collection() {
    // Setup test environment
    let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
    let collector = MetricsCollector::new(metrics);
    
    // Test component
    let component = create_test_component("test_metrics");
    
    // Test development metrics
    let dev_metrics = collector.collect_development_metrics(&component).await.unwrap();
    assert!(dev_metrics.performance_score <= 0.6);
    
    // Test production metrics
    let prod_metrics = collector.collect_production_metrics(&component).await.unwrap();
    assert!(prod_metrics.performance_score <= 0.9);
    
    // Test release metrics
    let rel_metrics = collector.collect_release_metrics(&component).await.unwrap();
    assert!(rel_metrics.performance_score <= 0.99);
    
    // Test performance analysis
    let analysis = collector.analyze_performance(&component, &dev_metrics).await.unwrap();
    assert!(!analysis.bottlenecks.is_empty());
    assert!(!analysis.recommendations.is_empty());
}

#[tokio::test]
async fn test_agent_assistance() {
    // Setup test environment
    let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
    let assistance = AgentAssistance::new(metrics);
    
    // Test component
    let component = create_test_component("test_assistance");
    
    // Test workload optimization
    let workload = assistance.optimize_workload(&component).await.unwrap();
    assert!(workload.cpu_allocation > 0.0);
    assert!(workload.memory_allocation > 0.0);
    
    // Test efficiency analysis
    let efficiency = assistance.analyze_efficiency(&component).await.unwrap();
    assert!(efficiency.resource_utilization > 0.0);
    assert!(efficiency.optimization_score > 0.0);
}

#[tokio::test]
async fn test_stage_transitions() {
    // Setup test environment
    let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
    let ml_service = Arc::new(MockMLService);
    let security_manager = Arc::new(MockSecurityManager);
    
    let checker = MLAgentChecker::new(
        Arc::new(SystemManager::default()),
        ml_service,
        security_manager,
        metrics.clone(),
    );
    
    // Test component
    let component = create_test_component("test_stages");
    
    // Test development stage
    {
        let mut metrics = metrics.write().await;
        metrics.set_development_metrics();
        drop(metrics);
        
        let stage = checker.check_component_readiness(&component).await.unwrap();
        assert!(matches!(stage, StageStatus::Development));
    }
    
    // Test production stage
    {
        let mut metrics = metrics.write().await;
        metrics.set_production_metrics();
        drop(metrics);
        
        let stage = checker.check_component_readiness(&component).await.unwrap();
        assert!(matches!(stage, StageStatus::Production));
    }
    
    // Test release stage
    {
        let mut metrics = metrics.write().await;
        metrics.set_release_metrics();
        drop(metrics);
        
        let stage = checker.check_component_readiness(&component).await.unwrap();
        assert!(matches!(stage, StageStatus::Release));
    }
}

#[tokio::test]
async fn test_validation_rules() {
    // Setup test environment
    let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
    let ml_service = Arc::new(MockMLService);
    let security_manager = Arc::new(MockSecurityManager);
    
    let checker = MLAgentChecker::new(
        Arc::new(SystemManager::default()),
        ml_service,
        security_manager,
        metrics.clone(),
    );
    
    // Test component
    let component = create_test_component("test_validation");
    
    // Test invalid metrics
    {
        let mut metrics = metrics.write().await;
        metrics.set_invalid_metrics();
        drop(metrics);
        
        let result = checker.check_component_readiness(&component).await;
        assert!(result.is_err());
    }
    
    // Test security validation
    {
        let mut metrics = metrics.write().await;
        metrics.set_security_failure();
        drop(metrics);
        
        let result = checker.check_component_readiness(&component).await;
        assert!(result.is_err());
    }
    
    // Test performance validation
    {
        let mut metrics = metrics.write().await;
        metrics.set_performance_failure();
        drop(metrics);
        
        let result = checker.check_component_readiness(&component).await;
        assert!(result.is_err());
    }
}
