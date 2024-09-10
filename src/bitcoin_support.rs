<<<<<<< HEAD
use std::error::Error;
use bitcoin::{
    Network as BitcoinNetwork,
    Address as BitcoinAddress,
    util::key::PrivateKey,
    util::psbt::PartiallySignedTransaction,
};
use bitcoincore_rpc::{Auth, Client, RpcApi};
use secp256k1::Secp256k1;
use log::{info, error};
use tokio::time::Duration;

pub struct BitcoinSupport {
    network: BitcoinNetwork,
    client: Client,
    secp: Secp256k1<bitcoin::secp256k1::All>,
}

impl BitcoinSupport {
    pub fn new(network: BitcoinNetwork, rpc_url: &str, rpc_user: &str, rpc_password: &str) -> Result<Self, Box<dyn Error>> {
        let auth = Auth::UserPass(rpc_user.to_string(), rpc_password.to_string());
        let client = Client::new(rpc_url, auth)?;
        let secp = Secp256k1::new();

        Ok(Self {
            network,
            client,
            secp,
        })
    }

    pub fn get_balance(&self, address: &BitcoinAddress) -> Result<u64, Box<dyn Error>> {
        let balance = self.client.get_received_by_address(address, None)?;
        Ok(balance)
    }

    pub fn create_and_sign_transaction(&self, from_address: &BitcoinAddress, to_address: &BitcoinAddress, amount: u64, private_key: &PrivateKey) -> Result<PartiallySignedTransaction, Box<dyn Error>> {
        // Step 1: List unspent transaction outputs (UTXOs) for the from_address
        let utxos = self.client.list_unspent(None, None, Some(&[from_address]), None, None)?;

        // Step 2: Create a transaction builder
        let mut tx_builder = bitcoin::util::psbt::PartiallySignedTransaction::from_unsigned_tx(bitcoin::Transaction {
            version: 2,
            lock_time: 0,
            input: vec![],
            output: vec![],
        })?;

        // Step 3: Add inputs from UTXOs
        let mut total_input = 0;
        for utxo in utxos {
            if total_input >= amount {
                break;
            }
            tx_builder.inputs.push(bitcoin::util::psbt::Input {
                non_witness_utxo: Some(utxo.tx_out().clone()),
                ..Default::default()
            });
            total_input += utxo.amount.to_sat();
        }

        if total_input < amount {
            return Err("Insufficient funds".into());
        }

        // Step 4: Add outputs
        tx_builder.outputs.push(bitcoin::util::psbt::Output {
            amount: amount,
            script_pubkey: to_address.script_pubkey(),
            ..Default::default()
        });

        // Add change output if necessary
        let change = total_input - amount;
        if change > 0 {
            tx_builder.outputs.push(bitcoin::util::psbt::Output {
                amount: change,
                script_pubkey: from_address.script_pubkey(),
                ..Default::default()
            });
        }

        // Step 5: Sign the transaction
        let mut psbt = bitcoin::util::psbt::PartiallySignedTransaction::from(tx_builder);
        let secp = Secp256k1::new();
        psbt.sign(&private_key, &secp)?;

        Ok(psbt)
    }

    pub fn broadcast_transaction(&self, psbt: &PartiallySignedTransaction) -> Result<String, Box<dyn Error>> {
        let tx = psbt.extract_tx();
        let txid = self.client.send_raw_transaction(&tx)?;
        Ok(txid.to_string())
    }

    pub fn get_network_info(&self) -> Result<bitcoincore_rpc::json::GetNetworkInfoResult, Box<dyn Error>> {
        let network_info = self.client.get_network_info()?;
        Ok(network_info)
    }

    pub async fn get_network_performance(&self) -> Result<f64, Box<dyn Error>> {
        let transaction_throughput = self.get_transaction_throughput().await?;
        let block_time = self.get_average_block_time().await?;
        let fee_rate = self.get_average_fee_rate().await?;
        
        // Combine metrics into a single performance score
        Ok(0.4 * transaction_throughput + 0.3 * (1.0 / block_time) + 0.3 * (1.0 / fee_rate))
    }

    async fn get_transaction_throughput(&self) -> Result<f64, Box<dyn Error>> {
        // Implement logic to get transaction throughput
        Ok(7.0) // Transactions per second, placeholder value
    }

    async fn get_average_block_time(&self) -> Result<f64, Box<dyn Error>> {
        // Implement logic to get average block time
        Ok(600.0) // Seconds, placeholder value
    }

    async fn get_average_fee_rate(&self) -> Result<f64, Box<dyn Error>> {
        // Implement logic to get average fee rate
        Ok(5.0) // Satoshis per byte, placeholder value
    }

