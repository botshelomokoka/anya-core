use anya_core::{
    revenue::{
        ml_revenue_features::MLRevenueFeatures,
        ml_revenue_tracking::MLRevenueTracker,
    },
    auth::enterprise::advanced_security::AdvancedSecurity,
    ml::advanced_features::AdvancedMLFeatures,
    web5::data_manager::Web5DataManager,
    monitoring::integrated_metrics::IntegratedMetrics,
};
use chrono::{Utc, Duration};

mod common;

#[tokio::test]
async fn test_complete_revenue_tracking() {
    // Setup components
    let (security, ml_features, web5_manager, metrics) = 
        common::setup_test_components().await;
        
    let revenue_features = Arc::new(MLRevenueFeatures::new(
        security.clone(),
        ml_features.clone(),
        web5_manager.clone(),
        metrics.clone(),
    ));

    // Test model revenue analysis
    let model_id = "test_model_1";
    let timeframe = TimeFrame::LastMonth;
    
    let analysis = revenue_features
        .analyze_model_revenue(model_id, timeframe)
        .await
        .expect("Revenue analysis should succeed");
        
    // Verify costs
    assert!(analysis.model_costs.total > 0.0, "Should have non-zero costs");
    assert!(analysis.model_costs.training_cost > 0.0, "Should have training costs");
    
    // Verify revenue
    assert!(analysis.prediction_revenue.total > analysis.model_costs.total,
            "Revenue should exceed costs");
            
    // Verify projections
    assert!(analysis.revenue_projections.confidence > 0.7,
            "Should have high confidence in projections");
            
    // Verify Web5 storage
    let stored_analysis = web5_manager
        .get_revenue_analysis(model_id)
        .await
        .expect("Analysis should be stored");
        
    assert_eq!(stored_analysis.model_costs.total, analysis.model_costs.total);
}

#[tokio::test]
async fn test_revenue_optimization() {
    let (security, ml_features, web5_manager, metrics) = 
        common::setup_test_components().await;
        
    let revenue_features = Arc::new(MLRevenueFeatures::new(
        security.clone(),
        ml_features.clone(),
        web5_manager.clone(),
        metrics.clone(),
    ));

    // Get initial analysis
    let model_id = "test_model_2";
    let initial_analysis = revenue_features
        .analyze_model_revenue(model_id, TimeFrame::LastMonth)
        .await
        .expect("Initial analysis should succeed");
        
    // Apply optimization suggestions
    for suggestion in &initial_analysis.optimization_suggestions {
        revenue_features
            .apply_optimization(model_id, suggestion)
            .await
            .expect("Optimization should succeed");
    }
    
    // Get updated analysis
    let updated_analysis = revenue_features
        .analyze_model_revenue(model_id, TimeFrame::LastMonth)
        .await
        .expect("Updated analysis should succeed");
        
    // Verify improvements
    assert!(
        updated_analysis.prediction_revenue.total > 
        initial_analysis.prediction_revenue.total,
        "Revenue should improve after optimizations"
    );
    
    assert!(
        updated_analysis.model_costs.total <
        initial_analysis.model_costs.total,
        "Costs should decrease after optimizations"
    );
}
