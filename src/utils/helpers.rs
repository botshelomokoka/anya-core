//! This module contains helper functions for the Anya project

use bitcoin::consensus::encode::{deserialize, serialize};
use bitcoin::Transaction;
use sha2::{Sha256, Digest};
use std::error::Error;
use bitcoin::util::amount::Amount;

/// Calculates the transaction ID (txid) from a raw transaction hex string
pub fn calculate_txid(tx_hex: &str) -> Result<String, Box<dyn Error>> {
    // 1. Deserialize the transaction hex
    let tx: Transaction = deserialize(&hex::decode(tx_hex)?)?;

    // 2. Calculate the double SHA-256 hash of the transaction
    let tx_bytes = serialize(&tx);
    let tx_hash = Sha256::digest(Sha256::digest(&tx_bytes));

    // 3. Reverse the bytes and convert to hex
    let txid = hex::encode(tx_hash.reverse());

    Ok(txid)
}

/// Converts a satoshi amount to Bitcoin
pub fn convert_satoshi_to_bitcoin(satoshi_amount: u64) -> f64 {
    Amount::from_sat(satoshi_amount).to_btc()
}

/// Converts a Bitcoin amount to satoshis
pub fn convert_bitcoin_to_satoshi(bitcoin_amount: f64) -> u64 {
    Amount::from_btc(bitcoin_amount).unwrap_or(Amount::ZERO).to_sat()
}

/// Validates a Bitcoin address
pub fn validate_bitcoin_address(address: &str) -> bool {
    bitcoin::Address::from_str(address).is_ok()
}

/// Generates a random mnemonic phrase
pub fn generate_mnemonic() -> Result<String, Box<dyn Error>> {
    use bip39::{Mnemonic, MnemonicType};
    let mnemonic = Mnemonic::new(MnemonicType::Words24, bip39::Language::English);
    Ok(mnemonic.phrase().to_string())
}

/// Derives a Bitcoin private key from a mnemonic phrase
pub fn derive_private_key(mnemonic: &str, derivation_path: &str) -> Result<bitcoin::PrivateKey, Box<dyn Error>> {
    use bip39::Mnemonic;
    use bitcoin::util::bip32::{ExtendedPrivKey, DerivationPath};
    use bitcoin::Network;

    let mnemonic = Mnemonic::from_phrase(mnemonic, bip39::Language::English)?;
    let seed = mnemonic.to_seed("");
    let master_key = ExtendedPrivKey::new_master(Network::Bitcoin, &seed)?;
    let derivation_path = DerivationPath::from_str(derivation_path)?;
    let derived_key = master_key.derive_priv(&derivation_path)?;
    
    Ok(derived_key.private_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_txid() {
        let tx_hex = "0100000001c997a5e56e104102fa209c6a852dd90660a20b2d9c352423edce25857fcd3704000000004847304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732e9d624c6c61548ab5fb8cd410220181522ec8eca07de4860a4acdd12909d831cc56cbbac4622082221a8768d1d0901ffffffff0200ca9a3b00000000434104ae1a62fe09c5f51b13905f07f06b99a2f7159b2225f374cd378d71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1baded5c72a704f7e6cd84cac00286bee0000000043410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac00000000";
        let expected_txid = "f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16";
        assert_eq!(calculate_txid(tx_hex).unwrap(), expected_txid);
    }

    #[test]
    fn test_convert_satoshi_to_bitcoin() {
        assert_eq!(convert_satoshi_to_bitcoin(100_000_000), 1.0);
        assert_eq!(convert_satoshi_to_bitcoin(50_000_000), 0.5);
    }

    #[test]
    fn test_convert_bitcoin_to_satoshi() {
        assert_eq!(convert_bitcoin_to_satoshi(1.0), 100_000_000);
        assert_eq!(convert_bitcoin_to_satoshi(0.5), 50_000_000);
    }

    #[test]
    fn test_validate_bitcoin_address() {
        assert!(validate_bitcoin_address("1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2"));
        assert!(!validate_bitcoin_address("invalid_address"));
    }

    #[test]
    fn test_generate_mnemonic() {
        let mnemonic = generate_mnemonic().unwrap();
        assert_eq!(mnemonic.split_whitespace().count(), 24);
    }

    #[test]
    fn test_derive_private_key() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let derivation_path = "m/44'/0'/0'/0/0";
        let private_key = derive_private_key(mnemonic, derivation_path).unwrap();
        assert_eq!(private_key.to_wif(), "L4rK1yDtCWekvXuE6oXD9jCYfFNV2cWRpVuPLBcCU2z8TrisoyY1");
    }
}
