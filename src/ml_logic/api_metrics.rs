//! Module documentation for $moduleName
//!
//! # Overview
//! This module is part of the Anya Core project, located at $modulePath.
//!
//! # Architecture
//! [Add module-specific architecture details]
//!
//! # API Reference
//! [Document public functions and types]
//!
//! # Usage Examples
//! `rust
//! // Add usage examples
//! `
//!
//! # Error Handling
//! This module uses proper error handling with Result types.
//!
//! # Security Considerations
//! [Document security features and considerations]
//!
//! # Performance
//! [Document performance characteristics]

use std::error::Error;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLMetrics {
    // Model Performance Metrics
    pub ml_accuracy: f32,
    pub ml_precision: f32,
    pub ml_recall: f32,
    pub ml_f1_score: f32,
    pub ml_auc_roc: f32,
    
    // Training Metrics
    pub ml_training_time: Duration,
    pub ml_epochs_completed: u32,
    pub ml_batch_size: u32,
    pub ml_learning_rate: f32,
    pub ml_loss_history: Vec<f32>,
    
    // Resource Usage
    pub ml_cpu_usage: f32,
    pub ml_memory_usage: u64,
    pub ml_gpu_usage: Option<f32>,
    pub ml_disk_io: u64,
    pub ml_network_bandwidth: u64,
    
    // Inference Metrics
    pub ml_inference_latency_p50: Duration,
    pub ml_inference_latency_p95: Duration,
    pub ml_inference_latency_p99: Duration,
    pub ml_requests_per_second: f32,
    pub ml_error_rate: f32,
    
    // Revenue Metrics
    pub ml_revenue_generated: u64,
    pub ml_cost_per_prediction: u64,
    pub ml_roi: f32,
    
    // Model Health
    pub ml_model_drift: f32,
    pub ml_data_drift: f32,
    pub ml_concept_drift: f32,
    pub ml_feature_importance: HashMap<String, f32>,
}

#[derive(Debug)]
pub struct MetricsCollector {
    metrics: Arc<RwLock<MLMetrics>>,
    start_time: Instant,
    latency_histogram: Arc<RwLock<Vec<Duration>>>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(MLMetrics::default())),
            start_time: Instant::now(),
            latency_histogram: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    pub async fn record_inference(&self, start: Instant, success: bool) {
        let duration = start.elapsed();
        let mut metrics = self.metrics.write().await;
        let mut histogram = self.latency_histogram.write().await;
        
        // Update latency histogram
        histogram.push(duration);
        if histogram.len() > 1000 {
            histogram.sort_unstable();
            metrics.ml_inference_latency_p50 = histogram[500];
            metrics.ml_inference_latency_p95 = histogram[950];
            metrics.ml_inference_latency_p99 = histogram[990];
            histogram.clear();
        }
        
        // Update request rate and errors
        let elapsed = self.start_time.elapsed().as_secs_f32();
        metrics.ml_requests_per_second = histogram.len() as f32 / elapsed;
        if !success {
            metrics.ml_error_rate += 1.0 / histogram.len() as f32;
        }
    }
    
    pub async fn update_model_metrics(&self, 
        accuracy: f32,
        precision: f32,
        recall: f32,
        loss: f32
    ) {
        let mut metrics = self.metrics.write().await;
        metrics.ml_accuracy = accuracy;
        metrics.ml_precision = precision;
        metrics.ml_recall = recall;
        metrics.ml_loss_history.push(loss);
        
        // Calculate F1 score
        metrics.ml_f1_score = 2.0 * (precision * recall) / (precision + recall);
    }
    
    pub async fn update_resource_usage(&self,
        cpu: f32,
        memory: u64,
        gpu: Option<f32>,
        disk_io: u64,
        network: u64
    ) {
        let mut metrics = self.metrics.write().await;
        metrics.ml_cpu_usage = cpu;
        metrics.ml_memory_usage = memory;
        metrics.ml_gpu_usage = gpu;
        metrics.ml_disk_io = disk_io;
        metrics.ml_network_bandwidth = network;
    }
    
    pub async fn update_revenue_metrics(&self,
        revenue: u64,
        cost: u64
    ) {
        let mut metrics = self.metrics.write().await;
        metrics.ml_revenue_generated = revenue;
        metrics.ml_cost_per_prediction = cost;
        metrics.ml_roi = (revenue as f32 - cost as f32) / cost as f32;
    }
    
    pub async fn update_drift_metrics(&self,
        model_drift: f32,
        data_drift: f32,
        concept_drift: f32,
        feature_importance: HashMap<String, f32>
    ) {
        let mut metrics = self.metrics.write().await;
        metrics.ml_model_drift = model_drift;
        metrics.ml_data_drift = data_drift;
        metrics.ml_concept_drift = concept_drift;
        metrics.ml_feature_importance = feature_importance;
    }
    
    pub async fn get_metrics(&self) -> MLMetrics {
        self.metrics.read().await.clone()
    }
}

