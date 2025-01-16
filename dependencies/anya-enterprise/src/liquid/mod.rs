use elements::{
    confidential::{Asset, Value, Nonce},
    encode::ElementsEncodable,
    issuance::{AssetIssuance, ContractHash},
    transaction::{ElementsTransaction, TxOut, TxIn},
    Address, OutPoint,
};
use bitcoin::secp256k1::{Secp256k1, SecretKey};
use thiserror::Error;
use log::{info, warn, error};
use metrics::{counter, gauge};

#[derive(Error, Debug)]
pub enum LiquidError {
    #[error("Asset issuance failed: {0}")]
    IssuanceError(String),
    #[error("Transaction creation failed: {0}")]
    TransactionError(String),
    #[error("Confidential transfer failed: {0}")]
    ConfidentialError(String),
    #[error("Blinding failed: {0}")]
    BlindingError(String),
}

pub struct LiquidModule {
    secp: Secp256k1<elements::All>,
    issuance_keys: Vec<SecretKey>,
    blinding_keys: Vec<SecretKey>,
    metrics: LiquidMetrics,
}

impl LiquidModule {
    pub fn new() -> Result<Self, LiquidError> {
        Ok(Self {
            secp: Secp256k1::new(),
            issuance_keys: Vec::new(),
            blinding_keys: Vec::new(),
            metrics: LiquidMetrics::new(),
        })
    }

    pub async fn issue_asset(
        &mut self,
        amount: u64,
        name: &str,
        destination: &Address,
    ) -> Result<(Asset, ElementsTransaction), LiquidError> {
        let issuance_key = SecretKey::new(&mut rand::thread_rng());
        let blinding_key = SecretKey::new(&mut rand::thread_rng());
        
        let contract = ContractHash::from_str(name)
            .map_err(|e| LiquidError::IssuanceError(e.to_string()))?;

        let issuance = AssetIssuance::new(
            &self.secp,
            &issuance_key,
            amount,
            Some(contract),
        );

        let confidential_value = Value::new_confidential(
            &self.secp,
            &blinding_key,
            amount,
        ).map_err(|e| LiquidError::ConfidentialError(e.to_string()))?;

        let txout = TxOut {
            asset: issuance.asset,
            value: confidential_value,
            nonce: Nonce::from_slice(&[0; 32])?,
            script_pubkey: destination.script_pubkey(),
            witness: elements::TxOutWitness::default(),
        };

        let tx = ElementsTransaction {
            version: 2,
            lock_time: 0,
            input: vec![],
            output: vec![txout],
            issuance: vec![issuance],
        };

        self.issuance_keys.push(issuance_key);
        self.blinding_keys.push(blinding_key);

        self.metrics.record_issuance(amount);
        info!("Issued Liquid asset {} with amount {}", name, amount);
        
        Ok((issuance.asset, tx))
    }

    pub async fn create_confidential_transfer(
        &self,
        asset: Asset,
        amount: u64,
        from: OutPoint,
        to: &Address,
    ) -> Result<ElementsTransaction, LiquidError> {
        let blinding_key = SecretKey::new(&mut rand::thread_rng());
        
        let confidential_value = Value::new_confidential(
            &self.secp,
            &blinding_key,
            amount,
        ).map_err(|e| LiquidError::ConfidentialError(e.to_string()))?;

        let txout = TxOut {
            asset,
            value: confidential_value,
            nonce: Nonce::from_slice(&[0; 32])?,
            script_pubkey: to.script_pubkey(),
            witness: elements::TxOutWitness::default(),
        };

        let tx = ElementsTransaction {
            version: 2,
            lock_time: 0,
            input: vec![TxIn::new(from)],
            output: vec![txout],
            issuance: vec![],
        };

        self.metrics.record_transfer(amount);
        info!("Created confidential transfer for asset {}", asset);
        
        Ok(tx)
    }

    pub async fn verify_confidential_amount(
        &self,
        value: &Value,
        expected_amount: u64,
    ) -> Result<bool, LiquidError> {
        value.verify_confidential_value(expected_amount)
            .map_err(|e| LiquidError::ConfidentialError(e.to_string()))
    }
}

struct LiquidMetrics {
    issuance_count: Counter,
    transfer_count: Counter,
    total_issued: Gauge,
    total_transferred: Gauge,
}

impl LiquidMetrics {
    fn new() -> Self {
        Self {
            issuance_count: counter!("liquid_issuance_total"),
            transfer_count: counter!("liquid_transfer_total"),
            total_issued: gauge!("liquid_total_issued"),
            total_transferred: gauge!("liquid_total_transferred"),
        }
    }

    fn record_issuance(&self, amount: u64) {
        self.issuance_count.increment(1);
        self.total_issued.add(amount as f64);
    }

    fn record_transfer(&self, amount: u64) {
        self.transfer_count.increment(1);
        self.total_transferred.add(amount as f64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_liquid_asset_lifecycle() {
        let mut liquid = LiquidModule::new().unwrap();
        
        let address = Address::from_str(
            "AzpwvvBWoNrwxm3E9RYm2kfYqJPFQm6yQHmDKHoJ3F1K9E1YUCVw"
        ).unwrap();
        
        // Issue asset
        let (asset, issuance_tx) = liquid.issue_asset(1000, "TEST", &address).await.unwrap();
        
        // Create confidential transfer
        let from = OutPoint::new(issuance_tx.txid(), 0);
        let transfer_tx = liquid.create_confidential_transfer(
            asset,
            500,
            from,
            &address,
        ).await.unwrap();
        
        // Verify amount
        let is_valid = liquid.verify_confidential_amount(
            &transfer_tx.output[0].value,
            500,
        ).await.unwrap();
        
        assert!(is_valid);
    }
}
