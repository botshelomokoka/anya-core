//! This module manages the DAO's treasury, handling both on-chain (RSK) and off-chain (Bitcoin) assets.

use anya_core::network::{bitcoin_client, rsk_client};
use anya_core::constants::{DAO_RSK_ADDRESS, DAO_BITCOIN_ADDRESS};
use std::collections::HashMap;
use anyhow::{Result, anyhow};
use web5::{Web5, Protocol};
use web5::did::DID;
use web5::dwn::{DwnApi, RecordQuery};
use serde::{Serialize, Deserialize};

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

/// Gets the total balance of the DAO treasury, combining on-chain (RSK) and off-chain (Bitcoin) assets
///
/// Returns:
///     A HashMap containing the total balance in RBTC and Bitcoin (satoshi)
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

    Ok(balance)
}

/// Allocates funds from the DAO treasury
///
/// # Arguments
///
/// * `chain` - The chain to allocate funds from ("bitcoin" or "rsk")
/// * `recipient_address` - The address to send the funds to
/// * `amount` - The amount to allocate
/// * `asset_type` - (Optional) The type of asset to allocate ("native" for RBTC or Bitcoin, or a specific token/asset ID)
pub async fn allocate_funds(chain: &str, recipient_address: &str, amount: u64, asset_type: Option<&str>) -> Result<()> {
    match chain {
        "bitcoin" => {
            let tx = bitcoin_client::create_transaction(recipient_address, amount)?;
            let txid = bitcoin_client::broadcast_transaction(&tx)?;
            
            // Log the transaction in Web5 DWN
            W5.dwn().records().create()
                .data(&Transaction {
                    from: DAO_BITCOIN_ADDRESS.to_string(),
                    to: recipient_address.to_string(),
                    value: amount,
                    data: Some(txid),
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
        _ => return Err(anyhow!("Invalid chain. Choose from 'bitcoin' or 'rsk'")),
    }
    Ok(())
}

/// Processes incoming funds to the DAO treasury
///
/// # Arguments
///
/// * `tx` - The transaction object (either Bitcoin or RSK)
pub async fn process_incoming_funds(tx: &Transaction) -> Result<()> {
    // Log the incoming transaction
    W5.dwn().records().create()
        .data(tx)
        .schema("transaction")
        .publish()
        .await?;

    // Update the balance
    if tx.to == DAO_RSK_ADDRESS {
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
    } else if tx.to == DAO_BITCOIN_ADDRESS {
        // For Bitcoin, we rely on the UTXO set, so no need to update a balance record
        // We might want to update some kind of cache or index for quicker balance lookups
    }

    Ok(())
}

// Other treasury management functions as needed, e.g.,
// handling proposals for fund allocation, generating reward distributions, etc.
