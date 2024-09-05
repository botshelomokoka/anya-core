//! This module generates and manages Bitcoin, Stacks, Web5, DLC, Lightning Network, and libp2p addresses.

use bitcoin::util::address::{Address, AddressType};
use bitcoin::util::key::PublicKey;
use bitcoin::util::bip32::{ExtendedPubKey, ChildNumber};
use bitcoin::network::constants::Network;
use std::str::FromStr;
use std::error::Error;
use stacks_common::types::{StacksAddress, StacksPublicKey};
use web5::did::{DID, DIDDocument};
use rust_dlc::{Oracle, Contract, Outcome, DlcParty, OracleInfo, ContractDescriptor, PayoutFunction};
use rust_lightning::ln::channelmanager::{ChannelManager, ChannelManagerReadArgs};
use rust_lightning::ln::peer_handler::{PeerManager, MessageHandler};
use rust_lightning::routing::router::Router;
use rust_lightning::util::events::EventHandler;
use rust_lightning::util::config::UserConfig;
use rust_lightning::util::logger::Logger;
use rust_lightning::util::persist::Persister;
use rust_bitcoin::blockdata::transaction::Transaction as BitcoinTransaction;
use rust_bitcoin::network::constants::Network as BitcoinNetwork;
use libp2p::{PeerId, Swarm, Transport, identity};
use libp2p::core::upgrade;
use libp2p::tcp::TokioTcpConfig;
use libp2p::mplex::MplexConfig;
use libp2p::noise::{Keypair, NoiseConfig, X25519Spec};
use neon::prelude::*;

/// Generates a new Bitcoin address from a public key.
///
/// # Arguments
///
/// * `key` - A BIP32 extended public key or a compressed public key.
/// * `address_type` - The type of address to generate ('p2pkh', 'p2sh-p2wpkh', or 'p2wpkh').
///
/// # Returns
///
/// A Bitcoin address as a string.
///
/// # Errors
///
/// Returns an error if the provided key or address type is invalid.
pub fn generate_new_bitcoin_address(key: &str, address_type: &str) -> Result<String, Box<dyn Error>> {
    let pubkey = if key.starts_with("xpub") {
        let xpub = ExtendedPubKey::from_str(key)?;
        xpub.public_key
    } else if key.len() == 66 {
        PublicKey::from_str(key)?
    } else {
        return Err("Invalid key. Provide a BIP32 extended public key or a compressed public key string.".into());
    };

    let address = match address_type {
        "p2pkh" => Address::p2pkh(&pubkey, Network::Bitcoin),
        "p2sh-p2wpkh" => Address::p2shwpkh(&pubkey, Network::Bitcoin)?,
        "p2wpkh" => Address::p2wpkh(&pubkey, Network::Bitcoin)?,
        _ => return Err("Invalid address type. Choose from 'p2pkh', 'p2sh-p2wpkh', or 'p2wpkh'.".into()),
    };

    Ok(address.to_string())
}

/// Generates a new Stacks address from a public key.
///
/// # Arguments
///
/// * `public_key` - A Stacks public key.
///
/// # Returns
///
/// A Stacks address as a string.
///
/// # Errors
///
/// Returns an error if the address generation fails.
pub fn generate_new_stacks_address(public_key: &StacksPublicKey) -> Result<String, Box<dyn Error>> {
    let stacks_address = StacksAddress::from_public_keys(
        0,
        &stacks_common::types::AddressHashMode::SerializeP2PKH,
        1,
        &vec![public_key.clone()],
    )?;
    Ok(stacks_address.to_string())
}

