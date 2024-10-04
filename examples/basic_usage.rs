use anya_core::{
    privacy::PrivacyModule,
    bitcoin::BitcoinModule,
    api::ApiServer,
};
use bitcoin::Network;
use log::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let privacy_module = PrivacyModule::new(vec![]).map_err(|e| {
        error!("Failed to create PrivacyModule: {}", e);
        e
    })?;

    let bitcoin_module = BitcoinModule::new(
        Network::Testnet,
        "http://localhost:18332",
        "rpcuser",
        "rpcpassword",
    ).map_err(|e| {
        error!("Failed to create BitcoinModule: {}", e);
        e
    })?;

    let api_server = ApiServer::new(privacy_module, bitcoin_module);
    info!("Starting API server on 127.0.0.1:8080");
    api_server.run("127.0.0.1", 8080).await.map_err(|e| {
        error!("API server error: {}", e);
        e
    })?;

    Ok(())
}