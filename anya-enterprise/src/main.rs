mod network;
mod ml;
mod bitcoin;
mod lightning;
mod dlc;
mod stacks;
mod advanced_analytics;
mod high_volume_trading;

use log::info;

fn main() {
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
    );
}

fn load_user_metrics() -> UserMetrics {
    let user_metrics_file = "user_metrics.json";
    match std::fs::read_to_string(user_metrics_file) {
        Ok(contents) => {
            match serde_json::from_str(&contents) {
                Ok(metrics) => metrics,
                Err(e) => {
                    eprintln!("Error parsing user metrics: {}", e);
                    UserMetrics::default()
                }
            }
        },
        Err(e) => {
            eprintln!("Error reading user metrics file: {}", e);
            UserMetrics::default()
        }
    }
}
}

fn run_enterprise_features(
    network: Network,
    ml: MachineLearning,
    bitcoin: Bitcoin,
    lightning: Lightning,
    dlc: DLC,
    stacks: Stacks,
    advanced_analytics: AdvancedAnalytics,
    high_volume_trading: HighVolumeTrading,
    user_metrics: &UserMetrics,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = tokio::runtime::Runtime::new()?;
    let (shutdown_sender, shutdown_receiver) = tokio::sync::broadcast::channel(1);

    ctrlc::set_handler(move || {
        println!("Received Ctrl+C, initiating graceful shutdown...");
        let _ = shutdown_sender.send(());
    })?;

    runtime.block_on(async {
        loop {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    println!("Received Ctrl+C, initiating graceful shutdown...");
                    break;
                }
                _ = shutdown_receiver.recv() => {
                    println!("Shutdown signal received, initiating graceful shutdown...");
                    break;
                }
                _ = async {
        // Run enterprise features based on user's tier and metrics
        if user_metrics.tier >= Tier::Premium {
            advanced_analytics.run();
            high_volume_trading.execute();
        }
        
        // Always run core features
        network.process();
        ml.train();
        bitcoin.update();
        lightning.process_payments();
        dlc.manage_contracts();
        stacks.interact();
        
        // Check for exit condition
        if should_exit() {
            break;
        }
    }
}

fn should_exit() -> bool {
    // TODO: Implement exit condition check
    false
}