//! This module manages the DAO's treasury, handling on-chain (RSK, STX) and off-chain (Bitcoin) assets.

use anya_core::network::{bitcoin_client, rsk_client, stx_client};
use anya_core::constants::{DAO_RSK_ADDRESS, DAO_BITCOIN_ADDRESS, DAO_STX_ADDRESS};
use std::collections::HashMap;
use anyhow::{Result, anyhow};
use web5::{Web5, Protocol};
use web5::did::{DID, DidResolver};
use web5::dwn::{DwnApi, RecordQuery};
use web5::credentials::{VerifiableCredential, create_credential, verify_credential};
use serde::{Serialize, Deserialize};
use bitcoin::{Address, Transaction as BtcTransaction, Network, Amount, Txid};
use lightning::{
    chain::chaininterface::ConfirmationTarget,
    ln::channelmanager::{ChannelManager, ChannelManagerReadArgs},
    util::config::UserConfig,
    util::events::Event,
};
use dlc::{DlcParty, Offer, Accept, Sign, Oracle, Contract, OracleInfo, Announcement};
use libp2p::{
    core::upgrade,
    floodsub::{Floodsub, FloodsubEvent, Topic},
    identity,
    mdns::{Mdns, MdnsEvent},
    noise,
    swarm::{NetworkBehaviourEventProcess, Swarm, SwarmBuilder},
    tcp::TokioTcpConfig,
    NetworkBehaviour, PeerId, Transport,
};
use clarity_repl::clarity::ClarityInstance;
use stacks_common::types::{StacksEpochId, StacksAddress, StacksTransaction, TransactionId};
use stacks_transactions::{
    AccountTransactionEffects, AssetIdentifier, PostConditionMode,
    TransactionVersion,
};
use tokio::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

lazy_static! {
    static ref W5: Web5 = Web5::connect(Some(Protocol::Rsk), None).unwrap();
}

#[derive(Serialize, Deserialize, Clone)]
struct TokenBalance {
    balance: u64,
}

#[derive(Serialize, Deserialize, Clone)]
struct Transaction {
    from: String,
    to: String,
    value: u64,
    data: Option<String>,
}

/// Gets the total balance of the DAO treasury, combining on-chain (RSK, STX) and off-chain (Bitcoin) assets
///
/// Returns:
///     A HashMap containing the total balance in RBTC, Bitcoin (satoshi), and STX
pub async fn get_treasury_balance() -> Result<HashMap<String, HashMap<String, u64>>> {
    let mut balance = HashMap::new();

    // RSK balance
    let rsk_balance: TokenBalance = W5.dwn().records().query()
        .from(&DID::parse(DAO_RSK_ADDRESS)?)
        .schema("token/balance")
        .recipient("rbtc")
        .first()
        .await?
        .ok_or_else(|| anyhow!("RBTC balance not found"))?
        .data()?;

    balance.insert("rsk".to_string(), {
        let mut rsk_assets = HashMap::new();
        rsk_assets.insert("rbtc".to_string(), rsk_balance.balance);
        rsk_assets
    });

    // Bitcoin balance
    let bitcoin_balance: u64 = bitcoin_client::get_utxos(DAO_BITCOIN_ADDRESS)?
        .iter()
        .map(|utxo| utxo.value)
        .sum();

    balance.insert("bitcoin".to_string(), {
        let mut btc_assets = HashMap::new();
        btc_assets.insert("satoshi".to_string(), bitcoin_balance);
        btc_assets
    });

    // STX balance
    let stx_balance = stx_client::get_account_balance(&DAO_STX_ADDRESS)?;

    balance.insert("stx".to_string(), {
        let mut stx_assets = HashMap::new();
        stx_assets.insert("stx".to_string(), stx_balance);
        stx_assets
    });

    Ok(balance)
}

/// Allocates funds from the DAO treasury
///
/// # Arguments
///
/// * `chain` - The chain to allocate funds from ("bitcoin", "rsk", or "stx")
/// * `recipient_address` - The address to send the funds to
/// * `amount` - The amount to allocate
/// * `asset_type` - (Optional) The type of asset to allocate ("native" for RBTC, Bitcoin, or STX, or a specific token/asset ID)
pub async fn allocate_funds(chain: &str, recipient_address: &str, amount: u64, asset_type: Option<&str>) -> Result<()> {
    match chain {
        "bitcoin" => {
            let tx = bitcoin_client::create_transaction(recipient_address, Amount::from_sat(amount))?;
            let txid = bitcoin_client::broadcast_transaction(&tx)?;
            
            // Log the transaction in Web5 DWN
            W5.dwn().records().create()
                .data(&Transaction {
                    from: DAO_BITCOIN_ADDRESS.to_string(),
                    to: recipient_address.to_string(),
                    value: amount,
                    data: Some(txid.to_string()),
                })
                .schema("transaction")
                .publish()
                .await?;
        }
        "rsk" => {
            match asset_type.unwrap_or("native") {
                "native" => {
                    let tx = rsk_client::send_rbtc(DAO_RSK_ADDRESS, recipient_address, amount)?;
                    
                    // Log the transaction in Web5 DWN
                    W5.dwn().records().create()
                        .data(&Transaction {
                            from: DAO_RSK_ADDRESS.to_string(),
                            to: recipient_address.to_string(),
                            value: amount,
                            data: Some(tx.hash().to_string()),
                        })
                        .schema("transaction")
                        .publish()
                        .await?;
                }
                token_address => {
                    let tx = rsk_client::send_token(token_address, DAO_RSK_ADDRESS, recipient_address, amount)?;
                    
                    // Log the transaction in Web5 DWN
                    W5.dwn().records().create()
                        .data(&Transaction {
                            from: DAO_RSK_ADDRESS.to_string(),
                            to: recipient_address.to_string(),
                            value: amount,
                            data: Some(tx.hash().to_string()),
                        })
                        .schema("transaction")
                        .publish()
                        .await?;
                }
            }
        }
        "stx" => {
            let tx = stx_client::send_stx(DAO_STX_ADDRESS, recipient_address, amount)?;
            
            // Log the transaction in Web5 DWN
            W5.dwn().records().create()
                .data(&Transaction {
                    from: DAO_STX_ADDRESS.to_string(),
                    to: recipient_address.to_string(),
                    value: amount,
                    data: Some(tx.txid().to_string()),
                })
                .schema("transaction")
                .publish()
                .await?;
        }
        _ => return Err(anyhow!("Invalid chain. Choose from 'bitcoin', 'rsk', or 'stx'")),
    }
    Ok(())
}

