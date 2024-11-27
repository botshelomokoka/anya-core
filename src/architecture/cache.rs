use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use crate::architecture::errors::{HexagonalError, HexagonalResult};
use crate::architecture::types::{Cache, CacheConfig, CacheEntry};

pub struct CacheLayer<T: Clone + Send + Sync + 'static> {
    cache: Arc<Cache<T>>,
    stats: Arc<RwLock<CacheStats>>,
}

#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    hits: u64,
    misses: u64,
    evictions: u64,
    total_lookups: u64,
    last_cleanup: Option<Instant>,
}

impl<T: Clone + Send + Sync + 'static> CacheLayer<T> {
    pub fn new(config: CacheConfig) -> Self {
        Self {
            cache: Arc::new(Cache::new()),
            stats: Arc::new(RwLock::new(CacheStats::default())),
        }
    }

    pub async fn get(&self, key: &str) -> Option<T> {
        let mut stats = self.stats.write().await;
        stats.total_lookups += 1;

        match self.cache.get(key).await {
            Some(value) => {
                stats.hits += 1;
                Some(value)
            }
            None => {
                stats.misses += 1;
                None
            }
        }
    }

    pub async fn set(&self, key: String, value: T) -> HexagonalResult<()> {
        self.cache.set(key, value).await
    }

    pub async fn remove(&self, key: &str) -> bool {
        let mut storage = self.cache.storage.write().await;
        storage.remove(key).is_some()
    }

    pub async fn clear(&self) {
        let mut storage = self.cache.storage.write().await;
        storage.clear();
    }

    pub async fn get_stats(&self) -> CacheStats {
        self.stats.read().await.clone()
    }

    pub async fn cleanup_expired(&self) -> u64 {
        let mut storage = self.cache.storage.write().await;
        let mut stats = self.stats.write().await;
        let now = chrono::Utc::now();
        let initial_len = storage.len();

        storage.retain(|_, entry| entry.expiry > now);

        let removed = initial_len - storage.len();
        stats.evictions += removed as u64;
        stats.last_cleanup = Some(Instant::now());

        removed as u64
    }

    pub async fn get_or_insert_with<F>(&self, key: String, f: F) -> HexagonalResult<T>
    where
        F: FnOnce() -> HexagonalResult<T>,
    {
        if let Some(value) = self.get(&key).await {
            return Ok(value);
        }

        let value = f()?;
        self.set(key, value.clone()).await?;
        Ok(value)
    }

    pub async fn get_multiple(&self, keys: &[String]) -> Vec<Option<T>> {
        let mut results = Vec::with_capacity(keys.len());
        for key in keys {
            results.push(self.get(key).await);
        }
        results
    }

    pub async fn set_multiple(&self, entries: Vec<(String, T)>) -> HexagonalResult<()> {
        for (key, value) in entries {
            self.set(key, value).await?;
        }
        Ok(())
    }

    pub async fn remove_multiple(&self, keys: &[String]) -> u64 {
        let mut removed = 0;
        for key in keys {
            if self.remove(key).await {
                removed += 1;
            }
        }
        removed
    }

    pub async fn get_size(&self) -> usize {
        self.cache.storage.read().await.len()
    }

    pub async fn contains_key(&self, key: &str) -> bool {
        self.cache.storage.read().await.contains_key(key)
    }

    pub async fn get_keys(&self) -> Vec<String> {
        self.cache.storage.read().await.keys().cloned().collect()
    }

    pub async fn update_ttl(&self, key: &str, new_ttl: Duration) -> bool {
        let mut storage = self.cache.storage.write().await;
        if let Some(entry) = storage.get_mut(key) {
            entry.expiry = chrono::Utc::now() + chrono::Duration::from_std(new_ttl).unwrap();
            true
        } else {
            false
        }
    }
}

pub struct CacheManager<T: Clone + Send + Sync + 'static> {
    cache_layer: Arc<CacheLayer<T>>,
    cleanup_interval: Duration,
}

impl<T: Clone + Send + Sync + 'static> CacheManager<T> {
    pub fn new(cache_layer: Arc<CacheLayer<T>>, cleanup_interval: Duration) -> Self {
        Self {
            cache_layer,
            cleanup_interval,
        }
    }

    pub async fn start_cleanup_task(self) {
        let cache_layer = self.cache_layer.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(self.cleanup_interval);
            loop {
                interval.tick().await;
                let removed = cache_layer.cleanup_expired().await;
                if removed > 0 {
                    log::info!("Cleaned up {} expired cache entries", removed);
                }
            }
        });
    }

    pub async fn get_cache_stats(&self) -> CacheStats {
        self.cache_layer.get_stats().await
    }

    pub fn get_cache_layer(&self) -> Arc<CacheLayer<T>> {
        self.cache_layer.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_cache_operations() {
        let config = CacheConfig {
            ttl: Duration::from_secs(1),
            max_size: 100,
        };
        let cache = CacheLayer::new(config);

        // Test set and get
        cache.set("key1".to_string(), "value1".to_string()).await.unwrap();
        assert_eq!(cache.get("key1").await, Some("value1".to_string()));

        // Test expiration
        tokio::time::sleep(Duration::from_secs(2)).await;
        assert_eq!(cache.get("key1").await, None);

        // Test stats
        let stats = cache.get_stats().await;
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
    }

    #[tokio::test]
    async fn test_cache_cleanup() {
        let config = CacheConfig {
            ttl: Duration::from_millis(100),
            max_size: 100,
        };
        let cache = CacheLayer::new(config);

        // Add some entries
        cache.set("key1".to_string(), "value1".to_string()).await.unwrap();
        cache.set("key2".to_string(), "value2".to_string()).await.unwrap();

        // Wait for expiration
        tokio::time::sleep(Duration::from_millis(200)).await;

        // Cleanup
        let removed = cache.cleanup_expired().await;
        assert_eq!(removed, 2);
        assert_eq!(cache.get_size().await, 0);
    }

    #[tokio::test]
    async fn test_cache_manager() {
        let config = CacheConfig {
            ttl: Duration::from_millis(100),
            max_size: 100,
        };
        let cache_layer = Arc::new(CacheLayer::new(config));
        let cache_manager = CacheManager::new(
            cache_layer.clone(),
            Duration::from_millis(50),
        );

        // Add some entries
        cache_layer.set("key1".to_string(), "value1".to_string()).await.unwrap();
        cache_layer.set("key2".to_string(), "value2".to_string()).await.unwrap();

        // Start cleanup task
        cache_manager.start_cleanup_task().await;

        // Wait for cleanup
        tokio::time::sleep(Duration::from_millis(200)).await;

        // Verify cleanup
        assert_eq!(cache_layer.get_size().await, 0);
    }
}
