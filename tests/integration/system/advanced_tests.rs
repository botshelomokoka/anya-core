use super::*;
use anya_core::{
    auth::{AuthManager, BlockchainAuth},
    ml::advanced_features::AdvancedMLFeatures,
    monitoring::integrated_system_metrics::IntegratedSystemMetrics,
    revenue::ml_revenue_tracking::MLRevenueTracker,
    web5::advanced_integration::AdvancedWeb5Integration,
};

#[tokio::test]
async fn test_advanced_system_integration() {
    let (auth_manager, ml_features, metrics, revenue_tracker, web5_integration) = 
        common::setup_test_system().await;

    // Test Web5 DID Authentication
    let did_auth_result = test_did_auth(&auth_manager, &web5_integration).await;
    assert!(did_auth_result.is_ok(), "DID authentication should succeed");

    // Test ML Processing with Revenue Tracking
    let ml_revenue_result = test_ml_revenue_tracking(
        &ml_features,
        &revenue_tracker,
        &metrics,
    ).await;
    assert!(ml_revenue_result.is_ok(), "ML revenue tracking should succeed");

    // Test System Metrics
    let metrics_result = test_system_metrics(&metrics).await;
    assert!(metrics_result.is_ok(), "System metrics should be recorded");

    // Test Web5 Protocol Integration
    let protocol_result = test_web5_protocol_integration(&web5_integration).await;
    assert!(protocol_result.is_ok(), "Web5 protocol integration should succeed");
}

async fn test_did_auth(
    auth_manager: &AuthManager,
    web5_integration: &AdvancedWeb5Integration,
) -> Result<(), Box<dyn std::error::Error>> {
    let test_did = common::create_test_did();
    let credentials = common::create_test_credentials(&test_did);
    
    // Verify DID authentication
    let auth_result = auth_manager.verify(&credentials).await?;
    assert!(auth_result, "DID authentication failed");
    
    // Test Web5 integration with DID
    let web5_result = web5_integration
        .verify_did_auth(&test_did)
        .await?;
    assert!(web5_result.is_valid, "Web5 DID verification failed");
    
    Ok(())
}

async fn test_ml_revenue_tracking(
    ml_features: &AdvancedMLFeatures,
    revenue_tracker: &MLRevenueTracker,
    metrics: &IntegratedSystemMetrics,
) -> Result<(), Box<dyn std::error::Error>> {
    let test_data = common::create_test_ml_data();
    let context = common::create_test_context();
    
    // Process ML with revenue tracking
    let (result, revenue) = revenue_tracker
        .track_ml_operation(
            MLOperationType::Processing,
            &context,
            || ml_features.process_data(&test_data, &context),
        )
        .await?;
        
    // Verify ML results
    assert!(result.confidence > 0.7, "ML confidence too low");
    assert!(revenue.total > 0.0, "No revenue generated");
    
    // Verify metrics were recorded
    let metrics_snapshot = metrics.get_metrics_snapshot().await;
    assert!(metrics_snapshot.ml_operations > 0, "ML operations not recorded");
    assert!(metrics_snapshot.revenue_tracked > 0.0, "Revenue not tracked");
    
    Ok(())
}

async fn test_system_metrics(
    metrics: &IntegratedSystemMetrics,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get initial metrics
    let initial = metrics.get_system_health().await;
    
    // Perform test operations
    let test_ops = common::perform_test_operations().await?;
    
    // Get updated metrics
    let updated = metrics.get_system_health().await;
    
    // Verify metrics were updated
    assert!(
        updated.total_operations > initial.total_operations,
        "Operations not tracked"
    );
    
    assert!(
        updated.system_health.is_healthy,
        "System health degraded"
    );
    
    Ok(())
}

async fn test_web5_protocol_integration(
    web5_integration: &AdvancedWeb5Integration,
) -> Result<(), Box<dyn std::error::Error>> {
    let test_protocol = common::create_test_protocol();
    let test_data = common::create_test_protocol_data();
    
    // Test protocol message processing
    let result = web5_integration
        .process_protocol_message(&test_protocol, &test_data)
        .await?;
        
    assert!(result.is_valid, "Protocol message invalid");
    
    // Test DWN sync
    let sync_stats = web5_integration
        .sync_integrated_data()
        .await?;
        
    assert!(sync_stats.records_synced > 0, "No records synced");
    
    Ok(())
}
