use log::info;
pub use plugin_manager::PluginManager;
pub use hexagonal::HexagonalArchitecture;

mod plugin_manager;
mod hexagonal;
use log::info; // Importing the logging functionality
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
/// Initializes the architecture module by setting up the plugin manager and hexagonal architecture.
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    plugin_manager::init()?;
    hexagonal::init()?;
    Ok(())
}