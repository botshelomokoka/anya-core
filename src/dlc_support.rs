use std::collections::HashMap;
use std::collections::HashMap;
use std::error::Error;
use log::{info, error};
use dlc::{DlcParty, Oracle, Announcement, Contract, Outcome};
use dlc_messages::{AcceptDlc, OfferDlc, SignDlc};
use dlc::secp_utils::{PublicKey as DlcPublicKey, SecretKey as DlcSecretKey};
use dlc::channel::{Channel, ChannelId};

use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey};
use bitcoin::network::constants::Network as BitcoinNetwork;


#[derive(Error, Debug)]
pub enum DlcError {
    #[error("Contract with channel ID {0:?} not found")]
    ContractNotFound(ChannelId),
    #[error("An error occurred: {0}")]
    Other(#[from] Box<dyn Error>),
}
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DLCSupportError {
    #[error("Contract with channel ID {0:?} not found")]
    ContractNotFound(ChannelId),
    #[error(transparent)]
    Other(#[from] Box<dyn Error>),
}

pub struct DLCSupport {
    network: BitcoinNetwork,
    secp: Secp256k1<bitcoin::secp256k1::All>,
    contracts: HashMap<ChannelId, Contract>,
    contracts: HashMap<ChannelId, Contract>,
}

impl DLCSupport {
    pub fn new(network: BitcoinNetwork) -> Self {
        Self {
    pub fn new(network: BitcoinNetwork) -> Self {
        Self {
            network,
            secp: Secp256k1::new(),
            contracts: HashMap::new(),
        }
    }

    /// Creates a new DLC contract.
    ///
    /// # Parameters
    /// - `oracle`: The oracle providing the signatures for the contract.
    /// - `announcement`: The announcement containing the oracle's public key and other details.
    ///
    /// # Returns
    /// A result containing the newly created `DlcContract` or an error if the creation fails.
    pub fn create_contract(&mut self, oracle: Oracle, announcement: Announcement) -> Result<Contract, Box<dyn Error>> {
        let contract = Contract::new(oracle, announcement);
        let channel_id = contract.channel_id();
        self.contracts.insert(channel_id, contract.clone());
        Ok(contract)
    }

    pub fn offer_contract(&self, contract: &Contract) -> Result<OfferDlc, Box<dyn Error>> {
    pub fn offer_contract(&self, contract: &Contract) -> Result<OfferDlc, Box<dyn Error>> {
        // Implementation for offering a contract
        let offer = OfferDlc::new(contract.clone());
        Ok(offer)
    }

    pub fn accept_contract(&self, offer: &OfferDlc) -> Result<AcceptDlc, Box<dyn Error>> {
        // Implementation for accepting a contract
        let accept = AcceptDlc::new(offer.clone());
        Ok(accept)
    }

    pub fn execute_contract(&mut self, channel_id: &ChannelId, outcome: Outcome) -> Result<(), DLCSupportError> {
        if let Some(contract) = self.contracts.get_mut(channel_id) {
            info!("Executing contract with channel ID: {:?}", channel_id);
            contract.execute(outcome)?;
            self.contracts.remove(channel_id);
            Ok(())
        } else {
            error!("Contract with channel ID {:?} not found", channel_id);
            Err(DLCSupportError::ContractNotFound(channel_id.clone()))
        }
    }
    
    pub fn close_contract(&mut self, channel_id: &ChannelId) -> Result<(), DlcError> {
        if let Some(contract) = self.contracts.remove(channel_id) {
            info!("Closing contract with channel ID: {:?}", channel_id);
            contract.close().map_err(DlcError::Other)?;
            Ok(())
        } else {
            error!("Contract with channel ID {:?} not found", channel_id);
            Err(DlcError::ContractNotFound(channel_id.clone()))
        }
    }ub fn close_contract(&mut self, channel_id: &ChannelId) -> Result<(), Box<dyn Error>> {
        if let Some(contract) = self.contracts.remove(channel_id) {
            info!("Closing contract with channel ID: {:?}", channel_id);
            contract.close().map_err(DlcError::Other)?;
            Ok(())
        } else {
            error!("Contract with channel ID {:?} not found", channel_id);
            Err(DlcError::ContractNotFound(channel_id.clone()))
        }
    }
}