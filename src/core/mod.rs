use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use thiserror::Error;

use crate::ml_logic::{MLMetrics, MLRevenueMetrics};
use crate::enterprise::EnterpriseConfig;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Business logic error: {0}")]
    BusinessLogic(String),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Agent error: {0}")]
    Agent(String),
    #[error("Integration error: {0}")]
    Integration(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreConfig {
    pub business_rules_enabled: bool,
    pub agent_system_enabled: bool,
    pub metrics_collection_enabled: bool,
    pub auto_scaling_enabled: bool,
}

pub struct AnyaCore {
    config: Arc<RwLock<CoreConfig>>,
    business_engine: Arc<RwLock<BusinessEngine>>,
    agent_system: Arc<RwLock<AgentSystem>>,
    metrics_collector: Arc<RwLock<MetricsCollector>>,
    enterprise_integration: Option<Arc<RwLock<EnterpriseIntegration>>>,
}

impl AnyaCore {
    pub async fn new(
        config: CoreConfig,
        enterprise_config: Option<EnterpriseConfig>,
    ) -> Result<Self, CoreError> {
        let business_engine = Arc::new(RwLock::new(BusinessEngine::new()));
        let agent_system = Arc::new(RwLock::new(AgentSystem::new()));
        let metrics_collector = Arc::new(RwLock::new(MetricsCollector::new()));
        
        let enterprise_integration = if let Some(config) = enterprise_config {
            Some(Arc::new(RwLock::new(EnterpriseIntegration::new(config)?)))
        } else {
            None
        };

        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            business_engine,
            agent_system,
            metrics_collector,
            enterprise_integration,
        })
    }

    pub async fn process_business_logic(&self, context: BusinessContext) -> Result<BusinessOutcome, CoreError> {
        let config = self.config.read().await;
        if !config.business_rules_enabled {
            return Ok(BusinessOutcome::default());
        }

        let mut engine = self.business_engine.write().await;
        let outcome = engine.process(context).await?;

        if let Some(enterprise) = &self.enterprise_integration {
            enterprise.write().await.process_outcome(&outcome).await?;
        }

        Ok(outcome)
    }

    pub async fn dispatch_agents(&self, task: AgentTask) -> Result<AgentResponse, CoreError> {
        let config = self.config.read().await;
        if !config.agent_system_enabled {
            return Ok(AgentResponse::default());
        }

        let mut agent_system = self.agent_system.write().await;
        let response = agent_system.dispatch(task).await?;

        if let Some(enterprise) = &self.enterprise_integration {
            enterprise.write().await.process_agent_response(&response).await?;
        }

        Ok(response)
    }

    pub async fn collect_metrics(&self) -> Result<CoreMetrics, CoreError> {
        let config = self.config.read().await;
        if !config.metrics_collection_enabled {
            return Ok(CoreMetrics::default());
        }

        let metrics_collector = self.metrics_collector.read().await;
        let metrics = metrics_collector.collect().await?;

        if let Some(enterprise) = &self.enterprise_integration {
            enterprise.write().await.process_metrics(&metrics).await?;
        }

        Ok(metrics)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessContext {
    pub operation_type: OperationType,
    pub parameters: HashMap<String, String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    ModelTraining,
    Prediction,
    ResourceScaling,
    RevenueOptimization,
    SecurityAudit,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BusinessOutcome {
    pub success: bool,
    pub actions_taken: Vec<BusinessAction>,
    pub metrics_impact: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BusinessAction {
    AdjustModel(ModelAdjustment),
    UpdatePricing(PricingUpdate),
    ScaleResources(ResourceAdjustment),
    NotifyStakeholders(Notification),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelAdjustment {
    pub model_id: String,
    pub parameter_updates: HashMap<String, f64>,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingUpdate {
    pub service_type: String,
    pub price_change: f64,
    pub effective_from: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAdjustment {
    pub resource_type: String,
    pub adjustment_factor: f64,
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub level: NotificationLevel,
    pub message: String,
    pub recipients: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationLevel {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTask {
    pub task_type: AgentTaskType,
    pub priority: u32,
    pub parameters: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentTaskType {
    ModelOptimization,
    DataValidation,
    SecurityCheck,
    PerformanceMonitoring,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentResponse {
    pub success: bool,
    pub findings: Vec<AgentFinding>,
    pub recommendations: Vec<AgentRecommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentFinding {
    pub category: String,
    pub severity: Severity,
    pub description: String,
    pub evidence: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRecommendation {
    pub action_type: String,
    pub description: String,
    pub impact_assessment: HashMap<String, f64>,
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CoreMetrics {
    pub ml_metrics: MLMetrics,
    pub revenue_metrics: MLRevenueMetrics,
    pub agent_metrics: AgentMetrics,
    pub system_health: SystemHealth,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentMetrics {
    pub active_agents: u32,
    pub completed_tasks: u64,
    pub success_rate: f64,
    pub average_response_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SystemHealth {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub network_latency: f64,
    pub error_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_core_business_logic() {
        let config = CoreConfig {
            business_rules_enabled: true,
            agent_system_enabled: true,
            metrics_collection_enabled: true,
            auto_scaling_enabled: true,
        };

        let core = AnyaCore::new(config, None).await.unwrap();

        let context = BusinessContext {
            operation_type: OperationType::ModelTraining,
            parameters: HashMap::new(),
            metadata: HashMap::new(),
        };

        let outcome = core.process_business_logic(context).await.unwrap();
        assert!(outcome.success);
    }

    #[tokio::test]
    async fn test_agent_system() {
        let config = CoreConfig {
            business_rules_enabled: true,
            agent_system_enabled: true,
            metrics_collection_enabled: true,
            auto_scaling_enabled: true,
        };

        let core = AnyaCore::new(config, None).await.unwrap();

        let task = AgentTask {
            task_type: AgentTaskType::ModelOptimization,
            priority: 1,
            parameters: HashMap::new(),
        };

        let response = core.dispatch_agents(task).await.unwrap();
        assert!(response.success);
    }
}
