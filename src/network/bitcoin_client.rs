//! This module provides a client interface for interacting with the Bitcoin network via RPC,
//! as well as integrations with Stacks, Web5, DLCs, Lightning Network, and libp2p.

use anyhow::{anyhow, Result};
use bitcoincore_rpc::{Auth, Client as BitcoinClient, RpcApi};
use bitcoin::{Address, BlockHash, Transaction, Txid, Network, PrivateKey, PublicKey};
use lightning::{
    ln::{channelmanager::ChannelManager, msgs::ChannelUpdate},
    routing::gossip::NodeId,
    util::config::UserConfig,
};
use dlc::{DlcTransaction, OracleInfo, Contract, Announcement, Attestation};
use libp2p::{
    Swarm, identity, PeerId, Multiaddr,
    core::transport::Transport,
    floodsub::{Floodsub, FloodsubEvent, Topic},
    mdns::{Mdns, MdnsEvent},
    swarm::NetworkBehaviourEventProcess
};
use stacks_blockchain::{
    chainstate::stacks::StacksAddress,
    clarity::vm::types::PrincipalData,
    clarity::vm::ClarityVersion,
    clarity::vm::clarity::ClarityInstance,
    clarity::vm::database::ClarityDatabase,
    clarity::vm::contexts::ContractContext,
    clarity::vm::types::Value as ClarityValue,
};
use web5::{
    did::{DID, DIDDocument, DIDMethod},
    dwn::{DWN, DataFormat, MessageReply, Permissions},
    credentials::{Credential, CredentialStatus, CredentialSubject, Issuer, Presentation}
};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

// Bitcoin RPC configuration
const RPC_USER: &str = "your_rpc_user";
const RPC_PASSWORD: &str = "your_rpc_password";
const RPC_HOST: &str = "localhost"; // Or your remote host
const RPC_PORT: u16 = 8332;

fn create_bitcoin_rpc_client() -> Result<BitcoinClient> {
    let rpc_url = format!("http://{}:{}", RPC_HOST, RPC_PORT);
    let auth = Auth::UserPass(RPC_USER.to_string(), RPC_PASSWORD.to_string());
    BitcoinClient::new(&rpc_url, auth).map_err(|e| anyhow!("Failed to create Bitcoin RPC client: {}", e))
}

/// Fetches unspent transaction outputs (UTXOs) for a given address.
pub fn get_utxos(address: &str) -> Result<Vec<HashMap<String, serde_json::Value>>> {
    let client = create_bitcoin_rpc_client()?;
    let utxos = client.list_unspent(None, None, Some(&[address]), None, None)?;
    
    Ok(utxos
        .into_iter()
        .map(|utxo| {
            let mut map = HashMap::new();
            map.insert("txid".to_string(), serde_json::Value::String(utxo.txid.to_string()));
            map.insert("vout".to_string(), serde_json::Value::Number(utxo.vout.into()));
            map.insert("value".to_string(), serde_json::Value::Number(utxo.amount.to_sat().into()));
            map
        })
        .collect())
}

/// Fetches the raw transaction data for a given transaction ID.
pub fn get_raw_transaction(txid: &str) -> Result<serde_json::Value> {
    let client = create_bitcoin_rpc_client()?;
    let tx = client.get_raw_transaction_verbose(&Txid::from_str(txid)?)?;
    Ok(serde_json::to_value(tx)?)
}

/// Broadcasts a raw transaction to the Bitcoin network
pub fn send_raw_transaction(tx_hex: &str) -> Result<String> {
    let client = create_bitcoin_rpc_client()?;
    let txid = client.send_raw_transaction(hex::decode(tx_hex)?)?;
    Ok(txid.to_string())
}

/// Estimates the fee rate for a transaction to be confirmed within the specified number of blocks.
pub fn estimate_fee(target_conf: u16) -> Result<f64> {
    let client = create_bitcoin_rpc_client()?;
    let fee_rate = client.estimate_smart_fee(target_conf, None)?;
    Ok(fee_rate.fee_rate.unwrap_or(0.0))
}

