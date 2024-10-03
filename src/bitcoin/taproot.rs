use bitcoin::util::taproot::{TaprootBuilder, TaprootSpendInfo};
use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey};

pub struct TaprootModule {
    secp: Secp256k1<bitcoin::secp256k1::All>,
}

impl TaprootModule {
    pub fn new() -> Self {
        Self {
            secp: Secp256k1::new(),
        }
    }

    pub fn create_taproot_address(&self, internal_key: &SecretKey) -> Result<String, Box<dyn std::error::Error>> {
        let public_key = PublicKey::from_secret_key(&self.secp, internal_key);
        let builder = TaprootBuilder::new();
        let spend_info = TaprootSpendInfo::new(&self.secp, public_key, builder)?;
        Ok(spend_info.address().to_string())
    }
}