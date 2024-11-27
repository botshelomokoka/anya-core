use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;

use super::{MLAgent, AgentConfig};
use crate::metrics::MetricsCollector;
use crate::analytics::{AnalyticsEngine, MarketAnalytics};
use crate::ml::models::{PredictionModel, ModelConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketMetrics {
    pub price_volatility: f64,
    pub market_sentiment: f64,
    pub trading_volume: f64,
    pub liquidity_index: f64,
    pub trend_strength: f64,
    pub market_efficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPrediction {
    pub price_direction: f64,
    pub volume_forecast: f64,
    pub volatility_forecast: f64,
    pub confidence_score: f64,
    pub risk_assessment: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketStrategy {
    pub pricing_adjustment: f64,
    pub volume_target: f64,
    pub risk_tolerance: f64,
    pub execution_speed: f64,
}

pub struct MarketAgent {
    metrics: Arc<MetricsCollector>,
    analytics: Arc<RwLock<AnalyticsEngine>>,
    prediction_model: Arc<RwLock<PredictionModel>>,
    market_analytics: Arc<RwLock<MarketAnalytics>>,
    current_strategy: RwLock<MarketStrategy>,
}

impl MarketAgent {
    pub fn new(
        metrics: Arc<MetricsCollector>,
        analytics: Arc<RwLock<AnalyticsEngine>>,
        model_config: ModelConfig,
    ) -> Result<Self> {
        Ok(Self {
            metrics,
            analytics,
            prediction_model: Arc::new(RwLock::new(PredictionModel::new(model_config)?)),
            market_analytics: Arc::new(RwLock::new(MarketAnalytics::new())),
            current_strategy: RwLock::new(MarketStrategy {
                pricing_adjustment: 0.0,
                volume_target: 1000.0,
                risk_tolerance: 0.5,
                execution_speed: 1.0,
            }),
        })
    }

    pub async fn analyze_market_conditions(&self) -> Result<MarketMetrics> {
        let analytics = self.analytics.read().await;
        let market = self.market_analytics.read().await;
        
        Ok(MarketMetrics {
            price_volatility: market.calculate_volatility().await?,
            market_sentiment: market.analyze_sentiment().await?,
            trading_volume: market.get_trading_volume().await?,
            liquidity_index: market.calculate_liquidity().await?,
            trend_strength: market.analyze_trend_strength().await?,
            market_efficiency: market.calculate_market_efficiency().await?,
        })
    }

    pub async fn predict_market_movement(&self) -> Result<MarketPrediction> {
        let market_metrics = self.analyze_market_conditions().await?;
        let model = self.prediction_model.read().await;
        
        // Prepare input features
        let features = vec![
            market_metrics.price_volatility,
            market_metrics.market_sentiment,
            market_metrics.trading_volume,
            market_metrics.liquidity_index,
            market_metrics.trend_strength,
        ];
        
        // Make predictions
        let predictions = model.predict(&features).await?;
        
        Ok(MarketPrediction {
            price_direction: predictions[0],
            volume_forecast: predictions[1],
            volatility_forecast: predictions[2],
            confidence_score: model.calculate_confidence(&predictions).await?,
            risk_assessment: model.assess_risk(&predictions).await?,
        })
    }

    pub async fn optimize_market_strategy(&self, metrics: &MarketMetrics) -> Result<MarketStrategy> {
        let prediction = self.predict_market_movement().await?;
        let market = self.market_analytics.read().await;
        
        // Calculate optimal strategy parameters
        let pricing_adjustment = self.calculate_pricing_adjustment(
            metrics,
            &prediction,
        ).await?;
        
        let volume_target = self.calculate_volume_target(
            metrics,
            &prediction,
        ).await?;
        
        let risk_tolerance = self.calculate_risk_tolerance(
            metrics,
            &prediction,
        ).await?;
        
        let execution_speed = self.calculate_execution_speed(
            metrics,
            &prediction,
        ).await?;
        
        Ok(MarketStrategy {
            pricing_adjustment,
            volume_target,
            risk_tolerance,
            execution_speed,
        })
    }

    async fn calculate_pricing_adjustment(
        &self,
        metrics: &MarketMetrics,
        prediction: &MarketPrediction,
    ) -> Result<f64> {
        let base_adjustment = prediction.price_direction;
        let volatility_factor = 1.0 - metrics.price_volatility;
        let sentiment_factor = metrics.market_sentiment;
        
        Ok(base_adjustment * volatility_factor * sentiment_factor)
    }

    async fn calculate_volume_target(
        &self,
        metrics: &MarketMetrics,
        prediction: &MarketPrediction,
    ) -> Result<f64> {
        let base_volume = metrics.trading_volume;
        let volume_growth = prediction.volume_forecast;
        let liquidity_factor = metrics.liquidity_index;
        
        Ok(base_volume * (1.0 + volume_growth) * liquidity_factor)
    }

    async fn calculate_risk_tolerance(
        &self,
        metrics: &MarketMetrics,
        prediction: &MarketPrediction,
    ) -> Result<f64> {
        let base_risk = 0.5;  // Default risk tolerance
        let market_risk = prediction.risk_assessment;
        let efficiency_factor = metrics.market_efficiency;
        
        Ok((base_risk * (1.0 - market_risk) * efficiency_factor)
            .max(0.1)  // Minimum risk tolerance
            .min(0.9)) // Maximum risk tolerance
    }

    async fn calculate_execution_speed(
        &self,
        metrics: &MarketMetrics,
        prediction: &MarketPrediction,
    ) -> Result<f64> {
        let base_speed = 1.0;
        let urgency_factor = prediction.confidence_score;
        let market_condition = metrics.trend_strength;
        
        Ok((base_speed * urgency_factor * market_condition)
            .max(0.5)  // Minimum speed
            .min(2.0)) // Maximum speed
    }

    pub async fn adjust_pricing_strategy(&self, metrics: &MarketMetrics) -> Result<()> {
        let optimal_strategy = self.optimize_market_strategy(metrics).await?;
        let mut current_strategy = self.current_strategy.write().await;
        
        // Update strategy with optimal parameters
        *current_strategy = optimal_strategy;
        
        Ok(())
    }
}

#[async_trait]
impl MLAgent for MarketAgent {
    async fn train(&self) -> Result<()> {
        let market_data = self.market_analytics.read().await.get_training_data().await?;
        let mut model = self.prediction_model.write().await;
        model.train(&market_data).await?;
        Ok(())
    }

    async fn predict(&self) -> Result<Vec<f64>> {
        let prediction = self.predict_market_movement().await?;
        Ok(vec![
            prediction.price_direction,
            prediction.volume_forecast,
            prediction.volatility_forecast,
            prediction.confidence_score,
            prediction.risk_assessment,
        ])
    }

    async fn update(&self) -> Result<()> {
        let metrics = self.analyze_market_conditions().await?;
        self.adjust_pricing_strategy(&metrics).await?;
        Ok(())
    }

    async fn get_metrics(&self) -> Result<Vec<f64>> {
        let metrics = self.analyze_market_conditions().await?;
        Ok(vec![
            metrics.price_volatility,
            metrics.market_sentiment,
            metrics.trading_volume,
            metrics.liquidity_index,
            metrics.trend_strength,
            metrics.market_efficiency,
        ])
    }
}
