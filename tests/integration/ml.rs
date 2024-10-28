use super::*;
use anya_core::ml::model::MLModel;

#[tokio::test]
async fn test_model_training_and_prediction() {
    let mut trainer = ModelTrainer::new(0.2); // 20% validation split
    
    // Add test data
    trainer.add_training_data(vec![1.0, 0.0, 0.0], 0.0); // Bitcoin file
    trainer.add_training_data(vec![0.0, 1.0, 0.0], 1.0); // Lightning file
    
    let model = trainer.train().await.expect("Failed to train model");
    assert!(model.validation_score > 0.7, "Model validation score too low");
}

#[tokio::test]
async fn test_feature_extraction() {
    let db = setup_test_db().await;
    let file_tracker = FileTracker::new();
    
    let test_content = r#"
        use bitcoin::secp256k1::SecretKey;
        use lightning::ln::channel::Channel;
    "#;
    
    let features = file_tracker.extract_features(test_content);
    assert_eq!(features.len(), 2, "Should detect Bitcoin and Lightning imports");
}