/// Generates a new Web5 DID.
///
/// # Returns
///
/// A Web5 DID as a string.
///
/// # Errors
///
/// Returns an error if the DID generation fails.
pub fn generate_new_web5_did() -> Result<String, Box<dyn Error>> {
    let mut runtime = neon::runtime::Runtime::new()?;
    let did = runtime.block_on(|cx| {
        let module = cx.module("./web5_bindings")?;
        let generate_web5_did: Handle<JsFunction> = module.get(&cx, "generateWeb5DID")?.downcast_or_throw(&cx)?;
        let result = generate_web5_did.call(&cx, cx.undefined(), vec![])?;
        let js_string: Handle<JsString> = result.downcast_or_throw(&cx)?;
        Ok(js_string.value(&cx))
    })?;
    Ok(did)
}

/// Generates a new DLC contract address.
///
/// # Arguments
///
/// * `oracle` - The oracle information for the DLC.
/// * `outcomes` - The possible outcomes of the DLC.
///
/// # Returns
///
/// A DLC contract address as a string.
///
/// # Errors
///
/// Returns an error if the contract address generation fails.
pub fn generate_new_dlc_address(oracle: OracleInfo, outcomes: Vec<Outcome>) -> Result<String, Box<dyn Error>> {
    let contract_descriptor = ContractDescriptor::new(oracle, outcomes, PayoutFunction::Winner);
    let contract = Contract::new(contract_descriptor, DlcParty::Offerer)?;
    Ok(contract.funding_script_pubkey().to_string())
}

/// Generates a new Lightning Network node address.
///
/// # Arguments
///
/// * `secret_key` - The secret key for the Lightning node.
///
/// # Returns
///
/// A Lightning Network node address as a string.
///
/// # Errors
///
/// Returns an error if the address generation fails.
pub fn generate_new_lightning_address(secret_key: &[u8; 32]) -> Result<String, Box<dyn Error>> {
    let secp_ctx = secp256k1::Secp256k1::new();
    let public_key = secp256k1::PublicKey::from_secret_key(&secp_ctx, &secp256k1::SecretKey::from_slice(secret_key)?);
    Ok(public_key.serialize().to_hex())
}

/// Generates a new libp2p PeerId.
///
/// # Returns
///
/// A libp2p PeerId as a string.
///
/// # Errors
///
/// Returns an error if the PeerId generation fails.
pub fn generate_new_libp2p_peer_id() -> Result<String, Box<dyn Error>> {
    let keypair = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(keypair.public());
    Ok(peer_id.to_string())
}

/// Validates a Bitcoin address.
///
/// # Arguments
///
/// * `address` - A Bitcoin address string.
///
/// # Returns
///
/// `true` if the address is valid, `false` otherwise.
pub fn validate_bitcoin_address(address: &str) -> bool {
    Address::from_str(address).is_ok()
}

/// Validates a Stacks address.
///
/// # Arguments
///
/// * `address` - A Stacks address string.
///
/// # Returns
///
/// `true` if the address is valid, `false` otherwise.
pub fn validate_stacks_address(address: &str) -> bool {
    StacksAddress::from_string(address).is_ok()
}

/// Validates a Web5 DID.
///
/// # Arguments
///
/// * `did` - A Web5 DID string.
///
/// # Returns
///
/// `true` if the DID is valid, `false` otherwise.
pub fn validate_web5_did(did: &str) -> bool {
    let mut runtime = neon::runtime::Runtime::new().unwrap();
    runtime.block_on(|cx| {
        let module = cx.module("./web5_bindings").unwrap();
        let validate_web5_did: Handle<JsFunction> = module.get(&cx, "validateWeb5DID").unwrap().downcast_or_throw(&cx).unwrap();
        let js_did = cx.string(did);
        let result = validate_web5_did.call(&cx, cx.undefined(), vec![js_did]).unwrap();
        let js_bool: Handle<JsBoolean> = result.downcast_or_throw(&cx).unwrap();
        js_bool.value(&cx)
    })
}

