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
    // Implement system manager functionality
}