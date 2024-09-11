<<<<<<< HEAD
mod architecture;
mod blockchain;
mod networking;
mod identity;
// ... other mod declarations ...

use log::{info, error};
use architecture::{PluginManager, HexagonalArchitecture};
use blockchain::BlockchainPlugin;
use networking::NetworkingPlugin;
use identity::IdentityPlugin;

fn main() {
    env_logger::init();
    info!("Anya Core Project - Initializing");
=======
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
>>>>>>> c9fe62bf07bc8e7e0a11b9b0e4e6375f56b5c4cc

    if let Err(e) = run() {
        error!("Application error: {}", e);
        std::process::exit(1);
    }
<<<<<<< HEAD
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize core architecture
    let mut plugin_manager = PluginManager::new();

    // Create and register plugins
    let blockchain_plugin = Box::new(BlockchainPlugin);
    let networking_plugin = Box::new(NetworkingPlugin);
    let identity_plugin = Box::new(IdentityPlugin);

    plugin_manager.register_plugin(blockchain_plugin.clone());
    plugin_manager.register_plugin(networking_plugin.clone());
    plugin_manager.register_plugin(identity_plugin.clone());

    // Initialize Hexagonal Architecture
    let hexagonal = HexagonalArchitecture::new(
        blockchain_plugin,
        networking_plugin,
        identity_plugin,
    );

    // Initialize architecture
    architecture::init()?;

    // Initialize plugins
    plugin_manager.init_all()?;

    // Initialize Hexagonal Architecture
    hexagonal.init()?;

    // ... initialize other components ...

    info!("Anya Core Project - All components initialized");
=======

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

>>>>>>> c9fe62bf07bc8e7e0a11b9b0e4e6375f56b5c4cc
    Ok(())
}