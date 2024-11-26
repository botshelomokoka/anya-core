use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn, error};
use metrics::{counter, gauge};

#[derive(Error, Debug)]
pub enum EvaluationError {
    #[error("Model evaluation failed: {0}")]
    ModelError(String),
    #[error("Network evaluation failed: {0}")]
    NetworkError(String),
    #[error("System stability error: {0}")]
    StabilityError(String),
}

pub struct MLEvaluationSystem {
    ml_core: Arc<Mutex<MLCore>>,
    system_evaluator: Arc<SystemEvaluator>,
    federated_learning: Arc<FederatedLearning>,
    metrics: EvaluationMetrics,
}

impl MLEvaluationSystem {
    pub async fn new(
        ml_core: Arc<Mutex<MLCore>>,
        blockchain: Arc<BlockchainInterface>,
        bitcoin_support: BitcoinSupport,
        web5_support: Web5Support,
    ) -> Result<Self> {
        // Referenced from ml_logic/system_evaluation.rs lines 31-54
        let system_evaluator = Arc::new(SystemEvaluator::new(
            blockchain.clone(),
            DataManager::new(),
            SecurityManager::new(),
        ));

        // Referenced from ml_logic/federated_learning.rs lines 479-488
        let federated_learning = Arc::new(setup_federated_learning(
            bitcoin_support,
            STXSupport::default(),
            LightningSupport::default(),
            web5_support,
            UserWallet::default(),
        ).await?);

        Ok(Self {
            ml_core,
            system_evaluator,
            federated_learning,
            metrics: EvaluationMetrics::new(),
        })
    }

    pub async fn evaluate_system(&self) -> Result<f64, EvaluationError> {
        // Referenced from ml_logic/system_evaluation.rs lines 596-603
        let performance = self.system_evaluator.evaluate_performance(&self.federated_learning).await
            .map_err(|e| EvaluationError::ModelError(e.to_string()))?;

        // Get model metrics
        // Referenced from ml_logic/federated_learning.rs lines 462-476
        let accuracy = self.federated_learning.get_model_accuracy().await
            .map_err(|e| EvaluationError::ModelError(e.to_string()))?;
        let loss = self.federated_learning.get_model_loss().await
            .map_err(|e| EvaluationError::ModelError(e.to_string()))?;
        
        self.metrics.record_evaluation(performance, accuracy, loss);

        if performance < 0.8 {
            warn!("System performance below threshold: {}", performance);
            self.metrics.record_performance_issue();
        }

        Ok(performance)
    }

    async fn validate_system_stability(&self) -> Result<(), EvaluationError> {
        // Referenced from ml/manager.rs lines 13-24
        let changes = self.detect_system_changes().await?;
        let impact = self.analyze_change_impact(&changes).await
            .map_err(|e| EvaluationError::StabilityError(e.to_string()))?;

        if impact.requires_model_update {
            self.update_models(changes).await?;
        }

        Ok(())
    }
}
