use anyhow::Result;
use std::sync::Arc;
use tokio::sync::{Mutex, broadcast};
use log::{info, warn, error};

#[derive(Error, Debug)]
pub enum Web5AgentError {
    #[error("DID operation failed: {0}")]
    DIDError(String),
    #[error("Protocol error: {0}")]
    ProtocolError(String),
    #[error("Data handling error: {0}")]
    DataError(String),
}

pub struct Web5MLAgent {
    dwn: Arc<DWN>,
    did_manager: Arc<DIDManager>,
    ml_registry: Arc<MLRegistry>,
    protocol_handler: Arc<ProtocolHandler>,
    tx: mpsc::Sender<AgentMessage>,
    rx: mpsc::Receiver<AgentMessage>,
    metrics: Web5AgentMetrics,
}

#[async_trait]
impl MLAgent for Web5MLAgent {
    async fn process_message(&mut self, message: AgentMessage) -> Result<()> {
        match message {
            AgentMessage::MLCoreUpdate(event) => {
                self.handle_ml_update(event).await?;
            },
            AgentMessage::SystemChange(event) => {
                self.handle_system_change(event).await?;
            },
            _ => {}
        }
        Ok(())
    }

    async fn observe(&mut self) -> Result<Vec<AgentMessage>> {
        let mut messages = Vec::new();
        
        // Process DWN records
        let records = self.process_dwn_records().await?;
        for record in records {
            messages.push(self.convert_to_agent_message(record)?);
        }

        // Process DID operations
        let did_updates = self.process_did_updates().await?;
        messages.extend(did_updates);

        Ok(messages)
    }

    async fn act(&mut self) -> Result<()> {
        // Update ML models with Web5 data
        self.update_web5_models().await?;
        
        // Process protocol updates
        self.handle_protocol_updates().await?;
        
        // Update metrics
        self.metrics.record_action();
        Ok(())
    }
}

impl Web5MLAgent {
    pub async fn new(
        dwn: Arc<DWN>,
        did_manager: Arc<DIDManager>,
        ml_registry: Arc<MLRegistry>,
    ) -> Result<Self> {
        let (tx, rx) = mpsc::channel(100);
        let protocol_handler = Arc::new(ProtocolHandler::new(
            dwn.clone(),
            did_manager.clone(),
        ));
        Ok(Self {
            dwn,
            did_manager,
            ml_registry,
            protocol_handler,
            tx,
            rx,
            metrics: Web5AgentMetrics::new(),
        })
    }
}