impl Default for MLMetrics {
    fn default() -> Self {
        Self {
            ml_accuracy: 0.0,
            ml_precision: 0.0,
            ml_recall: 0.0,
            ml_f1_score: 0.0,
            ml_auc_roc: 0.0,
            ml_training_time: Duration::from_secs(0),
            ml_epochs_completed: 0,
            ml_batch_size: 0,
            ml_learning_rate: 0.0,
            ml_loss_history: Vec::new(),
            ml_cpu_usage: 0.0,
            ml_memory_usage: 0,
            ml_gpu_usage: None,
            ml_disk_io: 0,
            ml_network_bandwidth: 0,
            ml_inference_latency_p50: Duration::from_secs(0),
            ml_inference_latency_p95: Duration::from_secs(0),
            ml_inference_latency_p99: Duration::from_secs(0),
            ml_requests_per_second: 0.0,
            ml_error_rate: 0.0,
            ml_revenue_generated: 0,
            ml_cost_per_prediction: 0,
            ml_roi: 0.0,
            ml_model_drift: 0.0,
            ml_data_drift: 0.0,
            ml_concept_drift: 0.0,
            ml_feature_importance: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[tokio::test]
    async fn test_metrics_collector() {
        let collector = MetricsCollector::new();
        
        // Test inference metrics
        let start = Instant::now();
        tokio::time::sleep(Duration::from_millis(10)).await;
        collector.record_inference(start, true).await;
        
        // Test model metrics
        collector.update_model_metrics(0.95, 0.94, 0.93, 0.1).await;
        
        // Test resource metrics
        collector.update_resource_usage(
            0.5,
            1024 * 1024,
            Some(0.8),
            1024,
            2048
        ).await;
        
        // Test revenue metrics
        collector.update_revenue_metrics(1000, 100).await;
        
        // Test drift metrics
        let mut feature_imp = HashMap::new();
        feature_imp.insert("feature1".to_string(), 0.5);
        collector.update_drift_metrics(0.1, 0.2, 0.3, feature_imp).await;
        
        // Verify metrics
        let metrics = collector.get_metrics().await;
        assert!(metrics.ml_accuracy > 0.0);
        assert!(metrics.ml_f1_score > 0.0);
        assert!(metrics.ml_roi > 0.0);
        assert!(metrics.ml_requests_per_second > 0.0);
    }
}

pub struct ApiMetricsCollector {
    payment_processor: PaymentProcessor,
    usage_tracker: UsageTracker,
    metrics_collector: MetricsCollector,
}

impl ApiMetricsCollector {
    pub async fn collect_and_process(&self, license_key: &str) -> Result<UsageMetrics, MetricsError> {
        let usage = self.usage_tracker.get_metrics(license_key).await?;
        self.payment_processor.process_charges(&usage).await?;
        Ok(usage)
    }
}
