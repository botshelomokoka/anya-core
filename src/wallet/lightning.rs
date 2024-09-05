//! This module provides Lightning Network functionality for the Anya project.

use std::error::Error;
use std::str::FromStr;
use serde_json::json;
use bitcoin::{Transaction, Network};
use bitcoin::util::psbt::PartiallySignedTransaction;
use lightning::ln::chan_utils::ChannelPublicKeys;
use lightning::ln::peer_handler::ErroringMessageHandler;
use lightning::ln::msgs::{ChannelMessageHandler, ChannelUpdate};
use lightning::util::events::{Event, EventHandler};
use lightning_invoice::Invoice;
use secp256k1::PublicKey;
use rust_lightning::ln::channelmanager::{ChannelManager, PaymentStatus};
use rust_lightning::chain::chaininterface::FeeEstimator;
use rust_lightning::util::config::UserConfig;
use rust_lightning::util::logger::Logger;
use rust_lightning::util::persist::DummyPersister;
use rust_lightning::routing::router::RouteParameters;
use rust_dlc::{Oracle, Contract, Outcome, DlcParty, OracleInfo, ContractDescriptor, PayoutFunction};
use stacks_common::types::StacksAddress;
use stacks_transactions::{TransactionSigner, TransactionVersion, PostConditionMode, StacksTransaction};
use stacks_transactions::account::AccountSpendingConditionSigner;
use stacks_transactions::transaction_signing::TransactionSigning;
use libp2p::{PeerId, Swarm, Transport};
use libp2p::core::upgrade;
use libp2p::tcp::TokioTcpConfig;
use libp2p::mplex::MplexConfig;
use libp2p::noise::{Keypair, NoiseConfig, X25519Spec};
use web5::{did::{DID, DIDDocument}, dids::methods::key::DIDKey};
use web5::credentials::{Credential, VerifiableCredential};

use crate::anya_core::network::bitcoin_client;
use crate::anya_core::network::stacks_client;
use crate::anya_core::network::dlc_client;
use crate::anya_core::utils::log_event;
use crate::anya_core::wallet::Wallet;

pub struct LightningClient {
    node: ChannelManager<ErroringMessageHandler, ChannelMessageHandler, EventHandler>,
    network: Network,
    stx_address: StacksAddress,
    dlc_oracle: Oracle,
    p2p_swarm: Swarm<libp2p::swarm::behaviour::Behaviour>,
    did: DID,
}

impl LightningClient {
    pub fn new(network: Network, stx_address: StacksAddress, dlc_oracle: Oracle, p2p_swarm: Swarm<libp2p::swarm::behaviour::Behaviour>) -> Result<Self, Box<dyn Error>> {
        let config = UserConfig::default();
        let logger = Logger::new();
        let fee_estimator = FeeEstimator::new_static(2000);
        let persister = DummyPersister;
        let node = ChannelManager::new(config, &network, &logger, fee_estimator, persister)?;
        let did = DIDKey::generate().unwrap();
        Ok(LightningClient { node, network, stx_address, dlc_oracle, p2p_swarm, did })
    }

    pub fn open_channel(&self, node_pubkey: &str, capacity: u64, push_msat: u64) -> Result<ChannelPublicKeys, Box<dyn Error>> {
        let pubkey = PublicKey::from_str(node_pubkey)?;
        let channel_value_satoshis = capacity;
        let push_msat = Some(push_msat);
        let user_channel_id = 0; // You might want to generate a unique ID here
        
        let (channel, _, _) = self.node.create_channel(pubkey, channel_value_satoshis, push_msat, user_channel_id)?;
        Ok(channel.get_channel_public_keys())
    }

    pub fn send_payment(&self, invoice: &Invoice, amount_msat: Option<u64>) -> Result<[u8; 32], Box<dyn Error>> {
        let payment_hash = invoice.payment_hash().clone();
        let payment_secret = invoice.payment_secret().cloned();
        let route_params = RouteParameters::from_payment_params_and_value(
            invoice.payment_params().clone(),
            amount_msat.unwrap_or(invoice.amount_milli_satoshis().unwrap_or(0)),
        );

        let payment_id = self.node.send_payment(payment_hash, payment_secret, route_params)?;
        
        // Wait for payment to complete
        loop {
            match self.node.get_payment_status(payment_id) {
                Some(PaymentStatus::Succeeded) => {
                    return Ok(payment_hash.into_inner());
                }
                Some(PaymentStatus::Failed(_)) => {
                    return Err("Payment failed".into());
                }
                _ => std::thread::sleep(std::time::Duration::from_millis(100)),
            }
        }
    }

    pub fn decode_invoice(&self, invoice: &str) -> Result<Invoice, Box<dyn Error>> {
        Invoice::from_str(invoice).map_err(|e| e.into())
    }

    pub fn get_stx_address(&self) -> &StacksAddress {
        &self.stx_address
    }

