use log::{info, warn};
use anyhow::Result;
use crate::ml_core::MLCore;
use crate::blockchain::BlockchainInterface;
use crate::data_management::DataManager;
use crate::security::SecurityManager;
use crate::ml_logic::federated_learning::FederatedLearning;

pub struct SystemEvaluator {
    blockchain: BlockchainInterface,
    data_manager: DataManager,
    security_manager: SecurityManager,
}

impl SystemEvaluator {
    pub fn new(blockchain: BlockchainInterface, data_manager: DataManager, security_manager: SecurityManager) -> Self {
        Self {
            blockchain,
            data_manager,
            security_manager,
        }
    }

    pub async fn evaluate_performance(&self, federated_learning: &FederatedLearning) -> Result<f64> {
        info!("Evaluating system performance...");
        let model_performance = self.evaluate_model_performance(&federated_learning.ml_core).await?;
        let network_performance = self.evaluate_network_performance().await?;
        let financial_performance = self.evaluate_financial_performance().await?;
        
        Ok((model_performance + network_performance + financial_performance) / 3.0)
    }

    async fn evaluate_model_performance(&self, ml_core: &MLCore) -> Result<f64> {
        let accuracy = ml_core.get_metric(MetricType::ModelAccuracy).unwrap_or(0.0);
        let loss = ml_core.get_metric(MetricType::ModelLoss).unwrap_or(1.0);
        
        Ok(0.5 * accuracy + 0.5 * (1.0 - loss))
    }

    async fn evaluate_network_performance(&self) -> Result<f64> {
        info!("Evaluating network performance...");
        self.blockchain.get_network_performance().await
    }

    async fn evaluate_financial_performance(&self) -> Result<f64> {
        let balance = self.blockchain.get_balance().await?;
        let target_balance = self.blockchain.get_target_balance().await?;
        
        Ok(self.calculate_roi(balance, target_balance))
    }

    fn calculate_roi(&self, balance: f64, target_balance: f64) -> f64 {
        (target_balance - balance) / balance
    }
}
