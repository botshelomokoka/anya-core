use super::advanced_metrics::AdvancedMetrics;
use crate::web5::advanced_integration::AdvancedWeb5Integration;
use metrics::{Counter, Gauge, Histogram};
use std::sync::Arc;

#[derive(Debug)]
pub struct IntegratedSystemMetrics {
    advanced_metrics: Arc<AdvancedMetrics>,
    web5_integration: Arc<AdvancedWeb5Integration>,
    system_counters: SystemCounters,
    system_gauges: SystemGauges,
    system_histograms: SystemHistograms,
}

impl IntegratedSystemMetrics {
    pub async fn track_system_operation<T>(
        &self,
        operation_type: SystemOperationType,
        context: &SecurityContext,
        operation: impl FnOnce() -> Result<T, MetricsError>,
    ) -> Result<T, MetricsError> {
        let tracking_start = std::time::Instant::now();
        
        // Track advanced metrics
        let result = self.advanced_metrics
            .track_integrated_operation(
                operation_type.into(),
                context,
                operation,
            )
            .await?;
            
        // Update system metrics
        self.update_system_metrics(
            operation_type,
            tracking_start.elapsed(),
        );
        
        Ok(result)
    }

    fn update_system_metrics(
        &self,
        operation_type: SystemOperationType,
        duration: Duration,
    ) {
        // Update counters
        self.system_counters.update(operation_type);
        
        // Update gauges
        self.system_gauges.update(operation_type);
        
        // Update histograms
        self.system_histograms.record(operation_type, duration);
    }

    pub async fn get_system_health(&self) -> SystemHealth {
        SystemHealth {
            web5_health: self.check_web5_health().await,
            ml_health: self.check_ml_health().await,
            revenue_health: self.check_revenue_health().await,
            system_health: self.check_system_health().await,
        }
    }
}

#[derive(Debug)]
struct SystemCounters {
    total_operations: Counter,
    successful_operations: Counter,
    failed_operations: Counter,
    system_events: Counter,
}

#[derive(Debug)]
struct SystemGauges {
    system_health: Gauge,
    component_health: Gauge,
    resource_usage: Gauge,
    operation_rate: Gauge,
}

#[derive(Debug)]
struct SystemHistograms {
    operation_duration: Histogram,
    processing_time: Histogram,
    response_time: Histogram,
    throughput: Histogram,
}
