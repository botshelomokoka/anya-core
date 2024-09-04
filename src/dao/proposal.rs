//! This module handles proposal creation and validation for the Anya DAO

use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use web5::{Web5, Protocol};
use web5::did::DID;
use web5::dwn::{DwnApi, RecordQuery};

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
    start_time: u64,
    end_time: Option<u64>,
    status: ProposalStatus,
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
    start_time: Option<u64>,
    end_time: Option<u64>,
) -> Result<Proposal> {
    let start_time = start_time.unwrap_or_else(|| {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    });

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
    };

    // Store the proposal in Web5 DWN
    W5.dwn().records().create()
        .data(&proposal)
        .schema("proposal")
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

    // Add more validation checks as needed, e.g., time constraints, content restrictions

    true
}

/// Generates a unique proposal ID
async fn generate_unique_proposal_id() -> Result<String> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos();
    
    let random_suffix: u64 = rand::random();
    
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

// Add other proposal-related functions as needed, such as:
// - get_proposal_by_id
// - update_proposal_status
// - get_all_proposals
