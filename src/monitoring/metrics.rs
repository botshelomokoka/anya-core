use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use metrics::{Counter, Gauge, Histogram, Key, KeyName, Unit};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceMetrics {
    pub proposal_count: u64,
    pub vote_count: u64,
    pub participation_rate: f64,
    pub execution_success_rate: f64,
    pub avg_proposal_duration: f64,
    pub active_voters: u64,
    pub quorum_achievement_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitMetrics {
    pub action_type: String,
    pub period: String,
    pub limit: u32,
    pub current_usage: u32,
    pub rejection_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub action_latencies: HashMap<String, f64>,
    pub cache_hit_rate: f64,
    pub storage_operation_times: HashMap<String, f64>,
    pub rpc_call_latencies: HashMap<String, f64>,
}

pub struct MetricsCollector {
    metrics_store: Arc<RwLock<HashMap<String, serde_json::Value>>>,
    counters: HashMap<String, Counter>,
    gauges: HashMap<String, Gauge>,
    histograms: HashMap<String, Histogram>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        let metrics_store = Arc::new(RwLock::new(HashMap::new()));
        
        Self {
            metrics_store,
            counters: HashMap::new(),
            gauges: HashMap::new(),
            histograms: HashMap::new(),
        }
    }

    pub async fn record_governance_action(&self, action_type: &str, duration: f64) {
        let key = format!("governance.action.{}", action_type);
        self.histograms.get(&key).map(|h| h.record(duration));
        
        let mut store = self.metrics_store.write().await;
        store.insert(key, serde_json::to_value(duration).unwrap());
    }

    pub async fn record_rate_limit(&self, action: &str, current: u32, limit: u32) {
        let metrics = RateLimitMetrics {
            action_type: action.to_string(),
            period: "1m".to_string(),
            limit,
            current_usage: current,
            rejection_count: if current > limit { 1 } else { 0 },
        };

        let mut store = self.metrics_store.write().await;
        store.insert(
            format!("rate_limit.{}", action),
            serde_json::to_value(metrics).unwrap(),
        );
    }

    pub async fn record_cache_operation(&self, operation: &str, hit: bool) {
        let key = format!("cache.{}", operation);
        if hit {
            self.counters.get(&format!("{}.hit", key))
                .map(|c| c.increment(1));
        } else {
            self.counters.get(&format!("{}.miss", key))
                .map(|c| c.increment(1));
        }
    }

    pub async fn update_governance_metrics(&self, metrics: GovernanceMetrics) {
        let mut store = self.metrics_store.write().await;
        store.insert(
            "governance_metrics".to_string(),
            serde_json::to_value(metrics).unwrap(),
        );
    }

    pub async fn record_performance_metric(&self, category: &str, operation: &str, duration: f64) {
        let key = format!("performance.{}.{}", category, operation);
        self.histograms.get(&key).map(|h| h.record(duration));
    }

    pub async fn get_metrics_report(&self) -> serde_json::Value {
        let store = self.metrics_store.read().await;
        serde_json::to_value(&*store).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_metrics_collection() {
        let collector = MetricsCollector::new();
        
        // Record some test metrics
        collector.record_governance_action("proposal_creation", 0.5).await;
        collector.record_rate_limit("proposal_creation", 5, 10).await;
        collector.record_cache_operation("state_lookup", true).await;
        
        // Verify metrics were recorded
        let report = collector.get_metrics_report().await;
        assert!(report.as_object().unwrap().contains_key("governance.action.proposal_creation"));
        assert!(report.as_object().unwrap().contains_key("rate_limit.proposal_creation"));
    }
}
