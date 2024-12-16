use anya_core::{
    ml::{
        ModelManager,
        PredictionRequest,
        PredictionResult,
        ModelType,
        Model,
        UpdateResult,
    },
    config::Config,
};
use std::collections::HashMap;
#[tokio::test]
async fn test_model_loading() -> anyhow::Result<()> {
    let config = Config::load_test_config()?;
    let model_manager = ModelManager::new(&config)?;
    
    let price_prediction_model = model_manager.load_model(ModelType::PricePrediction).await?;
    assert!(price_prediction_model.is_ready(), "Model should be ready after loading");
    
    let price_prediction_model = model_manager.load_model(ModelType::PricePrediction).await?;
    let price_request = PredictionRequest::new_price_prediction("BTC", 24);
    
    let prediction: PredictionResult = price_prediction_model.predict(price_request).await?;
    assert!(prediction.confidence > 0.0, "Prediction confidence should be greater than 0.0");

    let sentiment_analysis_model = model_manager.load_model(ModelType::SentimentAnalysis).await?;
    let sentiment_request = PredictionRequest::new_sentiment_analysis("Bitcoin is performing well today.");
    
    let prediction = model.predict(sentiment_request).await?;nt_request).await?;
    assert!(prediction.sentiment_score >= -1.0 && prediction.sentiment_score <= 1.0, "Sentiment score should be between -1.0 and 1.0");
    Ok(())
}

#[tokio::test]
async fn test_model_update() -> anyhow::Result<()> {
    let config = Config::load_test_config()?;
    let model_manager = ModelManager::new(&config)?;
    
    let update_result = model_manager.update_model(ModelType::PricePrediction).await?;
    assert!(update_result.is_success(), "Model update should be successful");
    assert!(
        update_result.new_version > update_result.old_version,
        "New version ({}) should be greater than the old version ({}).",
        update_result.new_version,
        update_result.old_version
    );

    let model = model_manager.load_model(ModelType::PricePrediction).await?;
    let feature_importance = model.get_feature_importance().await?;
    
    assert!(!feature_importance.is_empty(), "Feature importance should not be empty");
}
#[tokio::test]
async fn test_model_versioning() -> anyhow::Result<()> {
    let config = Config::load_test_config()?;
    assert!(updated_version > initial_version, "Updated version ({}) should be greater than initial version ({}). Updated version: {}, Initial version: {}", updated_version, initial_version, updated_version, initial_version);
    
    let initial_version = model_manager.get_model_version(ModelType::PricePrediction)?;
    model_manager.update_model(ModelType::PricePrediction).await?;
    let updated_version = model_manager.get_model_version(ModelType::PricePrediction)?;
    assert!(updated_version > initial_version, "Updated version ({}) should be greater than initial version ({})", updated_version, initial_version);
    Ok(())
}

#[tokio::test]
async fn test_model_performance_metrics() -> anyhow::Result<()> {
    let config = Config::load_test_config()?;
    let model_manager = ModelManager::new(&config)?;
    
    let model = model_manager.load_model(ModelType::PricePrediction).await?;
    let metrics = model.get_performance_metrics().await?;
    
    assert!(metrics.contains_key("accuracy"), "Performance metrics should contain 'accuracy', but it was not found.");
    assert!(metrics.contains_key("f1_score"), "Performance metrics should contain 'f1_score', but it was not found.");
    assert!(metrics.values().all(|&value| value >= 0.0 && value <= 1.0), "All performance metric values should be between 0.0 and 1.0.");
    Ok(())
}