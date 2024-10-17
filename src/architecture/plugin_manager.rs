use log::{info, error};
use std::collections::HashMap;
use std::any::Any;

pub trait Plugin: Any {
    fn name(&self) -> &'static str;
    fn init(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn shutdown(&self) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct PluginManager {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        PluginManager {
            plugins: HashMap::new(),
        }
    }

    pub fn register_plugin(&mut self, plugin: Box<dyn Plugin>) {
        let name = plugin.name().to_string();
        self.plugins.insert(name, plugin);
    }

    pub fn init_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        for (name, plugin) in &self.plugins {
            info!("Initializing plugin: {}", name);
            plugin.init()?;
        }
        Ok(())
    }

    pub fn shutdown_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        for (name, plugin) in &self.plugins {
            info!("Shutting down plugin: {}", name);
            plugin.shutdown()?;
        }
        Ok(())
    }
}

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Initializing plugin manager");
    let mut manager = PluginManager::new();
    // Register plugins here
    manager.init_all()?;
    Ok(())
}