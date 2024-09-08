use stacks_common::types::StacksAddress;
use stacks_common::util::hash::Sha256Sum;
use stacks_transactions::{
    TransactionVersion, TransactionAuth, TransactionPayload,
    StacksTransaction, SingleSigSpendingCondition, TransactionAnchorMode,
};
use stacks_rpc_client::StacksRpcClient;

pub struct StxSupport {
    rpc_client: StacksRpcClient,
}

impl StxSupport {
    pub fn new(node_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let rpc_client = StacksRpcClient::new(node_url)?;
        Ok(Self { rpc_client })
    }

    pub async fn get_balance(&self, address: &StacksAddress) -> Result<u64, Box<dyn std::error::Error>> {
        let balance = self.rpc_client.get_account_balance(address).await?;
        Ok(balance)
    }

    pub async fn transfer_stx(
        &self,
        sender: &StacksAddress,
        recipient: &StacksAddress,
        amount: u64,
        fee: u64,
        nonce: u64,
        private_key: &[u8; 32],
    ) -> Result<Sha256Sum, Box<dyn std::error::Error>> {
        let spending_condition = SingleSigSpendingCondition::new(nonce, fee);
        let auth = TransactionAuth::Standard(spending_condition);
        
        let payload = TransactionPayload::TokenTransfer(
            recipient.clone(),
            amount,
            TokenTransferMemo([0u8; 34]),
        );

        let tx = StacksTransaction::new(
            TransactionVersion::Mainnet,
            auth,
            payload,
        );

        let signed_tx = tx.sign(private_key)?;
        let tx_hash = self.rpc_client.broadcast_transaction(&signed_tx).await?;
        
        Ok(tx_hash)
    }

    pub async fn call_contract_function(
        &self,
        contract_address: &StacksAddress,
        contract_name: &str,
        function_name: &str,
        function_args: Vec<Value>,
        sender: &StacksAddress,
        fee: u64,
        nonce: u64,
        private_key: &[u8; 32],
    ) -> Result<Sha256Sum, Box<dyn std::error::Error>> {
        let spending_condition = SingleSigSpendingCondition::new(nonce, fee);
        let auth = TransactionAuth::Standard(spending_condition);
        
        let payload = TransactionPayload::ContractCall(
            contract_address.clone(),
            contract_name.to_string(),
            function_name.to_string(),
            function_args,
        );

        let tx = StacksTransaction::new(
            TransactionVersion::Mainnet,
            auth,
            payload,
        );

        let signed_tx = tx.sign(private_key)?;
        let tx_hash = self.rpc_client.broadcast_transaction(&signed_tx).await?;
        
        Ok(tx_hash)
    }
}
