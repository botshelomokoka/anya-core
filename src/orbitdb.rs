use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrbitDBError {
    #[error("Database error: {0}")]
    DatabaseError(String),
}

/// The OrbitDB struct represents a database instance.
pub struct OrbitDB {
    data: HashMap<String, String>,
}

impl OrbitDB {
    // Note: This function cannot be marked as `const` because `HashMap::new()` involves heap allocation.
    pub fn new() -> Result<Self, OrbitDBError> {
        Ok(Self {
            data: HashMap::new(),
        })
    }

    pub fn insert(&mut self, key: &str, value: &str) -> Result<(), OrbitDBError> {
        self.data.insert(key.to_string(), value.to_string());
        Ok(())
    }

    pub fn query(&self, query: &str) -> Result<Vec<String>, OrbitDBError> {
        // For simplicity, this example just returns values that contain the query string.
        let results: Vec<String> = self.data
            .values()
            .filter(|&value| value.contains(query))
            .cloned()
            .collect();
    
        if results.is_empty() {
            Err(OrbitDBError::DatabaseError("No matching records found".to_string()))
        } else {
            Ok(results)
        }
    }
}

fn main() {
    let mut db = OrbitDB::new().expect("Failed to create OrbitDB instance");

    db.insert("key1", "value1").expect("Failed to insert data");
    db.insert("key2", "value2").expect("Failed to insert data");

    match db.query("value") {
        Ok(results) => println!("Query results: {:?}", results),
        Err(e) => println!("Error querying database: {}", e),
    }
}