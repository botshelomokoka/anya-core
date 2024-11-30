use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingConfig {
    pub enabled: bool,
    pub min_instances: u32,
    pub max_instances: u32,
    pub ml_scale_up_threshold: f32,
    pub ml_scale_down_threshold: f32,
    pub ml_cooldown_period: u64,
    pub ml_evaluation_period: u64,
    pub ml_model_memory_limit: u32,
    pub ml_batch_size_limit: u32,
}

#[derive(Debug, Clone)]
pub struct ScalingMetrics {
    pub ml_cpu_utilization: f32,
    pub ml_memory_utilization: f32,
    pub ml_request_rate: f32,
    pub ml_error_rate: f32,
    pub ml_latency_ms: f32,
    pub ml_model_accuracy: f32,
    pub ml_inference_time: f32,
    pub ml_batch_throughput: f32,
}

pub struct AutoScaler {
    config: Arc<RwLock<ScalingConfig>>,
    metrics: Arc<RwLock<ScalingMetrics>>,
    last_scale: Arc<RwLock<Option<time::Instant>>>,
}

impl AutoScaler {
    pub fn new(config: ScalingConfig) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
            metrics: Arc::new(RwLock::new(ScalingMetrics {
                ml_cpu_utilization: 0.0,
                ml_memory_utilization: 0.0,
                ml_request_rate: 0.0,
                ml_error_rate: 0.0,
                ml_latency_ms: 0.0,
                ml_model_accuracy: 0.0,
                ml_inference_time: 0.0,
                ml_batch_throughput: 0.0,
            })),
            last_scale: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn update_metrics(&self, metrics: ScalingMetrics) {
        let mut current_metrics = self.metrics.write().await;
        *current_metrics = metrics;
    }

    pub async fn evaluate_scaling(&self) -> Option<ScalingDecision> {
        let config = self.config.read().await;
        if !config.enabled {
            return None;
        }

        let metrics = self.metrics.read().await;
        let last_scale = self.last_scale.read().await;

        // Check cooldown period
        if let Some(last_scale_time) = *last_scale {
            let elapsed = last_scale_time.elapsed().as_secs();
            if elapsed < config.ml_cooldown_period {
                return None;
            }
        }

        let load_factor = calculate_load_factor(&metrics);

        if load_factor > config.ml_scale_up_threshold {
            Some(ScalingDecision::ScaleUp)
        } else if load_factor < config.ml_scale_down_threshold {
            Some(ScalingDecision::ScaleDown)
        } else {
            None
        }
    }

    pub async fn start_monitoring(self: Arc<Self>) {
        let config = self.config.read().await;
        let evaluation_period = Duration::from_secs(config.ml_evaluation_period);
        drop(config);

        loop {
            time::sleep(evaluation_period).await;

            if let Some(decision) = self.evaluate_scaling().await {
                match decision {
                    ScalingDecision::ScaleUp => {
                        if let Err(e) = self.scale_up().await {
                            eprintln!("Failed to scale up: {}", e);
                        }
                    }
                    ScalingDecision::ScaleDown => {
                        if let Err(e) = self.scale_down().await {
                            eprintln!("Failed to scale down: {}", e);
                        }
                    }
                }
            }
        }
    }

    async fn scale_up(&self) -> Result<(), ScalingError> {
        let mut last_scale = self.last_scale.write().await;
        *last_scale = Some(time::Instant::now());
        
        // Implement actual scaling logic here
        Ok(())
    }

    async fn scale_down(&self) -> Result<(), ScalingError> {
        let mut last_scale = self.last_scale.write().await;
        *last_scale = Some(time::Instant::now());
        
        // Implement actual scaling logic here
        Ok(())
    }
}

fn calculate_load_factor(metrics: &ScalingMetrics) -> f32 {
    let ml_cpu_weight = 0.3;
    let ml_memory_weight = 0.2;
    let ml_request_weight = 0.15;
    let ml_error_weight = 0.1;
    let ml_accuracy_weight = 0.15;
    let ml_latency_weight = 0.1;

    (metrics.ml_cpu_utilization * ml_cpu_weight) +
    (metrics.ml_memory_utilization * ml_memory_weight) +
    (metrics.ml_request_rate * ml_request_weight) +
    (metrics.ml_error_rate * ml_error_weight) +
    (metrics.ml_model_accuracy * ml_accuracy_weight) +
    (metrics.ml_latency_ms / 1000.0 * ml_latency_weight)
}

#[derive(Debug)]
pub enum ScalingDecision {
    ScaleUp,
    ScaleDown,
}

#[derive(Debug, thiserror::Error)]
pub enum ScalingError {
    #[error("Failed to scale: {0}")]
    ScalingFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_autoscaler() {
        let config = ScalingConfig {
            enabled: true,
            min_instances: 1,
            max_instances: 10,
            ml_scale_up_threshold: 0.8,
            ml_scale_down_threshold: 0.2,
            ml_cooldown_period: 300,
            ml_evaluation_period: 60,
            ml_model_memory_limit: 1024,
            ml_batch_size_limit: 32,
        };

        let scaler = AutoScaler::new(config);

        // Test high load scenario
        scaler.update_metrics(ScalingMetrics {
            ml_cpu_utilization: 0.9,
            ml_memory_utilization: 0.85,
            ml_request_rate: 0.95,
            ml_error_rate: 0.1,
            ml_latency_ms: 200.0,
            ml_model_accuracy: 0.9,
            ml_inference_time: 0.5,
            ml_batch_throughput: 10.0,
        }).await;

        let decision = scaler.evaluate_scaling().await;
        assert!(matches!(decision, Some(ScalingDecision::ScaleUp)));

        // Test low load scenario
        scaler.update_metrics(ScalingMetrics {
            ml_cpu_utilization: 0.1,
            ml_memory_utilization: 0.15,
            ml_request_rate: 0.05,
            ml_error_rate: 0.01,
            ml_latency_ms: 50.0,
            ml_model_accuracy: 0.8,
            ml_inference_time: 0.2,
            ml_batch_throughput: 5.0,
        }).await;

        let decision = scaler.evaluate_scaling().await;
        assert!(matches!(decision, Some(ScalingDecision::ScaleDown)));
    }
}
