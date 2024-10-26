// Add new modules
pub mod rgb;
pub mod liquid;

// Re-export enterprise features
pub use rgb::RGBModule;
pub use liquid::LiquidModule;

// Update initialization
pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize existing modules...
    
    // Initialize RGB support
    let rgb_module = RGBModule::new();
    
    // Initialize Liquid support
    let liquid_module = LiquidModule::new()?;
    
    Ok(())
}
