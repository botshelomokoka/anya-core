use bitcoincore_rpc::{Auth, Client, RpcApi};
use dlc_btc_lib::{Dlc, ...}; // Added the new import
use std::error::Error;

pub struct BitcoinRPC {
    client: Client,
}

impl BitcoinRPC {
    pub fn new(url: &str, username: &str, password: &str) -> Result<Self, Box<dyn Error>> {
        let auth = Auth::UserPass(username.to_string(), password.to_string());
        let client = Client::new(url, auth)?;
        Ok(Self { client })
    }

    pub fn get_balance(&self) -> Result<f64, Box<dyn Error>> {
        Ok(self.client.get_balance(None, None)?.to_btc())
    }

    pub fn send_transaction(&self, address: &str, amount: f64) -> Result<String, Box<dyn Error>> {
        let txid = self.client.send_to_address(
            &address.parse()?,
            amount.into(),
            None,
            None,
            None,
            None,
            None,
            None
        )?;
        Ok(txid.to_string())
    }

    // Add more RPC methods as needed
}