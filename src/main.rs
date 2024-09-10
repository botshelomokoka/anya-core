mod network;
mod ml;
mod bitcoin;
mod lightning;
mod dlc;
mod stacks;

use log::{info, error};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    info!("Anya Core - Decentralized AI Assistant Framework");

    if let Err(e) = run() {
        error!("Application error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    // Initialize modules
    network::init()?;
    ml::init()?;
    bitcoin::init()?;
    lightning::init()?;
    dlc::init()?;
    stacks::init()?;

    // Start the main application loop
    // TODO: Implement main loop

    Ok(())
}