/// Fetches a block by its hash.
pub fn get_block(block_hash: &str) -> Result<serde_json::Value> {
    let client = create_bitcoin_rpc_client()?;
    let block_hash = BlockHash::from_str(block_hash)?;
    let block = client.get_block(&block_hash)?;
    Ok(serde_json::to_value(block)?)
}

/// Gets the current block count of the Bitcoin network.
pub fn get_block_count() -> Result<u64> {
    let client = create_bitcoin_rpc_client()?;
    Ok(client.get_block_count()?)
}

/// Fetches the balance of a given address.
pub fn get_address_balance(address: &str) -> Result<f64> {
    let client = create_bitcoin_rpc_client()?;
    let balance = client.get_received_by_address(
        &Address::from_str(address)?,
        Some(0)
    )?;
    Ok(balance.to_btc())
}

/// Validates a Bitcoin address.
pub fn validate_address(address: &str) -> Result<bool> {
    let client = create_bitcoin_rpc_client()?;
    let validation = client.validate_address(&Address::from_str(address)?)?;
    Ok(validation.is_valid)
}

// Stacks integration
pub struct StacksClient {
    clarity_instance: Arc<Mutex<ClarityInstance>>,
    network: Network,
}

impl StacksClient {
    pub fn new(network: Network) -> Result<Self> {
        let clarity_instance = Arc::new(Mutex::new(ClarityInstance::new(ClarityVersion::Clarity2, network.into())));
        Ok(Self { clarity_instance, network })
    }

    pub async fn get_stx_balance(&self, address: &StacksAddress) -> Result<u64> {
        let instance = self.clarity_instance.lock().await;
        let principal = PrincipalData::from(address.clone());
        let balance = instance.with_clarity_db(|db| {
            db.get_account_stx_balance(&principal)
        })?;
        Ok(balance.amount_unlocked)
    }

    pub async fn transfer_stx(&self, from: &PrincipalData, to: &PrincipalData, amount: u64) -> Result<String> {
        let instance = self.clarity_instance.lock().await;
        let result = instance.with_clarity_db(|db| {
            db.transfer_stx(from, to, amount)
        })?;
        Ok(result.to_string())
    }

    pub async fn call_contract_function(&self, contract_address: &StacksAddress, contract_name: &str, function_name: &str, args: Vec<ClarityValue>) -> Result<ClarityValue> {
        let instance = self.clarity_instance.lock().await;
        let contract_identifier = format!("{}.{}", contract_address, contract_name);
        let result = instance.with_clarity_db(|db| {
            let contract = db.get_contract(&contract_identifier)?;
            let context = ContractContext::new(contract_address.clone(), contract_name.to_string());
            db.call_function(&contract, function_name, &args, &context)
        })?;
        Ok(result)
    }
}

// Web5 integration
pub struct Web5Client {
    did: DID,
    dwn: DWN,
}

impl Web5Client {
    pub fn new(did: DID, dwn: DWN) -> Self {
        Self { did, dwn }
    }

    pub async fn create_did_document(&self) -> Result<DIDDocument> {
        let did_document = DIDDocument::new(self.did.clone(), DIDMethod::Key)?;
        Ok(did_document)
    }

    pub async fn store_data(&self, data: &[u8], permissions: Permissions) -> Result<String> {
        let message = self.dwn.create_message(DataFormat::Json, data, permissions)?;
        let reply = self.dwn.send_message(message).await?;
        match reply {
            MessageReply::Success(id) => Ok(id),
            MessageReply::Error(e) => Err(anyhow!("Failed to store data: {}", e)),
        }
    }

    pub async fn issue_credential(&self, subject: CredentialSubject, expiration: Option<chrono::DateTime<chrono::Utc>>) -> Result<Credential> {
        let issuer = Issuer::new(self.did.to_string());
        let credential = Credential::new(issuer, subject, expiration, None)?;
        Ok(credential)
    }

