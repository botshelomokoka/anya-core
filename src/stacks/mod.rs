use clarity_repl::repl::Session;
use stacks_rpc_client::StacksRpc;
use crate::Result;
use serde_json::Value;

pub struct StacksInterface {
    rpc: StacksRpc,
    session: Session,
}

impl StacksInterface {
    pub fn new(url: &str) -> Result<Self> {
        let rpc = StacksRpc::new(url);
        let session = Session::new(None);
        Ok(Self { rpc, session })
    }

    pub async fn get_account(&self, address: &str) -> Result<Value> {
        self.validate_input(address)?;
        self.rpc.get_account(address).await
    }

    pub async fn get_balance(&self, address: &str) -> Result<u64> {
        self.validate_input(address)?;
        let account = self.get_account(address).await?;
        account["balance"].as_str().unwrap().parse().map_err(|e| e.into())
    }

    pub async fn get_nonce(&self, address: &str) -> Result<u64> {
        self.validate_input(address)?;
        let account = self.get_account(address).await?;
        account["nonce"].as_u64().ok_or_else(|| "Invalid nonce".into())
    }

    pub async fn get_info(&self) -> Result<Value> {
        self.rpc.get_info().await
    }

    pub async fn get_block_by_height(&self, height: u64) -> Result<Value> {
        self.rpc.get_block_by_height(height).await
    }

    pub async fn get_block_by_hash(&self, hash: &str) -> Result<Value> {
        self.validate_input(hash)?;
        self.rpc.get_block_by_hash(hash).await
    }

    pub async fn get_transaction(&self, txid: &str) -> Result<Value> {
        self.validate_input(txid)?;
        self.rpc.get_transaction(txid).await
    }

    pub async fn broadcast_transaction(&self, tx: &str) -> Result<String> {
        self.validate_input(tx)?;
        self.rpc.broadcast_transaction(tx).await
    }

    pub async fn call_read_only_function(&self, contract_address: &str, contract_name: &str, function_name: &str, function_args: Vec<Value>) -> Result<Value> {
        self.validate_input(contract_address)?;
        self.validate_input(contract_name)?;
        self.validate_input(function_name)?;
        self.rpc.call_read_only_function(contract_address, contract_name, function_name, function_args).await
    }

    fn validate_input(&self, input: &str) -> Result<()> {
        if input.is_empty() {
            return Err("Input cannot be empty".into());
        }
        // Additional validation logic...
        Ok(())
    }
}