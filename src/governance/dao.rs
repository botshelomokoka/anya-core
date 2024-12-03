use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use serde::{Serialize, Deserialize};

/// Bitcoin-Inspired AGT Governance Model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AGTGovernanceProtocol {
    /// Total supply fixed at 21 million
    pub total_supply: u64,
    /// Initial block reward
    pub initial_block_reward: u64,
    /// Halving interval (same as Bitcoin)
    pub halving_interval: u64,
    /// Current block reward
    pub current_block_reward: u64,
    /// Blocks mined
    pub blocks_mined: u64,
}

impl AGTGovernanceProtocol {
    /// Create new governance protocol with Bitcoin-like supply
    pub fn new() -> Self {
        Self {
            total_supply: 21_000_000 * 100_000_000, // 21M with 8 decimal places
            initial_block_reward: 50 * 100_000_000, // 50 coins
            halving_interval: 210_000, // Bitcoin halving cycle
            current_block_reward: 50 * 100_000_000,
            blocks_mined: 0,
        }
    }

    /// Calculate total mined supply
    pub fn calculate_total_mined_supply(&self) -> u64 {
        let mut total_mined = 0;
        let mut current_reward = self.initial_block_reward;
        let mut blocks_processed = 0;

        while blocks_processed < self.blocks_mined && total_mined < self.total_supply {
            let cycle_blocks = std::cmp::min(
                self.halving_interval, 
                self.blocks_mined - blocks_processed
            );

            let cycle_supply = current_reward * cycle_blocks;
            total_mined += cycle_supply;
            
            // Halve reward every 210,000 blocks
            if blocks_processed % self.halving_interval == 0 {
                current_reward /= 2;
            }

            blocks_processed += cycle_blocks;
        }

        total_mined
    }

    /// Verify if minting would exceed max supply
    pub fn can_mint(&self, amount: u64) -> bool {
        self.calculate_total_mined_supply() + amount <= self.total_supply
    }
}

/// Governance Voting Mechanism
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DAOGovernance {
    /// Voting parameters
    pub voting_threshold: f64,
    pub proposal_threshold: u64,
    pub quorum_percentage: f64,

    /// Proposal tracking
    pub active_proposals: RwLock<Vec<Proposal>>,

    /// Governance token protocol
    pub token_protocol: Arc<Mutex<AGTGovernanceProtocol>>,
}

impl DAOGovernance {
    pub fn new() -> Self {
        Self {
            voting_threshold: 0.6, // 60% majority
            proposal_threshold: 100, // 100 AGT to propose
            quorum_percentage: 0.3, // 30% participation required
            active_proposals: RwLock::new(Vec::new()),
            token_protocol: Arc::new(Mutex::new(AGTGovernanceProtocol::new())),
        }
    }

    /// Submit a new proposal
    pub async fn submit_proposal(&self, proposal: Proposal) -> Result<(), String> {
        let mut proposals = self.active_proposals.write().await;
        
        // Check proposer's token balance meets threshold
        if proposal.proposer_token_balance < self.proposal_threshold {
            return Err("Insufficient tokens to submit proposal".to_string());
        }

        proposals.push(proposal);
        Ok(())
    }

    /// Cast a vote on a proposal
    pub async fn cast_vote(&self, proposal_id: u64, voter: String, vote: Vote) -> Result<(), String> {
        let mut proposals = self.active_proposals.write().await;
        
        if let Some(proposal) = proposals.iter_mut().find(|p| p.id == proposal_id) {
            proposal.votes.push(vote);
            Ok(())
        } else {
            Err("Proposal not found".to_string())
        }
    }

    /// Finalize proposal based on voting rules
    pub async fn finalize_proposal(&self, proposal_id: u64) -> Result<ProposalStatus, String> {
        let mut proposals = self.active_proposals.write().await;
        
        if let Some(proposal) = proposals.iter_mut().find(|p| p.id == proposal_id) {
            let total_votes: u64 = proposal.votes.iter().map(|v| v.voting_power).sum();
            let for_votes: u64 = proposal.votes.iter()
                .filter(|v| v.decision == VoteDecision::For)
                .map(|v| v.voting_power)
                .sum();

            let voting_percentage = (for_votes as f64) / (total_votes as f64);

            let status = if voting_percentage >= self.voting_threshold {
                ProposalStatus::Passed
            } else {
                ProposalStatus::Failed
            };

            proposal.status = status;
            Ok(status)
        } else {
            Err("Proposal not found".to_string())
        }
    }
}

/// Proposal data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub proposer: String,
    pub proposer_token_balance: u64,
    pub votes: Vec<Vote>,
    pub status: ProposalStatus,
}

/// Vote representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub voter: String,
    pub decision: VoteDecision,
    pub voting_power: u64,
}

/// Proposal status enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProposalStatus {
    Active,
    Passed,
    Failed,
    Executed,
}

/// Vote decision enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VoteDecision {
    For,
    Against,
    Abstain,
}
