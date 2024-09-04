//! This module provides PayNym functionality for Anya Wallet.

use bitcoin::Address;
use std::error::Error;
use std::collections::HashMap;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha2::{Sha256, Digest};
use bech32::{ToBase32, Variant};

// Import from other Anya modules
use crate::anya_core::network::bitcoin_client;
use crate::anya_core::wallet::address_management::generate_new_address;

pub struct PayNymClient {
    secret_key: SecretKey,
    public_key: PublicKey,
    paynym: String,
    contacts: HashMap<String, Address>,
}

impl PayNymClient {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::new(&mut rand::thread_rng());
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        
        let paynym = Self::generate_paynym(&public_key)?;
        
        Ok(PayNymClient {
            secret_key,
            public_key,
            paynym,
            contacts: HashMap::new(),
        })
    }

    fn generate_paynym(public_key: &PublicKey) -> Result<String, Box<dyn Error>> {
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

pub fn register_paynym(wallet: &mut Wallet) -> Result<String, Box<dyn Error>> {
    /// Registers a new PayNym for the user's wallet.
    ///
    /// # Arguments
    ///
    /// * `wallet` - The Anya Wallet object.
    ///
    /// # Returns
    ///
    /// The registered PayNym if successful, or an error if there's a problem.

    let paynym_client = PayNymClient::new()?;
    let new_paynym = paynym_client.get_paynym().to_string();

    // Associate the PayNym with a Bitcoin address from the wallet
    let address = wallet.get_next_available_address()?;
    
    // Store the PayNym in the wallet's data
    wallet.add_paynym(&new_paynym, &address)?;

    Ok(new_paynym)
}

pub fn resolve_paynym(wallet: &Wallet, paynym: &str) -> Result<Address, Box<dyn Error>> {
    /// Resolves a PayNym to its associated Bitcoin address.
    ///
    /// # Arguments
    ///
    /// * `wallet` - The Anya Wallet object
    /// * `paynym` - The PayNym to resolve
    ///
    /// # Returns
    ///
    /// The Bitcoin address associated with the PayNym, or an error if not found.

    if let Some(address) = wallet.get_paynym_client().get_contact_address(paynym) {
        Ok(address.clone())
    } else {
        Err("PayNym not found in contacts".into())
    }
}

pub fn send_to_paynym(wallet: &mut Wallet, paynym: &str, amount: u64) -> Result<String, Box<dyn Error>> {
    /// Sends a Bitcoin payment to a PayNym.
    ///
    /// # Arguments
    ///
    /// * `wallet` - The Anya Wallet object
    /// * `paynym` - The PayNym to send the payment to
    /// * `amount` - The amount of Bitcoin to send (in satoshis)
    ///
    /// # Returns
    ///
    /// The transaction ID if successful, or an error if there's a problem.

    // Resolve the PayNym to a Bitcoin address
    let address = resolve_paynym(wallet, paynym)?;

    // Create and send a Bitcoin transaction using the wallet
    let utxos = wallet.select_utxos_for_payment(amount)?;
    let private_keys = wallet.get_private_keys_for_inputs(&utxos)?;
    
    let tx = wallet.create_transaction(
        utxos,
        vec![(address, amount)],
        &private_keys
    )?;

    let txid = bitcoin_client::broadcast_transaction(&tx.serialize().hex())?;

    Ok(txid)
}

pub fn add_paynym_contact(wallet: &mut Wallet, paynym: &str, address: Address) -> Result<(), Box<dyn Error>> {
    /// Adds a PayNym contact to the wallet.
    ///
    /// # Arguments
    ///
    /// * `wallet` - The Anya Wallet object
    /// * `paynym` - The PayNym of the contact
    /// * `address` - The Bitcoin address associated with the PayNym
    ///
    /// # Returns
    ///
    /// Ok(()) if successful, or an error if there's a problem.

    wallet.get_paynym_client_mut().add_contact(paynym.to_string(), address);
    Ok(())
}

pub fn get_paynym_contacts(wallet: &Wallet) -> &HashMap<String, Address> {
    /// Gets all PayNym contacts stored in the wallet.
    ///
    /// # Arguments
    ///
    /// * `wallet` - The Anya Wallet object
    ///
    /// # Returns
    ///
    /// A reference to the HashMap containing all PayNym contacts.

    &wallet.get_paynym_client().contacts
}

// Add these methods to your Wallet struct
impl Wallet {
    fn get_paynym_client(&self) -> &PayNymClient {
        &self.paynym_client
    }

    fn get_paynym_client_mut(&mut self) -> &mut PayNymClient {
        &mut self.paynym_client
    }

    fn add_paynym(&mut self, paynym: &str, address: &Address) -> Result<(), Box<dyn Error>> {
        self.get_paynym_client_mut().add_contact(paynym.to_string(), address.clone());
        Ok(())
    }
}
