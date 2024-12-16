use anya_core::{
    auth::{AuthManager, BlockchainAuth},
    ml::advanced_features::AdvancedMLFeatures,
    monitoring::integrated_system_metrics::IntegratedSystemMetrics,
    revenue::ml_revenue_tracking::MLRevenueTracker,
    web5::advanced_integration::AdvancedWeb5Integration,
};
use chrono::{Utc, Duration};

mod common;

#[tokio::test]
async fn test_complete_system_integration() {
    // Setup components
    let (auth_manager, ml_features, metrics, revenue_tracker, web5_integration) = 
        common::setup_test_system().await;
        
    // Test integrated data processing
    let test_data = common::create_test_data();
    let context = common::create_test_context();
    
    let result = web5_integration
        .process_integrated_data(test_data, &context)
        .await
        .expect("Processing should succeed");
        
    // Verify ML results
    assert!(result.ml_result.confidence > 0.7);
    assert!(!result.ml_result.predictions.is_empty());
    
    // Verify revenue tracking
    assert!(result.revenue_impact.total > 0.0);
    
    // Verify Web5 storage
    let stored_record = web5_integration
        .get_record(&result.record_id)
        .await
        .expect("Record should be stored");
    assert_eq!(stored_record.data_type, DataType::IntegratedResult);
    
    // Verify metrics
    let metrics_snapshot = metrics
        .get_metrics_snapshot()
        .await;
    assert!(metrics_snapshot.successful_operations > 0);
}

#[tokio::test]
async fn test_system_health() {
    let (_, _, metrics, _, _) = common::setup_test_system().await;
    
    let health = metrics
        .get_system_health()
        .await;
        
    assert!(health.web5_health.is_healthy);
    assert!(health.ml_health.is_healthy);
    assert!(health.revenue_health.is_healthy);
    assert!(health.system_health.is_healthy);
}
