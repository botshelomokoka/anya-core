use std::collections::HashMap;
use std::error::Error;
use log::{info, error};
use dlc::{DlcParty, Oracle, Announcement, Contract, Outcome};
use dlc_messages::{AcceptDlc, OfferDlc, SignDlc};
use dlc::secp_utils::{PublicKey as DlcPublicKey, SecretKey as DlcSecretKey};
use dlc::channel::{Channel, ChannelId};
use dlc::contract::Contract as DlcContract;
use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey};
use bitcoin::network::constants::Network as BitcoinNetwork;

pub struct DLCSupport {
    network: BitcoinNetwork,
    secp: Secp256k1<bitcoin::secp256k1::All>,
    contracts: HashMap<ChannelId, DlcContract>,
}

impl DLCSupport {
    pub fn new(network: BitcoinNetwork) -> Self {
        Self {
            network,
            secp: Secp256k1::new(),
            contracts: HashMap::new(),
        }
    }

    pub fn create_contract(&mut self, oracle: Oracle, announcement: Announcement) -> Result<DlcContract, Box<dyn Error>> {
        let contract = DlcContract::new(oracle, announcement);
        let channel_id = contract.channel_id();
        self.contracts.insert(channel_id, contract.clone());
        Ok(contract)
    }

    pub fn offer_contract(&self, contract: &DlcContract) -> Result<OfferDlc, Box<dyn Error>> {
        // Implementation for offering a contract
        let offer = OfferDlc::new(contract.clone());
        Ok(offer)
    }

    pub fn accept_contract(&self, offer: &OfferDlc) -> Result<AcceptDlc, Box<dyn Error>> {
        // Implementation for accepting a contract
        let accept = AcceptDlc::new(offer.clone());
        Ok(accept)
    }

    pub fn sign_contract(&self, accept: &AcceptDlc) -> Result<SignDlc, Box<dyn Error>> {
        // Implementation for signing a contract
        let sign = SignDlc::new(accept.clone());
        Ok(sign)
    }

    pub fn execute_contract(&mut self, channel_id: &ChannelId, outcome: Outcome) -> Result<(), Box<dyn Error>> {
        if let Some(contract) = self.contracts.get_mut(channel_id) {
            info!("Executing contract with channel ID: {:?}", channel_id);
            contract.execute(outcome)?;
            self.contracts.remove(channel_id);
            Ok(())
        } else {
            error!("Contract with channel ID {:?} not found", channel_id);
            Err("Contract not found".into())
        }
    }

    pub fn get_contract(&self, channel_id: &ChannelId) -> Option<&DlcContract> {
        self.contracts.get(channel_id)
    }

    pub fn list_contracts(&self) -> Vec<&DlcContract> {
        self.contracts.values().collect()
    }

    pub fn close_contract(&mut self, channel_id: &ChannelId) -> Result<(), Box<dyn Error>> {
        if let Some(contract) = self.contracts.remove(channel_id) {
            info!("Closing contract with channel ID: {:?}", channel_id);
            contract.close()?;
            Ok(())
        } else {
            error!("Contract with channel ID {:?} not found", channel_id);
            Err("Contract not found".into())
        }
    }
}
