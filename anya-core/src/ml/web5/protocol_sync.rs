use anyhow::Result;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use serde::{Serialize, Deserialize};
use log::{info, warn, error};

#[derive(Error, Debug)]
pub enum ProtocolSyncError {
    #[error("Protocol sync failed: {0}")]
    SyncError(String),
    #[error("Version mismatch: {0}")]
    VersionError(String),
    #[error("DWN operation failed: {0}")]
    DWNError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProtocolState {
    protocol_id: String,
    version: String,
    last_sync: u64,
    peers: Vec<String>,
    status: SyncStatus,
}

pub struct Web5ProtocolSynchronizer {
    dwn: Arc<DWN>,
    did_manager: Arc<DIDManager>,
    data_handler: Arc<Web5DataHandler>,
    protocol_handler: Arc<ProtocolHandler>,
    protocol_states: RwLock<HashMap<String, ProtocolState>>,
    metrics: SyncMetrics,
}

impl Web5ProtocolSynchronizer {
    pub async fn new(
        dwn: Arc<DWN>,
        did_manager: Arc<DIDManager>,
        data_handler: Arc<Web5DataHandler>,
        protocol_handler: Arc<ProtocolHandler>,
    ) -> Result<Self> {
        Ok(Self {
            dwn,
            did_manager,
            data_handler,
            protocol_handler,
            protocol_states: RwLock::new(HashMap::new()),
            metrics: SyncMetrics::new(),
        })
    }

    pub async fn sync_protocols(&self) -> Result<()> {
        // Get registered protocols
        let protocols = self.protocol_handler.list_protocols().await?;
        
        for protocol in protocols {
            self.sync_protocol(&protocol).await?;
        }

        self.metrics.record_sync_cycle();
        Ok(())
    }

    async fn sync_protocol(&self, protocol: &ProtocolDefinition) -> Result<()> {
        let state = self.get_protocol_state(protocol).await?;
        
        // Check for updates from peers
        let updates = self.fetch_protocol_updates(&state).await?;
        
        if !updates.is_empty() {
            self.apply_protocol_updates(protocol, updates).await?;
            self.update_protocol_state(protocol).await?;
        }

        Ok(())
    }

    async fn fetch_protocol_updates(&self, state: &ProtocolState) -> Result<Vec<ProtocolUpdate>> {
        let query = RecordQuery::new()
            .with_protocol("ml.protocol.sync")
            .with_schema("protocol-update")
            .with_timestamp_gt(state.last_sync);

        let records = self.dwn.query_records(query).await?;
        
        let mut updates = Vec::new();
        for record in records {
            if let Ok(update) = self.validate_protocol_update(&record).await {
                updates.push(update);
            }
        }

        Ok(updates)
    }
}
