mod network;
mod ml;
mod bitcoin;
mod lightning;
mod dlc;
mod stacks;
mod advanced_analytics;
mod high_volume_trading;

use log::{info, error};
use tokio::time::{Duration, sleep};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Anya Enterprise - Advanced Decentralized AI Assistant Framework");
    
    // Initialize user metrics
    let user_metrics = load_user_metrics();
    
    // Initialize modules with enterprise features
    let network = network::init(&user_metrics);
    let ml = ml::init(&user_metrics);
    let bitcoin = bitcoin::init(&user_metrics);
    let lightning = lightning::init(&user_metrics);
    let dlc = dlc::init(&user_metrics);
    let stacks = stacks::init(&user_metrics);
    let advanced_analytics = advanced_analytics::init(&user_metrics);
    let high_volume_trading = high_volume_trading::init(&user_metrics);
    
    // Start the main application loop
    run_enterprise_features(
        network,
        ml,
        bitcoin,
        lightning,
        dlc,
        stacks,
        advanced_analytics,
        high_volume_trading,
        &user_metrics
    )
}

fn load_user_metrics() -> UserMetrics {
    let user_metrics_file = "user_metrics.json";
    match std::fs::read_to_string(user_metrics_file) {
        Ok(contents) => {
            match serde_json::from_str(&contents) {
                Ok(metrics) => metrics,
                Err(e) => {
                    error!("Error parsing user metrics: {}", e);
                    UserMetrics::default()
                }
            }
        },
        Err(e) => {
            error!("Error reading user metrics file: {}", e);
            UserMetrics::default()
        }
    }
}

fn run_enterprise_features(
    mut network: Network,
    mut ml: MachineLearning,
    mut bitcoin: Bitcoin,
    mut lightning: Lightning,
    mut dlc: DLC,
    mut stacks: Stacks,
    mut advanced_analytics: AdvancedAnalytics,
    mut high_volume_trading: HighVolumeTrading,
    user_metrics: &UserMetrics,
) -> Result<(), Box<dyn std::error::Error>> {
    let runtime = tokio::runtime::Runtime::new()?;
    let (shutdown_sender, mut shutdown_receiver) = tokio::sync::broadcast::channel(1);
    let should_exit = Arc::new(AtomicBool::new(false));
    let should_exit_clone = should_exit.clone();

    ctrlc::set_handler(move || {
        info!("Received Ctrl+C, initiating graceful shutdown...");
        let _ = shutdown_sender.send(());
        should_exit_clone.store(true, Ordering::SeqCst);
    })?;

    runtime.block_on(async {
        loop {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    info!("Received Ctrl+C, initiating graceful shutdown...");
                    break;
                }
                _ = shutdown_receiver.recv() => {
                    info!("Shutdown signal received, initiating graceful shutdown...");
                    break;
                }
                _ = async {
                    // Run enterprise features based on user's tier and metrics
                    if user_metrics.tier >= Tier::Premium {
                        advanced_analytics.run().await?;
                        high_volume_trading.execute().await?;
                    }
                    
                    // Always run core features
                    network.process().await?;
                    ml.train().await?;
                    bitcoin.update().await?;
                    lightning.process_payments().await?;
                    dlc.manage_contracts().await?;
                    stacks.interact().await?;
                    
                    // Check for exit condition
                    if should_exit.load(Ordering::SeqCst) {
                        break;
                    }

                    // Add a small delay to prevent busy-waiting
                    sleep(Duration::from_millis(100)).await;

                    Ok::<(), Box<dyn std::error::Error>>(())
                } => {
                    if let Err(e) = result {
                        error!("Error in main loop: {}", e);
                    }
                }
            }
        }

        // Perform cleanup operations
        info!("Cleaning up and shutting down...");
        network.shutdown().await?;
        ml.shutdown().await?;
        bitcoin.shutdown().await?;
        lightning.shutdown().await?;
        dlc.shutdown().await?;
        stacks.shutdown().await?;
        advanced_analytics.shutdown().await?;
        high_volume_trading.shutdown().await?;

        Ok(())
    })
}