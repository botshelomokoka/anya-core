//! This module handles proposal creation and validation for the Anya DAO

use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use web5::{Web5, Protocol};
use web5::did::DID;
use web5::dwn::{DwnApi, RecordQuery};
use stacks_common::types::StacksAddress;
use stacks_transactions::{
    AccountTransactionEffects, AssetIdentifier, PostConditionMode,
    StacksTransaction, TransactionVersion, TransactionPayload, TransactionSigner,
    StacksPublicKey, SingleSigSpendingCondition, TransactionAnchor,
    contract_call::ContractCall, post_condition::PostCondition,
};
use clarity_repl::clarity::{ClarityInstance, ClarityContract, Value as ClarityValue};
use web5_api::{Web5Api, CredentialsApi, DIDApi, DWNApi};
use web5_credentials::{Credential, VerifiableCredential, CredentialStatus, CredentialSchema};
use dlc::{DlcParty, Offer, Accept, Sign, Oracle, Contract, Announcement, Attestation};
use lightning::{
    chain::chaininterface::ConfirmationTarget,
    ln::channelmanager::{ChannelManager, ChannelManagerReadArgs},
    ln::invoice::Invoice,
    ln::chan_utils::ChannelPublicKeys,
    ln::msgs::{ChannelMessageHandler, RoutingMessageHandler},
    util::config::UserConfig,
    util::events::EventHandler,
};
use bitcoin::{
    Address, Transaction, TxIn, TxOut, OutPoint, Script, Network,
    SigHashType, PublicKey, PrivateKey, secp256k1::Secp256k1, hashes::Hash,
};
use libp2p::{
    core::{upgrade, transport::OrTransport, muxing::StreamMuxerBox},
    floodsub::{Floodsub, FloodsubEvent, Topic},
    identity, mdns::{Mdns, MdnsEvent},
    noise, mplex, tcp::TokioTcpConfig, websocket::WsConfig,
    swarm::{NetworkBehaviourEventProcess, Swarm, SwarmBuilder},
    NetworkBehaviour, PeerId, Transport,
};
use rand::Rng;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use chrono::{DateTime, Utc};

lazy_static! {
    static ref W5: Web5 = Web5::connect(Some(Protocol::Rsk), None).unwrap();
}

/// Represents a proposal in the Anya DAO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    id: String,
    proposer: String,
    title: String,
    description: String,
    options: Vec<String>,
    start_time: DateTime<Utc>,
    end_time: Option<DateTime<Utc>>,
    status: ProposalStatus,
    stx_address: Option<StacksAddress>,
    dlc_contract: Option<Contract>,
    lightning_invoice: Option<Invoice>,
    bitcoin_address: Option<Address>,
    p2p_peer_id: Option<PeerId>,
    votes: HashMap<String, usize>,
}

/// Represents the status of a proposal
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProposalStatus {
    Active,
    Completed,
    Cancelled,
}

/// Creates a new proposal
pub async fn create_proposal(
    proposer: String,
    title: String,
    description: String,
    options: Vec<String>,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    stx_address: Option<StacksAddress>,
    dlc_contract: Option<Contract>,
    lightning_invoice: Option<Invoice>,
    bitcoin_address: Option<Address>,
    p2p_peer_id: Option<PeerId>,
) -> Result<Proposal> {
    let start_time = start_time.unwrap_or_else(|| Utc::now());

    // Basic validation
    if title.is_empty() || description.is_empty() || options.len() < 2 {
        return Err(anyhow!("Invalid proposal parameters"));
    }

    if let Some(end) = end_time {
        if end <= start_time {
            return Err(anyhow!("End time must be after start time"));
        }
    }

    let proposal = Proposal {
        id: generate_unique_proposal_id().await?,
        proposer,
        title,
        description,
        options,
        start_time,
        end_time,
        status: ProposalStatus::Active,
        stx_address,
        dlc_contract,
        lightning_invoice,
        bitcoin_address,
        p2p_peer_id,
        votes: HashMap::new(),
    };

    // Store the proposal in Web5 DWN
    W5.dwn().records().create()
        .data(&proposal)
        .schema("proposal")
        .recipient(&DID::parse(&proposal.proposer)?)
        .publish()
        .await?;

    // Create a verifiable credential for the proposal
    let credential = Credential::new()
        .id(&proposal.id)
        .issuer(&W5.did())
        .type_("ProposalCredential")
        .subject(&proposal.proposer)
        .status(CredentialStatus::Active)
        .schema(CredentialSchema::new("https://example.com/proposal-schema.json"));

    let verifiable_credential = W5.credentials().issue(credential).await?;

    // Store the verifiable credential
    W5.dwn().records().create()
        .data(&verifiable_credential)
        .schema("verifiable-credential")
        .recipient(&DID::parse(&proposal.proposer)?)
        .publish()
        .await?;

    Ok(proposal)
}

