use std::sync::Arc;
use tokio::sync::RwLock;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::metrics::{UnifiedMetrics, ComponentHealth, MetricsError};
use crate::repository::{Repository, RepositoryManager};
use crate::security::SecurityContext;
use crate::validation::ValidationResult;

/// Core service trait that all services must implement
#[async_trait]
pub trait Service: Send + Sync {
    type Item;
    type Error;

    async fn process(&self, context: &SecurityContext, item: Self::Item) -> Result<Self::Item, Self::Error>;
    async fn validate(&self, item: &Self::Item) -> Result<ValidationResult, Self::Error>;
    async fn get_health(&self) -> Result<ComponentHealth, Self::Error>;
}

/// Generic service implementation with metrics and validation
pub struct GenericService<T, E> {
    repository: Arc<dyn Repository<Item = T, Error = E>>,
    metrics: Arc<RwLock<UnifiedMetrics>>,
    security: Arc<dyn SecurityManager>,
}

impl<T, E> GenericService<T, E>
where
    T: Clone + Send + Sync + 'static,
    E: std::error::Error + Send + Sync + 'static,
{
    pub fn new(
        repository: Arc<dyn Repository<Item = T, Error = E>>,
        metrics: Arc<RwLock<UnifiedMetrics>>,
        security: Arc<dyn SecurityManager>,
    ) -> Self {
        Self {
            repository,
            metrics,
            security,
        }
    }

    async fn update_metrics(&self, operation: &str, start_time: DateTime<Utc>) -> Result<(), MetricsError> {
        let duration = Utc::now().signed_duration_since(start_time);
        let mut metrics = self.metrics.write().await;
        metrics.system.ops_total += 1;
        metrics.system.ops_latency = duration.num_milliseconds() as f64;
        Ok(())
    }

    async fn check_security(&self, context: &SecurityContext) -> Result<(), E> {
        self.security.validate_context(context).await.map_err(|e| E::from(e))
    }
}

#[async_trait]
impl<T, E> Service for GenericService<T, E>
where
    T: Clone + Send + Sync + 'static,
    E: std::error::Error + Send + Sync + 'static,
{
    type Item = T;
    type Error = E;

    async fn process(&self, context: &SecurityContext, item: Self::Item) -> Result<Self::Item, Self::Error> {
        let start_time = Utc::now();

        // Check security context
        self.check_security(context).await?;

        // Validate item
        self.validate(&item).await?;

        // Process item through repository
        let result = self.repository.create(item).await?;

        // Update metrics
        self.update_metrics("process", start_time).await.map_err(|e| {
            error!("Failed to update metrics: {}", e);
            E::from(e)
        })?;

        Ok(result)
    }

    async fn validate(&self, item: &Self::Item) -> Result<ValidationResult, Self::Error> {
        // Delegate validation to repository
        self.repository.validate(item).await
    }

    async fn get_health(&self) -> Result<ComponentHealth, Self::Error> {
        let start_time = Utc::now();
        
        // Check repository health
        let items = self.repository.list().await?;
        let duration = Utc::now().signed_duration_since(start_time);
        
        Ok(ComponentHealth {
            operational: true,
            health_score: 100.0,
            last_incident: None,
            error_count: 0,
            warning_count: 0,
        })
    }
}

/// Service manager for coordinating multiple services
pub struct ServiceManager {
    services: HashMap<String, Arc<dyn Service<Item = Box<dyn Any + Send + Sync>, Error = Box<dyn std::error::Error + Send + Sync>>>>,
    metrics: Arc<RwLock<UnifiedMetrics>>,
    repository_manager: Arc<RepositoryManager>,
    security_manager: Arc<dyn SecurityManager>,
}

impl ServiceManager {
    pub fn new(
        metrics: Arc<RwLock<UnifiedMetrics>>,
        repository_manager: Arc<RepositoryManager>,
        security_manager: Arc<dyn SecurityManager>,
    ) -> Self {
        Self {
            services: HashMap::new(),
            metrics,
            repository_manager,
            security_manager,
        }
    }

    pub fn register_service<S>(&mut self, name: &str, service: S)
    where
        S: Service<Item = Box<dyn Any + Send + Sync>, Error = Box<dyn std::error::Error + Send + Sync>> + 'static,
    {
        self.services.insert(name.to_string(), Arc::new(service));
    }

    pub fn get_service(&self, name: &str) -> Option<Arc<dyn Service<Item = Box<dyn Any + Send + Sync>, Error = Box<dyn std::error::Error + Send + Sync>>>> {
        self.services.get(name).cloned()
    }

    pub async fn health_check(&self) -> HashMap<String, ComponentHealth> {
        let mut health = HashMap::new();
        for (name, service) in &self.services {
            match service.get_health().await {
                Ok(component_health) => {
                    health.insert(name.clone(), component_health);
                }
                Err(e) => {
                    error!("Health check failed for service {}: {}", name, e);
                    health.insert(name.clone(), ComponentHealth {
                        operational: false,
                        health_score: 0.0,
                        last_incident: Some(Utc::now()),
                        error_count: 1,
                        warning_count: 0,
                    });
                }
            }
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

    struct TestRepository;
    struct TestSecurityManager;

    #[async_trait]
    impl Repository for TestRepository {
        type Item = TestItem;
        type Error = Box<dyn std::error::Error + Send + Sync>;

        async fn create(&self, item: Self::Item) -> Result<Self::Item, Self::Error> {
            Ok(item)
        }

        async fn validate(&self, _item: &Self::Item) -> Result<ValidationResult, Self::Error> {
            Ok(ValidationResult::Valid)
        }
    }

    #[async_trait]
    impl SecurityManager for TestSecurityManager {
        async fn validate_context(&self, _context: &SecurityContext) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_generic_service() {
        let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
        let repository = Arc::new(TestRepository);
        let security = Arc::new(TestSecurityManager);
        
        let service = GenericService::new(repository, metrics, security);

        let context = SecurityContext::default();
        let item = TestItem {
            id: "1".to_string(),
            value: "test".to_string(),
        };

        let result = service.process(&context, item.clone()).await.unwrap();
        assert_eq!(result.value, item.value);

        let health = service.get_health().await.unwrap();
        assert!(health.operational);
        assert_eq!(health.health_score, 100.0);
    }
}
