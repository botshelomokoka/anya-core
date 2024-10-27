use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemProtocols {
    pub ml_training: ProtocolDefinition,
    pub ml_model: ProtocolDefinition,
    pub identity: ProtocolDefinition,
    pub data_exchange: ProtocolDefinition,
}

impl SystemProtocols {
    pub fn new() -> Self {
        Self {
            ml_training: ProtocolDefinition::new("ml.training")
                .with_actions(vec![
                    "TrainingDataSubmit",
                    "ModelUpdate",
                    "ValidationRequest",
                ])
                .with_schemas(vec![
                    "TrainingData",
                    "ModelState",
                    "ValidationResult",
                ])
                .build(),

            ml_model: ProtocolDefinition::new("ml.model")
                .with_actions(vec![
                    "ModelQuery",
                    "PredictionRequest",
                    "ModelValidation",
                ])
                .build(),

            identity: ProtocolDefinition::new("system.identity")
                .with_actions(vec![
                    "IdentityCreate",
                    "IdentityUpdate",
                    "IdentityVerify",
                ])
                .build(),

            data_exchange: ProtocolDefinition::new("system.data")
                .with_actions(vec![
                    "DataSubmit",
                    "DataRequest",
                    "DataValidate",
                ])
                .build(),
        }
    }
}