    pub fn create_dlc_contract(&self, counterparty: &str, outcome_map: Vec<(Outcome, u64)>) -> Result<Contract, Box<dyn Error>> {
        let oracle_info = OracleInfo::new(self.dlc_oracle.clone());
        let contract_descriptor = ContractDescriptor::new(outcome_map.clone());
        let payout_function = PayoutFunction::new(outcome_map);
        
        let dlc_party = DlcParty::new(self.network, oracle_info, contract_descriptor, payout_function);
        let contract = dlc_party.create_contract(counterparty)?;
        
        Ok(contract)
    }

    pub fn connect_to_peer(&mut self, peer_id: PeerId) -> Result<(), Box<dyn Error>> {
        self.p2p_swarm.dial(peer_id)?;
        Ok(())
    }

    pub fn get_did(&self) -> &DID {
        &self.did
    }

    pub fn create_verifiable_credential(&self, subject: &str, claims: serde_json::Value) -> Result<VerifiableCredential, Box<dyn Error>> {
        let credential = Credential::new(subject, claims);
        let vc = VerifiableCredential::issue(&credential, &self.did)?;
        Ok(vc)
    }
}

pub fn open_channel(wallet: &mut Wallet, node_pubkey: &str, capacity: u64, push_msat: u64) -> Result<String, Box<dyn Error>> {
    if !wallet.is_action_authorized("open_channel", &json!({
        "node_pubkey": node_pubkey,
        "capacity": capacity
    })) {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Channel opening not authorized")));
    }

    if wallet.get_balance() < capacity {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Insufficient funds to open channel")));
    }

    let utxos = wallet.select_utxos_for_payment(capacity);
    let lightning_client = wallet.get_lightning_client()?;
    let channel_keys = lightning_client.open_channel(node_pubkey, capacity, push_msat)?;

    let funding_output = (channel_keys, capacity);
    let funding_tx = wallet.create_transaction(utxos, vec![funding_output], wallet.get_private_keys_for_inputs(&utxos))?;

    let funding_txid = bitcoin_client::broadcast_transaction(&funding_tx)?;

    let channel_id = channel_keys.funding_pubkey.to_string();

    wallet.add_channel(&channel_id, node_pubkey, capacity, capacity - push_msat);

    log_event("channel_opened", &json!({
        "channel_id": channel_id,
        "node_pubkey": node_pubkey,
        "capacity": capacity,
        "push_msat": push_msat
    }));

    Ok(channel_id)
}

pub fn send_payment(wallet: &mut Wallet, invoice: &str, amount_msat: Option<u64>) -> Result<[u8; 32], Box<dyn Error>> {
    let lightning_client = wallet.get_lightning_client()?;
    let invoice_obj = lightning_client.decode_invoice(invoice)?;

    if let Some(amt) = amount_msat {
        if amt != invoice_obj.amount_milli_satoshis().unwrap_or(0) {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Payment amount doesn't match invoice amount")));
        }
    }

    if !wallet.is_action_authorized("send_payment", &json!({
        "invoice": invoice,
        "amount_msat": amount_msat
    })) {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Payment not authorized")));
    }

    let payment_preimage = lightning_client.send_payment(&invoice_obj, amount_msat)?;

    wallet.update_channel_balances_after_payment(&payment_preimage);

    log_event("payment_sent", &json!({
        "invoice": invoice,
        "amount_msat": amount_msat
    }));

    Ok(payment_preimage)
}

pub fn create_stx_transaction(wallet: &mut Wallet, recipient: &StacksAddress, amount: u64) -> Result<StacksTransaction, Box<dyn Error>> {
    let stx_client = stacks_client::new_client()?;
    let sender_address = wallet.get_lightning_client()?.get_stx_address().clone();
    
    let tx = StacksTransaction::new(
        TransactionVersion::Testnet,
        wallet.get_stx_account(),
        TransactionSigner::from_p2pkh(&sender_address)?,
        recipient.clone(),
        amount,
        0,
        PostConditionMode::Allow,
    );

    let signed_tx = wallet.sign_stx_transaction(tx)?;

    Ok(signed_tx)
}

pub fn create_dlc_contract(wallet: &mut Wallet, counterparty: &str, outcome_map: Vec<(Outcome, u64)>) -> Result<Contract, Box<dyn Error>> {
    let lightning_client = wallet.get_lightning_client()?;
    lightning_client.create_dlc_contract(counterparty, outcome_map)
}

pub fn connect_to_peer(wallet: &mut Wallet, peer_id: PeerId) -> Result<(), Box<dyn Error>> {
    let mut lightning_client = wallet.get_lightning_client_mut()?;
    lightning_client.connect_to_peer(peer_id)
}

pub fn create_verifiable_credential(wallet: &mut Wallet, subject: &str, claims: serde_json::Value) -> Result<VerifiableCredential, Box<dyn Error>> {
    let lightning_client = wallet.get_lightning_client()?;
    lightning_client.create_verifiable_credential(subject, claims)
}

// Additional Lightning Network functions can be implemented here
