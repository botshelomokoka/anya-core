use async_trait::async_trait;
use lru::LruCache;
use parking_lot::RwLock;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::broadcast;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("Cache entry expired")]
    Expired,
    #[error("Cache entry not found")]
    NotFound,
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CacheEntry {
    data: Vec<u8>,
    expires_at: Option<Instant>,
}

pub struct CacheConfig {
    pub max_size: NonZeroUsize,
    pub default_ttl: Option<Duration>,
    pub notify_on_evict: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_size: NonZeroUsize::new(1000).unwrap(),
            default_ttl: Some(Duration::from_secs(3600)), // 1 hour
            notify_on_evict: true,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CacheEvent {
    Set { key: String },
    Evict { key: String },
    Clear,
}

pub struct Web5Cache {
    cache: Arc<RwLock<LruCache<String, CacheEntry>>>,
    config: CacheConfig,
    event_tx: broadcast::Sender<CacheEvent>,
}

#[async_trait]
pub trait Cache {
    async fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>, CacheError>;
    async fn set<T: Serialize>(&self, key: &str, value: T, ttl: Option<Duration>) -> Result<(), CacheError>;
    async fn delete(&self, key: &str) -> bool;
    async fn clear(&self);
    fn subscribe(&self) -> broadcast::Receiver<CacheEvent>;
}

impl Web5Cache {
    pub fn new(config: CacheConfig) -> Self {
        let (event_tx, _) = broadcast::channel(100);
        Self {
            cache: Arc::new(RwLock::new(LruCache::new(config.max_size))),
            config,
            event_tx,
        }
    }

    fn is_expired(entry: &CacheEntry) -> bool {
        entry.expires_at
            .map(|expires| expires <= Instant::now())
            .unwrap_or(false)
    }
}

#[async_trait]
impl Cache for Web5Cache {
    async fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>, CacheError> {
        let cache = self.cache.read();
        if let Some(entry) = cache.peek(key) {
            if Self::is_expired(entry) {
                return Err(CacheError::Expired);
            }
            let value: T = serde_json::from_slice(&entry.data)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    async fn set<T: Serialize>(&self, key: &str, value: T, ttl: Option<Duration>) -> Result<(), CacheError> {
        let data = serde_json::to_vec(&value)?;
        let expires_at = ttl
            .or(self.config.default_ttl)
            .map(|ttl| Instant::now() + ttl);

        let entry = CacheEntry { data, expires_at };
        
        {
            let mut cache = self.cache.write();
            cache.put(key.to_string(), entry);
        }

        if self.config.notify_on_evict {
            let _ = self.event_tx.send(CacheEvent::Set {
                key: key.to_string(),
            });
        }

        Ok(())
    }

    async fn delete(&self, key: &str) -> bool {
        let mut cache = self.cache.write();
        let existed = cache.pop(key).is_some();

        if existed && self.config.notify_on_evict {
            let _ = self.event_tx.send(CacheEvent::Evict {
                key: key.to_string(),
            });
        }

        existed
    }

    async fn clear(&self) {
        let mut cache = self.cache.write();
        cache.clear();

        if self.config.notify_on_evict {
            let _ = self.event_tx.send(CacheEvent::Clear);
        }
    }

    fn subscribe(&self) -> broadcast::Receiver<CacheEvent> {
        self.event_tx.subscribe()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_cache_operations() {
        let config = CacheConfig {
            max_size: NonZeroUsize::new(10).unwrap(),
            default_ttl: Some(Duration::from_secs(1)),
            notify_on_evict: true,
        };
        let cache = Web5Cache::new(config);

        // Test basic set/get
        let value = json!({"name": "test", "value": 42});
        cache.set("key1", &value, None).await.unwrap();
        let result: serde_json::Value = cache.get("key1").await.unwrap().unwrap();
        assert_eq!(result, value);

        // Test expiration
        cache.set("key2", &value, Some(Duration::from_millis(100))).await.unwrap();
        sleep(Duration::from_millis(200)).await;
        assert!(cache.get::<serde_json::Value>("key2").await.is_err());

        // Test eviction notification
        let mut rx = cache.subscribe();
        cache.delete("key1").await;
        if let Ok(CacheEvent::Evict { key }) = rx.try_recv() {
            assert_eq!(key, "key1");
        } else {
            panic!("Expected eviction event");
        }
    }
}
