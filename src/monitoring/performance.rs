use metrics::{counter, gauge, histogram};
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use std::sync::Arc;
use tracing::{info, warn, error};

pub struct PerformanceMonitor {
    start_time: Instant,
    metrics: Arc<RwLock<PerformanceMetrics>>,
}

#[derive(Default)]
struct PerformanceMetrics {
    request_count: u64,
    error_count: u64,
    average_response_time: f64,
    memory_usage: f64,
    cpu_usage: f64,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            metrics: Arc::new(RwLock::new(PerformanceMetrics::default())),
        }
    }

    pub async fn record_request(&self, duration: Duration, success: bool) {
        let mut metrics = self.metrics.write().await;
        metrics.request_count += 1;
        if !success {
            metrics.error_count += 1;
        }

        // Update average response time
        let duration_ms = duration.as_millis() as f64;
        metrics.average_response_time = (metrics.average_response_time * (metrics.request_count - 1) as f64
            + duration_ms) / metrics.request_count as f64;

        // Record metrics
        counter!("requests_total", 1);
        histogram!("request_duration_ms", duration_ms);
        if !success {
            counter!("errors_total", 1);
        }
    }

    pub async fn update_system_metrics(&self, memory_usage: f64, cpu_usage: f64) {
        let mut metrics = self.metrics.write().await;
        metrics.memory_usage = memory_usage;
        metrics.cpu_usage = cpu_usage;

        // Record system metrics
        gauge!("memory_usage_bytes", memory_usage);
        gauge!("cpu_usage_percent", cpu_usage);

        // Log warnings if thresholds are exceeded
        if memory_usage > 90.0 {
            warn!("High memory usage detected: {:.2}%", memory_usage);
        }
        if cpu_usage > 80.0 {
            warn!("High CPU usage detected: {:.2}%", cpu_usage);
        }
    }

    pub async fn get_health_check(&self) -> HealthStatus {
        let metrics = self.metrics.read().await;
        let uptime = self.start_time.elapsed();
        let error_rate = if metrics.request_count > 0 {
            metrics.error_count as f64 / metrics.request_count as f64
        } else {
            0.0
        };

        HealthStatus {
            status: if error_rate < 0.05 && metrics.memory_usage < 90.0 && metrics.cpu_usage < 80.0 {
                "healthy"
            } else {
                "degraded"
            }.to_string(),
            uptime_seconds: uptime.as_secs(),
            error_rate,
            average_response_time_ms: metrics.average_response_time,
            memory_usage: metrics.memory_usage,
            cpu_usage: metrics.cpu_usage,
        }
    }

    pub async fn generate_performance_report(&self) -> PerformanceReport {
        let metrics = self.metrics.read().await;
        PerformanceReport {
            total_requests: metrics.request_count,
            total_errors: metrics.error_count,
            error_rate: if metrics.request_count > 0 {
                metrics.error_count as f64 / metrics.request_count as f64
            } else {
                0.0
            },
            average_response_time_ms: metrics.average_response_time,
            uptime_seconds: self.start_time.elapsed().as_secs(),
            memory_usage: metrics.memory_usage,
            cpu_usage: metrics.cpu_usage,
        }
    }
}

#[derive(serde::Serialize)]
pub struct HealthStatus {
    pub status: String,
    pub uptime_seconds: u64,
    pub error_rate: f64,
    pub average_response_time_ms: f64,
    pub memory_usage: f64,
    pub cpu_usage: f64,
}

#[derive(serde::Serialize)]
pub struct PerformanceReport {
    pub total_requests: u64,
    pub total_errors: u64,
    pub error_rate: f64,
    pub average_response_time_ms: f64,
    pub uptime_seconds: u64,
    pub memory_usage: f64,
    pub cpu_usage: f64,
}
