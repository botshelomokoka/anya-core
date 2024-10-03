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
}
