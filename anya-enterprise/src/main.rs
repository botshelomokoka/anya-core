mod network;
mod ml;
mod bitcoin;
mod lightning;
mod dlc;
mod stacks;
mod advanced_analytics;
mod high_volume_trading;
mod api;
mod error;
mod logging;

use log::{info, error};
use tokio::time::{Duration, sleep};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::api::PyConfig;
use crate::error::AnyaResult;

#[actix_web::main]
async fn main() -> AnyaResult<()> {
    logging::init()?;
    info!("Anya Enterprise - Advanced Decentralized AI Assistant Framework");
    
    let config = PyConfig::new();
    
    // Initialize modules with enterprise features
    let network = network::init(&config.inner)?;
    let ml = ml::init(&config.inner)?;
    let bitcoin = bitcoin::init(&config.inner)?;
    let lightning = lightning::init(&config.inner)?;
    let dlc = dlc::init(&config.inner)?;
    let stacks = stacks::init(&config.inner)?;
    let advanced_analytics = advanced_analytics::init(&config.inner)?;
    let high_volume_trading = high_volume_trading::init(&config.inner)?;
    
    // Start the API server
    let api_server = api::start_api_server(config.clone());
    
    // Start the main application loop
    let main_loop = run_enterprise_features(
        network,
        ml,
        bitcoin,
        lightning,
        dlc,
        stacks,
        advanced_analytics,
        high_volume_trading,
        &config
    );

    // Run both the API server and the main loop concurrently
    tokio::select! {
        _ = api_server => {
            error!("API server unexpectedly shut down");
        }
        result = main_loop => {
            if let Err(e) = result {
                error!("Error in main loop: {}", e);
            }
        }
    }

    Ok(())
}

// ... (update other functions to use AnyaResult and logging) ...