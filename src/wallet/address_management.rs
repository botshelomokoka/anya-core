//! This module generates and manages Bitcoin addresses.

use bitcoin::util::address::{Address, AddressType};
use bitcoin::util::key::PublicKey;
use bitcoin::util::bip32::{ExtendedPubKey, ChildNumber};
use bitcoin::network::constants::Network;
use std::str::FromStr;
use std::error::Error;

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
pub fn generate_new_address(key: &str, address_type: &str) -> Result<String, Box<dyn Error>> {
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

/// Validates a Bitcoin address.
///
/// # Arguments
///
/// * `address` - A Bitcoin address string.
///
/// # Returns
///
/// `true` if the address is valid, `false` otherwise.
pub fn validate_address(address: &str) -> bool {
    Address::from_str(address).is_ok()
}

/// Derives a child address from a BIP32 extended public key.
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
pub fn derive_child_address(xpub: &str, index: u32, address_type: &str) -> Result<String, Box<dyn Error>> {
    let extended_pubkey = ExtendedPubKey::from_str(xpub)?;
    let child_pubkey = extended_pubkey.derive_pub(&secp256k1::Secp256k1::new(), &[ChildNumber::from_normal_idx(index)?])?;
    generate_new_address(&child_pubkey.public_key.to_string(), address_type)
}

/// Generates a multi-signature (multisig) address.
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
pub fn generate_multisig_address(public_keys: Vec<&str>, required_signatures: usize) -> Result<String, Box<dyn Error>> {
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
    fn test_generate_new_address() {
        let xpub = "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8";
        let result = generate_new_address(xpub, "p2pkh");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_address() {
        let valid_address = "1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2";
        let invalid_address = "invalid_address";
        assert!(validate_address(valid_address));
        assert!(!validate_address(invalid_address));
    }

    #[test]
    fn test_derive_child_address() {
        let xpub = "xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8";
        let result = derive_child_address(xpub, 0, "p2pkh");
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_multisig_address() {
        let public_keys = vec![
            "0279BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
            "02F9308A019258C31049344F85F89D5229B531C845836F99B08601F113BCE036F9",
        ];
        let result = generate_multisig_address(public_keys, 2);
        assert!(result.is_ok());
    }
}
