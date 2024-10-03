mod data_processor;
mod model_trainer;
mod predictor;
mod optimizer;

pub use data_processor::{DataProcessor, ProcessedData};
pub use model_trainer::{ModelTrainer, TrainedModel};
pub use predictor::{Predictor, Prediction};
pub use optimizer::{Optimizer, OptimizedAction};

use std::collections::HashMap;

pub enum MetricType {
    ModelAccuracy,
    ProcessingTime,
    PredictionConfidence,
    OptimizationScore,
    TransactionFee,
}

pub struct MLCore {
    data_processor: DataProcessor,
    model_trainer: ModelTrainer,
    predictor: Predictor,
    optimizer: Optimizer,
    metrics: HashMap<MetricType, f64>,
}

impl MLCore {
    pub fn new() -> Self {
        Self {
            data_processor: DataProcessor::new(),
            model_trainer: ModelTrainer::new(),
            predictor: Predictor::new(),
            optimizer: Optimizer::new(),
            metrics: HashMap::new(),
        }
    }

    // ... (implement other methods as in the previous MLCore implementation)
}