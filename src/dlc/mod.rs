use rust_dlc::contract::Contract;

pub struct DLCManager {
    contracts: Vec<Contract>,
}

impl DLCManager {
    pub fn new() -> Self {
        Self { contracts: Vec::new() }
    }

    pub fn create_contract(&mut self, contract: Contract) {
        self.contracts.push(contract);
    }

    // Add more DLC-specific methods here
}