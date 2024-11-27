//! Module documentation for $moduleName
//!
//! # Overview
//! This module is part of the Anya Core project, located at $modulePath.
//!
//! # Architecture
//! [Add module-specific architecture details]
//!
//! # API Reference
//! [Document public functions and types]
//!
//! # Usage Examples
//! `ust
//! // Add usage examples
//! `
//!
//! # Error Handling
//! This module uses proper error handling with Result types.
//!
//! # Security Considerations
//! [Document security features and considerations]
//!
//! # Performance
//! [Document performance characteristics]

use std::error::Error;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct CodeFeatureExtractor {
    patterns: HashMap<String, Regex>,
    weights: HashMap<String, f64>,
}

impl CodeFeatureExtractor {
    pub fn new() -> Self  -> Result<(), Box<dyn Error>> {
        let mut patterns = HashMap::new();
        let mut weights = HashMap::new();

        // Bitcoin patterns
        patterns.insert("bitcoin_imports".to_string(), 
            Regex::new(r"use\s+bitcoin::")?);
        weights.insert("bitcoin_imports".to_string(), 1.0);

        // Lightning patterns
        patterns.insert("lightning_imports".to_string(),
            Regex::new(r"use\s+lightning::")?);
        weights.insert("lightning_imports".to_string(), 1.0);

        // Security patterns
        patterns.insert("crypto_operations".to_string(),
            Regex::new(r"encrypt|decrypt|sign|verify")?);
        weights.insert("crypto_operations".to_string(), 1.5);

        // Smart contract patterns
        patterns.insert("smart_contracts".to_string(),
            Regex::new(r"Script|Witness|Taproot")?);
        weights.insert("smart_contracts".to_string(), 1.2);

        Self { patterns, weights }
    }

    pub fn extract_weighted_features(&self, content: &str) -> HashMap<String, f64>  -> Result<(), Box<dyn Error>> {
        let mut features = HashMap::new();
        
        for (name, pattern) in &self.patterns {
            let count = pattern.find_iter(content).count() as f64;
            let weight = self.weights.get(name).unwrap_or(&1.0);
            features.insert(name.clone(), count * weight);
        }
        
        features
    }
}


