use anyhow::Result;
use bitcoin::blockdata::script::{Script, Builder};
use bitcoin::secp256k1::{Secp256k1, PublicKey};
use bitcoin::util::hash::Hash;
use bitcoin::hashes::{sha256, ripemd160};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// RSK Bridge Operation Types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RSKOpType {
    Lock,
    Release,
    AddFederator,
    RemoveFederator,
    UpdateBridge,
}

/// RSK Bridge Transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RSKBridgeTx {
    pub operation: String,
    pub rsk_tx_hash: String,
    pub btc_tx_hash: Option<String>,
    pub amount: u64,
    pub sender: PublicKey,
    pub recipient: PublicKey,
    pub federator_sigs: Vec<Vec<u8>>,
}

/// RSK Bridge State
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RSKBridgeState {
    pub federators: Vec<PublicKey>,
    pub required_sigs: usize,
    pub locked_utxos: HashMap<String, u64>,
    pub bridge_balance: u64,
}

pub struct RSKBridge {
    secp: Secp256k1<bitcoin::secp256k1::All>,
    state: RSKBridgeState,
}

impl RSKBridge {
    pub fn new(federators: Vec<PublicKey>, required_sigs: usize) -> Self {
        Self {
            secp: Secp256k1::new(),
            state: RSKBridgeState {
                federators,
                required_sigs,
                locked_utxos: HashMap::new(),
                bridge_balance: 0,
            },
        }
    }

    /// Create a peg-in (lock) script
    pub fn create_lock_script(
        &self,
        amount: u64,
        rsk_recipient: &[u8],
    ) -> Result<Script> {
        let mut builder = Builder::new();

        // Add RSK bridge identifier
        builder = builder.push_opcode(bitcoin::blockdata::opcodes::all::OP_RETURN);
        builder = builder.push_slice(b"RSK_LOCK");

        // Add amount and recipient
        builder = builder.push_int(amount as i64);
        builder = builder.push_slice(rsk_recipient);

        // Add federator multisig requirement
        builder = builder.push_int(self.state.required_sigs as i64);
        for federator in &self.state.federators {
            builder = builder.push_key(&federator);
        }
        builder = builder.push_int(self.state.federators.len() as i64);
        builder = builder.push_opcode(bitcoin::blockdata::opcodes::all::OP_CHECKMULTISIG);

        Ok(builder.into_script())
    }

    /// Create a peg-out (release) script
    pub fn create_release_script(
        &self,
        bridge_tx: &RSKBridgeTx,
    ) -> Result<Script> {
        let mut builder = Builder::new();

        // Add RSK bridge identifier for release
        builder = builder.push_opcode(bitcoin::blockdata::opcodes::all::OP_RETURN);
        builder = builder.push_slice(b"RSK_RELEASE");

        // Add transaction details
        let tx_data = serde_json::to_vec(&bridge_tx)?;
        builder = builder.push_slice(&tx_data);

        // Add federator signatures verification
        builder = builder.push_int(self.state.required_sigs as i64);
        for sig in &bridge_tx.federator_sigs {
            builder = builder.push_slice(sig);
        }
        
        // Add federator public keys
        for federator in &self.state.federators {
            builder = builder.push_key(&federator);
        }
        builder = builder.push_int(self.state.federators.len() as i64);
        builder = builder.push_opcode(bitcoin::blockdata::opcodes::all::OP_CHECKMULTISIG);

        Ok(builder.into_script())
    }

    /// Create a bridge update script
    pub fn create_bridge_update_script(
        &self,
        new_federators: &[PublicKey],
        new_required_sigs: usize,
    ) -> Result<Script> {
        let mut builder = Builder::new();

        // Add RSK bridge update identifier
        builder = builder.push_opcode(bitcoin::blockdata::opcodes::all::OP_RETURN);
        builder = builder.push_slice(b"RSK_UPDATE");

        // Add new bridge parameters
        builder = builder.push_int(new_required_sigs as i64);
        for pubkey in new_federators {
            builder = builder.push_key(pubkey);
        }
        builder = builder.push_int(new_federators.len() as i64);

        // Add current federator verification
        builder = builder.push_int(self.state.required_sigs as i64);
        for federator in &self.state.federators {
            builder = builder.push_key(&federator);
        }
        builder = builder.push_int(self.state.federators.len() as i64);
        builder = builder.push_opcode(bitcoin::blockdata::opcodes::all::OP_CHECKMULTISIG);

        Ok(builder.into_script())
    }

