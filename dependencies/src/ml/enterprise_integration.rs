use crate::integration::unified_data_system::UnifiedDataSystem;
use crate::ml::agents::Web5AgentSystem;
use anyhow::Result;

pub struct MLEnterpriseIntegration {
    unified_system: Arc<UnifiedDataSystem>,
    agent_system: Arc<Web5AgentSystem>,
    metrics: MLEnterpriseMetrics,
}

impl MLEnterpriseIntegration {
    pub async fn process_enterprise_data(&self, data: &UnifiedDataRecord) -> Result<MLInsights> {
        // Track metrics
        self.metrics.record_processing_start();

        // Process with appropriate ML models
        let insights = match data.data_type {
            DataType::Transaction => {
                self.process_transaction_data(data).await?
            },
            DataType::UserBehavior => {
                self.process_user_behavior(data).await?
            },
            DataType::MarketData => {
                self.process_market_data(data).await?
            },
            _ => self.process_generic_data(data).await?,
        };

        // Store insights in Web5 DWN
        self.store_ml_insights(&insights).await?;

        // Update metrics
        self.metrics.record_processing_complete(&insights);

        Ok(insights)
    }

    async fn store_ml_insights(&self, insights: &MLInsights) -> Result<()> {
        let record = UnifiedDataRecord {
            data_type: DataType::MLPrediction,
            content: serde_json::to_value(insights)?,
            metadata: RecordMetadata::new("ml_insights"),
            permissions: vec!["enterprise_read".to_string()],
        };

        self.unified_system.process_data(record).await?;
        Ok(())
    }

    pub async fn get_revenue_predictions(&self, timeframe: TimeFrame) -> Result<RevenuePredictions> {
        let historical_data = self.unified_system
            .get_revenue_insights(timeframe)
            .await?;

        let predictions = self.agent_system
            .predict_revenue(historical_data)
            .await?;

        Ok(predictions)
    }
}
