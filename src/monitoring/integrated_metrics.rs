use crate::{
    auth::{AuthManager, enterprise::advanced_security::AdvancedSecurity},
    ml::advanced_features::AdvancedMLFeatures,
    revenue::ml_revenue_tracking::MLRevenueTracker,
    web5::data_manager::Web5DataManager,
};
use metrics::{Counter, Gauge, Histogram};
use opentelemetry::trace::{Tracer, TracerProvider};
use std::sync::Arc;

#[derive(Debug)]
pub struct IntegratedMetrics {
    auth_metrics: AuthMetrics,
    ml_metrics: MLMetrics,
    revenue_metrics: RevenueMetrics,
    web5_metrics: Web5Metrics,
    system_metrics: SystemMetrics,
}

impl IntegratedMetrics {
    pub fn new() -> Self {
        Self {
            auth_metrics: AuthMetrics::new(),
            ml_metrics: MLMetrics::new(),
            revenue_metrics: RevenueMetrics::new(),
            web5_metrics: Web5Metrics::new(),
            system_metrics: SystemMetrics::new(),
        }
    }

    pub async fn track_operation<T>(
        &self,
        operation_type: OperationType,
        context: &SecurityContext,
        operation: impl FnOnce() -> Result<T, MetricsError>,
    ) -> Result<T, MetricsError> {
        // Start tracking all metrics
        let tracking_start = std::time::Instant::now();
        
        // Track auth metrics
        self.auth_metrics.start_operation(operation_type, context);
        
        // Track ML metrics
        let ml_tracking = self.ml_metrics.start_tracking();
        
        // Execute operation
        let result = operation()?;
        
        // Record completion metrics
        let duration = tracking_start.elapsed();
        
        // Update all metrics
        self.update_all_metrics(operation_type, duration, &result, context).await?;
        
        Ok(result)
    }

    async fn update_all_metrics<T>(
        &self,
        operation_type: OperationType,
        duration: Duration,
        result: &T,
        context: &SecurityContext,
    ) -> Result<(), MetricsError> {
        // Update auth metrics
        self.auth_metrics.record_operation_complete(operation_type, duration);
        
        // Update ML metrics
        self.ml_metrics.record_processing_complete(duration);
        
        // Update revenue metrics
        self.revenue_metrics.record_operation_complete(operation_type, result);
        
        // Update Web5 metrics
        self.web5_metrics.record_data_operation(operation_type);
        
        // Update system metrics
        self.system_metrics.record_operation_complete(duration);
        
        Ok(())
    }

    pub async fn get_metrics_snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            auth_metrics: self.auth_metrics.get_snapshot().await,
            ml_metrics: self.ml_metrics.get_snapshot().await,
            revenue_metrics: self.revenue_metrics.get_snapshot().await,
            web5_metrics: self.web5_metrics.get_snapshot().await,
            system_metrics: self.system_metrics.get_snapshot().await,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuthMetrics {
    pub successful_auths: Counter,
    pub failed_auths: Counter,
    pub auth_duration: Histogram,
    pub active_sessions: Gauge,
}

#[derive(Debug, Clone)]
pub struct MLMetrics {
    pub prediction_duration: Histogram,
    pub training_duration: Histogram,
    pub model_accuracy: Gauge,
    pub feature_extraction_errors: Counter,
}

#[derive(Debug, Clone)]
pub struct RevenueMetrics {
    pub total_revenue: Counter,
    pub operation_costs: Counter,
    pub profit_margin: Gauge,
    pub revenue_per_operation: Histogram,
}

#[derive(Debug, Clone)]
pub struct Web5Metrics {
    pub dwn_operations: Counter,
    pub sync_duration: Histogram,
    pub storage_size: Gauge,
    pub sync_errors: Counter,
}

#[derive(Debug, Clone)]
pub struct SystemMetrics {
    pub cpu_usage: Gauge,
    pub memory_usage: Gauge,
    pub open_connections: Gauge,
    pub operation_latency: Histogram,
}
