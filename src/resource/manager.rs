use std::sync::Arc;
use tokio::sync::{RwLock, Semaphore};
use std::collections::HashMap;
use tracing::{info, warn, error};
use crate::config::CONFIG;

pub struct ResourceManager {
    connection_pool: Arc<Semaphore>,
    memory_pool: Arc<RwLock<MemoryPool>>,
    resource_limits: Arc<RwLock<ResourceLimits>>,
    active_resources: Arc<RwLock<HashMap<String, ResourceUsage>>>,
}

struct MemoryPool {
    total_bytes: u64,
    used_bytes: u64,
    reserved_bytes: u64,
}

struct ResourceLimits {
    max_connections: u32,
    max_memory_bytes: u64,
    max_cpu_percent: f64,
}

#[derive(Clone)]
struct ResourceUsage {
    memory_bytes: u64,
    cpu_percent: f64,
    connection_count: u32,
}

impl ResourceManager {
    pub async fn new() -> Self {
        let config = CONFIG.read().await;
        let max_connections = config.get_number("max_connections").unwrap_or(100) as u32;
        let max_memory = config.get_number("max_memory_bytes").unwrap_or(1024 * 1024 * 1024) as u64; // 1GB default
        
        Self {
            connection_pool: Arc::new(Semaphore::new(max_connections as usize)),
            memory_pool: Arc::new(RwLock::new(MemoryPool {
                total_bytes: max_memory,
                used_bytes: 0,
                reserved_bytes: max_memory / 10, // 10% reserved for system
            })),
            resource_limits: Arc::new(RwLock::new(ResourceLimits {
                max_connections,
                max_memory_bytes: max_memory,
                max_cpu_percent: 80.0,
            })),
            active_resources: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn acquire_connection(&self) -> Result<ResourceGuard, String> {
        match self.connection_pool.try_acquire() {
            Ok(permit) => Ok(ResourceGuard {
                permit: Some(permit),
                manager: self.clone(),
            }),
            Err(_) => {
                warn!("Connection pool exhausted");
                Err("Connection pool exhausted".to_string())
            }
        }
    }

    pub async fn allocate_memory(&self, bytes: u64) -> Result<(), String> {
        let mut pool = self.memory_pool.write().await;
        if pool.used_bytes + bytes > pool.total_bytes - pool.reserved_bytes {
            error!("Memory allocation failed - requested: {} bytes, available: {} bytes",
                bytes, pool.total_bytes - pool.used_bytes - pool.reserved_bytes);
            return Err("Insufficient memory".to_string());
        }
        pool.used_bytes += bytes;
        Ok(())
    }

    pub async fn release_memory(&self, bytes: u64) {
        let mut pool = self.memory_pool.write().await;
        pool.used_bytes = pool.used_bytes.saturating_sub(bytes);
    }

    pub async fn register_resource(&self, id: String, usage: ResourceUsage) {
        let mut resources = self.active_resources.write().await;
        resources.insert(id, usage);
    }

    pub async fn unregister_resource(&self, id: &str) {
        let mut resources = self.active_resources.write().await;
        resources.remove(id);
    }

    pub async fn get_resource_usage(&self) -> HashMap<String, ResourceUsage> {
        self.active_resources.read().await.clone()
    }

    pub async fn check_resource_health(&self) -> ResourceHealth {
        let pool = self.memory_pool.read().await;
        let limits = self.resource_limits.read().await;
        let memory_usage = (pool.used_bytes as f64 / pool.total_bytes as f64) * 100.0;
        let connection_usage = (limits.max_connections - self.connection_pool.available_permits() as u32) as f64
            / limits.max_connections as f64 * 100.0;

        ResourceHealth {
            memory_usage_percent: memory_usage,
            connection_usage_percent: connection_usage,
            is_healthy: memory_usage < 90.0 && connection_usage < 90.0,
        }
    }
}

#[derive(Clone)]
pub struct ResourceGuard {
    permit: Option<tokio::sync::SemaphorePermit>,
    manager: ResourceManager,
}

impl Drop for ResourceGuard {
    fn drop(&mut self) {
        if self.permit.take().is_some() {
            info!("Released connection back to pool");
        }
    }
}

pub struct ResourceHealth {
    pub memory_usage_percent: f64,
    pub connection_usage_percent: f64,
    pub is_healthy: bool,
}
