//! This module provides PayNym functionality for Anya Wallet.

use bitcoin::{Address, Network};
use std::error::Error;
use std::collections::HashMap;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha2::{Sha256, Digest};
use bech32::{ToBase32, Variant};
use rand::thread_rng;
use thiserror::Error;

// Import from other Anya modules
use crate::network::bitcoin_client;
use crate::wallet::address_management::generate_new_address;
use crate::wallet::Wallet;

#[derive(Error, Debug)]
pub enum PayNymError {
    #[error("Bech32 encoding error: {0}")]
    Bech32Error(#[from] bech32::Error),
    #[error("Secp256k1 error: {0}")]
    Secp256k1Error(#[from] secp256k1::Error),
    #[error("Bitcoin error: {0}")]
    BitcoinError(#[from] bitcoin::util::address::Error),
    #[error("PayNym not found in contacts")]
    PayNymNotFound,
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Transaction creation failed: {0}")]
    TransactionCreationFailed(String),
}

pub struct PayNymClient {
    secret_key: SecretKey,
    public_key: PublicKey,
    paynym: String,
    contacts: HashMap<String, Address>,
}

impl PayNymClient {
    pub fn new() -> Result<Self, PayNymError> {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::new(&mut thread_rng());
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        
        let paynym = Self::generate_paynym(&public_key)?;
        
        Ok(PayNymClient {
            secret_key,
            public_key,
            paynym,
            contacts: HashMap::new(),
        })
    }

    fn generate_paynym(public_key: &PublicKey) -> Result<String, PayNymError> {
        let mut hasher = Sha256::new();
        hasher.update(public_key.serialize());
        let result = hasher.finalize();
        
        let paynym = bech32::encode("pm", result[..20].to_base32(), Variant::Bech32)?;
        Ok(paynym)
    }

    pub fn get_paynym(&self) -> &str {
        &self.paynym
    }

    pub fn add_contact(&mut self, paynym: String, address: Address) {
        self.contacts.insert(paynym, address);
    }

    pub fn get_contact_address(&self, paynym: &str) -> Option<&Address> {
        self.contacts.get(paynym)
    }
}

pub fn register_paynym(wallet: &mut Wallet) -> Result<String, PayNymError> {
    let paynym_client = PayNymClient::new()?;
    let new_paynym = paynym_client.get_paynym().to_string();

    let address = generate_new_address(wallet, Network::Bitcoin)?;
    
    wallet.add_paynym(&new_paynym, &address)?;

    Ok(new_paynym)
}

pub fn resolve_paynym(wallet: &Wallet, paynym: &str) -> Result<Address, PayNymError> {
    wallet.get_paynym_client().get_contact_address(paynym)
        .cloned()
        .ok_or(PayNymError::PayNymNotFound)
}

pub fn send_to_paynym(wallet: &mut Wallet, paynym: &str, amount: u64) -> Result<String, PayNymError> {
    let address = resolve_paynym(wallet, paynym)?;

    let utxos = wallet.select_utxos_for_payment(amount)
        .map_err(|e| PayNymError::InsufficientFunds)?;
    let private_keys = wallet.get_private_keys_for_inputs(&utxos)
        .map_err(|e| PayNymError::TransactionCreationFailed(e.to_string()))?;
    
    let tx = wallet.create_transaction(
        utxos,
        vec![(address, amount)],
        &private_keys
    ).map_err(|e| PayNymError::TransactionCreationFailed(e.to_string()))?;

    bitcoin_client::broadcast_transaction(&tx)
        .map_err(|e| PayNymError::TransactionCreationFailed(e.to_string()))
}

pub fn add_paynym_contact(wallet: &mut Wallet, paynym: &str, address: Address) -> Result<(), PayNymError> {
    wallet.get_paynym_client_mut().add_contact(paynym.to_string(), address);
    Ok(())
}

pub fn get_paynym_contacts(wallet: &Wallet) -> &HashMap<String, Address> {
    &wallet.get_paynym_client().contacts
}

impl Wallet {
    pub fn get_paynym_client(&self) -> &PayNymClient {
        &self.paynym_client
    }

    pub fn get_paynym_client_mut(&mut self) -> &mut PayNymClient {
        &mut self.paynym_client
    }

    pub fn add_paynym(&mut self, paynym: &str, address: &Address) -> Result<(), PayNymError> {
        self.get_paynym_client_mut().add_contact(paynym.to_string(), address.clone());
        Ok(())
    }
}
