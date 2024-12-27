use std::sync::Arc;
use tokio::sync::RwLock;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use crate::metrics::{UnifiedMetrics, ComponentHealth, MetricsError};
use crate::security::SecurityContext;
use crate::validation::ValidationResult;

/// Core repository trait that all repositories must implement
#[async_trait]
pub trait Repository: Send + Sync {
    type Item;
    type Error;

    async fn create(&self, item: Self::Item) -> Result<Self::Item, Self::Error>;
    async fn read(&self, id: &str) -> Result<Option<Self::Item>, Self::Error>;
    async fn update(&self, id: &str, item: Self::Item) -> Result<Self::Item, Self::Error>;
    async fn delete(&self, id: &str) -> Result<(), Self::Error>;
    async fn list(&self) -> Result<Vec<Self::Item>, Self::Error>;
}

/// Generic repository implementation with metrics and validation
pub struct GenericRepository<T, E> {
    items: Arc<RwLock<HashMap<String, T>>>,
    metrics: Arc<RwLock<UnifiedMetrics>>,
    validator: Arc<dyn Validator<T>>,
    _error: std::marker::PhantomData<E>,
}

impl<T, E> GenericRepository<T, E>
where
    T: Clone + Send + Sync + 'static,
    E: std::error::Error + Send + Sync + 'static,
{
    pub fn new(
        metrics: Arc<RwLock<UnifiedMetrics>>,
        validator: Arc<dyn Validator<T>>,
    ) -> Self {
        Self {
            items: Arc::new(RwLock::new(HashMap::new())),
            metrics,
            validator,
            _error: std::marker::PhantomData,
        }
    }

    async fn validate(&self, item: &T) -> Result<ValidationResult, E> {
        self.validator.validate(item).await
    }

    async fn update_metrics(&self, operation: &str, start_time: DateTime<Utc>) -> Result<(), MetricsError> {
        let duration = Utc::now().signed_duration_since(start_time);
        let mut metrics = self.metrics.write().await;
        metrics.system.ops_total += 1;
        metrics.system.ops_latency = duration.num_milliseconds() as f64;
        Ok(())
    }
}

#[async_trait]
impl<T, E> Repository for GenericRepository<T, E>
where
    T: Clone + Send + Sync + 'static,
    E: std::error::Error + Send + Sync + 'static,
{
    type Item = T;
    type Error = E;

    async fn create(&self, item: Self::Item) -> Result<Self::Item, Self::Error> {
        let start_time = Utc::now();
        
        // Validate item
        self.validate(&item).await?;
        
        // Store item
        let mut items = self.items.write().await;
        let id = uuid::Uuid::new_v4().to_string();
        items.insert(id, item.clone());
        
        // Update metrics
        self.update_metrics("create", start_time).await.map_err(|e| {
            error!("Failed to update metrics: {}", e);
            e
        })?;
        
        Ok(item)
    }

    async fn read(&self, id: &str) -> Result<Option<Self::Item>, Self::Error> {
        let start_time = Utc::now();
        
        let items = self.items.read().await;
        let item = items.get(id).cloned();
        
        self.update_metrics("read", start_time).await.map_err(|e| {
            error!("Failed to update metrics: {}", e);
            e
        })?;
        
        Ok(item)
    }

    async fn update(&self, id: &str, item: Self::Item) -> Result<Self::Item, Self::Error> {
        let start_time = Utc::now();
        
        // Validate item
        self.validate(&item).await?;
        
        // Update item
        let mut items = self.items.write().await;
        if items.contains_key(id) {
            items.insert(id.to_string(), item.clone());
            
            self.update_metrics("update", start_time).await.map_err(|e| {
                error!("Failed to update metrics: {}", e);
                e
            })?;
            
            Ok(item)
        } else {
            Err(E::from("Item not found"))
        }
    }

    async fn delete(&self, id: &str) -> Result<(), Self::Error> {
        let start_time = Utc::now();
        
        let mut items = self.items.write().await;
        if items.remove(id).is_some() {
            self.update_metrics("delete", start_time).await.map_err(|e| {
                error!("Failed to update metrics: {}", e);
                e
            })?;
            
            Ok(())
        } else {
            Err(E::from("Item not found"))
        }
    }

    async fn list(&self) -> Result<Vec<Self::Item>, Self::Error> {
        let start_time = Utc::now();
        
        let items = self.items.read().await;
        let items_vec = items.values().cloned().collect();
        
        self.update_metrics("list", start_time).await.map_err(|e| {
            error!("Failed to update metrics: {}", e);
            e
        })?;
        
        Ok(items_vec)
    }
}

