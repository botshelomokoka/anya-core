use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct CodeFeatureExtractor {
    patterns: HashMap<String, Regex>,
    weights: HashMap<String, f64>,
}

impl CodeFeatureExtractor {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();
        let mut weights = HashMap::new();

        // Bitcoin patterns
        patterns.insert("bitcoin_imports".to_string(), 
            Regex::new(r"use\s+bitcoin::").unwrap());
        weights.insert("bitcoin_imports".to_string(), 1.0);

        // Lightning patterns
        patterns.insert("lightning_imports".to_string(),
            Regex::new(r"use\s+lightning::").unwrap());
        weights.insert("lightning_imports".to_string(), 1.0);

        // Security patterns
        patterns.insert("crypto_operations".to_string(),
            Regex::new(r"encrypt|decrypt|sign|verify").unwrap());
        weights.insert("crypto_operations".to_string(), 1.5);

        // Smart contract patterns
        patterns.insert("smart_contracts".to_string(),
            Regex::new(r"Script|Witness|Taproot").unwrap());
        weights.insert("smart_contracts".to_string(), 1.2);

        Self { patterns, weights }
    }

    pub fn extract_weighted_features(&self, content: &str) -> HashMap<String, f64> {
        let mut features = HashMap::new();
        
        for (name, pattern) in &self.patterns {
            let count = pattern.find_iter(content).count() as f64;
            let weight = self.weights.get(name).unwrap_or(&1.0);
            features.insert(name.clone(), count * weight);
        }
        
        features
    }
}
