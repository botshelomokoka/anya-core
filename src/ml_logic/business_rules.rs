use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;
use thiserror::Error;

use crate::ml_logic::{
    api_metrics::MLMetrics,
    revenue_tracking::MLRevenueMetrics,
};

#[derive(Debug, Error)]
pub enum BusinessRuleError {
    #[error("Invalid rule configuration: {0}")]
    InvalidConfig(String),
    #[error("Rule execution failed: {0}")]
    ExecutionError(String),
    #[error("Threshold violation: {0}")]
    ThresholdViolation(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessRule {
    pub name: String,
    pub description: String,
    pub rule_type: RuleType,
    pub conditions: Vec<Condition>,
    pub actions: Vec<Action>,
    pub priority: u32,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleType {
    MLPerformance,
    Revenue,
    ResourceUsage,
    SecurityCompliance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub metric: String,
    pub operator: Operator,
    pub value: f64,
    pub time_window: Option<u64>, // in seconds
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operator {
    GreaterThan,
    LessThan,
    Equals,
    NotEquals,
    Between(f64, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    AdjustMLParameters(MLParameterAdjustment),
    ModifyPricing(PricingAdjustment),
    ScaleResources(ResourceScaling),
    NotifyAdmin(String),
    PauseService(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLParameterAdjustment {
    pub parameter: String,
    pub adjustment: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingAdjustment {
    pub service_type: String,
    pub percentage_change: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceScaling {
    pub resource_type: String,
    pub scale_factor: f32,
}

pub struct BusinessRuleEngine {
    rules: Arc<RwLock<Vec<BusinessRule>>>,
    metrics: Arc<RwLock<MLMetrics>>,
    revenue_metrics: Arc<RwLock<MLRevenueMetrics>>,
    action_history: Arc<RwLock<Vec<(String, Action)>>>,
}

impl BusinessRuleEngine {
    pub fn new(
        metrics: Arc<RwLock<MLMetrics>>,
        revenue_metrics: Arc<RwLock<MLRevenueMetrics>>,
    ) -> Self {
        Self {
            rules: Arc::new(RwLock::new(Vec::new())),
            metrics,
            revenue_metrics,
            action_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn add_rule(&self, rule: BusinessRule) -> Result<(), BusinessRuleError> {
        // Validate rule
        self.validate_rule(&rule)?;
        
        // Add rule to collection
        let mut rules = self.rules.write().await;
        rules.push(rule);
        rules.sort_by_key(|r| r.priority);
        
        Ok(())
    }

    pub async fn evaluate_rules(&self) -> Result<Vec<Action>, BusinessRuleError> {
        let rules = self.rules.read().await;
        let metrics = self.metrics.read().await;
        let revenue_metrics = self.revenue_metrics.read().await;
        let mut actions = Vec::new();

        for rule in rules.iter().filter(|r| r.enabled) {
            if self.evaluate_conditions(&rule.conditions, &metrics, &revenue_metrics).await? {
                actions.extend(rule.actions.clone());
                
                // Record actions in history
                let mut history = self.action_history.write().await;
                for action in &rule.actions {
                    history.push((rule.name.clone(), action.clone()));
                }
            }
        }

        Ok(actions)
    }

    async fn evaluate_conditions(
        &self,
        conditions: &[Condition],
        metrics: &MLMetrics,
        revenue_metrics: &MLRevenueMetrics,
    ) -> Result<bool, BusinessRuleError> {
        for condition in conditions {
            let value = self.get_metric_value(condition.metric.as_str(), metrics, revenue_metrics)?;
            
            if !self.check_condition(value, &condition.operator) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn get_metric_value(
        &self,
        metric: &str,
        metrics: &MLMetrics,
        revenue_metrics: &MLRevenueMetrics,
    ) -> Result<f64, BusinessRuleError> {
        match metric {
            // ML Performance Metrics
            "ml_accuracy" => Ok(metrics.ml_accuracy as f64),
            "ml_precision" => Ok(metrics.ml_precision as f64),
            "ml_recall" => Ok(metrics.ml_recall as f64),
            "ml_f1_score" => Ok(metrics.ml_f1_score as f64),
            
            // Resource Usage Metrics
            "ml_cpu_usage" => Ok(metrics.ml_cpu_usage as f64),
            "ml_memory_usage" => Ok(metrics.ml_memory_usage as f64),
            
            // Revenue Metrics
            "ml_revenue_per_prediction" => Ok(revenue_metrics.ml_revenue_per_prediction as f64),
            "ml_profit_margin" => Ok(revenue_metrics.ml_profit_margin as f64),
            "ml_roi" => Ok(revenue_metrics.ml_roi as f64),
            
            _ => Err(BusinessRuleError::InvalidConfig(format!("Unknown metric: {}", metric))),
        }
    }

    fn check_condition(&self, value: f64, operator: &Operator) -> bool {
        match operator {
            Operator::GreaterThan => value > operator.value,
            Operator::LessThan => value < operator.value,
            Operator::Equals => (value - operator.value).abs() < f64::EPSILON,
            Operator::NotEquals => (value - operator.value).abs() >= f64::EPSILON,
            Operator::Between(min, max) => value >= *min && value <= *max,
        }
    }

    fn validate_rule(&self, rule: &BusinessRule) -> Result<(), BusinessRuleError> {
        if rule.conditions.is_empty() {
            return Err(BusinessRuleError::InvalidConfig(
                "Rule must have at least one condition".to_string(),
            ));
        }
        
        if rule.actions.is_empty() {
            return Err(BusinessRuleError::InvalidConfig(
                "Rule must have at least one action".to_string(),
            ));
        }
        
        Ok(())
    }

    pub async fn get_action_history(&self) -> Vec<(String, Action)> {
        self.action_history.read().await.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_business_rules() {
        // Create test metrics
        let metrics = Arc::new(RwLock::new(MLMetrics {
            ml_accuracy: 0.95,
            ml_precision: 0.94,
            ml_recall: 0.93,
            ml_f1_score: 0.94,
            ml_cpu_usage: 0.7,
            ml_memory_usage: 1024 * 1024 * 1024, // 1GB
            ..MLMetrics::default()
        }));

        let revenue_metrics = Arc::new(RwLock::new(MLRevenueMetrics {
            ml_revenue_per_prediction: 0.5,
            ml_profit_margin: 0.3,
            ml_roi: 1.5,
            ..MLRevenueMetrics::default()
        }));

        let engine = BusinessRuleEngine::new(metrics.clone(), revenue_metrics.clone());

        // Create test rule
        let rule = BusinessRule {
            name: "High CPU Usage Alert".to_string(),
            description: "Alert when CPU usage is too high".to_string(),
            rule_type: RuleType::ResourceUsage,
            conditions: vec![Condition {
                metric: "ml_cpu_usage".to_string(),
                operator: Operator::GreaterThan,
                value: 0.8,
                time_window: Some(300), // 5 minutes
            }],
            actions: vec![Action::NotifyAdmin(
                "High CPU usage detected".to_string(),
            )],
            priority: 1,
            enabled: true,
        };

        // Add rule
        engine.add_rule(rule).await.unwrap();

        // Evaluate rules
        let actions = engine.evaluate_rules().await.unwrap();
        assert!(!actions.is_empty());

        // Check action history
        let history = engine.get_action_history().await;
        assert_eq!(history.len(), actions.len());
    }
}
