use crate::ml_logic::{
    FederatedLearning,
    MLCore,
    ProcessedData,
    TrainedModel,
    Prediction,
    OptimizedAction,
};
use crate::blockchain::{
    BitcoinSupport,
    STXSupport,
    LightningSupport,
};
use crate::web5::Web5Support;
use crate::wallet::UserWallet;

use tokio;
use std::error::Error;
use std::time::Duration;
use bitcoin::Amount;

#[tokio::test]
async fn test_model_training() -> Result<(), Box<dyn Error>> {
    // Setup test environment
    let fl = setup_test_environment().await?;
    
    // Test data
    let user_id = "test_user";
    let test_input = vec![0.1, 0.2, 0.3, 0.4, 0.5];
    
    // Test local model training
    fl.train_local_model(user_id, &test_input).await?;
    
    // Verify model metrics
    let accuracy = fl.get_model_accuracy().await?;
    let loss = fl.get_model_loss().await?;
    
    assert!(accuracy > 0.0 && accuracy <= 1.0);
    assert!(loss >= 0.0);
    
    Ok(())
}

#[tokio::test]
async fn test_model_aggregation() -> Result<(), Box<dyn Error>> {
    let mut fl = setup_test_environment().await?;
    
    // Train multiple local models
    let test_users = vec!["user1", "user2", "user3"];
    let test_inputs = vec![
        vec![0.1, 0.2, 0.3],
        vec![0.2, 0.3, 0.4],
        vec![0.3, 0.4, 0.5],
    ];
    
    for (user, input) in test_users.iter().zip(test_inputs.iter()) {
        fl.train_local_model(user, input).await?;
    }
    
    // Test aggregation
    fl.aggregate_models().await?;
    
    // Verify aggregation results
    let diversity = fl.calculate_model_diversity();
    assert!(diversity >= 0.0 && diversity <= 1.0);
    
    Ok(())
}

#[tokio::test]
async fn test_model_versioning() -> Result<(), Box<dyn Error>> {
    let mut fl = setup_test_environment().await?;
    
    // Train and update model
    fl.train_local_model("test_user", &vec![0.1, 0.2, 0.3]).await?;
    fl.update_model_version().await?;
    
    // Verify version update
    let model_hash = fl.compute_model_hash().await?;
    assert_eq!(model_hash.len(), 32); // SHA-256 hash
    
    Ok(())
}

#[tokio::test]
async fn test_performance_benchmarks() -> Result<(), Box<dyn Error>> {
    let fl = setup_test_environment().await?;
    
    // Benchmark training time
    let start = std::time::Instant::now();
    fl.train_local_model("bench_user", &vec![0.1, 0.2, 0.3]).await?;
    let training_time = start.elapsed();
    
    // Verify performance metrics
    assert!(training_time < Duration::from_secs(5)); // Should complete within 5 seconds
    
    // Test convergence rate
    let convergence_rate = fl.get_convergence_rate().await?;
    assert!(convergence_rate > 0.0);
    
    Ok(())
}

#[tokio::test]
async fn test_security_features() -> Result<(), Box<dyn Error>> {
    let fl = setup_test_environment().await?;
    
    // Test data encryption
    let test_data = b"sensitive data";
    let encrypted = fl.encrypt_web5_data(test_data).await?;
    let decrypted = fl.decrypt_web5_data(&encrypted).await?;
    
    assert_eq!(test_data, &decrypted[..]);
    
    // Test data provenance
    let metadata = serde_json::json!({
        "source": "test_user",
        "timestamp": chrono::Utc::now().timestamp(),
    });
    
    fl.verify_data_provenance(&metadata).await?;
    
    Ok(())
}

async fn setup_test_environment() -> Result<FederatedLearning, Box<dyn Error>> {
    // Initialize test components
    let bitcoin_support = BitcoinSupport::new_test_instance();
    let stx_support = STXSupport::new_test_instance();
    let lightning_support = LightningSupport::new_test_instance();
    let web5_support = Web5Support::new_test_instance();
    let user_wallet = UserWallet::new_test_instance();
    
    // Create federated learning instance
    FederatedLearning::new(
        bitcoin_support,
        stx_support,
        lightning_support,
        web5_support,
        user_wallet,
    )
}
