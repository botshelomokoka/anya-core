use super::super::data_manager::{ProtocolDefinition, SchemaDefinition};
use serde_json::json;

pub const IDENTITY_PROTOCOL_ID: &str = "https://anya.protocol/identity/v1";

pub fn get_identity_protocol() -> ProtocolDefinition {
    ProtocolDefinition {
        protocol_id: IDENTITY_PROTOCOL_ID.to_string(),
        types: vec![
            SchemaDefinition {
                schema_id: "IdentityCredential".to_string(),
                schema: json!({
                    "type": "object",
                    "properties": {
                        "did": { "type": "string" },
                        "verificationMethod": { "type": "array" },
                        "authentication": { "type": "array" },
                        "assertionMethod": { "type": "array" },
                        "keyAgreement": { "type": "array" }
                    },
                    "required": ["did", "verificationMethod"]
                }),
            },
            SchemaDefinition {
                schema_id: "VerifiableClaim".to_string(),
                schema: json!({
                    "type": "object",
                    "properties": {
                        "id": { "type": "string" },
                        "type": { "type": "array" },
                        "issuer": { "type": "string" },
                        "issuanceDate": { "type": "string" },
                        "credentialSubject": { "type": "object" },
                        "proof": { "type": "object" }
                    }
                }),
            },
        ],
        rules: vec![
            // Add identity-specific rules
        ],
    }
}
