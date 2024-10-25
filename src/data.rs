use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;
use log::{info, error};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub amount: f64,
    pub flagged: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub balance: f64,
    pub alert: bool,
}

pub fn load_data(file_path: &str) -> Result<Vec<u8>, String> {
    fs::read(file_path).map_err(|e| e.to_string())
}

pub fn process_data(data: Vec<u8>) -> Result<(), String> {
    // Deserialize data
    let transactions: Vec<Transaction> = serde_json::from_slice(&data).map_err(|e| e.to_string())?;
    
    // Process each transaction
    for transaction in transactions {
        if transaction.amount > 10000.0 {
            info!("High value transaction detected: {:?}", transaction);
        }
    }

    Ok(())
}

pub fn save_data<T: Serialize>(data: &T, file_path: &str) -> Result<(), String> {
    let serialized_data = serde_json::to_vec(data).map_err(|e| e.to_string())?;
    fs::write(file_path, serialized_data).map_err(|e| e.to_string())
}