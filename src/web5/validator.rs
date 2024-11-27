use jsonschema::{Draft, JSONSchema};
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Schema validation failed: {0}")]
    SchemaError(String),
    #[error("Invalid data type: expected {expected}, got {got}")]
    TypeError { expected: String, got: String },
    #[error("Required field missing: {0}")]
    MissingField(String),
}

pub struct SchemaValidator {
    schemas: HashMap<String, JSONSchema>,
}

impl SchemaValidator {
    pub fn new() -> Self {
        Self {
            schemas: HashMap::new(),
        }
    }

    pub fn register_schema(&mut self, name: &str, schema: &str) -> Result<(), ValidationError> {
        let schema: Value = serde_json::from_str(schema).map_err(|e| {
            ValidationError::SchemaError(format!("Invalid schema JSON: {}", e))
        })?;

        let compiled = JSONSchema::options()
            .with_draft(Draft::Draft7)
            .compile(&schema)
            .map_err(|e| ValidationError::SchemaError(format!("Invalid schema: {}", e)))?;

        self.schemas.insert(name.to_string(), compiled);
        Ok(())
    }

    pub fn validate(&self, schema_name: &str, data: &Value) -> Result<(), ValidationError> {
        let schema = self.schemas.get(schema_name).ok_or_else(|| {
            ValidationError::SchemaError(format!("Schema not found: {}", schema_name))
        })?;

        if let Err(errors) = schema.validate(data) {
            let error_messages: Vec<String> = errors.map(|e| e.to_string()).collect();
            return Err(ValidationError::SchemaError(error_messages.join(", ")));
        }

        Ok(())
    }

    pub fn validate_field_type(&self, value: &Value, expected_type: &str) -> Result<(), ValidationError> {
        let got_type = match value {
            Value::Null => "null",
            Value::Bool(_) => "boolean",
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
        };

        if got_type != expected_type {
            return Err(ValidationError::TypeError {
                expected: expected_type.to_string(),
                got: got_type.to_string(),
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_schema_validation() {
        let mut validator = SchemaValidator::new();
        
        // Register a test schema
        let schema = r#"{
            "type": "object",
            "properties": {
                "name": { "type": "string" },
                "age": { "type": "number" }
            },
            "required": ["name"]
        }"#;
        
        validator.register_schema("test", schema).unwrap();

        // Test valid data
        let valid_data = json!({
            "name": "John",
            "age": 30
        });
        assert!(validator.validate("test", &valid_data).is_ok());

        // Test invalid data
        let invalid_data = json!({
            "age": "not a number"
        });
        assert!(validator.validate("test", &invalid_data).is_err());
    }

    #[test]
    fn test_field_type_validation() {
        let validator = SchemaValidator::new();
        
        // Test string validation
        let string_value = json!("test");
        assert!(validator.validate_field_type(&string_value, "string").is_ok());
        assert!(validator.validate_field_type(&string_value, "number").is_err());

        // Test number validation
        let number_value = json!(42);
        assert!(validator.validate_field_type(&number_value, "number").is_ok());
        assert!(validator.validate_field_type(&number_value, "string").is_err());
    }
}
