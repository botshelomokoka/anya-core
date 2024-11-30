use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLRevenueMetrics {
    // Revenue Streams
    pub ml_prediction_revenue: u64,
    pub ml_training_revenue: u64,
    pub ml_api_revenue: u64,
    pub ml_subscription_revenue: u64,
    
    // Costs
    pub ml_compute_costs: u64,
    pub ml_storage_costs: u64,
    pub ml_bandwidth_costs: u64,
    pub ml_license_costs: u64,
    
    // Usage Statistics
    pub ml_predictions_count: u64,
    pub ml_training_hours: f32,
    pub ml_api_calls: u64,
    pub ml_active_subscriptions: u32,
    
    // Performance Metrics
    pub ml_revenue_per_prediction: f32,
    pub ml_cost_per_prediction: f32,
    pub ml_profit_margin: f32,
    pub ml_roi: f32,
    
    // Time-based Metrics
    pub ml_daily_revenue: HashMap<String, u64>,
    pub ml_monthly_revenue: HashMap<String, u64>,
    pub ml_revenue_growth_rate: f32,
}

pub struct MLRevenueTracker {
    metrics: Arc<RwLock<MLRevenueMetrics>>,
    start_time: Instant,
    daily_snapshots: Arc<RwLock<Vec<MLRevenueMetrics>>>,
}

impl MLRevenueTracker {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(MLRevenueMetrics::default())),
            start_time: Instant::now(),
            daily_snapshots: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    pub async fn record_prediction_revenue(&self, amount: u64) {
        let mut metrics = self.metrics.write().await;
        metrics.ml_prediction_revenue += amount;
        metrics.ml_predictions_count += 1;
        
        // Update per-prediction metrics
        metrics.ml_revenue_per_prediction = metrics.ml_prediction_revenue as f32 / metrics.ml_predictions_count as f32;
        self.update_profit_metrics(&mut metrics).await;
    }
    
    pub async fn record_training_revenue(&self, amount: u64, hours: f32) {
        let mut metrics = self.metrics.write().await;
        metrics.ml_training_revenue += amount;
        metrics.ml_training_hours += hours;
        self.update_profit_metrics(&mut metrics).await;
    }
    
    pub async fn record_api_revenue(&self, amount: u64, calls: u64) {
        let mut metrics = self.metrics.write().await;
        metrics.ml_api_revenue += amount;
        metrics.ml_api_calls += calls;
        self.update_profit_metrics(&mut metrics).await;
    }
    
    pub async fn record_subscription_revenue(&self, amount: u64, active_subs: u32) {
        let mut metrics = self.metrics.write().await;
        metrics.ml_subscription_revenue += amount;
        metrics.ml_active_subscriptions = active_subs;
        self.update_profit_metrics(&mut metrics).await;
    }
    
    pub async fn record_costs(&self,
        compute: u64,
        storage: u64,
        bandwidth: u64,
        license: u64
    ) {
        let mut metrics = self.metrics.write().await;
        metrics.ml_compute_costs += compute;
        metrics.ml_storage_costs += storage;
        metrics.ml_bandwidth_costs += bandwidth;
        metrics.ml_license_costs += license;
        
        self.update_profit_metrics(&mut metrics).await;
    }
    
    async fn update_profit_metrics(&self, metrics: &mut MLRevenueMetrics) {
        let total_revenue = metrics.ml_prediction_revenue +
            metrics.ml_training_revenue +
            metrics.ml_api_revenue +
            metrics.ml_subscription_revenue;
            
        let total_costs = metrics.ml_compute_costs +
            metrics.ml_storage_costs +
            metrics.ml_bandwidth_costs +
            metrics.ml_license_costs;
            
        if metrics.ml_predictions_count > 0 {
            metrics.ml_cost_per_prediction = total_costs as f32 / metrics.ml_predictions_count as f32;
        }
        
        if total_revenue > 0 {
            metrics.ml_profit_margin = (total_revenue as f32 - total_costs as f32) / total_revenue as f32;
            metrics.ml_roi = (total_revenue as f32 - total_costs as f32) / total_costs as f32;
        }
    }
    
    pub async fn take_daily_snapshot(&self) {
        let metrics = self.metrics.read().await;
        let mut snapshots = self.daily_snapshots.write().await;
        
        // Add current metrics to snapshots
        snapshots.push(metrics.clone());
        
        // Calculate growth rate if we have enough data
        if snapshots.len() >= 2 {
            let current_revenue = self.calculate_total_revenue(&metrics);
            let previous_revenue = self.calculate_total_revenue(&snapshots[snapshots.len() - 2]);
            
            if previous_revenue > 0 {
                let mut metrics = self.metrics.write().await;
                metrics.ml_revenue_growth_rate = (current_revenue as f32 - previous_revenue as f32) / previous_revenue as f32;
            }
        }
        
        // Keep only last 30 days
        if snapshots.len() > 30 {
            snapshots.remove(0);
        }
    }
    
    fn calculate_total_revenue(&self, metrics: &MLRevenueMetrics) -> u64 {
        metrics.ml_prediction_revenue +
        metrics.ml_training_revenue +
        metrics.ml_api_revenue +
        metrics.ml_subscription_revenue
    }
    
    pub async fn get_metrics(&self) -> MLRevenueMetrics {
        self.metrics.read().await.clone()
    }
}

impl Default for MLRevenueMetrics {
    fn default() -> Self {
        Self {
            ml_prediction_revenue: 0,
            ml_training_revenue: 0,
            ml_api_revenue: 0,
            ml_subscription_revenue: 0,
            ml_compute_costs: 0,
            ml_storage_costs: 0,
            ml_bandwidth_costs: 0,
            ml_license_costs: 0,
            ml_predictions_count: 0,
            ml_training_hours: 0.0,
            ml_api_calls: 0,
            ml_active_subscriptions: 0,
            ml_revenue_per_prediction: 0.0,
            ml_cost_per_prediction: 0.0,
            ml_profit_margin: 0.0,
            ml_roi: 0.0,
            ml_daily_revenue: HashMap::new(),
            ml_monthly_revenue: HashMap::new(),
            ml_revenue_growth_rate: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_revenue_tracking() {
        let tracker = MLRevenueTracker::new();
        
        // Test prediction revenue
        tracker.record_prediction_revenue(100).await;
        tracker.record_prediction_revenue(200).await;
        
        // Test training revenue
        tracker.record_training_revenue(500, 2.5).await;
        
        // Test API revenue
        tracker.record_api_revenue(300, 1000).await;
        
        // Test subscription revenue
        tracker.record_subscription_revenue(1000, 10).await;
        
        // Test costs
        tracker.record_costs(200, 100, 50, 300).await;
        
        // Take snapshot
        tracker.take_daily_snapshot().await;
        
        // Verify metrics
        let metrics = tracker.get_metrics().await;
        assert_eq!(metrics.ml_predictions_count, 2);
        assert_eq!(metrics.ml_training_hours, 2.5);
        assert_eq!(metrics.ml_api_calls, 1000);
        assert_eq!(metrics.ml_active_subscriptions, 10);
        
        // Verify calculations
        assert!(metrics.ml_profit_margin > 0.0);
        assert!(metrics.ml_roi > 0.0);
        assert!(metrics.ml_revenue_per_prediction > 0.0);
        assert!(metrics.ml_cost_per_prediction > 0.0);
    }
}
