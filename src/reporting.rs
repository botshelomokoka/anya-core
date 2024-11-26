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

use crate::ml_core::MLCore;

pub struct SystemWideReporter {
    ml_core: MLCore,
}

impl SystemWideReporter {
    pub fn new(ml_core: MLCore) -> Self {
        Self { ml_core }
    }

    pub fn generate_report(&self) -> Report {
        let metrics = self.ml_core.get_metrics().clone();
        Report {
            report_type: ReportType::Periodic,
            metrics,
            operational_status: OperationalStatus::Operational,
        }
    }
}