mod plugin_manager;
mod hexagonal;

use log::info;

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing architecture module");
    plugin_manager::init()?;
    hexagonal::init()?;
    Ok(())
}

pub use plugin_manager::PluginManager;
pub use hexagonal::HexagonalArchitecture;