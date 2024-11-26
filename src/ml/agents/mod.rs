use async_trait::async_trait;
use tokio::sync::{mpsc, broadcast};
use anyhow::Result;

#[derive(Debug, Clone)]
pub enum AgentMessage {
    MLCoreUpdate(MLCoreEvent),
    MLLogicUpdate(MLLogicEvent),
    DAORulesUpdate(DAORulesEvent),
    FederatedLearningUpdate(FederatedEvent),
    SystemChange(SystemChangeEvent),
}

#[async_trait]
pub trait MLAgent: Send + Sync {
    async fn process_message(&mut self, message: AgentMessage) -> Result<()>;
    async fn observe(&mut self) -> Result<Vec<AgentMessage>>;
    async fn act(&mut self) -> Result<()>;
}

pub struct AgentCoordinator {
    agents: Vec<Box<dyn MLAgent>>,
    message_bus: broadcast::Sender<AgentMessage>,
    ml_registry: Arc<MLRegistry>,
}

