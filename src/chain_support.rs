use async_trait::async_trait;
use bitcoin::{Address, Network, Transaction, TxIn, TxOut};
use std::str::FromStr;
use bitcoin_wallet::{account::Account, wallet::Wallet, mnemonic::Mnemonic};
use bitcoin_wallet::{account::Account, wallet::Wallet};
use bitcoincore_rpc::{Auth, Client, RpcApi};

#[async_trait]
pub trait ChainSupport {
    async fn verify_transaction(&self, tx_hash: &str) -> Result<bool, Box<dyn std::error::Error>>;
    async fn get_balance(&self, address: &str) -> Result<u64, Box<dyn std::error::Error>>;
    async fn send_transaction(&self, to: &str, amount: u64) -> Result<String, Box<dyn std::error::Error>>;
    async fn create_wallet(&self, name: &str) -> Result<(), Box<dyn std::error::Error>>;
    async fn sign_transaction(&self, psbt: PartiallySignedTransaction) -> Result<Transaction, Box<dyn std::error::Error>>;
}

pub struct BitcoinSupport {
    client: Client,
    wallet: Wallet,
}

impl BitcoinSupport {
    pub fn new(rpc_url: &str, rpc_user: &str, rpc_pass: &str, network: Network) -> Result<Self, Box<dyn std::error::Error>> {
        let auth = Auth::UserPass(rpc_user.to_string(), rpc_pass.to_string());
        let mnemonic = Mnemonic::new_random()?;
        let wallet = Wallet::new(network, Account::new(0, 0, 0, mnemonic)?);
        let wallet = Wallet::new(network, Account::new(0, 0, 0)?);
        Ok(Self { client, wallet })
    }
}

#[async_trait]
impl ChainSupport for BitcoinSupport {
    async fn verify_transaction(&self, tx_hash: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let tx = self.client.get_transaction(tx_hash, None)?;
        Ok(tx.confirmations > 0)
    }

    async fn get_balance(&self, address: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let addr = Address::from_str(address)?;
        let balance = self.client.get_received_by_address(&addr, None)?;
        Ok(balance.as_sat())
    }

    async fn send_transaction(&self, to: &str, amount: u64) -> Result<String, Box<dyn std::error::Error>> {
        let to_addr = Address::from_str(to)?;
        let tx = Transaction {
            version: 2,
            lock_time: 0,
            input: vec![],
            output: vec![TxOut {
                value: amount,
                script_pubkey: to_addr.script_pubkey(),
            }],
        };
        let txid = self.client.send_raw_transaction(&tx)?;
        Ok(txid.to_string())
    }

    async fn create_wallet(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.client.create_wallet(name, None, None, None, None)?;
        Ok(())
    }

    async fn sign_transaction(&self, mut psbt: PartiallySignedTransaction) -> Result<Transaction, Box<dyn std::error::Error>> {
        self.wallet.sign(&mut psbt, bitcoin::SigHashType::All)?;
        let tx = psbt.extract_tx();
        Ok(tx)
    }
}