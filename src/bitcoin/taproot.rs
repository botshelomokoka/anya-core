use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey, Message};
use bitcoin::util::taproot::{TaprootBuilder, TaprootSpendInfo, LeafVersion};
use bitcoin::util::key::KeyPair;
use bitcoin::util::schnorr::{TapTweak, UntweakedPublicKey, TweakedPublicKey};
use thiserror::Error;
use log::{info, warn};

#[derive(Error, Debug)]
pub enum TaprootError {
    #[error("Key generation error: {0}")]
    KeyGenError(String),
    #[error("Script error: {0}")]
    ScriptError(String),
    #[error("Signature error: {0}")]
    SignatureError(String),
}

pub struct TaprootModule {
    secp: Secp256k1<bitcoin::secp256k1::All>,
    internal_key: KeyPair,
}

impl TaprootModule {
    pub fn new() -> Result<Self, TaprootError> {
        let secp = Secp256k1::new();
        let internal_key = KeyPair::new(&secp, &mut rand::thread_rng());
        
        Ok(Self {
            secp,
            internal_key,
        })
    }

    pub fn create_taproot_spend_info(&self, scripts: Vec<(bitcoin::Script, u8)>) -> Result<TaprootSpendInfo, TaprootError> {
        let mut builder = TaprootBuilder::new();
        
        // Add all scripts to the tree
        for (script, weight) in scripts {
            builder = builder.add_leaf(weight, script)
                .map_err(|e| TaprootError::ScriptError(e.to_string()))?;
        }

        // Finalize the Taproot construction
        let spend_info = builder.finalize(
            &self.secp,
            UntweakedPublicKey::from_keypair(&self.internal_key)
        ).map_err(|e| TaprootError::ScriptError(e.to_string()))?;

        Ok(spend_info)
    }

    pub fn sign_taproot(&self, msg: &[u8], spend_info: &TaprootSpendInfo) -> Result<bitcoin::schnorr::Signature, TaprootError> {
        let msg = Message::from_slice(msg)
            .map_err(|e| TaprootError::SignatureError(e.to_string()))?;
        
        let tweaked_keypair = self.internal_key.tap_tweak(&self.secp, spend_info.merkle_root());
        
        let signature = self.secp.sign_schnorr(
            &msg,
            &tweaked_keypair,
            &mut rand::thread_rng()
        );

        Ok(signature)
    }

    pub fn verify_taproot_signature(
        &self,
        msg: &[u8],
        signature: &bitcoin::schnorr::Signature,
        public_key: &TweakedPublicKey,
    ) -> Result<bool, TaprootError> {
        let msg = Message::from_slice(msg)
            .map_err(|e| TaprootError::SignatureError(e.to_string()))?;

        Ok(self.secp.verify_schnorr(signature, &msg, &public_key).is_ok())
    }

    pub fn create_taproot_address(&self, spend_info: &TaprootSpendInfo) -> Result<bitcoin::Address, TaprootError> {
        let tweaked_pubkey = spend_info.output_key();
        let address = bitcoin::Address::p2tr(
            &self.secp,
            tweaked_pubkey,
            None,
            bitcoin::Network::Bitcoin
        );
        
        Ok(address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::Script;

    #[test]
    fn test_taproot_workflow() {
        let taproot = TaprootModule::new().unwrap();
        
        // Create test scripts
        let script1 = Script::new();
        let script2 = Script::new();
        let scripts = vec![(script1, 1), (script2, 1)];
        
        // Test spend info creation
        let spend_info = taproot.create_taproot_spend_info(scripts).unwrap();
        
        // Test signing
        let msg = b"test message";
        let signature = taproot.sign_taproot(msg, &spend_info).unwrap();
        
        // Test verification
        let result = taproot.verify_taproot_signature(
            msg,
            &signature,
            &spend_info.output_key()
        ).unwrap();
        
        assert!(result);
    }
}
