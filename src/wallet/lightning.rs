//! This module provides Lightning Network functionality for the Anya project.

use std::error::Error;
use std::str::FromStr;
use serde_json::json;
use bitcoin::Transaction;
use bitcoin::util::psbt::PartiallySignedTransaction;
use lightning::ln::chan_utils::ChannelPublicKeys;
use lightning::ln::msgs::ChannelMessageHandler;
use lightning::ln::peer_handler::ErroringMessageHandler;
use lightning::util::events::{Event, EventHandler};
use lightning_invoice::Invoice;
use secp256k1::PublicKey;

use crate::anya_core::network::bitcoin_client;
use crate::anya_core::utils::log_event;
use crate::anya_core::wallet::Wallet;

pub struct LightningClient {
    node: lightning::ln::Node<ErroringMessageHandler, ChannelMessageHandler, EventHandler>,
}

impl LightningClient {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let config = lightning::util::config::UserConfig::default();
        let network = bitcoin::Network::Bitcoin;
        let logger = lightning::util::logger::Logger::new();
        let fee_estimator = lightning::chain::chaininterface::FeeEstimator::new_static(2000);
        let persister = lightning::util::persist::DummyPersister;
        let node = lightning::ln::Node::new(config, &network, &logger, fee_estimator, persister)?;
        Ok(LightningClient { node })
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
        let route_params = lightning::routing::router::RouteParameters::from_payment_params_and_value(
            invoice.payment_params().clone(),
            amount_msat.unwrap_or(invoice.amount_milli_satoshis().unwrap_or(0)),
        );

        let payment_id = self.node.send_payment(payment_hash, payment_secret, route_params)?;
        
        // Wait for payment to complete
        loop {
            match self.node.get_payment_status(payment_id) {
                Some(lightning::ln::channelmanager::PaymentStatus::Succeeded) => {
                    return Ok(payment_hash.into_inner());
                }
                Some(lightning::ln::channelmanager::PaymentStatus::Failed(_)) => {
                    return Err("Payment failed".into());
                }
                _ => std::thread::sleep(std::time::Duration::from_millis(100)),
            }
        }
    }

    pub fn decode_invoice(&self, invoice: &str) -> Result<Invoice, Box<dyn Error>> {
        Invoice::from_str(invoice).map_err(|e| e.into())
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
    let lightning_client = LightningClient::new()?;
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
    let lightning_client = LightningClient::new()?;
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

// Additional Lightning Network functions can be implemented here
