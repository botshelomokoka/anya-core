use anyhow::Result;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use serde::{Serialize, Deserialize};
use log::{info, warn, error};

#[derive(Error, Debug)]
pub enum AgentCoordinatorError {
    #[error("Agent synchronization failed: {0}")]
    SyncError(String),
    #[error("Task distribution failed: {0}")]
    TaskError(String),
    #[error("Protocol error: {0}")]
    ProtocolError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentTask {
    task_id: String,
    agent_did: String,
    task_type: TaskType,
    status: TaskStatus,
    priority: u8,
    created_at: u64,
}

pub struct Web5AgentCoordinator {
    dwn: Arc<DWN>,
    did_manager: Arc<DIDManager>,
    data_handler: Arc<Web5DataHandler>,
    protocol_handler: Arc<ProtocolHandler>,
    model_coordinator: Arc<Web5ModelCoordinator>,
    training_coordinator: Arc<Web5TrainingCoordinator>,
    active_tasks: RwLock<HashMap<String, AgentTask>>,
    metrics: CoordinatorMetrics,
}

impl Web5AgentCoordinator {
    pub async fn new(
        dwn: Arc<DWN>,
        did_manager: Arc<DIDManager>,
        data_handler: Arc<Web5DataHandler>,
        protocol_handler: Arc<ProtocolHandler>,
        model_coordinator: Arc<Web5ModelCoordinator>,
        training_coordinator: Arc<Web5TrainingCoordinator>,
    ) -> Result<Self> {
        Ok(Self {
            dwn,
            did_manager,
            data_handler,
            protocol_handler,
            model_coordinator,
            training_coordinator,
            active_tasks: RwLock::new(HashMap::new()),
            metrics: CoordinatorMetrics::new(),
        })
    }

    pub async fn coordinate_agents(&self) -> Result<()> {
        // Get available agents
        let agents = self.discover_active_agents().await?;
        
        // Distribute tasks
        self.distribute_tasks(&agents).await?;
        
        // Monitor task progress
        self.monitor_task_progress().await?;
        
        // Process completed tasks
        self.process_completed_tasks().await?;
        
        self.metrics.record_coordination_cycle();
        Ok(())
    }

    async fn distribute_tasks(&self, agents: &[String]) -> Result<()> {
        let tasks = self.generate_agent_tasks().await?;
        
        for task in tasks {
            if let Some(agent_did) = self.select_agent_for_task(&task, agents) {
                self.assign_task(&task, &agent_did).await?;
            }
        }
        Ok(())
    }

    async fn assign_task(&self, task: &AgentTask, agent_did: &str) -> Result<()> {
        let record = MLDataRecord {
            protocol: "ml.agent.task".to_string(),
            schema: "agent-task".to_string(),
            data: serde_json::to_vec(task)?,
            owner_did: agent_did.to_string(),
            permissions: vec![Permission::OwnerOnly],
        };

        self.data_handler.store_training_data(&record.data