// src/validation/agent_validation.rs
pub struct AgentValidator {
    core_validator: CoreAgentValidator,
    enterprise_validator: EnterpriseAgentValidator,
    integration_validator: IntegrationAgentValidator,
}

impl AgentValidator {
    pub async fn validate_agent_state(&self, agent: &dyn MLAgent) -> Result<ValidationReport> {
        let agent_type = agent.get_type();
        match agent_type {
            AgentType::Core(core_type) => {
                self.core_validator.validate(agent, core_type).await
            },
            AgentType::Enterprise(ent_type) => {
                self.enterprise_validator.validate(agent, ent_type).await
            },
            AgentType::Integration(int_type) => {
                self.integration_validator.validate(agent, int_type).await
            },
        }
    }
}