use crate::ml::{MLModel, SimpleLinearRegression, MLInput, MLOutput, MLError};
use log::info;

pub struct AIModule {
    ml_model: Box<dyn MLModel>,
}

impl AIModule {
    pub fn new() -> Self {
        AIModule {
            ml_model: Box::new(SimpleLinearRegression::new()),
        }
    }

    pub fn train(&mut self, data: &[MLInput]) -> Result<(), MLError> {
        self.ml_model.update(data)
    }

    pub fn predict(&self, input: &MLInput) -> Result<MLOutput, MLError> {
        self.ml_model.predict(input)
    }
}

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing AI module");
    // Perform any necessary initialization
    Ok(())
}