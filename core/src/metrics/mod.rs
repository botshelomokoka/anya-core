use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use prometheus::{Counter, Gauge, Histogram, Registry};

/// Core metrics types that all components must implement
#[async_trait::async_trait]
pub trait MetricsProvider: Send + Sync {
    async fn collect_metrics(&self) -> Result<UnifiedMetrics, MetricsError>;
    async fn update_metrics(&self, metrics: UnifiedMetrics) -> Result<(), MetricsError>;
    async fn get_health(&self) -> Result<ComponentHealth, MetricsError>;
}

/// Unified metrics error type
#[derive(thiserror::Error, Debug)]
pub enum MetricsError {
    #[error("Collection error: {0}")]
    CollectionError(String),
    #[error("Update error: {0}")]
    UpdateError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// Core system metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    // Resource metrics
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_traffic: f64,
    pub error_rate: f64,
    
    // Operation metrics
    pub ops_total: u64,
    pub ops_success: u64,
    pub ops_failed: u64,
    pub ops_latency: f64,
    
    // Health metrics
    pub health_score: f64,
    pub last_check: DateTime<Utc>,
}

/// ML system metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLMetrics {
    pub model_accuracy: f64,
    pub training_time: f64,
    pub inference_time: f64,
    pub model_size: f64,
    pub dataset_size: usize,
    pub last_trained: DateTime<Utc>,
}

/// Security metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMetrics {
    pub vulnerability_count: usize,
    pub security_score: f64,
    pub last_audit: DateTime<Utc>,
    pub critical_issues: usize,
    pub encryption_status: bool,
}

/// Protocol metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolMetrics {
    pub sync_status: SyncStatus,
    pub block_height: u64,
    pub peer_count: usize,
    pub network_health: f64,
    pub last_block: DateTime<Utc>,
}

/// Enterprise metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseMetrics {
    pub transaction_count: u64,
    pub total_volume: f64,
    pub success_rate: f64,
    pub revenue: f64,
    pub active_users: u64,
}

/// Validation metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationMetrics {
    pub validation_score: f64,
    pub error_count: usize,
    pub warning_count: usize,
    pub last_validation: DateTime<Utc>,
}

/// Unified metrics that combines all subsystems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedMetrics {
    pub system: SystemMetrics,
    pub ml: Option<MLMetrics>,
    pub security: Option<SecurityMetrics>,
    pub protocol: Option<ProtocolMetrics>,
    pub enterprise: Option<EnterpriseMetrics>,
    pub validation: Option<ValidationMetrics>,
    pub custom: HashMap<String, f64>,
    pub timestamp: DateTime<Utc>,
}

/// Component health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub operational: bool,
    pub health_score: f64,
    pub last_incident: Option<DateTime<Utc>>,
    pub error_count: usize,
    pub warning_count: usize,
}

/// Protocol sync status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncStatus {
    Synced,
    Syncing,
    Behind,
    Error,
}

/// Metrics manager for collecting and aggregating metrics
pub struct MetricsManager {
    registry: Registry,
    metrics: Arc<RwLock<UnifiedMetrics>>,
    counters: HashMap<String, Counter>,
    gauges: HashMap<String, Gauge>,
    histograms: HashMap<String, Histogram>,
}

impl MetricsManager {
    pub fn new() -> Self {
        Self {
            registry: Registry::new(),
            metrics: Arc::new(RwLock::new(UnifiedMetrics {
                system: SystemMetrics {
                    cpu_usage: 0.0,
                    memory_usage: 0.0,
                    disk_usage: 0.0,
                    network_traffic: 0.0,
                    error_rate: 0.0,
                    ops_total: 0,
                    ops_success: 0,
                    ops_failed: 0,
                    ops_latency: 0.0,
                    health_score: 100.0,
                    last_check: Utc::now(),
                },
                ml: None,
                security: None,
                protocol: None,
                enterprise: None,
                validation: None,
                custom: HashMap::new(),
                timestamp: Utc::now(),
            })),
            counters: HashMap::new(),
            gauges: HashMap::new(),
            histograms: HashMap::new(),
        }
    }

