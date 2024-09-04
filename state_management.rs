use serde_json;
use schnorr;
use log;
use std::collections::HashMap;

pub struct Node {
    dao_progress: f64,
    network_state: HashMap<String, serde_json::Value>,
    user_data: HashMap<String, serde_json::Value>,
    federated_nodes: Vec<String>,
    schnorr_keypair: schnorr::KeyPair,
}

impl Node {
    pub fn new() -> Self {
        log::set_max_level(log::LevelFilter::Info);
        Node {
            dao_progress: 0.0,
            network_state: HashMap::new(),
            user_data: HashMap::new(),
            federated_nodes: Vec::new(),
            schnorr_keypair: schnorr::generate_keypair(),
        }
    }

    pub fn merge_state(&mut self, remote_state: &serde_json::Value, remote_node_pubkey: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let signature = remote_state["signature"].as_str().ok_or("Missing signature")?;
        if !self.verify_signature(signature, remote_state, remote_node_pubkey)? {
            return Err("Invalid signature".into());
        }

        for (key, value) in remote_state.as_object().unwrap() {
            if !self.network_state.contains_key(key) {
                continue;
            }

            if value.is_object() {
                // Recursive merge for nested objects
                if let Some(local_value) = self.network_state.get_mut(key) {
                    self.merge_state(value, remote_node_pubkey)?;
                }
            } else {
                self.network_state.insert(key.clone(), value.clone());
            }
        }

        Ok(())
    }

    fn verify_signature(&self, signature: &str, data: &serde_json::Value, pubkey: &[u8]) -> Result<bool, Box<dyn std::error::Error>> {
        let serialized_data = serde_json::to_string(data)?;
        Ok(schnorr::verify(signature, serialized_data.as_bytes(), pubkey))
    }

    pub fn get_state(&self) -> serde_json::Value {
        let mut state = serde_json::Map::new();
        for (key, value) in self.network_state.iter() {
            if key != "federated_nodes" && key != "schnorr_keypair" {
                state.insert(key.clone(), value.clone());
            }
        }
        serde_json::Value::Object(state)
    }

    pub fn sign_state(&self) -> Result<String, Box<dyn std::error::Error>> {
        let serialized_state = serde_json::to_string(&self.get_state())?;
        Ok(schnorr::sign(serialized_state.as_bytes(), &self.schnorr_keypair.private_key))
    }
}
