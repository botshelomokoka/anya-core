use serde::{Serialize, Deserialize};
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
pub struct ProtocolVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl ProtocolVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControl {
    pub roles: Vec<String>,
    pub permissions: Vec<Permission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub action: String,
    pub resource: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolDefinition {
    pub protocol_uri: String,
    pub version: ProtocolVersion,
    pub types: HashMap<String, TypeDefinition>,
    pub structure: ProtocolStructure,
    pub access_control: AccessControl,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeDefinition {
    pub schema: String,
    pub formats: Vec<String>,
    pub required_fields: Vec<String>,
    pub version: ProtocolVersion,
}

impl ProtocolDefinition {
    pub fn validate_compatibility(&self, other: &ProtocolDefinition) -> bool {
        self.version.major == other.version.major &&
        self.version.minor >= other.version.minor
    }

    pub fn check_permission(&self, role: &str, action: &str, resource: &str) -> bool {
        self.access_control.permissions.iter().any(|p| {
            p.role == role && p.action == action && p.resource == resource
        })
    }
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
    protocol_uri: String,
    version: ProtocolVersion,
    types: HashMap<String, TypeDefinition>,
    structure: ProtocolStructure,
    access_control: AccessControl,
}

impl ProtocolBuilder {
    pub fn new(protocol_uri: &str) -> Self {
        Self {
            protocol_uri: protocol_uri.to_string(),
            version: ProtocolVersion::new(1, 0, 0),
            types: HashMap::new(),
            structure: ProtocolStructure {
                actions: vec![],
                relationships: vec![],
            },
            access_control: AccessControl {
                roles: vec!["admin".to_string(), "user".to_string()],
                permissions: Vec::new(),
            },
        }
    }

    pub fn version(mut self, major: u32, minor: u32, patch: u32) -> Self {
        self.version = ProtocolVersion::new(major, minor, patch);
        self
    }

    pub fn add_type(mut self, name: &str, schema: &str, formats: Vec<&str>) -> Self {
        self.types.insert(name.to_string(), TypeDefinition {
            schema: schema.to_string(),
            formats: formats.iter().map(|s| s.to_string()).collect(),
            required_fields: Vec::new(),
            version: self.version.clone(),
        });
        self
    }

    pub fn add_permission(mut self, action: &str, role: &str, resource: &str) -> Self {
        self.access_control.permissions.push(Permission {
            action: action.to_string(),
            role: role.to_string(),
            resource: resource.to_string(),
        });
        self
    }

    pub fn add_action(mut self, name: &str, who: &str, can: &str) -> Self {
        self.structure.actions.push(ActionDefinition {
            name: name.to_string(),
            who: who.to_string(),
            can: can.to_string(),
        });
        self
    }

    pub fn add_relationship(mut self, from: &str, to: &str, type_name: &str) -> Self {
        self.structure.relationships.push(RelationshipDefinition {
            from: from.to_string(),
            to: to.to_string(),
            type_name: type_name.to_string(),
        });
        self
    }

    pub fn build(self) -> ProtocolDefinition {
        ProtocolDefinition {
            protocol_uri: self.protocol_uri,
            version: self.version,
            types: self.types,
            structure: self.structure,
            access_control: self.access_control,
        }
    }
}

pub fn create_ml_protocol() -> ProtocolDefinition {
    ProtocolBuilder::new("https://anya.ai/ml-protocol")
        .version(1, 0, 0)
        .add_type(
            "model",
            "https://schema.org/MLModel",
            vec!["application/octet-stream"],
        )
        .add_type(
            "training_data",
            "https://schema.org/Dataset",
            vec!["application/json"],
        )
        .add_type(
            "metrics",
            "https://schema.org/MLMetrics",
            vec!["application/json"],
        )
        .add_permission("write", "model_trainer", "model")
        .add_permission("read", "model_user", "model")
        .add_permission("write", "data_provider", "training_data")
        .add_action("write", "author", "write")
        .add_action("read", "recipient", "read")
        .add_relationship("model", "training_data", "uses")
        .build()
}

pub fn create_data_protocol() -> ProtocolDefinition {
    ProtocolBuilder::new("https://anya.ai/data-protocol")
        .version(1, 0, 0)
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
        .add_permission("write", "author", "data")
        .add_permission("read", "recipient", "data")
        .add_action("write", "author", "write")
        .add_action("read", "recipient", "read")
        .add_relationship("data", "metadata", "describes")
        .build()
}

pub fn create_revenue_protocol() -> ProtocolDefinition {
    ProtocolBuilder::new("https://anya.ai/revenue-protocol")
        .version(1, 0, 0)
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
        .add_permission("write", "author", "revenue_data")
        .add_permission("read", "organization", "revenue_data")
        .add_action("write", "author", "write")
        .add_action("read", "organization", "read")
        .add_relationship("revenue_data", "prediction", "predicts")
        .build()
}

pub fn create_security_protocol() -> ProtocolDefinition {
    ProtocolBuilder::new("https://anya.ai/security-protocol")
        .version(1, 0, 0)
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
        .add_permission("write", "security_admin", "audit_log")
        .add_permission("read", "security_auditor", "audit_log")
        .add_permission("write", "security_admin", "access_control")
        .add_action("write", "security_admin", "write")
        .add_action("read", "security_auditor", "read")
        .add_relationship("audit_log", "access_control", "enforces")
        .build()
}

pub fn create_federated_learning_protocol() -> ProtocolDefinition {
    ProtocolBuilder::new("https://anya.ai/federated-protocol")
        .version(1, 0, 0)
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
        .add_permission("participate", "node", "training_round")
        .add_permission("aggregate", "coordinator", "training_round")
        .add_action("participate", "node", "write")
        .add_action("aggregate", "coordinator", "write")
        .add_relationship("training_round", "aggregation_result", "produces")
        .build()
}

pub fn create_governance_protocol() -> ProtocolDefinition {
    ProtocolBuilder::new("https://anya.ai/governance-protocol")
        .version(1, 0, 0)
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
        .add_permission("propose", "member", "proposal")
        .add_permission("vote", "member", "vote")
        .add_action("propose", "member", "write")
        .add_action("vote", "member", "write")
        .add_relationship("proposal", "vote", "receives")
        .build()
}

pub fn create_analytics_protocol() -> ProtocolDefinition {
    ProtocolBuilder::new("https://anya.ai/analytics-protocol")
        .version(1, 0, 0)
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
        .add_permission("record", "system", "performance_metrics")
        .add_permission("analyze", "analyst", "performance_metrics")
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
            .version(1, 0, 0)
            .add_type("test", "test://schema", vec!["application/json"])
            .add_permission("write", "author", "test")
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
