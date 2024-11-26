// src/metrics/agent_metrics.rs
pub struct AgentMetrics {
    core_metrics: CoreAgentMetrics,
    enterprise_metrics: EnterpriseAgentMetrics,
    integration_metrics: IntegrationAgentMetrics,
}

impl AgentMetrics {
    pub fn new() -> Self {
        Self {
            core_metrics: CoreAgentMetrics::new(),
            enterprise_metrics: EnterpriseAgentMetrics::new(),
            integration_metrics: IntegrationAgentMetrics::new(),
        }
    }

    pub fn record_agent_action(&self, agent_type: AgentType, action: &AgentAction) {
        match agent_type {
            AgentType::Core(core_type) => self.core_metrics.record_action(core_type, action),
            AgentType::Enterprise(ent_type) => self.enterprise_metrics.record_action(ent_type, action),
            AgentType::Integration(int_type) => self.integration_metrics.record_action(int_type, action),
        }
    }
}