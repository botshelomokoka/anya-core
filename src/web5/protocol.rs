use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Protocol {
    pub types: HashMap<String, SchemaDefinition>,
    pub protocol_url: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SchemaDefinition {
    pub schema: String,
    pub description: String,
}

impl Protocol {
    pub fn new() -> Self {
        let mut types = HashMap::new();
        
        // Define data schemas
        types.insert(
            "record".to_string(),
            SchemaDefinition {
                schema: r#"{
                    "type": "object",
                    "properties": {
                        "id": { "type": "string" },
                        "table": { "type": "string" },
                        "data": { "type": "object" },
                        "created_at": { "type": "integer" },
                        "updated_at": { "type": "integer" }
                    },
                    "required": ["id", "table", "data"]
                }"#.to_string(),
                description: "General data record".to_string(),
            },
        );

        types.insert(
            "schema".to_string(),
            SchemaDefinition {
                schema: r#"{
                    "type": "object",
                    "properties": {
                        "name": { "type": "string" },
                        "fields": {
                            "type": "object",
                            "additionalProperties": {
                                "type": "object",
                                "properties": {
                                    "type": { "type": "string" },
                                    "required": { "type": "boolean" }
                                }
                            }
                        }
                    },
                    "required": ["name", "fields"]
                }"#.to_string(),
                description: "Table schema definition".to_string(),
            },
        );

        Protocol {
            types,
            protocol_url: "https://anya.dev/protocol/v1".to_string(),
            description: "Anya Web5 Data Protocol".to_string(),
        }
    }

    pub fn to_dwn_protocol(&self) -> serde_json::Value {
        serde_json::json!({
            "protocol": self.protocol_url,
            "published": true,
            "types": self.types,
            "structure": {
                "record": {
                    "actions": [
                        { "who": "anyone", "can": "write" },
                        { "who": "anyone", "can": "read" }
                    ]
                },
                "schema": {
                    "actions": [
                        { "who": "author", "can": "write" },
                        { "who": "anyone", "can": "read" }
                    ]
                }
            }
        })
    }
}
