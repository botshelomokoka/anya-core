use anya_core::{
    auth::{AuthManager, enterprise::advanced_security::AdvancedSecurity},
    ml::advanced_processing::AdvancedMLProcessor,
    revenue::advanced_tracking::AdvancedRevenueTracker,
    web5::data_manager::Web5DataManager,
};
use chrono::{Utc, Duration};
use did_key::Ed25519KeyPair;
use bitcoin::Network;

mod common;

#[tokio::test]
async fn test_complete_system_integration() {
    // Setup all components
    let db = common::setup_test_db().await;
    let (auth_key, web5_manager) = common::create_test_identity().await;
    
    let auth_manager = Arc::new(AuthManager::new());
    let security = Arc::new(AdvancedSecurity::new(auth_manager.clone()));
    let processor = Arc::new(AdvancedMLProcessor::new(
        security.clone(),
        web5_manager.clone(),
    ));
    let revenue_tracker = Arc::new(AdvancedRevenueTracker::new(
        auth_manager.clone(),
        security.clone(),
        web5_manager.clone(),
        processor.clone(),
    ));

    // Test complete processing flow
    let test_request = UnifiedProcessingRequest {
        credentials: common::create_test_credentials(),
        security_context: common::create_test_security_context(),
        data: common::create_test_processing_data(),
    };

    // Process request through entire system
    let result = process_complete_request(
        &test_request,
        &security,
        &processor,
        &revenue_tracker,
        &web5_manager,
    ).await.expect("Processing should succeed");

    // Verify ML insights
    assert!(result.ml_insights.confidence > 0.7, "ML confidence should be high");
    assert!(!result.ml_insights.predictions.is_empty(), "Should have predictions");

    // Verify revenue tracking
    assert!(result.revenue_metrics.total > 0.0, "Should have revenue impact");
    assert!(result.revenue_metrics.ml_cost > 0.0, "Should track ML costs");

    // Verify Web5 storage
    let stored_data = web5_manager
        .get_record(&result.data.record_id)
        .await
        .expect("Data should be stored in Web5");
    assert_eq!(stored_data.data_type, DataType::ProcessedResult);

    // Verify security audit logs
    let audit_logs = security
        .get_audit_logs_for_operation(&result.operation_id)
        .await
        .expect("Should have audit logs");
    assert!(!audit_logs.is_empty(), "Should have security audit entries");
}

#[tokio::test]
async fn test_revenue_tracking_integration() {
    let (security, processor, revenue_tracker, web5_manager) = 
        common::setup_test_system().await;

    // Process multiple requests to test revenue accumulation
    let requests = common::generate_test_requests(10);
    let mut total_revenue = 0.0;

    for request in requests {
        let result = process_complete_request(
            &request,
            &security,
            &processor,
            &revenue_tracker,
            &web5_manager,
        ).await.expect("Processing should succeed");

        total_revenue += result.revenue_metrics.total;
    }

    // Verify revenue tracking accuracy
    let tracked_revenue = revenue_tracker
        .get_total_revenue(TimeFrame::Last24Hours)
        .await
        .expect("Should get revenue data");

    assert_eq!(
        tracked_revenue.total,
        total_revenue,
        "Total revenue should match processed amount"
    );

    // Verify ML model updates from revenue data
    let model_updates = processor
        .get_revenue_based_updates()
        .await
        .expect("Should get model updates");
    assert!(!model_updates.is_empty(), "Should have model updates from revenue");
}

#[tokio::test]
async fn test_security_integration() {
    let (security, processor, revenue_tracker, web5_manager) = 
        common::setup_test_system().await;

    // Test invalid credentials
    let invalid_request = UnifiedProcessingRequest {
        credentials: common::create_invalid_credentials(),
        security_context: common::create_test_security_context(),
        data: common::create_test_processing_data(),
    };

    let result = process_complete_request(
        &invalid_request,
        &security,
        &processor,
        &revenue_tracker,
        &web5_manager,
    ).await;

    assert!(result.is_err(), "Should reject invalid credentials");

    // Verify security metrics
    let metrics = security.get_security_metrics().await;
    assert!(metrics.failed_auth_attempts > 0, "Should track failed attempts");
}

async fn process_complete_request(
    request: &UnifiedProcessingRequest,
    security: &AdvancedSecurity,
    processor: &AdvancedMLProcessor,
    revenue_tracker: &AdvancedRevenueTracker,
    web5_manager: &Web5DataManager,
) -> Result<UnifiedResponse, ProcessingError> {
    // Verify security context
    let context = security
        .verify_multi_factor(&request.credentials, &request.security_context)
        .await?;

    // Process with revenue tracking
    let (result, revenue_impact) = revenue_tracker
        .track_operation(
            OperationType::Processing,
            &context,
            || processor.process_with_revenue(&request.data, &context),
        )
        .await?;

    // Store in Web5
    web5_manager
        .store_processing_result(&result)
        .await?;

    Ok(UnifiedResponse {
        success: true,
        data: result,
        ml_insights: result.insights,
        revenue_metrics: revenue_impact,
        security_metrics: context.metrics,
    })
}
