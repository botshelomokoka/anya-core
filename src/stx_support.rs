use anyhow::Result;
use stacks_core::{
    StacksAddress,
    StacksPublicKey,
    StacksPrivateKey,
    StacksTransaction,
    StacksNetwork,
    StacksEpochId,
    Network,
};
use clarity_repl::clarity::types::QualifiedContractIdentifier;
#[derive(Debug)]
pub enum StacksNetwork {
    Mainnet,
    Testnet,
    Mocknet,
}


#[derive(Debug)]
pub enum StacksNetwork {
    Mainnet,
    Testnet,
    Mocknet,
}


use stacks_rpc_client::{
    StacksRpcClient,
    PoxInfo,
    AccountBalanceResponse,
    TransactionStatus,
};
use log::{info, error};

pub struct STXSupport {
    network: StacksNetwork,
    rpc_client: StacksRpcClient,
}
impl StacksNetwork {
    pub fn get_rpc_url(&self) -> &str {
        match self {
            StacksNetwork::Mainnet => "https://stacks-node-api.mainnet.stacks.co",
            StacksNetwork::Testnet => "https://stacks-node-api.testnet.stacks.co",
            StacksNetwork::Mocknet => "http://localhost:3999",
        }
    }
}


impl STXSupport {
    pub fn get_rpc_url(&self) -> String {
        match self.network {
            Network::Mainnet => "https://stacks-node-api.mainnet.stacks.co".to_string(),
            Network::Testnet => "https://stacks-node-api.testnet.stacks.co".to_string(),
impl StacksNetwork {
    pub fn get_rpc_url(&self) -> &str {
        match self {
            StacksNetwork::Mainnet => "https://stacks-node-api.mainnet.stacks.co",
            StacksNetwork::Testnet => "https://stacks-node-api.testnet.stacks.co",
            StacksNetwork::Mocknet => "http://localhost:3999",
        }
    }
}


impl STXSupport {
    pub fn new(network: StacksNetwork) -> Self {
        let rpc_client = StacksRpcClient::new(network.get_rpc_url());
        info!("Initialized STXSupport with network: {:?}", network);
        Self {
            network,
            rpc_client,
        }
    }

    pub async fn get_balance(&self, address: &StacksAddress) -> Result<u64> {
        let balance = self.rpc_client.get_account_balance(address).await?;
        info!("Fetched balance for address {}: {}", address, balance.stx.balance);
        Ok(balance.stx.balance)
    }
    pub async fn get_network_performance(&self) -> Result<f64> {
        // Implement actual network performance calculation
        // This is a placeholder implementation
        let blocks_per_second = self.rpc_client.get_network_block_rate().await?;
        let transactions_per_block = self.rpc_client.get_average_transactions_per_block().await?;
        let performance = blocks_per_second * transactions_per_block as f64;
        info!("Calculated network performance: {}", performance);
        Ok(performance)
    }
    pub async fn get_pox_info(&self) -> Result<PoxInfo> {
        let pox_info = self.rpc_client.get_pox_info().await?;
        info!("Fetched PoX info: {:?}", pox_info);
        Ok(pox_info)
    }

    pub async fn deploy_contract(&self, contract: QualifiedContractIdentifier, code: &str, sender: &StacksPrivateKey) -> Result<TransactionStatus> {
        let tx = StacksTransaction::smart_contract(
            sender,
            contract,
            code.to_string(),
            self.network,
        );
        let tx_hex = tx.serialize_hex()?;
        let response = self.rpc_client.broadcast_transaction(&tx_hex).await?;
        info!("Deployed contract: {:?}", response);
        Ok(response)
        let tx = StacksTransaction::smart_contract(
            sender,
            contract,
            code.to_string(),
            self.network,
        );
        let tx_hex = tx.serialize_hex()?;
        let response = self.rpc_client.broadcast_transaction(&tx_hex).await?;
        info!("Deployed contract: {:?}", response);
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stacks_core::StacksNetwork;
    use stacks_rpc_client::AccountBalanceResponse;
    use tokio::runtime::Runtime;
    use stacks_core::StacksNetwork;
    use stacks_rpc_client::AccountBalanceResponse;
    use tokio::runtime::Runtime;

    #[tokio::test]
    async fn test_get_balance() {
        let network = StacksNetwork::Testnet;
        let stx_support = STXSupport::new(network);
        let address = StacksAddress::from_string("SP2C2WJ2WJ2WJ2WJ2WJ2WJ2WJ2WJ2WJ2WJ2WJ2WJ2").unwrap();
        let balance = stx_support.get_balance(&address).await;
        assert!(balance.is_ok());
    }

    #[tokio::test]
    async fn test_get_network_performance() {
        let network = StacksNetwork::Testnet;
        let stx_support = STXSupport::new(network);
        let stx_support = STXSupport::new(network);
        let performance = stx_support.get_network_performance().await;
        assert!(performance.is_ok());
    }

    #[tokio::test]
    async fn test_get_pox_info() {
        let network = StacksNetwork::Testnet;
        let stx_support = STXSupport::new(network);
        let pox_info = stx_support.get_pox_info().await;
        assert!(pox_info.is_ok());
    }

    #[tokio::test]
    async fn test_deploy_contract() {
        let network = StacksNetwork::Testnet;
        let stx_support = STXSupport::new(network);
        let contract = QualifiedContractIdentifier::from_str("SP2C2WJ2WJ2WJ2WJ2WJ2WJ2WJ2WJ2WJ2WJ2WJ2WJ2.contract-name").unwrap();
        let code = "(define-public (hello-world) (ok \"hello world\"))";
        let sender = StacksPrivateKey::from_string("your-private-key-here").unwrap();
        let result = stx_support.deploy_contract(contract, code, &sender).await;
        assert!(result.is_ok()); // The method should succeed, so we check for Ok
    }
}
        let network = StacksNetwork::Testnet;
        let stx_support = STXSupport::new(network);
        let address = StacksAddress::from_string("SP2C2WJ2WJ2WJ2WJ2WJ2WJ2WJ2WJ2WJ2WJ2WJ2WJ2").unwrap();
        let balance = stx_support.get_balance(&address).await;
        assert!(balance.is_ok());
    }

    #[tokio::test]
    async fn test_get_network_performance() {
        let network = StacksNetwork::Testnet;
        let stx_support = STXSupport::new(network);
        let stx_support = STXSupport::new(network);
        let performance = stx_support.get_network_performance().await;
        assert!(performance.is_ok());
    }

    #[tokio::test]
    async fn test_get_pox_info() {
        let network = StacksNetwork::Testnet;
        let stx_support = STXSupport::new(network);
        let pox_info = stx_support.get_pox_info().await;
        assert!(pox_info.is_ok());
    }

    #[tokio::test]
    async fn test_deploy_contract() {
        let network = StacksNetwork::Testnet;
        let stx_support = STXSupport::new(network);
        let contract = QualifiedContractIdentifier::from_str("SP2C2WJ2WJ2WJ2WJ2WJ2WJ2WJ2WJ2WJ2WJ2WJ2WJ2.contract-name").unwrap();
        let code = "(define-public (hello-world) (ok \"hello world\"))";
        let sender = StacksPrivateKey::from_string("your-private-key-here").unwrap();
        let result = stx_support.deploy_contract(contract, code, &sender).await;
        assert!(result.is_ok()); // The method should succeed, so we check for Ok
    }
}