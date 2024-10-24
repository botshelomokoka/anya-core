use crate::ml_core::MetricType;
use crate::management::OperationalStatus;
use std::collections::HashMap;

pub struct Report {
    pub report_type: ReportType,
    pub metrics: HashMap<MetricType, f64>,
    pub operational_status: OperationalStatus,
}

pub enum ReportType {
    Periodic,
    ConfigUpdate,
    BlockchainUpdate,
    // Add other report types as needed
}

pub struct SystemWideReporter {
    // Implement reporter functionality
}