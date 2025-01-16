use super::super::data_manager::{ProtocolDefinition, SchemaDefinition, ProtocolRule};
use serde_json::json;

pub mod credentials;
pub mod verification;
pub mod resolution;

pub const IDENTITY_PROTOCOL_ID: &str = "https://anya.protocol/identity/v1";

#[derive(Debug)]
pub struct IdentityProtocol {
    pub definition: ProtocolDefinition,
    pub credentials: credentials::CredentialManager,
    pub verification: verification::VerificationManager,
    pub resolution: resolution::ResolutionManager,
}

impl IdentityProtocol {
    pub fn new() -> Self {
        Self {
            definition: get_identity_protocol(),
            credentials: credentials::CredentialManager::new(),
            verification: verification::VerificationManager::new(),
            resolution: resolution::ResolutionManager::new(),
        }
    }

    pub async fn initialize(&self) -> Result<(), IdentityError> {
        self.credentials.initialize().await?;
        self.verification.initialize().await?;
        self.resolution.initialize().await?;
        Ok(())
    }
}

pub fn get_identity_protocol() -> ProtocolDefinition {
    ProtocolDefinition {
        protocol_id: IDENTITY_PROTOCOL_ID.to_string(),
        types: get_identity_schemas(),
        rules: get_identity_rules(),
    }
}

fn get_identity_schemas() -> Vec<SchemaDefinition> {
    vec![
        SchemaDefinition {
            schema_id: "IdentityCredential".to_string(),
            schema: json!({
                "type": "object",
                "properties": {
                    "did": { "type": "string" },
                    "verificationMethod": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "id": { "type": "string" },
                                "type": { "type": "string" },
                                "controller": { "type": "string" },
                                "publicKeyMultibase": { "type": "string" }
                            },
                            "required": ["id", "type", "controller", "publicKeyMultibase"]
                        }
                    },
                    "authentication": {
                        "type": "array",
                        "items": { "type": "string" }
                    },
                    "assertionMethod": {
                        "type": "array",
                        "items": { "type": "string" }
                    },
                    "keyAgreement": {
                        "type": "array",
                        "items": { "type": "string" }
                    }
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
                    "type": {
                        "type": "array",
                        "items": { "type": "string" }
                    },
                    "issuer": { "type": "string" },
                    "issuanceDate": {
                        "type": "string",
                        "format": "date-time"
                    },
                    "credentialSubject": {
                        "type": "object"
                    },
                    "proof": {
                        "type": "object",
                        "properties": {
                            "type": { "type": "string" },
                            "created": {
                                "type": "string",
                                "format": "date-time"
                            },
                            "verificationMethod": { "type": "string" },
                            "proofPurpose": { "type": "string" },
                            "proofValue": { "type": "string" }
                        },
                        "required": [
                            "type",
                            "created",
                            "verificationMethod",
                            "proofPurpose",
                            "proofValue"
                        ]
                    }
                },
                "required": [
                    "id",
                    "type",
                    "issuer",
                    "issuanceDate",
                    "credentialSubject",
                    "proof"
                ]
            }),
        },
    ]
}

fn get_identity_rules() -> Vec<ProtocolRule> {
    vec![
        ProtocolRule {
            action: "write".to_string(),
            participant: "issuer".to_string(),
            conditions: vec![
                "auth.verified = true".to_string(),
                "auth.role = 'issuer'".to_string(),
            ],
        },
        ProtocolRule {
            action: "read".to_string(),
            participant: "any".to_string(),
            conditions: vec![],
        },
        ProtocolRule {
            action: "verify".to_string(),
            participant: "verifier".to_string(),
            conditions: vec![
                "auth.verified = true".to_string(),
                "auth.role = 'verifier'".to_string(),
            ],
        },
    ]
}
