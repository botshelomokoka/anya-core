use bitcoin::Network;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use log::info;

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing Bitcoin integration");
    let rpc = Client::new(
        "http://localhost:8332".to_string(),
        Auth::UserPass("rpcuser".to_string(), "rpcpassword".to_string()),
    )?;
    let blockchain_info = rpc.get_blockchain_info()?;
    info!("Connected to Bitcoin network: {:?}", blockchain_info.chain);
    Ok(())
}