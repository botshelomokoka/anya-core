
//! This module generates financial reports for the Anya DAO, considering both on-chain (RSK) and 
//! off-chain (Bitcoin) treasury components.

use anya_core::network::{bitcoin_client, rsk_client};
use anya_core::constants::{DAO_RSK_ADDRESS, DAO_BITCOIN_ADDRESS, ANYA_TOKEN_CONTRACT_ADDRESS};
use web5::{Web5, Protocol};
use web5::did::DID;
use web5::dwn::{DwnApi, RecordQuery};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use anyhow::{Result, anyhow};
use std::str::FromStr;

lazy_static! {
    static ref W5: Web5 = Web5::connect(Some(Protocol::Rsk), None).unwrap();
}

#[derive(Serialize, Deserialize)]
struct TokenBalance {
    balance: u64,
}

#[derive(Serialize, Deserialize)]
struct Transaction {
    from: String,
    to: String,
    value: u64,
    data: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct BitcoinTransaction {
    txid: String,
    vout: Vec<BitcoinOutput>,
    vin: Vec<BitcoinInput>,
}

#[derive(Serialize, Deserialize)]
struct BitcoinOutput {
    value: u64,
    script_pub_key: ScriptPubKey,
}

#[derive(Serialize, Deserialize)]
struct BitcoinInput {
    txid: String,
}

#[derive(Serialize, Deserialize)]
struct ScriptPubKey {
    addresses: Vec<String>,
}

/// Generates a report on the DAO's treasury holdings.
pub async fn generate_treasury_report() -> Result<HashMap<String, HashMap<String, u64>>> {
    let mut report = HashMap::new();

    // On-chain assets (RSK)
    let mut rsk_report = HashMap::new();
    
    // Using Web5
    let rbtc_balance: TokenBalance = W5.dwn().records().query()
        .from(&DID::parse(DAO_RSK_ADDRESS)?)
        .schema("token/balance")
        .recipient("rbtc")
        .first()
        .await?
        .ok_or_else(|| anyhow!("RBTC balance not found"))?
        .data()?;
    rsk_report.insert("rbtc_balance".to_string(), rbtc_balance.balance);
    
    let mut token_balances = HashMap::new();
    if let Some(anya_token_address) = ANYA_TOKEN_CONTRACT_ADDRESS {
        let anya_token_did = DID::parse(anya_token_address)?;
        let anya_balance: TokenBalance = W5.dwn().records().query()
            .from(&anya_token_did)
            .schema("token/balance")
            .recipient(DAO_RSK_ADDRESS)
            .first()
            .await?
            .ok_or_else(|| anyhow!("ANYA token balance not found"))?
            .data()?;
        token_balances.insert("ANYA".to_string(), anya_balance.balance);
    }
    rsk_report.insert("token_balances".to_string(), token_balances);

    report.insert("rsk".to_string(), rsk_report);

    // Off-chain assets (Bitcoin)
    let mut bitcoin_report = HashMap::new();
    let utxos = bitcoin_client::get_utxos(DAO_BITCOIN_ADDRESS).await?;
    bitcoin_report.insert("utxos".to_string(), utxos.len() as u64);
    bitcoin_report.insert("total_balance".to_string(), utxos.iter().map(|utxo| utxo.value).sum());

    report.insert("bitcoin".to_string(), bitcoin_report);

    Ok(report)
}

/// Generates a report on the DAO's income and expenses within a specified time period
pub async fn generate_income_and_expense_report(start_time: u64, end_time: u64) -> Result<HashMap<String, HashMap<String, HashMap<String, u64>>>> {
    let mut report = HashMap::new();
    report.insert("income".to_string(), HashMap::new());
    report.insert("expenses".to_string(), HashMap::new());

    for category in ["income", "expenses"] {
        for chain in ["rsk", "bitcoin"] {
            report.get_mut(category).unwrap().insert(chain.to_string(), HashMap::new());
        }
    }

    // Fetch RSK transactions using Web5
    let rsk_transactions: Vec<Transaction> = W5.dwn().records().query()
        .from(&DID::parse(DAO_RSK_ADDRESS)?)
        .schema("transaction")
        .filter(RecordQuery::created_at().gte(start_time).lte(end_time))
        .execute()
        .await?
        .into_iter()
        .map(|record| record.data())
        .collect::<Result<Vec<_>, _>>()?;

    // Fetch Bitcoin transactions
    let bitcoin_transactions: Vec<BitcoinTransaction> = W5.dwn().records().query()
        .from(&DID::parse(DAO_BITCOIN_ADDRESS)?)
        .schema("transaction")
        .filter(RecordQuery::created_at().gte(start_time).lte(end_time))
        .execute()
        .await?
        .into_iter()
        .map(|record| record.data())
        .collect::<Result<Vec<_>, _>>()?;

    // Categorize RSK transactions
    for tx in rsk_transactions {
        let category = if tx.to == DAO_RSK_ADDRESS { "income" } else { "expenses" };
        let amount = report.get_mut(category).unwrap()
            .get_mut("rsk").unwrap()
            .entry("total".to_string())
            .or_insert(0);
        *amount += tx.value;
        // Further categorization can be done here based on tx.data
    }

    // Categorize Bitcoin transactions
    for tx in bitcoin_transactions {
        for output in tx.vout {
            if output.script_pub_key.addresses.contains(&DAO_BITCOIN_ADDRESS.to_string()) {
                let amount = report.get_mut("income").unwrap()
                    .get_mut("bitcoin").unwrap()
                    .entry("total".to_string())
                    .or_insert(0);
                *amount += output.value;
            }
        }
        // Note: For expenses, we'd need to track the previous outputs spent by this transaction
        // This would require additional API calls to fetch previous transactions
    }

    Ok(report)
}

// Additional financial reporting functions can be added here as needed
