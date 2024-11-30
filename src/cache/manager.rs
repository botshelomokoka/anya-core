use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::time::sleep;
use tracing::{info, warn};

#[derive(Clone)]
pub struct CacheManager {
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    config: CacheConfig,
}

struct CacheEntry {
    value: Vec<u8>,
    expiry: Instant,
    last_accessed: Instant,
}

#[derive(Clone)]
pub struct CacheConfig {
    pub ttl: Duration,
    pub max_size: usize,
    pub cleanup_interval: Duration,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            ttl: Duration::from_secs(3600), // 1 hour
            max_size: 1024 * 1024 * 100,    // 100MB
            cleanup_interval: Duration::from_secs(300), // 5 minutes
        }
    }
}

impl CacheManager {
    pub fn new(config: CacheConfig) -> Self {
        let cache_manager = Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            config,
        };

        // Start cleanup task
        let cm = cache_manager.clone();
        tokio::spawn(async move {
            loop {
                sleep(cm.config.cleanup_interval).await;
                cm.cleanup().await;
            }
        });

        cache_manager
    }

    pub async fn set(&self, key: String, value: Vec<u8>) -> Result<(), String> {
        let mut cache = self.cache.write().await;
        
        // Check size limit
        let total_size: usize = cache.values()
            .map(|entry| entry.value.len())
            .sum();
        
        if total_size + value.len() > self.config.max_size {
            warn!("Cache size limit exceeded, performing cleanup");
            drop(cache); // Release the write lock
            self.cleanup().await;
            cache = self.cache.write().await;
        }

        cache.insert(key, CacheEntry {
            value,
            expiry: Instant::now() + self.config.ttl,
            last_accessed: Instant::now(),
        });

        Ok(())
    }

    pub async fn get(&self, key: &str) -> Option<Vec<u8>> {
        let mut cache = self.cache.write().await;
        
        if let Some(entry) = cache.get_mut(key) {
            if entry.expiry > Instant::now() {
                entry.last_accessed = Instant::now();
                return Some(entry.value.clone());
            } else {
                cache.remove(key);
            }
        }
        
        None
    }

    pub async fn remove(&self, key: &str) {
        let mut cache = self.cache.write().await;
        cache.remove(key);
    }

    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }

    async fn cleanup(&self) {
        let mut cache = self.cache.write().await;
        let now = Instant::now();
        
        // Remove expired entries
        cache.retain(|_, entry| entry.expiry > now);

        // If still over size limit, remove least recently accessed entries
        let mut total_size: usize = cache.values()
            .map(|entry| entry.value.len())
            .sum();

        if total_size > self.config.max_size {
            let mut entries: Vec<_> = cache.iter()
                .map(|(k, v)| (k.clone(), v.last_accessed))
                .collect();
            
            entries.sort_by_key(|&(_, last_accessed)| last_accessed);

            for (key, _) in entries {
                if let Some(entry) = cache.remove(&key) {
                    total_size -= entry.value.len();
                    if total_size <= self.config.max_size {
                        break;
                    }
                }
            }
        }

        info!("Cache cleanup completed. Current size: {} bytes", total_size);
    }

    pub async fn get_stats(&self) -> CacheStats {
        let cache = self.cache.read().await;
        let total_entries = cache.len();
        let total_size: usize = cache.values()
            .map(|entry| entry.value.len())
            .sum();
        let expired_entries = cache.values()
            .filter(|entry| entry.expiry <= Instant::now())
            .count();

        CacheStats {
            total_entries,
            total_size,
            expired_entries,
            max_size: self.config.max_size,
        }
    }
}

#[derive(Debug)]
pub struct CacheStats {
    pub total_entries: usize,
    pub total_size: usize,
    pub expired_entries: usize,
    pub max_size: usize,
}
