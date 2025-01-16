use anya_core::{
    privacy::PrivacyModule,
    bitcoin::BitcoinModule,
    api::ApiServer,
};
use log::{info, error};
use bitcoin::Network;
use anyhow::Result;

fn handle_error(module: &str) -> impl Fn(anyhow::Error) -> anyhow::Error {
    move |err| {
        error!("Error initializing {}: {}", module, err);
        err
    }
}

fn create_privacy_module() -> Result<PrivacyModule, anyhow::Error> {
    PrivacyModule::new(vec![]).map_err(handle_error("PrivacyModule"))
}

fn create_bitcoin_module() -> Result<BitcoinModule, anyhow::Error> {
    BitcoinModule::new(
        Network::Testnet,
        "http://localhost:18332",
        "rpcuser",
        "rpcpassword",
    ).map_err(handle_error("BitcoinModule"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bitcoin_module = create_bitcoin_module()?;
    let privacy_module = create_privacy_module()?;
    let api_server = ApiServer::new(privacy_module, bitcoin_module);
    info!("Starting API server on 127.0.0.1:8080");
    api_server.run("127.0.0.1", 8080).await.map_err(|e| {
        error!("API server error: {}", e);
        e
    })?;

    Ok(())
}