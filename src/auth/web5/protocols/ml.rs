use super::super::data_manager::{ProtocolDefinition, SchemaDefinition};
use serde_json::json;

pub const ML_PROTOCOL_ID: &str = "https://anya.protocol/ml/v1";

pub fn get_ml_protocol() -> ProtocolDefinition {
    ProtocolDefinition {
        protocol_id: ML_PROTOCOL_ID.to_string(),
        types: vec![
            SchemaDefinition {
                schema_id: "ModelTraining".to_string(),
                schema: json!({
                    "type": "object",
                    "properties": {
                        "model_id": { "type": "string" },
                        "training_data": {
                            "type": "array",
                            "items": { "type": "object" }
                        },
                        "hyperparameters": { "type": "object" },
                        "metrics": { "type": "object" }
                    }
                }),
            },
            SchemaDefinition {
                schema_id: "ModelPrediction".to_string(),
                schema: json!({
                    "type": "object",
                    "properties": {
                        "model_id": { "type": "string" },
                        "input": { "type": "object" },
                        "prediction": { "type": "object" },
                        "confidence": { "type": "number" }
                    }
                }),
            },
        ],
        rules: vec![
            // Add rules...
        ],
    }
}
