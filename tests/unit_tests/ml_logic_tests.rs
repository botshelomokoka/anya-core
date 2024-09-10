use anya_core::ml::{
    ModelManager,
    PredictionRequest,
    PredictionResult,
    ModelType,
    Model,
    UpdateResult,
};
use anya_core::config::Config;
use anyhow::Result;
use std::collections::HashMap;

#[tokio::test]
async fn test_model_loading() -> Result<()> {
    let config = Config::load_test_config()?;
    let model_manager = ModelManager::new(&config)?;
    
    let model = model_manager.load_model(ModelType::PricePrediction).await?;
    assert!(model.is_ready());
    Ok(())
}

#[tokio::test]
async fn test_price_prediction() -> Result<()> {
    let config = Config::load_test_config()?;
    let model_manager = ModelManager::new(&config)?;
    
    let model = model_manager.load_model(ModelType::PricePrediction).await?;
    let request = PredictionRequest::new_price_prediction("BTC", 24);
    
    let prediction: PredictionResult = model.predict(request).await?;
    assert!(prediction.confidence > 0.0);
    assert!(prediction.value > 0.0);
    Ok(())
}

#[tokio::test]
async fn test_sentiment_analysis() -> Result<()> {
    let config = Config::load_test_config()?;
    let model_manager = ModelManager::new(&config)?;
    
    let model = model_manager.load_model(ModelType::SentimentAnalysis).await?;
    let request = PredictionRequest::new_sentiment_analysis("Bitcoin is performing well today.");
    
    let prediction: PredictionResult = model.predict(request).await?;
    assert!(prediction.sentiment_score >= -1.0 && prediction.sentiment_score <= 1.0);
    Ok(())
}

#[tokio::test]
async fn test_model_update() -> Result<()> {
    let config = Config::load_test_config()?;
    let model_manager = ModelManager::new(&config)?;
    
    let update_result: UpdateResult = model_manager.update_model(ModelType::PricePrediction).await?;
    assert!(update_result.is_success());
    assert!(update_result.new_version > update_result.old_version);
    Ok(())
}

#[tokio::test]
async fn test_feature_importance() -> Result<()> {
    let config = Config::load_test_config()?;
    let model_manager = ModelManager::new(&config)?;
    
    let model = model_manager.load_model(ModelType::PricePrediction).await?;
    let feature_importance: HashMap<String, f64> = model.get_feature_importance().await?;
    
    assert!(!feature_importance.is_empty());
    assert!(feature_importance.values().all(|importance| *importance >= 0.0));
    Ok(())
}

#[tokio::test]
async fn test_model_versioning() -> Result<()> {
    let config = Config::load_test_config()?;
    let model_manager = ModelManager::new(&config)?;
    
    let initial_version = model_manager.get_model_version(ModelType::PricePrediction)?;
    model_manager.update_model(ModelType::PricePrediction).await?;
    let updated_version = model_manager.get_model_version(ModelType::PricePrediction)?;
    
    assert!(updated_version > initial_version);
    Ok(())
}

#[tokio::test]
async fn test_model_performance_metrics() -> Result<()> {
    let config = Config::load_test_config()?;
    let model_manager = ModelManager::new(&config)?;
    
    let model = model_manager.load_model(ModelType::PricePrediction).await?;
    let performance_metrics = model.get_performance_metrics().await?;
    
    assert!(performance_metrics.contains_key("accuracy"));
    assert!(performance_metrics.contains_key("f1_score"));
    assert!(performance_metrics.values().all(|&value| value >= 0.0 && value <= 1.0));
    Ok(())
}

#[tokio::test]
async fn test_model_batch_prediction() -> Result<()> {
    let config = Config::load_test_config()?;
    let model_manager = ModelManager::new(&config)?;
    
    let model = model_manager.load_model(ModelType::PricePrediction).await?;
    let requests = vec![
        PredictionRequest::new_price_prediction("BTC", 24),
        PredictionRequest::new_price_prediction("ETH", 48),
        PredictionRequest::new_price_prediction("XRP", 12),
    ];
    
    let predictions: Vec<PredictionResult> = model.predict_batch(requests).await?;
    assert_eq!(predictions.len(), 3);
    assert!(predictions.iter().all(|pred| pred.confidence > 0.0 && pred.value > 0.0));
    Ok(())
}
