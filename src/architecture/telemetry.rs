use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use crate::architecture::errors::HexagonalError;
use crate::architecture::types::{
    Span, SpanGuard, Trace, PerformanceMetrics, ErrorMetrics,
    ResourceMetrics, BusinessMetrics, CompleteMetrics, ResourceUtilization
};

pub struct Telemetry {
    traces: Arc<RwLock<Vec<Trace>>>,
    spans: Arc<RwLock<Vec<Span>>>,
    performance_metrics: Arc<RwLock<PerformanceMetrics>>,
    error_metrics: Arc<RwLock<ErrorMetrics>>,
    resource_metrics: Arc<RwLock<ResourceMetrics>>,
    business_metrics: Arc<RwLock<BusinessMetrics>>,
}

impl Default for Telemetry {
    fn default() -> Self {
        Self::new()
    }
}

impl Telemetry {
    pub fn new() -> Self {
        Self {
            traces: Arc::new(RwLock::new(Vec::new())),
            spans: Arc::new(RwLock::new(Vec::new())),
            performance_metrics: Arc::new(RwLock::new(PerformanceMetrics {
                latency: Vec::new(),
                throughput: 0.0,
                error_rate: 0.0,
                resource_utilization: ResourceUtilization {
                    cpu_percent: 0.0,
                    memory_percent: 0.0,
                    disk_percent: 0.0,
                    network_bandwidth: 0.0,
                },
            })),
            error_metrics: Arc::new(RwLock::new(ErrorMetrics {
                error_count: 0,
                error_types: std::collections::HashMap::new(),
                error_rates: std::collections::HashMap::new(),
            })),
            resource_metrics: Arc::new(RwLock::new(ResourceMetrics {
                cpu_usage: 0.0,
                memory_usage: 0.0,
                disk_usage: 0.0,
                network_usage: 0.0,
            })),
            business_metrics: Arc::new(RwLock::new(BusinessMetrics {
                transactions_processed: 0,
                active_users: 0,
                success_rate: 0.0,
                revenue: 0.0,
            })),
        }
    }

    pub async fn start_trace(&self, name: &str) -> String {
        let trace_id = uuid::Uuid::new_v4().to_string();
        let trace = Trace {
            id: trace_id.clone(),
            parent_id: None,
            name: name.to_string(),
            start_time: chrono::Utc::now(),
            end_time: None,
            attributes: std::collections::HashMap::new(),
        };

        self.traces.write().await.push(trace);
        trace_id
    }

    pub async fn end_trace(&self, trace_id: &str) {
        let mut traces = self.traces.write().await;
        if let Some(trace) = traces.iter_mut().find(|t| t.id == trace_id) {
            trace.end_time = Some(chrono::Utc::now());
        }
    }

    pub async fn start_span(&self, name: &str) -> SpanGuard {
        let span = Span {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            start_time: chrono::Utc::now(),
            end_time: None,
        };

        self.spans.write().await.push(span.clone());
        SpanGuard::new(span)
    }

    pub async fn record_latency(&self, duration: Duration) {
        let mut metrics = self.performance_metrics.write().await;
        metrics.latency.push(duration);

        // Calculate rolling average
        if metrics.latency.len() > 100 {
            metrics.latency.remove(0);
        }
    }

    pub async fn update_throughput(&self, requests_per_second: f64) {
        self.performance_metrics.write().await.throughput = requests_per_second;
    }

    pub async fn record_error(&self, error: &HexagonalError) {
        let mut metrics = self.error_metrics.write().await;
        metrics.error_count += 1;

        let error_type = format!("{:?}", error);
        *metrics.error_types.entry(error_type.clone()).or_insert(0) += 1;

        // Update error rates
        let total_requests = metrics.error_count as f64;
        for (error_type, count) in &metrics.error_types {
            metrics.error_rates.insert(
                error_type.clone(),
                *count as f64 / total_requests,
            );
        }
    }

    pub async fn update_resource_metrics(&self, metrics: ResourceMetrics) {
        *self.resource_metrics.write().await = metrics;
    }

    pub async fn update_business_metrics(&self, metrics: BusinessMetrics) {
        *self.business_metrics.write().await = metrics;
    }

    pub async fn get_complete_metrics(&self) -> CompleteMetrics {
        CompleteMetrics {
            performance: self.performance_metrics.read().await.clone(),
            errors: self.error_metrics.read().await.clone(),
            resources: self.resource_metrics.read().await.clone(),
            business: self.business_metrics.read().await.clone(),
        }
    }

    pub async fn get_traces(&self) -> Vec<Trace> {
        self.traces.read().await.clone()
    }

    pub async fn get_spans(&self) -> Vec<Span> {
        self.spans.read().await.clone()
    }

    pub async fn clear_old_traces(&self, older_than: Duration) {
        let mut traces = self.traces.write().await;
        let now = chrono::Utc::now();
        traces.retain(|trace| {
            now.signed_duration_since(trace.start_time)
                < chrono::Duration::from_std(older_than).unwrap()
        });
    }

    pub async fn clear_old_spans(&self, older_than: Duration) {
        let mut spans = self.spans.write().await;
        let now = chrono::Utc::now();
        spans.retain(|span| {
            now.signed_duration_since(span.start_time)
                < chrono::Duration::from_std(older_than).unwrap()
        });
    }
}

pub struct MetricsCollector {
    telemetry: Arc<Telemetry>,
    collection_interval: Duration,
}

impl MetricsCollector {
    pub fn new(telemetry: Arc<Telemetry>, collection_interval: Duration) -> Self {
        Self {
            telemetry,
            collection_interval,
        }
    }

    pub async fn start(self) {
        let telemetry = self.telemetry.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(self.collection_interval);
            loop {
                interval.tick().await;
                if let Err(e) = self.collect_metrics().await {
                    log::error!("Failed to collect metrics: {}", e);
                }
            }
        });
    }

    async fn collect_metrics(&self) -> Result<(), HexagonalError> {
        // Collect system metrics
        let sys_info = sysinfo::System::new_all();
        
        let resource_metrics = ResourceMetrics {
            cpu_usage: sys_info.global_cpu_info().cpu_usage() as f64,
            memory_usage: sys_info.used_memory() as f64 / sys_info.total_memory() as f64 * 100.0,
            disk_usage: 0.0, // Implement disk usage collection
            network_usage: 0.0, // Implement network usage collection
        };

        self.telemetry.update_resource_metrics(resource_metrics).await;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_telemetry_trace_lifecycle() {
        let telemetry = Telemetry::new();

        // Start trace
        let trace_id = telemetry.start_trace("test_operation").await;
        
        // Simulate some work
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        // End trace
        telemetry.end_trace(&trace_id).await;

        // Verify trace
        let traces = telemetry.get_traces().await;
        assert_eq!(traces.len(), 1);
        assert_eq!(traces[0].name, "test_operation");
        assert!(traces[0].end_time.is_some());
    }

    #[tokio::test]
    async fn test_metrics_recording() {
        let telemetry = Telemetry::new();

        // Record performance metrics
        telemetry.record_latency(Duration::from_millis(100)).await;
        telemetry.update_throughput(1000.0).await;

        // Record error
        telemetry
            .record_error(&HexagonalError::NetworkError("Test error".into()))
            .await;

        // Verify metrics
        let metrics = telemetry.get_complete_metrics().await;
        assert_eq!(metrics.performance.latency.len(), 1);
        assert_eq!(metrics.performance.throughput, 1000.0);
        assert_eq!(metrics.errors.error_count, 1);
    }
}
