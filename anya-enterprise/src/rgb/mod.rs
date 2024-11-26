use rgb_core::{
    contract::{Contract, ContractId},
    schema::{Schema, SchemaId, StateType, Transition},
    validation::{Status, Validity},
};
use bitcoin::{OutPoint, Transaction};
use thiserror::Error;
use log::{info, warn, error};

#[derive(Error, Debug)]
pub enum RGBError {
    #[error("Contract creation failed: {0}")]
    ContractCreationError(String),
    #[error("Asset transfer failed: {0}")]
    TransferError(String),
    #[error("State validation failed: {0}")]
    ValidationError(String),
}

pub struct RGBModule {
    contracts: Vec<Contract>,
    schemas: Vec<Schema>,
}

impl RGBModule {
    pub fn new() -> Self {
        Self {
            contracts: Vec::new(),
            schemas: Vec::new(),
        }
    }

    pub async fn create_asset(&mut self, name: &str, supply: u64) -> Result<ContractId, RGBError> {
        let schema = Schema::rgb20();
        let contract = Contract::new(
            &schema,
            vec![StateType::Amount(supply)],
            name.to_string(),
        ).map_err(|e| RGBError::ContractCreationError(e.to_string()))?;

        let contract_id = contract.contract_id();
        self.contracts.push(contract);
        
        info!("Created RGB asset {} with ID {}", name, contract_id);
        Ok(contract_id)
    }

    pub async fn transfer_asset(
        &mut self,
        contract_id: ContractId,
        amount: u64,
        destination: OutPoint,
    ) -> Result<Transaction, RGBError> {
        let contract = self.get_contract(contract_id)?;
        
        let transition = Transition::new(
            contract,
            vec![StateType::Amount(amount)],
            destination,
        ).map_err(|e| RGBError::TransferError(e.to_string()))?;

        let tx = transition.bitcoin_transaction();
        info!("Created RGB transfer transaction {}", tx.txid());
        
        Ok(tx)
    }

    pub async fn validate_state(&self, contract_id: ContractId) -> Result<Status, RGBError> {
        let contract = self.get_contract(contract_id)?;
        let status = contract.validate()
            .map_err(|e| RGBError::ValidationError(e.to_string()))?;
        
        Ok(status)
    }

    fn get_contract(&self, contract_id: ContractId) -> Result<&Contract, RGBError> {
        self.contracts.iter()
            .find(|c| c.contract_id() == contract_id)
            .ok_or_else(|| RGBError::ValidationError("Contract not found".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::hashes::Hash;

    #[tokio::test]
    async fn test_rgb_asset_lifecycle() {
        let mut rgb = RGBModule::new();
        
        // Create asset
        let contract_id = rgb.create_asset("TEST", 1000).await.unwrap();
        
        // Create transfer
        let destination = OutPoint::new(bitcoin::Txid::all_zeros(), 0);
        let tx = rgb.transfer_asset(contract_id, 100, destination).await.unwrap();
        
        // Validate state
        let status = rgb.validate_state(contract_id).await.unwrap();
        assert!(matches!(status, Status::Valid));
    }
}
