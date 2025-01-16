use crate::{
    auth::web5::{Web5DataManager, protocols::identity::SecurityManager},
    ml::{MLModel, FileAnalysisModel, agents::Web5AgentSystem},
    infrastructure::database::Pool,
    bitcoin::{UTXOManager, FeeEstimator},
    auth::keys::KeyManager,
};
use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

#[derive(Debug)]
pub struct UnifiedDataSystem {
    web5_manager: Arc<Web5DataManager>,
    ml_system: Arc<Web5AgentSystem>,
    security: Arc<SecurityManager>,
    db: Pool<Postgres>,
    utxo_manager: Arc<UTXOManager>,
    key_manager: Arc<KeyManager>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnifiedDataRecord {
    id: String,
    data_type: DataType,
    content: serde_json::Value,
    metadata: RecordMetadata,
    permissions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DataType {
    Transaction,
    MLPrediction,
    UserBehavior,
    MarketData,
    SecurityAudit,
    BlockchainMetrics,
}

impl UnifiedDataSystem {
    pub async fn process_data(&self, data: UnifiedDataRecord) -> Result<ProcessingResult> {
        // Encrypt sensitive data
        let encrypted = self.security.encrypt_data(&data)?;
        
        // Store in Web5 DWN
        let record_id = self.web5_manager.store_data(encrypted).await?;
        
        // Process with ML system
        let ml_insights = self.ml_system.analyze_data(&data).await?;
        
        // Generate revenue metrics
        let revenue_metrics = self.calculate_revenue_potential(&data, &ml_insights)?;
        
        // Store results
        self.store_processed_results(record_id, &ml_insights, &revenue_metrics).await?;
        
        Ok(ProcessingResult {
            record_id,
            insights: ml_insights,
            revenue_metrics,
        })
    }

    async fn store_processed_results(
        &self,
        record_id: String,
        insights: &MLInsights,
        metrics: &RevenueMetrics,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO processed_data 
            (record_id, insights, revenue_metrics, processed_at)
            VALUES ($1, $2, $3, NOW())
            "#,
            record_id,
            serde_json::to_value(insights)?,
            serde_json::to_value(metrics)?,
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub async fn get_revenue_insights(&self, timeframe: TimeFrame) -> Result<Vec<RevenueInsight>> {
        let insights = sqlx::query_as!(
            RevenueInsight,
            r#"
            SELECT 
                data_type,
                COUNT(*) as volume,
                SUM(revenue_generated) as total_revenue,
                AVG(ml_confidence) as avg_confidence
            FROM processed_data
            WHERE processed_at >= $1
            GROUP BY data_type
            "#,
            timeframe.start_date,
        )
        .fetch_all(&self.db)
        .await?;

        Ok(insights)
    }
}
