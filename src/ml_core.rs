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
pub fn process_data(&mut self) -> ProcessedData {
    let processed_data = self.data_processor.process();
    self.metrics.insert(MetricType::ProcessingTime, 1.0); // Placeholder value
    processed_data
}

pub fn train_model(&mut self, data: ProcessedData) -> TrainedModel {
    let trained_model = self.model_trainer.train(data);
    self.metrics.insert(MetricType::ModelAccuracy, 0.95); // Placeholder value
    trained_model
}

pub fn make_prediction(&mut self, model: TrainedModel) -> Prediction {
    let prediction = self.predictor.predict(model);
    self.metrics.insert(MetricType::PredictionConfidence, 0.9); // Placeholder value
    prediction
}

pub fn optimize(&mut self, action: OptimizedAction) -> OptimizedAction {
    let optimized_action = self.optimizer.optimize(action);
    self.metrics.insert(MetricType::OptimizationScore, 0.85); // Placeholder value
    optimized_action
}

pub fn get_metric(&self, metric: MetricType) -> Option<&f64> {
    self.metrics.get(&metric)
}