    pub async fn register_counter(&mut self, name: &str, help: &str) -> Result<(), MetricsError> {
        let counter = Counter::new(name, help)?;
        self.registry.register(Box::new(counter.clone()))?;
        self.counters.insert(name.to_string(), counter);
        Ok(())
    }

    pub async fn register_gauge(&mut self, name: &str, help: &str) -> Result<(), MetricsError> {
        let gauge = Gauge::new(name, help)?;
        self.registry.register(Box::new(gauge.clone()))?;
        self.gauges.insert(name.to_string(), gauge);
        Ok(())
    }

    pub async fn register_histogram(&mut self, name: &str, help: &str) -> Result<(), MetricsError> {
        let histogram = Histogram::new(name, help)?;
        self.registry.register(Box::new(histogram.clone()))?;
        self.histograms.insert(name.to_string(), histogram);
        Ok(())
    }

    pub async fn update_metrics(&self, metrics: UnifiedMetrics) -> Result<(), MetricsError> {
        let mut current = self.metrics.write().await;
        *current = metrics;
        Ok(())
    }

    pub async fn get_metrics(&self) -> Result<UnifiedMetrics, MetricsError> {
        Ok(self.metrics.read().await.clone())
    }

    pub async fn increment_counter(&self, name: &str) -> Result<(), MetricsError> {
        if let Some(counter) = self.counters.get(name) {
            counter.inc();
            Ok(())
        } else {
            Err(MetricsError::UpdateError(format!("Counter {} not found", name)))
        }
    }

    pub async fn set_gauge(&self, name: &str, value: f64) -> Result<(), MetricsError> {
        if let Some(gauge) = self.gauges.get(name) {
            gauge.set(value);
            Ok(())
        } else {
            Err(MetricsError::UpdateError(format!("Gauge {} not found", name)))
        }
    }

    pub async fn observe_histogram(&self, name: &str, value: f64) -> Result<(), MetricsError> {
        if let Some(histogram) = self.histograms.get(name) {
            histogram.observe(value);
            Ok(())
        } else {
            Err(MetricsError::UpdateError(format!("Histogram {} not found", name)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_metrics_manager() {
        let mut manager = MetricsManager::new();

        // Test counter
        manager.register_counter("test_counter", "Test counter").await.unwrap();
        manager.increment_counter("test_counter").await.unwrap();

        // Test gauge
        manager.register_gauge("test_gauge", "Test gauge").await.unwrap();
        manager.set_gauge("test_gauge", 42.0).await.unwrap();

        // Test histogram
        manager.register_histogram("test_histogram", "Test histogram").await.unwrap();
        manager.observe_histogram("test_histogram", 0.5).await.unwrap();

        // Test metrics update
        let metrics = UnifiedMetrics {
            system: SystemMetrics {
                cpu_usage: 50.0,
                memory_usage: 60.0,
                disk_usage: 70.0,
                network_traffic: 1000.0,
                error_rate: 0.1,
                ops_total: 1000,
                ops_success: 990,
                ops_failed: 10,
                ops_latency: 50.0,
                health_score: 99.0,
                last_check: Utc::now(),
            },
            ml: Some(MLMetrics {
                model_accuracy: 0.95,
                training_time: 100.0,
                inference_time: 10.0,
                model_size: 50.0,
                dataset_size: 1000,
                last_trained: Utc::now(),
            }),
            security: None,
            protocol: None,
            enterprise: None,
            validation: None,
            custom: HashMap::new(),
            timestamp: Utc::now(),
        };

        manager.update_metrics(metrics.clone()).await.unwrap();
        let retrieved = manager.get_metrics().await.unwrap();
        
        assert_eq!(retrieved.system.cpu_usage, metrics.system.cpu_usage);
        assert_eq!(retrieved.system.memory_usage, metrics.system.memory_usage);
        assert!(retrieved.ml.is_some());
    }
}