/// Processes incoming funds to the DAO treasury
///
/// # Arguments
///
/// * `tx` - The transaction object (Bitcoin, RSK, or STX)
pub async fn process_incoming_funds(tx: &Transaction) -> Result<()> {
    // Log the incoming transaction
    W5.dwn().records().create()
        .data(tx)
        .schema("transaction")
        .publish()
        .await?;

    // Update the balance
    match tx.to.as_str() {
        DAO_RSK_ADDRESS => {
            let mut balance: TokenBalance = W5.dwn().records().query()
                .from(&DID::parse(DAO_RSK_ADDRESS)?)
                .schema("token/balance")
                .recipient("rbtc")
                .first()
                .await?
                .ok_or_else(|| anyhow!("RBTC balance not found"))?
                .data()?;

            balance.balance += tx.value;

            W5.dwn().records().update()
                .data(&balance)
                .schema("token/balance")
                .recipient("rbtc")
                .publish()
                .await?;
        }
        DAO_BITCOIN_ADDRESS => {
            // For Bitcoin, we rely on the UTXO set, so no need to update a balance record
            // We might want to update some kind of cache or index for quicker balance lookups
        }
        DAO_STX_ADDRESS => {
            let mut balance: TokenBalance = W5.dwn().records().query()
                .from(&DID::parse(DAO_STX_ADDRESS)?)
                .schema("token/balance")
                .recipient("stx")
                .first()
                .await?
                .ok_or_else(|| anyhow!("STX balance not found"))?
                .data()?;

            balance.balance += tx.value;

            W5.dwn().records().update()
                .data(&balance)
                .schema("token/balance")
                .recipient("stx")
                .publish()
                .await?;
        }
        _ => return Err(anyhow!("Invalid recipient address")),
    }

    Ok(())
}

/// Creates a Discreet Log Contract (DLC) for treasury management
pub async fn create_treasury_dlc(
    counterparty: &DlcParty,
    oracle: &Oracle,
    contract_terms: &Contract,
) -> Result<Offer> {
    let offer = Offer::new(counterparty, oracle, contract_terms)?;
    Ok(offer)
}

/// Initializes a Lightning Network node for the treasury
pub async fn init_lightning_node(
    network: Network,
    storage_dir: &std::path::Path,
) -> Result<Arc<ChannelManager>> {
    let config = UserConfig::default();
    let chain_monitor = Arc::new(chainmonitor::ChainMonitor::new(None, &filter, &logger));
    let channel_manager = Arc::new(ChannelManager::new(
        &fee_estimator,
        &chain_monitor,
        &tx_broadcaster,
        &logger,
        &keys_manager,
        config,
        network,
    ));

    // Load channel state from disk or initialize a new one
    let channel_manager = if let Some(stored_channel_manager) = ChannelManagerReadArgs::new(
        &storage_dir,
        &keys_manager,
        &fee_estimator,
        &chain_monitor,
        &tx_broadcaster,
        &logger,
    )
    .read_channel_manager()?
    {
        stored_channel_manager
    } else {
        channel_manager
    };

    Ok(channel_manager)
}

/// Sets up a libp2p node for peer-to-peer communication
pub async fn setup_p2p_node() -> Result<Swarm<TreasuryBehaviour>> {
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    println!("Local peer id: {:?}", peer_id);

    let transport = TokioTcpConfig::new()
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::NoiseConfig::xx(id_keys).into_authenticated())
        .multiplex(libp2p::yamux::YamuxConfig::default())
        .boxed();

    let mut behaviour = TreasuryBehaviour {
        floodsub: Floodsub::new(peer_id),
        mdns: Mdns::new(Default::default()).await?,
    };

    let swarm = SwarmBuilder::new(transport, behaviour, peer_id)
        .executor(Box::new(|fut| {
            tokio::spawn(fut);
        }))
        .build();

    Ok(swarm)
}

#[derive(NetworkBehaviour)]
struct TreasuryBehaviour {
    floodsub: Floodsub,
    mdns: Mdns,
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for TreasuryBehaviour {
    fn inject_event(&mut self, event: FloodsubEvent) {
        if let FloodsubEvent::Message(message) = event {
            println!(
                "Received: '{:?}' from {:?}",
                String::from_utf8_lossy(&message.data),
                message.source
            );
        }
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for TreasuryBehaviour {
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

// Other treasury management functions as needed, e.g.,
// handling proposals for fund allocation, generating reward distributions, etc.
