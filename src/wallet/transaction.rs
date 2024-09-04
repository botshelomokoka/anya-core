use bitcoin::blockdata::transaction::{Transaction, TxIn, TxOut};
use bitcoin::blockdata::script::Script;
use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use bitcoin::util::key::{PrivateKey, PublicKey};
use bitcoin::util::psbt::PartiallySignedTransaction;
use bitcoin::consensus::encode::serialize;
use bitcoin::hashes::hex::ToHex;
use bitcoin::OutPoint;
use std::error::Error;
use std::str::FromStr;
use rand::Rng;

use crate::network::bitcoin_client;

pub struct UTXOInput {
    pub outpoint: OutPoint,
    pub value: u64,
}

pub struct TransactionOutput {
    pub address: Address,
    pub value: u64,
}

pub fn create_transaction(
    inputs: Vec<UTXOInput>,
    outputs: Vec<TransactionOutput>,
    private_keys: Vec<PrivateKey>,
    fee_rate: Option<u64>,
    change_address: Option<Address>,
) -> Result<Transaction, Box<dyn Error>> {
    let total_input_value: u64 = inputs.iter().map(|input| input.value).sum();
    let total_output_value: u64 = outputs.iter().map(|output| output.value).sum();
    let fee_rate = fee_rate.unwrap_or_else(|| bitcoin_client::estimate_fee());

    let estimated_tx_size = 148 * inputs.len() + 34 * outputs.len() + 10;
    let change = total_input_value.saturating_sub(total_output_value + fee_rate * estimated_tx_size as u64);

    let mut tx = Transaction {
        version: 2,
        lock_time: 0,
        input: inputs.into_iter().map(|input| TxIn {
            previous_output: input.outpoint,
            script_sig: Script::new(),
            sequence: 0xFFFFFFFF,
            witness: Vec::new(),
        }).collect(),
        output: outputs.into_iter().map(|output| TxOut {
            value: output.value,
            script_pubkey: output.address.script_pubkey(),
        }).collect(),
    };

    if change > 0 {
        let change_address = change_address.unwrap_or_else(|| generate_new_address());
        tx.output.push(TxOut {
            value: change,
            script_pubkey: change_address.script_pubkey(),
        });
    }

    sign_transaction(&mut tx, &private_keys)?;

    Ok(tx)
}

fn sign_transaction(tx: &mut Transaction, private_keys: &[PrivateKey]) -> Result<(), Box<dyn Error>> {
    let mut psbt = PartiallySignedTransaction::from_unsigned_tx(tx.clone())?;

    for (i, input) in tx.input.iter().enumerate() {
        let prev_tx = bitcoin_client::get_raw_transaction(&input.previous_output.txid.to_hex())?;
        let script_pubkey = &prev_tx.output[input.previous_output.vout as usize].script_pubkey;

        if script_pubkey.is_p2pkh() {
            psbt.inputs[i].non_witness_utxo = Some(prev_tx.clone());
        } else if script_pubkey.is_p2sh() {
            let redeem_script = fetch_redeem_script(script_pubkey)?;
            psbt.inputs[i].redeem_script = Some(redeem_script);
        } else if script_pubkey.is_v0_p2wpkh() {
            psbt.inputs[i].witness_utxo = Some(prev_tx.output[input.previous_output.vout as usize].clone());
        } else {
            return Err(format!("Unsupported address type for input {}", i).into());
        }
    }

    psbt.sign(private_keys, &Network::Bitcoin)?;

    let final_tx = psbt.extract_tx();
    *tx = final_tx;

    Ok(())
}

pub fn broadcast_transaction(tx: &Transaction) -> Result<String, Box<dyn Error>> {
    let tx_hex = serialize(tx).to_hex();
    bitcoin_client::send_raw_transaction(&tx_hex)
}

fn generate_new_address() -> Address {
    let secp = bitcoin::secp256k1::Secp256k1::new();
    let mut rng = rand::thread_rng();
    let private_key = PrivateKey::new(&secp, &mut rng);
    let public_key = PublicKey::from_private_key(&secp, &private_key);
    Address::p2wpkh(&public_key, Network::Bitcoin).unwrap()
}

fn fetch_redeem_script(script_pubkey: &Script) -> Result<Script, Box<dyn Error>> {
    // This is a placeholder implementation. In a real-world scenario,
    // you would typically fetch this from a database or derive it from other information.
    let redeem_script_hex = "5221030000000000000000000000000000000000000000000000000000000000000001210300000000000000000000000000000000000000000000000000000000000000022103000000000000000000000000000000000000000000000000000000000000000353ae";
    Ok(Script::from_str(redeem_script_hex)?)
}
