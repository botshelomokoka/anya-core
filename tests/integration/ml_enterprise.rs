use anya_core::{
    auth::{AuthManager, BlockchainAuth},
    ml::enterprise_processing::MLProcessor,
    revenue::tracking::RevenueTracker,
    web5::data_manager::Web5DataManager,
};
use did_key::Ed25519KeyPair;
use chrono::{Utc, Duration};

mod common;

#[tokio::test]
async fn test_full_ml_processing_flow() {
    let db = common::setup_test_db().await;
    let (auth_key, web5_manager) = common::create_test_identity().await;
    
    let processor = MLProcessor::new(
        Arc::new(AuthManager::new()),
        Arc::new(web5_manager),
        Arc::new(RevenueTracker::new(db.clone())),
    );

    // Test data processing with revenue tracking
    let test_data = UnifiedDataRecord {
        id: "test-1".to_string(),
        data_type: DataType::MarketData,
        content: serde_json::json!({
            "market": "BTC/USD",
            "price": 50000,
            "volume": 100
        }),
        metadata: RecordMetadata::new("test"),
        permissions: vec!["read".to_string()],
    };

    let context = SecurityContext {
        access_level: AccessLevel::MLProcessing,
        permissions: vec![Permission::ReadWrite],
        session_key: [0u8; 32],
        context_data: vec![],
    };

    let result = processor
        .process_enterprise_data(&test_data, &context)
        .await
        .expect("Processing should succeed");

    // Verify ML insights
    assert!(result.confidence > 0.5, "Confidence should be reasonable");
    assert!(!result.insights.is_empty(), "Should have insights");

    // Verify revenue tracking
    let revenue_impact = result.revenue_impact;
    assert!(revenue_impact.total > 0.0, "Should have revenue impact");

    // Verify Web5 storage
    let stored_record = web5_manager
        .get_record(&result.insights[0].model_id)
        .await
        .expect("Record should be stored");
    assert_eq!(stored_record.data_type, DataType::MLPrediction);
}

#[tokio::test]
async fn test_model_updates_with_feedback() {
    let db = common::setup_test_db().await;
    let processor = common::create_test_processor(db).await;

    // Initial prediction
    let initial_result = processor
        .process_market_data(&common::create_test_market_data(), &common::create_test_context())
        .await
        .expect("Initial processing should succeed");

    // Provide feedback
    let feedback = ProcessingFeedback {
        model_id: initial_result.insights[0].model_id.clone(),
        actual_outcome: serde_json::json!({"price": 51000}),
        feedback_type: FeedbackType::MarketMovement,
        timestamp: Utc::now(),
    };

    processor.update_models(&feedback).await.expect("Model update should succeed");

    // Verify model improvement
    let updated_result = processor
        .process_market_data(&common::create_test_market_data(), &common::create_test_context())
        .await
        .expect("Updated processing should succeed");

    assert!(
        updated_result.confidence > initial_result.confidence,
        "Model confidence should improve with feedback"
    );
}

#[tokio::test]
async fn test_revenue_tracking_accuracy() {
    let db = common::setup_test_db().await;
    let processor = common::create_test_processor(db.clone()).await;
    let revenue_tracker = RevenueTracker::new(db);

    // Process multiple requests
    let requests = common::generate_test_requests(10);
    let mut total_revenue = 0.0;

    for request in requests {
        let result = processor
            .process_enterprise_data(&request, &common::create_test_context())
            .await
            .expect("Processing should succeed");

        total_revenue += result.revenue_impact.total;
    }

    // Verify revenue tracking
    let tracked_revenue = revenue_tracker
        .get_total_revenue(TimeFrame::Last24Hours)
        .await
        .expect("Revenue tracking should work");

    assert_eq!(
        tracked_revenue.total,
        total_revenue,
        "Tracked revenue should match processed revenue"
    );
}
