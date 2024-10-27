// src/management.rs
use crate::data_feed::{DataFeed, DataSource};
use crate::reporting::ReportType;
use std::collections::HashMap;

pub enum ManagementAction {
    UpdateConfig(HashMap<String, String>),
    RequestReport(ReportType),
    Shutdown,
    AddDataFeed(DataSource, Box<dyn DataFeed>),
    RemoveDataFeed(DataSource),
}

pub enum OperationalStatus {
    Normal,
    Shutdown,
    // Add other status types as needed
}

pub struct SystemManager {
    data_feeds: HashMap<DataSource, Box<dyn DataFeed>>,
    status: OperationalStatus,
}

impl SystemManager {
    pub fn new() -> Self {
        SystemManager {
            data_feeds: HashMap::new(),
            status: OperationalStatus::Normal,
        }
    }

    pub fn perform_action(&mut self, action: ManagementAction) {
        match action {
            ManagementAction::UpdateConfig(config) => {
                // Implement configuration update logic
            }
            ManagementAction::RequestReport(report_type) => {
                // Implement report request logic
            }
            ManagementAction::Shutdown => {
                self.status = OperationalStatus::Shutdown;
            }
            ManagementAction::AddDataFeed(source, feed) => {
                self.data_feeds.insert(source, feed);
            }
            ManagementAction::RemoveDataFeed(source) => {
                self.data_feeds.remove(&source);
            }
        }
    }

    pub fn get_status(&self) -> &OperationalStatus {
        &self.status
    }
}