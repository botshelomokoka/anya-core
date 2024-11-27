use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::collections::HashMap;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Invalid protocol definition: {0}")]
    InvalidProtocol(String),
    #[error("Schema validation failed: {0}")]
    SchemaValidation(String),
    #[error("Protocol registration failed: {0}")]
    Registration(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolDefinition {
    pub protocol_uri: String,
    pub version: String,
    pub types: HashMap<String, TypeDefinition>,
    pub structure: ProtocolStructure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeDefinition {
    pub schema: String,
    pub data_formats: Vec<String>,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolStructure {
    pub actions: Vec<ActionDefinition>,
    pub relationships: Vec<RelationshipDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionDefinition {
    pub name: String,
    pub who: String,
    pub can: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipDefinition {
    pub from: String,
    pub to: String,
    pub type_name: String,
}

pub struct ProtocolBuilder {
    definition: ProtocolDefinition,
}

impl ProtocolBuilder {
    pub fn new(protocol_uri: &str) -> Self {
        Self {
            definition: ProtocolDefinition {
                protocol_uri: protocol_uri.to_string(),
                version: "1.0".to_string(),
                types: HashMap::new(),
                structure: ProtocolStructure {
                    actions: vec![],
                    relationships: vec![],
                },
            },
        }
    }

    pub fn add_type(mut self, name: &str, schema: &str, formats: Vec<&str>) -> Self {
        self.definition.types.insert(
            name.to_string(),
            TypeDefinition {
                schema: schema.to_string(),
                data_formats: formats.into_iter().map(String::from).collect(),
                required: false,
            },
        );
        self
    }

    pub fn add_action(mut self, name: &str, who: &str, can: &str) -> Self {
        self.definition.structure.actions.push(ActionDefinition {
            name: name.to_string(),
            who: who.to_string(),
            can: can.to_string(),
        });
        self
    }

    pub fn add_relationship(mut self, from: &str, to: &str, type_name: &str) -> Self {
        self.definition.structure.relationships.push(RelationshipDefinition {
            from: from.to_string(),
            to: to.to_string(),
            type_name: type_name.to_string(),
        });
        self
    }

    pub fn build(self) -> ProtocolDefinition {
        self.definition
    }
}

// Standard protocol definitions
pub fn create_ml_protocol() -> ProtocolDefinition {
    ProtocolBuilder::new("https://anya.ai/ml-protocol")
        .add_type(
            "model",
            "https://schema.org/MLModel",
            vec!["application/octet-stream"],
        )
        .add_type(
            "training_data",
            "https://schema.org/Dataset",
            vec!["application/octet-stream", "application/json"],
        )
        .add_type(
            "metrics",
            "https://schema.org/MLMetrics",
            vec!["application/json"],
        )
        .add_action("write", "author", "write")
        .add_action("read", "recipient", "read")
        .add_relationship("model", "training_data", "uses")
        .build()
}

pub fn create_data_protocol() -> ProtocolDefinition {
    ProtocolBuilder::new("https://anya.ai/data-protocol")
        .add_type(
            "data",
            "https://schema.org/DataFeed",
            vec!["application/octet-stream", "application/json"],
        )
        .add_type(
            "metadata",
            "https://schema.org/PropertyValue",
            vec!["application/json"],
        )
        .add_action("write", "author", "write")
        .add_action("read", "recipient", "read")
        .add_relationship("data", "metadata", "describes")
        .build()
}

pub fn create_revenue_protocol() -> ProtocolDefinition {
    ProtocolBuilder::new("https://anya.ai/revenue-protocol")
        .add_type(
            "revenue_data",
            "https://schema.org/MonetaryAmount",
            vec!["application/json"],
        )
        .add_type(
            "prediction",
            "https://schema.org/Prediction",
            vec!["application/json"],
        )
        .add_action("write", "author", "write")
        .add_action("read", "organization", "read")
        .add_relationship("revenue_data", "prediction", "predicts")
        .build()
}

pub fn create_security_protocol() -> ProtocolDefinition {
    ProtocolBuilder::new("https://anya.ai/security-protocol")
        .add_type(
            "audit_log",
            "https://schema.org/SecurityAction",
            vec!["application/json"],
        )
        .add_type(
            "access_control",
            "https://schema.org/PermissionsPolicy",
            vec!["application/json"],
        )
        .add_type(
            "encryption_key",
            "https://schema.org/CryptoKey",
            vec!["application/x-pem-file"],
        )
        .add_action("write", "security_admin", "write")
        .add_action("read", "security_auditor", "read")
        .add_relationship("audit_log", "access_control", "enforces")
        .build()
}

pub fn create_federated_learning_protocol() -> ProtocolDefinition {
    ProtocolBuilder::new("https://anya.ai/federated-protocol")
        .add_type(
            "training_round",
            "https://schema.org/MLTrainingRound",
            vec!["application/octet-stream"],
        )
        .add_type(
            "aggregation_result",
            "https://schema.org/MLAggregation",
            vec!["application/octet-stream"],
        )
        .add_type(
            "participant_metrics",
            "https://schema.org/MLParticipantMetrics",
            vec!["application/json"],
        )
        .add_action("participate", "node", "write")
        .add_action("aggregate", "coordinator", "write")
        .add_relationship("training_round", "aggregation_result", "produces")
        .build()
}

pub fn create_governance_protocol() -> ProtocolDefinition {
    ProtocolBuilder::new("https://anya.ai/governance-protocol")
        .add_type(
            "policy",
            "https://schema.org/Policy",
            vec!["application/json"],
        )
        .add_type(
            "vote",
            "https://schema.org/Vote",
            vec!["application/json"],
        )
        .add_type(
            "proposal",
            "https://schema.org/Proposal",
            vec!["application/json"],
        )
        .add_action("propose", "member", "write")
        .add_action("vote", "member", "write")
        .add_relationship("proposal", "vote", "receives")
        .build()
}

pub fn create_analytics_protocol() -> ProtocolDefinition {
    ProtocolBuilder::new("https://anya.ai/analytics-protocol")
        .add_type(
            "performance_metrics",
            "https://schema.org/PerformanceMetrics",
            vec!["application/json"],
        )
        .add_type(
            "usage_stats",
            "https://schema.org/UsageStatistics",
            vec!["application/json"],
        )
        .add_type(
            "resource_metrics",
            "https://schema.org/ResourceMetrics",
            vec!["application/json"],
        )
        .add_action("record", "system", "write")
        .add_action("analyze", "analyst", "read")
        .add_relationship("performance_metrics", "usage_stats", "correlates")
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_builder() {
        let protocol = ProtocolBuilder::new("test://protocol")
            .add_type("test", "test://schema", vec!["application/json"])
            .add_action("write", "author", "write")
            .build();

        assert_eq!(protocol.protocol_uri, "test://protocol");
        assert_eq!(protocol.types.len(), 1);
        assert_eq!(protocol.structure.actions.len(), 1);
    }

    #[test]
    fn test_ml_protocol() {
        let protocol = create_ml_protocol();
        assert!(protocol.types.contains_key("model"));
        assert!(protocol.types.contains_key("training_data"));
        assert!(protocol.types.contains_key("metrics"));
    }

    #[test]
    fn test_federated_protocol() {
        let protocol = create_federated_learning_protocol();
        assert!(protocol.types.contains_key("training_round"));
        assert!(protocol.types.contains_key("aggregation_result"));
        let actions: Vec<_> = protocol.structure.actions
            .iter()
            .map(|a| &a.name)
            .collect();
        assert!(actions.contains(&"participate".to_string()));
        assert!(actions.contains(&"aggregate".to_string()));
    }

    #[test]
    fn test_governance_protocol() {
        let protocol = create_governance_protocol();
        assert!(protocol.types.contains_key("policy"));
        assert!(protocol.types.contains_key("vote"));
        let actions: Vec<_> = protocol.structure.actions
            .iter()
            .map(|a| &a.name)
            .collect();
        assert!(actions.contains(&"propose".to_string()));
        assert!(actions.contains(&"vote".to_string()));
    }

    #[test]
    fn test_security_protocol() {
        let protocol = create_security_protocol();
        assert!(protocol.types.contains_key("audit_log"));
        assert!(protocol.types.contains_key("access_control"));
        let actions: Vec<_> = protocol.structure.actions
            .iter()
            .map(|a| &a.who)
            .collect();
        assert!(actions.contains(&"security_admin".to_string()));
        assert!(actions.contains(&"security_auditor".to_string()));
    }
}