    pub async fn verify_credential(&self, credential: &Credential) -> Result<bool> {
        credential.verify().await
    }

    pub async fn create_presentation(&self, credentials: Vec<Credential>) -> Result<Presentation> {
        let presentation = Presentation::new(credentials, None)?;
        Ok(presentation)
    }
}

// DLC integration
pub struct DlcClient {
    network: Network,
}

impl DlcClient {
    pub fn new(network: Network) -> Self {
        Self { network }
    }

    pub fn create_dlc(&self, oracle_info: OracleInfo, collateral: u64) -> Result<DlcTransaction> {
        let contract = Contract::new(oracle_info, collateral)?;
        let dlc_tx = DlcTransaction::new(contract, self.network)?;
        Ok(dlc_tx)
    }

    pub fn execute_dlc(&self, dlc: DlcTransaction, announcement: Announcement, attestation: Attestation) -> Result<Transaction> {
        let closing_tx = dlc.execute(announcement, attestation)?;
        Ok(closing_tx)
    }
}

// Lightning Network integration
pub struct LightningClient {
    channel_manager: Arc<ChannelManager>,
    network: Network,
}

impl LightningClient {
    pub fn new(seed: &[u8; 32], network: Network) -> Result<Self> {
        let user_config = UserConfig::default();
        let channel_manager = ChannelManager::new(seed, user_config, &network)?;
        Ok(Self {
            channel_manager: Arc::new(channel_manager),
            network,
        })
    }

    pub fn open_channel(&self, node_id: &NodeId, capacity: u64) -> Result<ChannelUpdate> {
        let channel = self.channel_manager.create_channel(node_id, capacity)?;
        Ok(channel)
    }

    pub fn send_payment(&self, invoice: &str) -> Result<()> {
        self.channel_manager.send_payment(invoice)?;
        Ok(())
    }
}

// libp2p integration
#[derive(NetworkBehaviourEventProcess)]
pub struct P2pBehaviour {
    floodsub: Floodsub,
    mdns: Mdns,
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for P2pBehaviour {
    fn inject_event(&mut self, event: FloodsubEvent) {
        if let FloodsubEvent::Message(message) = event {
            println!("Received: '{:?}' from {:?}", String::from_utf8_lossy(&message.data), message.source);
        }
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for P2pBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer, _) in list {
                    self.floodsub.add_node_to_partial_view(peer);
                }
            }
            MdnsEvent::Expired(list) => {
                for (peer, _) in list {
                    if !self.mdns.has_node(&peer) {
                        self.floodsub.remove_node_from_partial_view(&peer);
                    }
                }
            }
        }
    }
}

pub struct P2pClient {
    swarm: Swarm<P2pBehaviour>,
}

impl P2pClient {
    pub async fn new() -> Result<Self> {
        let id_keys = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(id_keys.public());
        let transport = libp2p::development_transport(id_keys).await?;
        
        let mut behaviour = P2pBehaviour {
            floodsub: Floodsub::new(peer_id),
            mdns: Mdns::new(Default::default()).await?,
        };

        let topic = Topic::new("anya-network");
        behaviour.floodsub.subscribe(topic);

        let swarm = Swarm::new(transport, behaviour, peer_id);
        Ok(Self { swarm })
    }

    pub async fn connect_to_peer(&mut self, addr: Multiaddr) -> Result<()> {
        self.swarm.dial(addr)?;
        Ok(())
    }

    pub async fn broadcast_message(&mut self, message: &[u8]) -> Result<()> {
        let topic = Topic::new("anya-network");
        self.swarm.behaviour_mut().floodsub.publish(topic, message);
        Ok(())
    }
}

pub fn select_rpc_config() -> RpcConfig {
    // Use Blockstream endpoint by default
    RpcConfig::new("https://bitcoin-mainnet.public.blastapi.io", None)
}
