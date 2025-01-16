use anyhow::Result;

pub mod research;
pub mod analysis;
pub mod bip_monitor;

pub struct MLManager {
    research: research::ResearchManager,
    analysis: analysis::AnalysisManager,
    bip_monitor: bip_monitor::BipMonitor,
}

impl MLManager {
    pub fn new() -> Self {
        Self {
            research: research::ResearchManager::new(),
            analysis: analysis::AnalysisManager::new(),
            bip_monitor: bip_monitor::BipMonitor::new(),
        }
    }

    pub async fn monitor_bips(&self) -> Result<()> {
        self.bip_monitor.start_monitoring().await
    }
}