/// Validator trait for repository items
#[async_trait]
pub trait Validator<T>: Send + Sync {
    async fn validate(&self, item: &T) -> Result<ValidationResult, ValidationError>;
}

/// Repository manager for coordinating multiple repositories
pub struct RepositoryManager {
    repositories: HashMap<String, Arc<dyn Repository<Item = Box<dyn Any + Send + Sync>, Error = Box<dyn std::error::Error + Send + Sync>>>>,
    metrics: Arc<RwLock<UnifiedMetrics>>,
}

impl RepositoryManager {
    pub fn new(metrics: Arc<RwLock<UnifiedMetrics>>) -> Self {
        Self {
            repositories: HashMap::new(),
            metrics,
        }
    }

    pub fn register_repository<R>(&mut self, name: &str, repository: R)
    where
        R: Repository<Item = Box<dyn Any + Send + Sync>, Error = Box<dyn std::error::Error + Send + Sync>> + 'static,
    {
        self.repositories.insert(name.to_string(), Arc::new(repository));
    }

    pub fn get_repository(&self, name: &str) -> Option<Arc<dyn Repository<Item = Box<dyn Any + Send + Sync>, Error = Box<dyn std::error::Error + Send + Sync>>>> {
        self.repositories.get(name).cloned()
    }

    pub async fn health_check(&self) -> HashMap<String, ComponentHealth> {
        let mut health = HashMap::new();
        for (name, repo) in &self.repositories {
            // Perform basic health check
            let start_time = Utc::now();
            let result = repo.list().await;
            let duration = Utc::now().signed_duration_since(start_time);
            
            health.insert(name.clone(), ComponentHealth {
                operational: result.is_ok(),
                health_score: if result.is_ok() { 100.0 } else { 0.0 },
                last_incident: if result.is_err() { Some(Utc::now()) } else { None },
                error_count: if result.is_err() { 1 } else { 0 },
                warning_count: 0,
            });
        }
        health
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug)]
    struct TestItem {
        id: String,
        value: String,
    }

    struct TestValidator;

    #[async_trait]
    impl Validator<TestItem> for TestValidator {
        async fn validate(&self, item: &TestItem) -> Result<ValidationResult, ValidationError> {
            if item.value.is_empty() {
                Ok(ValidationResult::Invalid("Value cannot be empty".to_string()))
            } else {
                Ok(ValidationResult::Valid)
            }
        }
    }

    #[tokio::test]
    async fn test_generic_repository() {
        let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
        let validator = Arc::new(TestValidator);
        let repo = GenericRepository::<TestItem, Box<dyn std::error::Error + Send + Sync>>::new(
            metrics.clone(),
            validator,
        );

        // Test create
        let item = TestItem {
            id: "1".to_string(),
            value: "test".to_string(),
        };
        let created = repo.create(item.clone()).await.unwrap();
        assert_eq!(created.value, item.value);

        // Test read
        let read = repo.read(&item.id).await.unwrap().unwrap();
        assert_eq!(read.value, item.value);

        // Test update
        let updated_item = TestItem {
            id: item.id.clone(),
            value: "updated".to_string(),
        };
        let updated = repo.update(&item.id, updated_item.clone()).await.unwrap();
        assert_eq!(updated.value, "updated");

        // Test list
        let items = repo.list().await.unwrap();
        assert_eq!(items.len(), 1);

        // Test delete
        repo.delete(&item.id).await.unwrap();
        let items = repo.list().await.unwrap();
        assert_eq!(items.len(), 0);
    }
}