/// Checks if a proposal is valid based on its structure and content
pub fn is_proposal_valid(proposal: &Proposal) -> bool {
    // Check for required fields
    if proposal.title.is_empty() || proposal.description.is_empty() || proposal.options.len() < 2 {
        return false;
    }

    // Validate STX address if present
    if let Some(stx_address) = &proposal.stx_address {
        if !stx_address.is_valid() {
            return false;
        }
    }

    // Validate DLC contract if present
    if let Some(dlc_contract) = &proposal.dlc_contract {
        if !dlc_contract.is_valid() {
            return false;
        }
    }

    // Validate Lightning invoice if present
    if let Some(lightning_invoice) = &proposal.lightning_invoice {
        if !lightning_invoice.is_valid() {
            return false;
        }
    }

    // Validate Bitcoin address if present
    if let Some(bitcoin_address) = &proposal.bitcoin_address {
        if !bitcoin_address.is_valid() {
            return false;
        }
    }

    // Validate libp2p PeerId if present
    if let Some(peer_id) = &proposal.p2p_peer_id {
        if peer_id.to_base58().is_empty() {
            return false;
        }
    }

    true
}

/// Generates a unique proposal ID
async fn generate_unique_proposal_id() -> Result<String> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos();
    
    let mut rng = rand::thread_rng();
    let random_suffix: u64 = rng.gen();
    
    let id = format!("{}-{}", timestamp, random_suffix);
    
    // Check if the ID already exists in the DWN
    let existing_proposal = W5.dwn().records().query()
        .schema("proposal")
        .filter(format!("id == '{}'", id))
        .first()
        .await?;
    
    if existing_proposal.is_some() {
        // If the ID already exists, recursively try again
        generate_unique_proposal_id().await
    } else {
        Ok(id)
    }
}

/// Retrieves a proposal by its ID
pub async fn get_proposal_by_id(id: &str) -> Result<Option<Proposal>> {
    let proposal = W5.dwn().records().query()
        .schema("proposal")
        .filter(format!("id == '{}'", id))
        .first()
        .await?;

    Ok(proposal.map(|record| record.data::<Proposal>().ok()).flatten())
}

/// Updates the status of a proposal
pub async fn update_proposal_status(id: &str, new_status: ProposalStatus) -> Result<()> {
    let mut proposal = get_proposal_by_id(id).await?.ok_or_else(|| anyhow!("Proposal not found"))?;
    proposal.status = new_status;

    W5.dwn().records().update()
        .record_id(id)
        .data(&proposal)
        .publish()
        .await?;

    Ok(())
}

/// Retrieves all proposals
pub async fn get_all_proposals() -> Result<Vec<Proposal>> {
    let proposals = W5.dwn().records().query()
        .schema("proposal")
        .execute()
        .await?;

    proposals.into_iter()
        .filter_map(|record| record.data::<Proposal>().ok())
        .collect()
}

/// Casts a vote for a proposal
pub async fn cast_vote(proposal_id: &str, voter: &str, option_index: usize) -> Result<()> {
    let mut proposal = get_proposal_by_id(proposal_id).await?.ok_or_else(|| anyhow!("Proposal not found"))?;
    
    if proposal.status != ProposalStatus::Active {
        return Err(anyhow!("Proposal is not active"));
    }
    
    if option_index >= proposal.options.len() {
        return Err(anyhow!("Invalid option index"));
    }
    
    proposal.votes.insert(voter.to_string(), option_index);
    
    W5.dwn().records().update()
        .record_id(proposal_id)
        .data(&proposal)
        .publish()
        .await?;
    
    Ok(())
}

/// Counts the votes for a proposal
pub async fn count_votes(proposal_id: &str) -> Result<HashMap<String, usize>> {
    let proposal = get_proposal_by_id(proposal_id).await?.ok_or_else(|| anyhow!("Proposal not found"))?;
    
    let mut vote_counts = HashMap::new();
    for option in &proposal.options {
        vote_counts.insert(option.clone(), 0);
    }
    
    for &option_index in proposal.votes.values() {
        if let Some(option) = proposal.options.get(option_index) {
            *vote_counts.entry(option.clone()).or_insert(0) += 1;
        }
    }
    
    Ok(vote_counts)
}
