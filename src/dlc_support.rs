use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, error};
use dlc::{DlcManager, OracleInfo, Offer, Contract, Outcome};
use bitcoin::{Network as BitcoinNetwork, Address as BitcoinAddress, Transaction};
use lightning::util::config::UserConfig;
use crate::bitcoin_support::BitcoinSupport;

pub struct DLCSupport {
    dlc_manager: Arc<Mutex<DlcManager>>,
    bitcoin_support: Arc<BitcoinSupport>,
    network: BitcoinNetwork,
}

impl DLCSupport {
    pub async fn new(bitcoin_support: Arc<BitcoinSupport>, network: BitcoinNetwork) -> Result<Self, Box<dyn Error>> {
        let dlc_manager = Arc::new(Mutex::new(DlcManager::new(network)));
        
        Ok(DLCSupport {
            dlc_manager,
            bitcoin_support,
            network,
        })
    }

    pub async fn create_offer(&self, oracle_info: OracleInfo, contract: Contract) -> Result<Offer, Box<dyn Error>> {
        let offer = self.dlc_manager.lock().await.create_offer(oracle_info, contract)?;
        info!("Created DLC offer");
        Ok(offer)
    }

    pub async fn accept_offer(&self, offer: Offer) -> Result<Contract, Box<dyn Error>> {
        let contract = self.dlc_manager.lock().await.accept_offer(offer)?;
        info!("Accepted DLC offer");
        Ok(contract)
    }

    pub async fn sign_contract(&self, contract: Contract) -> Result<Transaction, Box<dyn Error>> {
        let signed_tx = self.dlc_manager.lock().await.sign_contract(contract)?;
        info!("Signed DLC contract");
        Ok(signed_tx)
    }

    pub async fn execute_contract(&self, contract: Contract, outcome: Outcome) -> Result<Transaction, Box<dyn Error>> {
        let execution_tx = self.dlc_manager.lock().await.execute_contract(contract, outcome)?;
        info!("Executed DLC contract");
        Ok(execution_tx)
    }

    pub async fn get_contract_status(&self, contract_id: &str) -> Result<String, Box<dyn Error>> {
        let status = self.dlc_manager.lock().await.get_contract_status(contract_id)?;
        Ok(status)
    }
}
