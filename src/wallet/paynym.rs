//! This module provides PayNym functionality for Anya Wallet.

use bitcoin::{Address, Network};
use std::error::Error;
use std::collections::HashMap;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha2::{Sha256, Digest};
use bech32::{ToBase32, Variant};
use rand::thread_rng;
use thiserror::Error;
use stacks_common::types::{StacksAddress, StacksPublicKeyBuffer};
use stacks_common::util::hash::Hash160;
use stacks_transactions::{TransactionVersion, StacksTransaction, TransactionPayload, TransactionSigner};
use stacks_transactions::account::AccountSpendingConditionSigner;
use stacks_transactions::transaction_signing::TransactionSigning;
use rust_lightning::ln::msgs::ChannelUpdate;
use rust_lightning::ln::channelmanager::{ChannelManager, ChannelManagerReadArgs};
use rust_lightning::ln::peer_handler::{PeerManager, MessageHandler};
use rust_lightning::routing::router::Router;
use rust_lightning::util::events::EventHandler;
use rust_dlc::{self, DlcParty, OracleInfo, ContractDescriptor, PayoutFunction};
use rust_bitcoin::blockdata::transaction::Transaction as BitcoinTransaction;
use rust_bitcoin::network::constants::Network as BitcoinNetwork;
use libp2p::{PeerId, Swarm, Transport};
use libp2p::core::upgrade;
use libp2p::tcp::TokioTcpConfig;
use libp2p::mplex::MplexConfig;
use libp2p::noise::{Keypair, NoiseConfig, X25519Spec};
use web5::{did::{DID, DIDDocument}, dids::methods::key::DIDKey};

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
    #[error("Stacks error: {0}")]
    StacksError(String),
    #[error("Lightning error: {0}")]
    LightningError(String),
    #[error("DLC error: {0}")]
    DlcError(String),
    #[error("Libp2p error: {0}")]
    Libp2pError(String),
    #[error("Web5 error: {0}")]
    Web5Error(String),
}

pub struct PayNymClient {
    secret_key: SecretKey,
    public_key: PublicKey,
    paynym: String,
    contacts: HashMap<String, Address>,
    stx_address: StacksAddress,
    lightning_node_id: PublicKey,
    dlc_oracle_pubkey: PublicKey,
    peer_id: PeerId,
    did: DID,
}

impl PayNymClient {
    pub fn new() -> Result<Self, PayNymError> {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::new(&mut thread_rng());
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        
        let paynym = Self::generate_paynym(&public_key)?;
        
        let stx_address = StacksAddress::from_public_keys(
            0,
            &StacksPublicKeyBuffer::from_public_key(&public_key),
            1,
            StacksPublicKeyBuffer::from_public_key(&public_key),
            stacks_common::types::AddressHashMode::SerializeP2PKH,
        ).map_err(|e| PayNymError::StacksError(e.to_string()))?;

        let lightning_node_id = PublicKey::from_secret_key(&secp, &secret_key);
        let dlc_oracle_pubkey = PublicKey::from_secret_key(&secp, &secret_key);
        let peer_id = PeerId::from_public_key(&libp2p::core::PublicKey::Ed25519(
            libp2p::core::identity::ed25519::PublicKey::decode(&public_key.serialize()).unwrap()
        ));

        let did = DIDKey::new(&secret_key.serialize())
            .map_err(|e| PayNymError::Web5Error(e.to_string()))?;

        Ok(PayNymClient {
            secret_key,
            public_key,
            paynym,
            contacts: HashMap::new(),
            stx_address,
            lightning_node_id,
            dlc_oracle_pubkey,
            peer_id,
            did,
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

    pub fn get_stx_address(&self) -> &StacksAddress {
        &self.stx_address
    }

    pub fn get_lightning_node_id(&self) -> &PublicKey {
        &self.lightning_node_id
    }

    pub fn get_dlc_oracle_pubkey(&self) -> &PublicKey {
        &self.dlc_oracle_pubkey
    }

    pub fn get_peer_id(&self) -> &PeerId {
        &self.peer_id
    }

    pub fn get_did(&self) -> &DID {
        &self.did
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
        .map_err(|_| PayNymError::InsufficientFunds)?;
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

pub fn create_lightning_channel(wallet: &mut Wallet, paynym: &str, capacity: u64) -> Result<ChannelUpdate, PayNymError> {
    let peer_pubkey = wallet.get_paynym_client().get_contact_address(paynym)
        .ok_or(PayNymError::PayNymNotFound)?;
    
    let channel_manager = wallet.get_lightning_channel_manager()
        .map_err(|e| PayNymError::LightningError(e.to_string()))?;

    let channel_update = channel_manager.create_channel(peer_pubkey, capacity, 0)
        .map_err(|e| PayNymError::LightningError(e.to_string()))?;

    Ok(channel_update)
}

pub fn create_dlc_contract(wallet: &mut Wallet, paynym: &str, oracle: &str, outcome_map: HashMap<String, u64>) -> Result<rust_dlc::DlcTransaction, PayNymError> {
    let peer_pubkey = wallet.get_paynym_client().get_contact_address(paynym)
        .ok_or(PayNymError::PayNymNotFound)?;
    
    let oracle_info = OracleInfo::new(oracle.to_string(), wallet.get_paynym_client().get_dlc_oracle_pubkey().clone());
    
    let contract_descriptor = ContractDescriptor::new(
        outcome_map.iter().map(|(k, v)| (k.clone(), *v)).collect(),
        PayoutFunction::Winner
    );

    let dlc_manager = wallet.get_dlc_manager()
        .map_err(|e| PayNymError::DlcError(e.to_string()))?;

    let dlc_transaction = dlc_manager.create_contract(peer_pubkey, oracle_info, contract_descriptor)
        .map_err(|e| PayNymError::DlcError(e.to_string()))?;

    Ok(dlc_transaction)
}

pub fn connect_to_peer(wallet: &mut Wallet, peer_id: PeerId) -> Result<(), PayNymError> {
    let swarm = wallet.get_libp2p_swarm()
        .map_err(|e| PayNymError::Libp2pError(e.to_string()))?;

    swarm.dial(peer_id)
        .map_err(|e| PayNymError::Libp2pError(e.to_string()))?;

    Ok(())
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

    pub fn get_lightning_channel_manager(&self) -> Result<&ChannelManager, PayNymError> {
        self.lightning_channel_manager.as_ref()
            .ok_or_else(|| PayNymError::LightningError("Lightning channel manager not initialized".to_string()))
    }

    pub fn get_dlc_manager(&self) -> Result<&rust_dlc::DlcManager, PayNymError> {
        self.dlc_manager.as_ref()
            .ok_or_else(|| PayNymError::DlcError("DLC manager not initialized".to_string()))
    }

    pub fn get_libp2p_swarm(&self) -> Result<&Swarm<MplexConfig>, PayNymError> {
        self.libp2p_swarm.as_ref()
            .ok_or_else(|| PayNymError::Libp2pError("Libp2p swarm not initialized".to_string()))
    }
}