    pub async fn get_balance_async(&self) -> Result<f64, Box<dyn Error>> {
        // Implement method to get Bitcoin balance
        Ok(500.0) // Placeholder value
    }

    pub async fn handle_bitcoin_operations(&self) {
        loop {
            match self.get_network_performance().await {
                Ok(performance) => info!("Bitcoin network performance: {}", performance),
                Err(e) => error!("Failed to get Bitcoin network performance: {:?}", e),
            }

            match self.get_balance_async().await {
                Ok(balance) => info!("Current Bitcoin balance: {} BTC", balance),
                Err(e) => error!("Failed to get Bitcoin balance: {:?}", e),
            }

            // Add more Bitcoin-related operations here

            tokio::time::sleep(Duration::from_secs(300)).await;
        }
    }

    // Add more Bitcoin-related operations as needed
=======
use std::str::FromStr;
use std::sync::Arc;
use bitcoin::{
    Network as BitcoinNetwork,
    Address as BitcoinAddress,
    Transaction,
    TxIn,
    TxOut,
    OutPoint,
    blockdata::script::Script,
    util::amount::Amount,
};
use bitcoin_rpc::Client as BitcoinRpcClient;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use log::{info, error};

pub struct BitcoinSupport {
    network: BitcoinNetwork,
    rpc_client: Arc<BitcoinRpcClient>,
    secp: Secp256k1<secp256k1::All>,
}

impl BitcoinSupport {
    pub fn new(network: BitcoinNetwork, rpc_url: &str, rpc_user: &str, rpc_pass: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let rpc_client = Arc::new(BitcoinRpcClient::new(rpc_url, rpc_user, rpc_pass)?);
        Ok(Self {
            network,
            rpc_client,
            secp: Secp256k1::new(),
        })
    }

    pub fn generate_address(&self, private_key: &SecretKey) -> Result<BitcoinAddress, Box<dyn std::error::Error>> {
        let public_key = PublicKey::from_secret_key(&self.secp, private_key);
        let address = BitcoinAddress::p2wpkh(&public_key, self.network)?;
        Ok(address)
    }

    pub async fn get_balance(&self, address: &BitcoinAddress) -> Result<Amount, Box<dyn std::error::Error>> {
        let balance = self.rpc_client.get_address_balance(&address.to_string(), None).await?;
        Ok(Amount::from_sat(balance.confirmed))
    }

    pub async fn create_and_send_transaction(
        &self,
        from_address: &BitcoinAddress,
        to_address: &BitcoinAddress,
        amount: Amount,
        fee_rate: u64,
        private_key: &SecretKey,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let utxos = self.rpc_client.list_unspent(Some(1), None, Some(&[from_address.to_string()]), None, None).await?;
        
        let mut inputs = Vec::new();
        let mut total_input = Amount::from_sat(0);
        for utxo in utxos {
            inputs.push(TxIn {
                previous_output: OutPoint::from_str(&format!("{}:{}", utxo.txid, utxo.vout))?,
                script_sig: Script::new(),
                sequence: 0xFFFFFFFF,
                witness: Vec::new(),
            });
            total_input += Amount::from_sat(utxo.amount.to_sat());
            if total_input >= amount + Amount::from_sat(fee_rate) {
                break;
            }
        }

        if total_input < amount + Amount::from_sat(fee_rate) {
            return Err("Insufficient funds".into());
        }

        let mut outputs = vec![TxOut {
            value: amount.to_sat(),
            script_pubkey: to_address.script_pubkey(),
        }];

        let change = total_input - amount - Amount::from_sat(fee_rate);
        if change > Amount::from_sat(0) {
            outputs.push(TxOut {
                value: change.to_sat(),
                script_pubkey: from_address.script_pubkey(),
            });
        }

        let mut transaction = Transaction {
            version: 2,
            lock_time: 0,
            input: inputs,
            output: outputs,
        };

        // Sign the transaction
        for (i, input) in transaction.input.iter_mut().enumerate() {
            let sighash = transaction.signature_hash(i, &from_address.script_pubkey(), 1);
            let signature = self.secp.sign(&secp256k1::Message::from_slice(&sighash)?, private_key);
            let mut sig_with_hashtype = signature.serialize_der().to_vec();
            sig_with_hashtype.push(1); // SIGHASH_ALL
            input.witness = vec![sig_with_hashtype, private_key.public_key(&self.secp).serialize().to_vec()];
        }

        let tx_hex = hex::encode(bitcoin::consensus::serialize(&transaction));
        let txid = self.rpc_client.send_raw_transaction(&tx_hex).await?;

        Ok(txid)
    }
>>>>>>> b706d7c49205d3634e6b11d0309d8911a18a435c
}
