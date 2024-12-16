use std::error::Error;
use log::info;
use tokio;

mod system_alignment;
use system_alignment::SystemAlignment;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    env_logger::init();
    
    info!("Starting system-wide alignment...");
    
    // Initialize system alignment
    let mut alignment = SystemAlignment::new();
    
    // Run alignment
    alignment.align_system().await?;
    
    info!("System alignment completed successfully!");
    
    Ok(())
} 