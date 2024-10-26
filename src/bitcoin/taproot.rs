use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey};
use bitcoin::util::taproot::{TaprootBuilder, TaprootSpendInfo};

pub struct TaprootModule {
    secp_context: Secp256k1<bitcoin::secp256k1::All>,
}

impl TaprootModule {
    pub fn new() -> Self {
        Self {
            secp_context: Secp256k1::new(),
        }
    }

    pub fn create_taproot_address(&self, internal_key: &SecretKey) -> Result<String, Box<dyn std::error::Error>> {
        let public_key = PublicKey::from_secret_key(&self.secp_context, internal_key);
        let spend_info = TaprootSpendInfo::new(&self.secp, public_key, TaprootBuilder::new())?;ilder)?;
        let taproot_spend_info = TaprootSpendInfo::new(&self.secp_context, public_key, builder)?;
        Ok(taproot_spend_info.address().to_string())
}