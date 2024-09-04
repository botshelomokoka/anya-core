use anya_core::network::{bitcoin_client, rsk_client};
use anya_core::constants::{DAO_RSK_ADDRESS, DAO_PRIVATE_KEY};
use anyhow::{Result, anyhow};
use web5::{Web5, Protocol};
use web5::did::DID;
use web5::dwn::{DwnApi, RecordQuery};
use serde_json::Value;
use std::collections::HashMap;

lazy_static! {
    static ref W5: Web5 = Web5::connect(Some(Protocol::Rsk), None).unwrap();
}

/// Executes an approved proposal
///
/// # Arguments
///
/// * `proposal` - The proposal containing details and execution instructions
///
/// # Returns
///
/// A Result indicating success or failure of the execution
pub async fn execute_proposal(proposal: &Value) -> Result<()> {
    if proposal["status"] != "approved" {
        return Err(anyhow!("Only approved proposals can be executed"));
    }

    match proposal["chain"].as_str() {
        Some("bitcoin") => execute_on_bitcoin(proposal).await,
        Some("rsk") => execute_on_rsk(proposal).await,
        _ => Err(anyhow!("Invalid chain specified in proposal")),
    }
}

/// Executes a proposal on the Bitcoin blockchain
///
/// # Arguments
///
/// * `proposal` - The proposal containing Bitcoin-specific execution details
///
/// # Returns
///
/// A Result indicating success or failure of the execution
async fn execute_on_bitcoin(proposal: &Value) -> Result<()> {
    match proposal["type"].as_str() {
        Some("send_bitcoin") => {
            let recipient_address = proposal["recipient_address"]
                .as_str()
                .ok_or_else(|| anyhow!("Missing recipient_address"))?;
            let amount = proposal["amount"]
                .as_u64()
                .ok_or_else(|| anyhow!("Invalid amount"))?;

            // Construct and broadcast the transaction
            let tx = bitcoin_client::create_transaction(recipient_address, amount)?;
            let txid = bitcoin_client::broadcast_transaction(&tx)?;

            // Update proposal status and log execution
            update_proposal_status(proposal, "executed").await?;
            log_event("proposal_executed", &[("proposal_id", &proposal["id"]), ("txid", &txid.into())]).await?;

            Ok(())
        }
        // Handle other Bitcoin proposal types as needed, e.g., Taproot asset issuance/transfer
        _ => Err(anyhow!("Unsupported Bitcoin proposal type")),
    }
}

/// Executes a proposal on the RSK network
///
/// # Arguments
///
/// * `proposal` - The proposal containing RSK-specific execution details
///
/// # Returns
///
/// A Result indicating success or failure of the execution
async fn execute_on_rsk(proposal: &Value) -> Result<()> {
    match proposal["type"].as_str() {
        Some("call_contract_function") => {
            let contract_address = proposal["contract_address"]
                .as_str()
                .ok_or_else(|| anyhow!("Missing contract_address"))?;
            let function_name = proposal["function_name"]
                .as_str()
                .ok_or_else(|| anyhow!("Missing function_name"))?;
            let function_args = proposal["function_args"]
                .as_array()
                .ok_or_else(|| anyhow!("Invalid function_args"))?;

            // Interact with the smart contract using Web5
            let contract_did = DID::parse(contract_address)?;
            let result = W5.dwn().records().query()
                .from(&contract_did)
                .schema("contract/function")
                .filter(|r| r["name"] == function_name && r["args"] == function_args)
                .first()
                .await?
                .ok_or_else(|| anyhow!("Contract function not found"))?;

            // Process the result
            let tx_hash = result["txHash"].as_str().ok_or_else(|| anyhow!("Missing txHash"))?;

            // Update proposal status based on the transaction result
            if result["status"].as_str() == Some("success") {
                update_proposal_status(proposal, "executed").await?;
                log_event("proposal_executed", &[("proposal_id", &proposal["id"]), ("tx_hash", tx_hash)]).await?;
            } else {
                update_proposal_status(proposal, "failed").await?;
                log_event("proposal_execution_failed", &[("proposal_id", &proposal["id"]), ("tx_hash", tx_hash)]).await?;
                return Err(anyhow!("Transaction failed or status unknown"));
            }

            Ok(())
        }
        _ => Err(anyhow!("Unsupported RSK proposal type")),
    }
}

/// Updates the status of a proposal
///
/// # Arguments
///
/// * `proposal` - The proposal to update
/// * `new_status` - The new status to set
///
/// # Returns
///
/// A Result indicating success or failure of the update
async fn update_proposal_status(proposal: &Value, new_status: &str) -> Result<()> {
    let proposal_did = DID::parse(&proposal["id"].as_str().ok_or_else(|| anyhow!("Invalid proposal ID"))?)?;
    
    let mut updated_proposal = proposal.clone();
    updated_proposal["status"] = new_status.into();

    W5.dwn().records().write(&proposal_did, "proposal", &updated_proposal).await?;
    Ok(())
}

/// Logs an event related to proposal execution
///
/// # Arguments
///
/// * `event_type` - The type of event to log
/// * `details` - Additional details about the event
///
/// # Returns
///
/// A Result indicating success or failure of the logging
async fn log_event(event_type: &str, details: &[(&str, &str)]) -> Result<()> {
    let event = HashMap::from_iter(details.iter().cloned());
    let event_did = DID::new()?;
    
    W5.dwn().records().write(&event_did, "event", &event).await?;
    Ok(())
}
