//! This module handles the governance aspects of the Anya DAO, including voting and proposal management.

use anya_core::network::{bitcoin_client, rsk_client};
use anya_core::utils::helpers;
use anya_core::constants::ANYA_TOKEN_CONTRACT_ADDRESS;
use crate::dao::proposal::{create_proposal, is_proposal_valid};
use crate::dao::membership_management::is_member;
use crate::dao::executor::execute_proposal;

use web5::{Web5, Protocol};
use web5::did::DID;
use web5::dwn::{DwnApi, RecordQuery};
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

lazy_static! {
    static ref W5: Web5 = Web5::connect(Some(Protocol::Rsk), None).unwrap();
}

// Data storage functions using Web5 DWN

async fn store_proposal(proposal: &Proposal, txid: &str) -> Result<()> {
    W5.dwn().records().create()
        .data(proposal)
        .schema("proposal")
        .recipient(&DID::parse(&proposal.proposer)?)
        .publish()
        .await?;
    Ok(())
}

async fn get_proposal_by_id(proposal_id: &str) -> Result<Proposal> {
    W5.dwn().records().query()
        .schema("proposal")
        .filter(format!("id == '{}'", proposal_id))
        .first()
        .await?
        .ok_or_else(|| anyhow!("Proposal not found"))
        .and_then(|record| record.data())
}

async fn get_votes_for_proposal(proposal_id: &str) -> Result<Vec<Vote>> {
    W5.dwn().records().query()
        .schema("vote")
        .filter(format!("proposal_id == '{}'", proposal_id))
        .execute()
        .await?
        .into_iter()
        .map(|record| record.data())
        .collect()
}

async fn record_vote(proposal_id: &str, voter_address: &str, vote_option: &str, amount: u64, txid: &str) -> Result<()> {
    let vote = Vote {
        voter_address: voter_address.to_string(),
        option: vote_option.to_string(),
        amount,
        txid: txid.to_string(),
    };
    W5.dwn().records().create()
        .data(&vote)
        .schema("vote")
        .recipient(&DID::parse(voter_address)?)
        .publish()
        .await?;
    Ok(())
}

async fn update_proposal_status(proposal_id: &str, new_status: ProposalStatus) -> Result<()> {
    let mut proposal = get_proposal_by_id(proposal_id).await?;
    proposal.status = new_status;
    W5.dwn().records().update()
        .data(&proposal)
        .schema("proposal")
        .recipient(&DID::parse(&proposal.proposer)?)
        .publish()
        .await?;
    Ok(())
}

async fn get_current_epoch() -> Result<u64> {
    W5.dwn().records().query()
        .schema("epoch")
        .first()
        .await?
        .ok_or_else(|| anyhow!("Epoch data not found"))
        .and_then(|record| record.data::<HashMap<String, u64>>())
        .map(|epoch_data| epoch_data["current_epoch"])
}

async fn set_current_epoch(new_epoch: u64) -> Result<()> {
    let epoch_data = HashMap::from([("current_epoch".to_string(), new_epoch)]);
    W5.dwn().records().create()
        .data(&epoch_data)
        .schema("epoch")
        .publish()
        .await?;
    Ok(())
}

// Main governance functions

async fn submit_proposal(proposer: &str, title: &str, description: &str, options: Vec<String>, start_time: Option<u64>, end_time: Option<u64>, chain: &str) -> Result<String> {
    if !is_member(proposer).await? {
        return Err(anyhow!("Only DAO members can submit proposals"));
    }

    let proposal = create_proposal(proposer, title, description, options, start_time, end_time, chain)?;
    
    if !is_proposal_valid(&proposal) {
        return Err(anyhow!("Invalid proposal"));
    }

    let op_return_data = helpers::encode_proposal_data(&proposal)?;
    let tx = bitcoin_client::create_op_return_transaction(&op_return_data, proposer)?;
    let txid = bitcoin_client::broadcast_transaction(&tx)?;

    store_proposal(&proposal, &txid).await?;

    Ok(txid)
}

async fn get_proposals() -> Result<Vec<Proposal>> {
    let bitcoin_proposals = get_proposals_from_bitcoin().await?;
    let rsk_proposals = get_proposals_from_rsk().await?;

    let mut all_proposals = bitcoin_proposals;
    all_proposals.extend(rsk_proposals);
    
    Ok(all_proposals.into_iter().filter(|p| p.status == ProposalStatus::Active).collect())
}

async fn get_proposals_from_bitcoin() -> Result<Vec<Proposal>> {
    let proposal_transactions = bitcoin_client::get_op_return_transactions()?;

    let mut proposals = Vec::new();
    for tx in proposal_transactions {
        match helpers::decode_proposal_data(&tx.op_return) {
            Ok(proposal_data) if is_proposal_valid(&proposal_data) => proposals.push(proposal_data),
            Err(e) => eprintln!("Error decoding proposal data from transaction {}: {}", tx.txid, e),
            _ => {}
        }
    }

    Ok(proposals)
}

async fn get_proposals_from_rsk() -> Result<Vec<Proposal>> {
    W5.dwn().records().query()
        .schema("proposal")
        .filter("chain == 'rsk'")
        .execute()
        .await?
        .into_iter()
        .map(|record| record.data())
        .collect()
}

async fn vote_on_proposal(proposal_id: &str, vote_option: &str, voter_address: &str, amount: u64) -> Result<String> {
    if !is_member(voter_address).await? {
        return Err(anyhow!("Only DAO members can vote"));
    }

    let proposal = get_proposal_by_id(proposal_id).await?;
    if proposal.status != ProposalStatus::Active {
        return Err(anyhow!("Invalid or inactive proposal"));
    }

    if !proposal.options.contains(&vote_option.to_string()) {
        return Err(anyhow!("Invalid vote option"));
    }

    // Cast vote on the appropriate chain
    if proposal.chain == "bitcoin" {
        let vote_data = helpers::encode_vote_data(proposal_id, vote_option, amount)?;
        let tx = bitcoin_client::create_op_return_transaction(&vote_data, voter_address)?;
        let txid = bitcoin_client::broadcast_transaction(&tx)?;
        record_vote(proposal_id, voter_address, vote_option, amount, &txid).await?;
        Ok(txid)
    } else {
        Err(anyhow!("Voting on RSK chain not implemented"))
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Proposal {
    id: String,
    proposer: String,
    title: String,
    description: String,
    options: Vec<String>,
    start_time: u64,
    end_time: Option<u64>,
    status: ProposalStatus,
    chain: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum ProposalStatus {
    Active,
    Completed,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize)]
struct Vote {
    voter_address: String,
    option: String,
    amount: u64,
    txid: String,
}
