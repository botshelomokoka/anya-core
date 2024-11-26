use super::data_manager::{ProtocolDefinition, SchemaDefinition, ProtocolRule};
use serde_json::json;

pub const ANYA_PROTOCOL_ID: &str = "https://anya.protocol/v1";

pub fn get_anya_protocol() -> ProtocolDefinition {
    ProtocolDefinition {
        protocol_id: ANYA_PROTOCOL_ID.to_string(),
        types: vec![
            SchemaDefinition {
                schema_id: "FileAnalysis".to_string(),
                schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string" },
                        "category": { "type": "string" },
                        "importance_score": { "type": "number" },
                        "analysis_timestamp": { "type": "string", "format": "date-time" }
                    }
                }),
            },
            SchemaDefinition {
                schema_id: "MLModel".to_string(),
                schema: json!({
                    "type": "object",
                    "properties": {
                        "version": { "type": "string" },
                        "features": { "type": "array" },
                        "weights": { "type": "array" },
                        "validation_score": { "type": "number" }
                    }
                }),
            },
        ],
        rules: vec![
            ProtocolRule {
                action: "write".to_string(),
                participant: "owner".to_string(),
                conditions: vec!["auth.verified = true".to_string()],
            },
            ProtocolRule {
                action: "read".to_string(),
                participant: "any".to_string(),
                conditions: vec![],
            },
        ],
    }
}
