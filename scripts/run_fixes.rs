use std::error::Error;
use log::info;
use tokio;

mod automated_fixes;
use automated_fixes::AutomatedFixes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    env_logger::init();
    
    info!("Starting automated fix process...");
    
    // Initialize automated fixes with project root
    let mut fixes = AutomatedFixes::new(String::from("./"));
    
    // Run all fixes
    fixes.run_all_fixes().await?;
    
    info!("Automated fixes completed successfully!");
    
    Ok(())
} 