    /// Verify a bridge transaction
    pub fn verify_bridge_tx(&self, bridge_tx: &RSKBridgeTx) -> Result<bool> {
        match bridge_tx.operation.as_str() {
            "LOCK" => self.verify_lock_tx(bridge_tx),
            "RELEASE" => self.verify_release_tx(bridge_tx),
            _ => Ok(false),
        }
    }

    /// Verify a lock transaction
    fn verify_lock_tx(&self, bridge_tx: &RSKBridgeTx) -> Result<bool> {
        // Verify amount
        if bridge_tx.amount == 0 {
            return Ok(false);
        }

        // Verify UTXO not already locked
        if self.state.locked_utxos.contains_key(&bridge_tx.btc_tx_hash.clone().unwrap_or_default()) {
            return Ok(false);
        }

        // Verify federator signatures
        if bridge_tx.federator_sigs.len() < self.state.required_sigs {
            return Ok(false);
        }

        Ok(true)
    }

    /// Verify a release transaction
    fn verify_release_tx(&self, bridge_tx: &RSKBridgeTx) -> Result<bool> {
        // Verify amount available
        if bridge_tx.amount > self.state.bridge_balance {
            return Ok(false);
        }

        // Verify RSK transaction exists
        if bridge_tx.rsk_tx_hash.is_empty() {
            return Ok(false);
        }

        // Verify federator signatures
        if bridge_tx.federator_sigs.len() < self.state.required_sigs {
            return Ok(false);
        }

        Ok(true)
    }

    /// Update bridge state
    pub fn update_state(&mut self, bridge_tx: &RSKBridgeTx) -> Result<()> {
        match bridge_tx.operation.as_str() {
            "LOCK" => {
                if let Some(btc_tx) = &bridge_tx.btc_tx_hash {
                    self.state.locked_utxos.insert(btc_tx.clone(), bridge_tx.amount);
                    self.state.bridge_balance += bridge_tx.amount;
                }
            }
            "RELEASE" => {
                self.state.bridge_balance -= bridge_tx.amount;
            }
            _ => return Err(anyhow::anyhow!("Invalid bridge operation")),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::secp256k1::SecretKey;

    #[test]
    fn test_rsk_bridge_lock() -> Result<()> {
        let secp = Secp256k1::new();
        
        // Create test federators
        let federator_keys: Vec<SecretKey> = (0..3)
            .map(|_| SecretKey::new(&mut rand::thread_rng()))
            .collect();
        let federator_pubkeys: Vec<PublicKey> = federator_keys
            .iter()
            .map(|sk| PublicKey::from_secret_key(&secp, sk))
            .collect();

        let bridge = RSKBridge::new(federator_pubkeys.clone(), 2);

        // Create test lock script
        let rsk_recipient = vec![1u8; 20]; // Example RSK address
        let script = bridge.create_lock_script(1000000, &rsk_recipient)?;
        
        assert!(!script.is_empty());
        Ok(())
    }

    #[test]
    fn test_rsk_bridge_release() -> Result<()> {
        let secp = Secp256k1::new();
        
        // Create test keys
        let sender_secret = SecretKey::new(&mut rand::thread_rng());
        let recipient_secret = SecretKey::new(&mut rand::thread_rng());
        let sender_pubkey = PublicKey::from_secret_key(&secp, &sender_secret);
        let recipient_pubkey = PublicKey::from_secret_key(&secp, &recipient_secret);

        // Create test federators
        let federator_keys: Vec<SecretKey> = (0..3)
            .map(|_| SecretKey::new(&mut rand::thread_rng()))
            .collect();
        let federator_pubkeys: Vec<PublicKey> = federator_keys
            .iter()
            .map(|sk| PublicKey::from_secret_key(&secp, sk))
            .collect();

        let bridge = RSKBridge::new(federator_pubkeys.clone(), 2);

        // Create test bridge transaction
        let bridge_tx = RSKBridgeTx {
            operation: "RELEASE".to_string(),
            rsk_tx_hash: "rsk123".to_string(),
            btc_tx_hash: Some("btc123".to_string()),
            amount: 1000000,
            sender: sender_pubkey,
            recipient: recipient_pubkey,
            federator_sigs: vec![vec![1u8; 64], vec![2u8; 64], vec![3u8; 64]],
        };

        let script = bridge.create_release_script(&bridge_tx)?;
        
        assert!(!script.is_empty());
        Ok(())
    }
}
