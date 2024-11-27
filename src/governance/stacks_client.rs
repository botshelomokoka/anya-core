use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use clarity_sdk::{
    clarity_type::ClarityType,
    types::{QualifiedContractIdentifier, Value, ToClarityValue},
};
use stacks_rpc_client::{
    PrincipalData,
    StacksRpc,
    TransactionVersion,
    StacksTransaction,
    PostCondition,
    FeeEstimator,
};
use stacks_common::types::{StacksAddress, StacksNetwork, ChainID};

#[derive(Debug, Clone)]
pub struct StacksContractClient {
    network: Arc<StacksNetwork>,
    wallet: Arc<StacksWallet>,
    contract_address: StacksAddress,
    rpc_client: Arc<StacksRpc>,
    fee_estimator: Arc<FeeEstimator>,
    nonce_manager: Arc<RwLock<NonceManager>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractCallResponse {
    pub txid: String,
    pub status: String,
    pub result: Option<Value>,
    pub post_conditions: Vec<PostCondition>,
}

impl StacksContractClient {
    pub fn new(
        network: StacksNetwork,
        wallet: StacksWallet,
        contract_address: StacksAddress,
        rpc_url: &str,
    ) -> Self {
        let rpc_client = StacksRpc::new(rpc_url);
        let fee_estimator = FeeEstimator::new(rpc_client.clone());
        let nonce_manager = NonceManager::new(wallet.address());
        
        Self {
            network: Arc::new(network),
            wallet: Arc::new(wallet),
            contract_address,
            rpc_client: Arc::new(rpc_client),
            fee_estimator: Arc::new(fee_estimator),
            nonce_manager: Arc::new(RwLock::new(nonce_manager)),
        }
    }

    pub async fn call_contract<T: ToClarityValue>(
        &self,
        contract_name: &str,
        function_name: &str,
        args: &[T],
        post_conditions: Vec<PostCondition>,
    ) -> Result<ContractCallResponse, Error> {
        let nonce = self.get_next_nonce().await?;
        let fee = self.estimate_fee(contract_name, function_name, args).await?;
        
        let tx = TransactionBuilder::contract_call()
            .version(self.get_tx_version())
            .chain_id(self.network.chain_id())
            .sender(self.wallet.address())
            .contract_name(contract_name)
            .function_name(function_name)
            .function_args(args.iter().map(|a| a.to_clarity_value()).collect())
            .nonce(nonce)
            .fee_rate(fee)
            .post_conditions(post_conditions)
            .build();

        let signed_tx = self.wallet.sign_transaction(tx)?;
        let response = self.rpc_client.broadcast_transaction(signed_tx).await?;
        
        Ok(ContractCallResponse {
            txid: response.txid,
            status: response.status,
            result: response.result,
            post_conditions: response.post_conditions,
        })
    }

    pub async fn call_read_only<T: ToClarityValue, R: TryFrom<Value>>(
        &self,
        contract_name: &str,
        function_name: &str,
        args: &[T],
    ) -> Result<R, Error> {
        let result = self.rpc_client
            .call_read_only(
                &self.contract_address,
                contract_name,
                function_name,
                args.iter().map(|a| a.to_clarity_value()).collect(),
            )
            .await?;
            
        R::try_from(result).map_err(|_| Error::DeserializationError)
    }

    pub async fn deploy_contract(
        &self,
        contract_id: &QualifiedContractIdentifier,
        source_code: &str,
        version: TransactionVersion,
    ) -> Result<ContractCallResponse, Error> {
        let nonce = self.get_next_nonce().await?;
        let fee = self.estimate_deploy_fee(source_code).await?;
        
        let tx = TransactionBuilder::deploy_contract()
            .version(version)
            .chain_id(self.network.chain_id())
            .contract_name(contract_id.name.clone())
            .code_body(source_code.to_string())
            .nonce(nonce)
            .fee_rate(fee)
            .build();

        let signed_tx = self.wallet.sign_transaction(tx)?;
        let response = self.rpc_client.broadcast_transaction(signed_tx).await?;
        
        Ok(ContractCallResponse {
            txid: response.txid,
            status: response.status,
            result: response.result,
            post_conditions: response.post_conditions,
        })
    }

    async fn get_next_nonce(&self) -> Result<u64, Error> {
        let mut nonce_manager = self.nonce_manager.write().await;
        let current_nonce = self.rpc_client
            .get_nonce(&self.wallet.address())
            .await?;
            
        nonce_manager.update_nonce(current_nonce);
        Ok(nonce_manager.next_nonce())
    }

    async fn estimate_fee<T: ToClarityValue>(
        &self,
        contract_name: &str,
        function_name: &str,
        args: &[T],
    ) -> Result<u64, Error> {
        self.fee_estimator
            .estimate_contract_call(
                contract_name,
                function_name,
                args.iter().map(|a| a.to_clarity_value()).collect(),
            )
            .await
            .map_err(Error::FeeEstimationError)
    }

    async fn estimate_deploy_fee(&self, source_code: &str) -> Result<u64, Error> {
        self.fee_estimator
            .estimate_contract_deploy(source_code)
            .await
            .map_err(Error::FeeEstimationError)
    }

    fn get_tx_version(&self) -> TransactionVersion {
        match self.network.chain_id() {
            ChainID::Mainnet => TransactionVersion::Mainnet,
            _ => TransactionVersion::Testnet,
        }
    }
}

#[derive(Debug)]
struct NonceManager {
    address: StacksAddress,
    current_nonce: u64,
}

impl NonceManager {
    fn new(address: StacksAddress) -> Self {
        Self {
            address,
            current_nonce: 0,
        }
    }

    fn update_nonce(&mut self, chain_nonce: u64) {
        self.current_nonce = std::cmp::max(self.current_nonce, chain_nonce);
    }

    fn next_nonce(&mut self) -> u64 {
        let nonce = self.current_nonce;
        self.current_nonce += 1;
        nonce
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("RPC error: {0}")]
    RpcError(String),
    
    #[error("Transaction error: {0}")]
    TransactionError(String),
    
    #[error("Wallet error: {0}")]
    WalletError(String),
    
    #[error("Fee estimation error: {0}")]
    FeeEstimationError(String),
    
    #[error("Deserialization error")]
    DeserializationError,
    
    #[error("Network error: {0}")]
    NetworkError(String),
}
