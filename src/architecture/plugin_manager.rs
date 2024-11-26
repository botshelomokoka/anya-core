use std::collections::HashMap;
use std::any::Any;
use thiserror::Error;
use log::info;
use metrics::{increment_counter, histogram};
use tokio::time::Instant;

/// Custom error type for the Plugin Manager
#[derive(Error, Debug)]
pub enum PluginManagerError {
    #[error("Failed to register plugin '{0}': {1}")]
    RegistrationError(String, #[source] Box<dyn std::error::Error>),

    #[error("Failed to initialize plugin '{0}': {1}")]
    InitializationError(String, #[source] Box<dyn std::error::Error>),

    #[error("Failed to shutdown plugin '{0}': {1}")]
    ShutdownError(String, #[source] Box<dyn std::error::Error>),

    #[error("Plugin '{0}' not found")]
    PluginNotFound(String),
}

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

    /// Registers a new plugin with the Plugin Manager.
    ///
    /// # Arguments
    ///
    /// * `plugin` - A boxed trait object implementing the `Plugin` trait.
    ///
    /// # Errors
    ///
    /// Returns `PluginManagerError::RegistrationError` if the plugin is already registered or fails to register.
    pub fn register_plugin(&mut self, plugin: Box<dyn Plugin>) -> Result<(), PluginManagerError> {
        let start = Instant::now();
        let name = plugin.name().to_string();

        if self.plugins.contains_key(&name) {
            increment_counter!("plugin_registration_failures_total");
            return Err(PluginManagerError::RegistrationError(
                name.clone(),
                "Plugin already registered".into(),
            ));
        }

        self.plugins.insert(name.clone(), plugin);
        
        let elapsed = start.elapsed();
        histogram!("plugin_registration_duration_seconds", elapsed.as_secs_f64());
        increment_counter!("plugin_registration_success_total");
        
        Ok(())
    }

    /// Initializes all registered plugins.
    ///
    /// # Errors
    ///
    /// Returns `PluginManagerError::InitializationError` if any plugin fails to initialize.
    pub fn init_all(&self) -> Result<(), PluginManagerError> {
        let start = Instant::now();
        
        for (name, plugin) in &self.plugins {
            info!("Initializing plugin: {}", name);
            if let Err(e) = plugin.init() {
                increment_counter!("plugin_initialization_failures_total");
                return Err(PluginManagerError::InitializationError(name.clone(), e));
            }
        }

        let elapsed = start.elapsed();
        histogram!("plugin_initialization_duration_seconds", elapsed.as_secs_f64());
        increment_counter!("plugin_initialization_success_total");
        
        Ok(())
    }

    /// Shuts down all registered plugins.
    ///
    /// # Errors
    ///
    /// Returns `PluginManagerError::ShutdownError` if any plugin fails to shutdown.
    pub fn shutdown_all(&self) -> Result<(), PluginManagerError> {
        let start = Instant::now();
        
        for (name, plugin) in &self.plugins {
            info!("Shutting down plugin: {}", name);
            if let Err(e) = plugin.shutdown() {
                increment_counter!("plugin_shutdown_failures_total");
                return Err(PluginManagerError::ShutdownError(name.clone(), e));
            }
        }

        let elapsed = start.elapsed();
        histogram!("plugin_shutdown_duration_seconds", elapsed.as_secs_f64());
        increment_counter!("plugin_shutdown_success_total");
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    struct MockPlugin {
        name: &'static str,
        should_fail_init: bool,
        should_fail_shutdown: bool,
    }

    impl Plugin for MockPlugin {
        fn name(&self) -> &'static str {
            self.name
        }

        fn init(&self) -> Result<(), Box<dyn Error>> {
            if self.should_fail_init {
                Err("Initialization failed".into())
            } else {
                Ok(())
            }
        }

        fn shutdown(&self) -> Result<(), Box<dyn Error>> {
            if self.should_fail_shutdown {
                Err("Shutdown failed".into())
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn test_register_plugin_success() {
        let mut manager = PluginManager::new();
        let plugin = Box::new(MockPlugin { 
            name: "TestPlugin", 
            should_fail_init: false,
            should_fail_shutdown: false,
        });
        assert!(manager.register_plugin(plugin).is_ok());
    }

    #[test]
    fn test_register_plugin_duplicate() {
        let mut manager = PluginManager::new();
        let plugin1 = Box::new(MockPlugin { 
            name: "TestPlugin", 
            should_fail_init: false,
            should_fail_shutdown: false,
        });
        let plugin2 = Box::new(MockPlugin { 
            name: "TestPlugin", 
            should_fail_init: false,
            should_fail_shutdown: false,
        });
        assert!(manager.register_plugin(plugin1).is_ok());
        let result = manager.register_plugin(plugin2);
        assert!(matches!(result, Err(PluginManagerError::RegistrationError(_, _))));
    }

    #[test]
    fn test_init_all_success() {
        let mut manager = PluginManager::new();
        let plugin = Box::new(MockPlugin { 
            name: "TestPlugin", 
            should_fail_init: false,
            should_fail_shutdown: false,
        });
        manager.register_plugin(plugin).unwrap();
        assert!(manager.init_all().is_ok());
    }

    #[test]
    fn test_init_all_failure() {
        let mut manager = PluginManager::new();
        let plugin = Box::new(MockPlugin { 
            name: "FailingPlugin", 
            should_fail_init: true,
            should_fail_shutdown: false,
        });
        manager.register_plugin(plugin).unwrap();
        let result = manager.init_all();
        assert!(matches!(result, Err(PluginManagerError::InitializationError(_, _))));
    }

    #[test]
    fn test_shutdown_all_success() {
        let mut manager = PluginManager::new();
        let plugin = Box::new(MockPlugin { 
            name: "TestPlugin", 
            should_fail_init: false,
            should_fail_shutdown: false,
        });
        manager.register_plugin(plugin).unwrap();
        assert!(manager.shutdown_all().is_ok());
    }

    #[test]
    fn test_shutdown_all_failure() {
        let mut manager = PluginManager::new();
        let plugin = Box::new(MockPlugin { 
            name: "FailingPlugin", 
            should_fail_init: false,
            should_fail_shutdown: true,
        });
        manager.register_plugin(plugin).unwrap();
        let result = manager.shutdown_all();
        assert!(matches!(result, Err(PluginManagerError::ShutdownError(_, _))));
    }
}