/// Derives a child Bitcoin address from a BIP32 extended public key.
///
/// # Arguments
///
/// * `xpub` - A BIP32 extended public key.
/// * `index` - The index of the child key to derive.
/// * `address_type` - The type of address to generate ('p2pkh', 'p2sh-p2wpkh', or 'p2wpkh').
///
/// # Returns
///
/// A Bitcoin address as a string.
///
/// # Errors
///
/// Returns an error if the derivation or address generation fails.
pub fn derive_child_bitcoin_address(xpub: &str, index: u32, address_type: &str) -> Result<String, Box<dyn Error>> {
    let extended_pubkey = ExtendedPubKey::from_str(xpub)?;
    let child_pubkey = extended_pubkey.derive_pub(&secp256k1::Secp256k1::new(), &[ChildNumber::from_normal_idx(index)?])?;
    generate_new_bitcoin_address(&child_pubkey.public_key.to_string(), address_type)
}

/// Generates a multi-signature (multisig) Bitcoin address.
///
/// # Arguments
///
/// * `public_keys` - A vector of public key strings.
/// * `required_signatures` - The number of signatures required to spend from this address.
///
/// # Returns
///
/// A Bitcoin multisig address as a string.
///
/// # Errors
///
/// Returns an error if the multisig address generation fails.
pub fn generate_multisig_bitcoin_address(public_keys: Vec<&str>, required_signatures: usize) -> Result<String, Box<dyn Error>> {
    if public_keys.len() < required_signatures {
        return Err("Number of required signatures cannot exceed the number of public keys.".into());
    }

    let pubkeys: Result<Vec<PublicKey>, _> = public_keys.iter().map(|&key| PublicKey::from_str(key)).collect();
    let pubkeys = pubkeys?;

    let multisig_script = bitcoin::util::psbt::PsbtSighashType::from_str("SIGHASH_ALL")?
        .to_script_code(&pubkeys, required_signatures)?;

    let address = Address::p2sh(&multisig_script, Network::Bitcoin)?;
    Ok(address.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_new_bitcoin_address() {
        let xpub = "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8";
        let result = generate_new_bitcoin_address(xpub, "p2pkh");
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_new_stacks_address() {
        let public_key = StacksPublicKey::from_hex("02a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1").unwrap();
        let result = generate_new_stacks_address(&public_key);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_new_web5_did() {
        let result = generate_new_web5_did();
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_new_dlc_address() {
        let oracle = OracleInfo::new(vec![1, 2, 3], "test_oracle".to_string());
        let outcomes = vec![Outcome::new("Outcome A", 100), Outcome::new("Outcome B", 200)];
        let result = generate_new_dlc_address(oracle, outcomes);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_new_lightning_address() {
        let secret_key = [0u8; 32];
        let result = generate_new_lightning_address(&secret_key);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_new_libp2p_peer_id() {
        let result = generate_new_libp2p_peer_id();
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_bitcoin_address() {
        let valid_address = "1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2";
        let invalid_address = "invalid_address";
        assert!(validate_bitcoin_address(valid_address));
        assert!(!validate_bitcoin_address(invalid_address));
    }

    #[test]
    fn test_validate_stacks_address() {
        let valid_address = "SP2J6ZY48GV1EZ5V2V5RB9MP66SW86PYKKNRV9EJ7";
        let invalid_address = "invalid_address";
        assert!(validate_stacks_address(valid_address));
        assert!(!validate_stacks_address(invalid_address));
    }

    #[test]
    fn test_validate_web5_did() {
        let valid_did = "did:example:123456789abcdefghi";
        let invalid_did = "invalid_did";
        assert!(validate_web5_did(valid_did));
        assert!(!validate_web5_did(invalid_did));
    }

    #[test]
    fn test_derive_child_bitcoin_address() {
        let xpub = "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8";
        let result = derive_child_bitcoin_address(xpub, 0, "p2pkh");
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_multisig_bitcoin_address() {
        let public_keys = vec![
            "0279BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
            "02F9308A019258C31049344F85F89D5229B531C845836F99B08601F113BCE036F9",
        ];
        let result = generate_multisig_bitcoin_address(public_keys, 2);
        assert!(result.is_ok());
    }
}
