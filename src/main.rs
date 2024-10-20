mod architecture;
mod blockchain;
mod networking;
mod identity;
mod network;
mod ml;
mod bitcoin;
mod lightning;
mod dlc;
mod stacks;

use log::{info, error};
use architecture::{PluginManager, HexagonalArchitecture};
use blockchain::BlockchainPlugin;
use networking::NetworkingPlugin;
use identity::IdentityPlugin;
use std::error::Error;
use std::sync::Arc;
use tokio::time::Duration;
use actix_web::{App, HttpServer, web};
use yew::prelude::*;
use crate::api::ApiHandler;
use crate::unified_network::UnifiedNetworkManager;
use crate::rate_limiter::RateLimiter;
use crate::ui::web_interface::WebInterface;

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <WebInterface />
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    yew::start_app::<App>();

    env_logger::init();
    info!("Anya Core - Decentralized AI Assistant Framework");

    if let Err(e) = run() {
        error!("Application error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    initialize_core_architecture()?;
    let mut plugin_manager = PluginManager::new();

    // Create and register plugins
    let blockchain_plugin = Box::new(BlockchainPlugin);
    let networking_plugin = Box::new(NetworkingPlugin);
    let identity_plugin = Box::new(IdentityPlugin);

    plugin_manager.register_plugin(blockchain_plugin.clone());
    plugin_manager.register_plugin(networking_plugin.clone());
    plugin_manager.register_plugin(identity_plugin.clone());

    let hexagonal = HexagonalArchitecture::new(
fn initialize_plugins() -> Result<(), Box<dyn Error>> {
    architecture::init()?;
    
    let mut plugin_manager = PluginManager::new();

    let blockchain_plugin = Box::new(BlockchainPlugin);
    let networking_plugin = Box::new(NetworkingPlugin);
    let identity_plugin = Box::new(IdentityPlugin);

    plugin_manager.register_plugin(blockchain_plugin.clone());
    plugin_manager.register_plugin(networking_plugin.clone());
    plugin_manager.register_plugin(identity_plugin.clone());

    plugin_manager.init_all()?;
    Ok(())
}

fn initialize_hexagonal_architecture() -> Result<(), Box<dyn Error>> {
    let blockchain_plugin = Box::new(BlockchainPlugin);
    let networking_plugin = Box::new(NetworkingPlugin);
    let identity_plugin = Box::new(IdentityPlugin);

    let hexagonal = HexagonalArchitecture::new(
        blockchain_plugin.clone(),
        networking_plugin.clone(),
        identity_plugin.clone(),
    );

    hexagonal.init()?;
    Ok(())
}

fn initialize_other_modules() -> Result<(), Box<dyn Error>> {
    network::init()?;
    ml::init()?;
    bitcoin::init()?;
    lightning::init()?;
    dlc::init()?;
    stacks::init()?;
    Ok(())
}

fn start_main_loop() -> Result<(), Box<dyn Error>> {
    // TODO: Implement main loop
    info!("Anya Core Project - All components initialized");
    Ok(())
}

#[actix_web::main]
async fn actix_main() -> std::io::Result<()> {
    let rate_limiter = Arc::new(RateLimiter::new());
    let unified_network_manager = Arc::new(UnifiedNetworkManager::new());

    // Start network load monitoring
    let rate_limiter_clone = Arc::clone(&rate_limiter);
    let unified_network_manager_clone = Arc::clone(&unified_network_manager);
    tokio::spawn(async move {
        unified_network_manager_clone.monitor_network_load(rate_limiter_clone).await;
    });

    // Periodically auto-adjust system parameters
    let unified_network_manager_clone = Arc::clone(&unified_network_manager);
    tokio::spawn(async move {
        loop {
            if let Err(e) = unified_network_manager_clone.auto_adjust().await {
                log::error!("Failed to auto-adjust system parameters: {}", e);
            }
            tokio::time::sleep(Duration::from_secs(3600)).await; // Auto-adjust every hour
        }
    });

    // Set up API server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ApiHandler::new(Arc::clone(&rate_limiter))))
            .configure(api::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
