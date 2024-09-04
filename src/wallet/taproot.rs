//! This module provides Taproot asset functionality for Anya Wallet, with a focus on Taro support.

use bitcoin::consensus::encode::serialize;
use bitcoin::hashes::sha256::Hash as Sha256;
use bitcoin::hashes::Hash;
use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey};
use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey};
use bitcoin::{Address, Network, OutPoint, Script, Transaction, TxIn, TxOut};
use bitcoin::taproot::{TapTweakHash, TaprootBuilder, LeafVersion, ControlBlock, TapLeafHash, TapBranchHash};
use std::str::FromStr;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaroAsset {
    asset_id: [u8; 32],
    amount: u64,
    metadata: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct TaroCommitment {
    assets: Vec<TaroAsset>,
}

#[derive(Error, Debug)]
pub enum TaroError {
    #[error("Invalid script")]
    InvalidScript,
    #[error("Bitcoin error: {0}")]
    BitcoinError(#[from] bitcoin::Error),
    #[error("Secp256k1 error: {0}")]
    Secp256k1Error(#[from] bitcoin::secp256k1::Error),
}

impl TaroAsset {
    pub fn new(asset_id: [u8; 32], amount: u64, metadata: Option<Vec<u8>>) -> Self {
        TaroAsset {
            asset_id,
            amount,
            metadata,
        }
    }
}

impl TaroCommitment {
    pub fn from_assets(assets: &[TaroAsset]) -> Self {
        TaroCommitment {
            assets: assets.to_vec(),
        }
    }

    pub fn to_script_pubkey(&self, address: &Script) -> Script {
        let mut builder = TaprootBuilder::new();
        for (i, asset) in self.assets.iter().enumerate() {
            let leaf = Script::new_v1_p2tr(&Secp256k1::new(), &asset.asset_id.into(), None);
            builder = builder.add_leaf(i as u8, leaf).expect("Failed to add leaf");
        }
        let (output_key, _) = builder.finalize(&Secp256k1::new(), address.as_bytes().into()).expect("Failed to finalize Taproot");
        Script::new_v1_p2tr(&Secp256k1::new(), &output_key, None)
    }

    pub fn parse_from_script_pubkey(script: &Script) -> Result<Self, TaroError> {
        if !script.is_v1_p2tr() {
            return Err(TaroError::InvalidScript);
        }
        
        // This is a simplified parsing. In a real implementation, you'd need to
        // reconstruct the Taproot tree and extract the asset information.
        Ok(TaroCommitment { assets: vec![] })
    }
}

use crate::network::bitcoin_client;

/// Creates a Taproot output containing a Taro asset.
pub fn create_taproot_asset_output(output_data: &OutputData) -> Result<TxOut, TaroError> {
    let asset = TaroAsset::new(output_data.asset_id, output_data.amount, output_data.metadata.clone());
    let commitment = TaroCommitment::from_assets(&[asset]);
    let address = Address::from_str(&output_data.address)?;
    let taproot_script = commitment.to_script_pubkey(&address.script_pubkey());

    Ok(TxOut {
        value: output_data.value,
        script_pubkey: taproot_script,
    })
}

/// Signs a Taproot input spending a Taro asset.
pub fn sign_taproot_input(
    tx: &mut Transaction,
    input_index: usize,
    private_key: &ExtendedPrivKey,
    script_path: &Script,
    leaf_version: LeafVersion,
    control_block: &ControlBlock,
) -> Result<Vec<Vec<u8>>, TaroError> {
    let secp = Secp256k1::new();

    let sighash = tx.sighash_taproot(
        input_index,
        &[],
        TapTweakHash::from_script::<Sha256>(script_path, leaf_version),
        bitcoin::sighash::TapSighashType::All,
    );

    let signature = private_key.sign_taproot(&secp, sighash.as_byte_array())?;

    Ok(vec![
        signature.to_vec(),
        script_path.to_bytes(),
        control_block.serialize(),
    ])
}

/// Fetches UTXOs containing Taro assets associated with an address
pub fn get_taproot_asset_utxos(address: &str) -> Result<Vec<TaprootAssetUtxo>, TaroError> {
    let utxos = bitcoin_client::get_utxos(address)?;
    let mut taproot_asset_utxos = Vec::new();

    for utxo in utxos {
        let tx = bitcoin_client::get_raw_transaction(&utxo.txid)?;
        let script_pubkey = &tx.output[utxo.vout as usize].script_pubkey;

        if script_pubkey.is_v1_p2tr() {
            if let Ok(commitment) = TaroCommitment::parse_from_script_pubkey(script_pubkey) {
                for asset in commitment.assets {
                    taproot_asset_utxos.push(TaprootAssetUtxo {
                        txid: utxo.txid,
                        vout: utxo.vout,
                        value: utxo.value,
                        asset_id: asset.asset_id,
                        amount: asset.amount,
                    });
                }
            }
        }
    }

    Ok(taproot_asset_utxos)
}

#[derive(Debug, Clone)]
pub struct OutputData {
    pub address: String,
    pub value: u64,
    pub asset_id: [u8; 32],
    pub amount: u64,
    pub metadata: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct TaprootAssetUtxo {
    pub txid: bitcoin::Txid,
    pub vout: u32,
    pub value: u64,
    pub asset_id: [u8; 32],
    pub amount: u64,
}

/// Generates a new Taproot address for a Taro asset
pub fn generate_taproot_address(
    extended_public_key: &ExtendedPubKey,
    asset_id: &[u8; 32],
    network: Network,
) -> Result<Address, TaroError> {
    let secp = Secp256k1::new();
    let internal_key = extended_public_key.public_key;
    
    let asset_script = Script::new_v1_p2tr(&secp, asset_id, None);
    let mut builder = TaprootBuilder::new();
    builder = builder.add_leaf(0, asset_script)?;
    
    let (output_key, _) = builder.finalize(&secp, internal_key.x_only_public_key().0).expect("Taproot finalization failed");
    
    Ok(Address::p2tr(&secp, output_key, None, network))
}

/// Verifies a Taproot signature for a Taro asset transaction
pub fn verify_taproot_signature(
    tx: &Transaction,
    input_index: usize,
    public_key: &PublicKey,
    signature: &[u8],
    script_path: &Script,
    leaf_version: LeafVersion,
    control_block: &ControlBlock,
) -> Result<bool, TaroError> {
    let secp = Secp256k1::new();

    let sighash = tx.sighash_taproot(
        input_index,
        &[],
        TapTweakHash::from_script::<Sha256>(script_path, leaf_version),
        bitcoin::sighash::TapSighashType::All,
    );

    let msg = bitcoin::secp256k1::Message::from_slice(sighash.as_byte_array())?;
    let sig = bitcoin::secp256k1::Signature::from_slice(signature)?;

    Ok(secp.verify_schnorr(&sig, &msg, &public_key.x_only_public_key().0).is_ok())
}
