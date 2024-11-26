use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MLProtocolDefinition {
    name: String,
    version: String,
    types: Vec<MLDataType>,
    roles: Vec<Role>,
    rules: Vec<Rule>,
}

impl MLProtocolDefinition {
    pub fn new_training_protocol() -> Self {
        Self {
            name: "ml.training.protocol".to_string(),
            version: "1.0.0".to_string(),
            types: vec![
                MLDataType::TrainingData,
                MLDataType::ModelState,
                MLDataType::ValidationResults,
            ],
            roles: vec![
                Role::DataProvider,
                Role::ModelTrainer,
                Role::Validator,
            ],
            rules: vec![
                Rule::RequireDataEncryption,
                Rule::RequireOwnerConsent,
                Rule::RequireValidation,
            ],
        }
    }
}

