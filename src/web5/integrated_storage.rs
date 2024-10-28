use crate::{
    auth::{AuthManager, BlockchainAuth},
    ml::advanced_features::AdvancedMLFeatures,
    revenue::ml_revenue_tracking::MLRevenueTracker,
    monitoring::integrated_metrics::IntegratedMetrics,
};
use did_key::{DIDCore, Ed25519KeyPair};
use web5::dwn::{DataFormat, Message, MessageStore};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct IntegratedWeb5Storage {
    auth_manager: Arc<AuthManager>,
    message_store: MessageStore,
    metrics: Arc<IntegratedMetrics>,
    key_pair: Ed25519KeyPair,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntegratedRecord {
    record_type: RecordType,
    data: serde_json::Value,
    metadata: RecordMetadata,
    auth_context: AuthContext,
    revenue_data: Option<RevenueData>,
    ml_insights: Option<MLInsights>,
}

impl IntegratedWeb5Storage {
    pub async fn store_integrated_data(
        &self,
        record: IntegratedRecord,
        context: &SecurityContext,
    ) -> Result<String, StorageError> {
        // Track metrics
        let tracking_start = std::time::Instant::now();
        
        // Verify auth context
        self.auth_manager
            .verify(&record.auth_context.credentials)
            .await?;
            
        // Process with ML if needed
        if let Some(ml_data) = &record.ml_insights {
            self.process_ml_data(ml_data, context).await?;
        }
        
        // Track revenue if present
        if let Some(revenue_data) = &record.revenue_data {
            self.track_revenue_data(revenue_data, context).await?;
        }
        
        // Create Web5 message
        let message = Message::new()
            .with_data(record)
            .with_schema(&record.record_type.to_string())
            .with_protocol("integrated-data")
            .sign_with(&self.key_pair)?;
            
        // Store message
        let record_id = self.message_store.store(message).await?;
        
        // Update metrics
        self.metrics.web5_metrics.record_storage_operation(
            tracking_start.elapsed(),
            &record,
        );
        
        Ok(record_id)
    }

    pub async fn query_integrated_data(
        &self,
        query: IntegratedQuery,
        context: &SecurityContext,
    ) -> Result<Vec<IntegratedRecord>, StorageError> {
        // Verify auth
        self.auth_manager
            .verify(&context.credentials)
            .await?;
            
        // Query records
        let messages = self.message_store
            .query()
            .protocol("integrated-data")
            .filter(query.into_filter())
            .execute()
            .await?;
            
        // Process records
        let records = messages
            .into_iter()
            .filter_map(|msg| msg.data::<IntegratedRecord>().ok())
            .collect();
            
        Ok(records)
    }

    pub async fn sync_with_dwn(&self) -> Result<SyncStats, StorageError> {
        let tracking_start = std::time::Instant::now();
        
        // Perform sync
        let stats = self.message_store.sync().await?;
        
        // Update metrics
        self.metrics.web5_metrics.record_sync_operation(
            tracking_start.elapsed(),
            &stats,
        );
        
        Ok(stats)
    }
}
