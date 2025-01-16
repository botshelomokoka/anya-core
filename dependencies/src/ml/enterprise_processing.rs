use crate::{
    auth::{AuthManager, BlockchainAuth},
    web5::data_manager::Web5DataManager,
    revenue::tracking::RevenueTracker,
};
use serde::{Serialize, Deserialize};
use std::sync::Arc;

pub struct MLProcessor {
    auth_manager: Arc<AuthManager>,
    web5_manager: Arc<Web5DataManager>,
    revenue_tracker: Arc<RevenueTracker>,
    model_registry: ModelRegistry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MLProcessingResult {
    insights: Vec<MLInsight>,
    confidence: f64,
    revenue_impact: RevenueImpact,
    processing_metrics: ProcessingMetrics,
}

impl MLProcessor {
    pub async fn process_enterprise_data(
        &self,
        data: &UnifiedDataRecord,
        context: &SecurityContext,
    ) -> Result<MLProcessingResult> {
        // Track processing start for revenue
        let tracking_id = self.revenue_tracker
            .start_processing_track(data)
            .await?;

        // Process with appropriate models
        let results = match data.data_type {
            DataType::Transaction => {
                self.process_transaction_data(data, context).await?
            },
            DataType::UserBehavior => {
                self.process_user_behavior(data, context).await?
            },
            DataType::MarketData => {
                self.process_market_data(data, context).await?
            },
            _ => self.process_generic_data(data, context).await?,
        };

        // Store results in Web5 DWN
        let record = self.prepare_web5_record(&results)?;
        self.web5_manager.store_data(record).await?;

        // Complete revenue tracking
        self.revenue_tracker
            .complete_processing_track(tracking_id, &results)
            .await?;

        Ok(results)
    }

    async fn process_market_data(
        &self,
        data: &UnifiedDataRecord,
        context: &SecurityContext,
    ) -> Result<MLProcessingResult> {
        let models = self.model_registry.get_market_models()?;
        
        let mut insights = Vec::new();
        let mut confidence_sum = 0.0;
        
        for model in models {
            let prediction = model.predict(data)?;
            insights.push(MLInsight {
                model_id: model.id(),
                prediction: prediction.clone(),
                confidence: prediction.confidence,
                metadata: self.generate_insight_metadata(&prediction),
            });
            confidence_sum += prediction.confidence;
        }

        let avg_confidence = confidence_sum / models.len() as f64;
        
        Ok(MLProcessingResult {
            insights,
            confidence: avg_confidence,
            revenue_impact: self.calculate_revenue_impact(&insights)?,
            processing_metrics: self.collect_processing_metrics(),
        })
    }

    async fn update_models(&self, feedback: &ProcessingFeedback) -> Result<()> {
        let models = self.model_registry.get_affected_models(feedback)?;
        
        for model in models {
            model.update_with_feedback(feedback).await?;
            
            // Store updated model in Web5 DWN
            let record = self.prepare_model_record(model)?;
            self.web5_manager.store_data(record).await?;
        }

        Ok(())
    }
}
