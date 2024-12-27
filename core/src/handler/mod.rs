use std::sync::Arc;
use tokio::sync::RwLock;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use crate::metrics::{UnifiedMetrics, ComponentHealth, MetricsError};
use crate::service::{Service, ServiceManager};
use crate::security::SecurityContext;
use crate::validation::ValidationResult;

/// Core handler trait that all handlers must implement
#[async_trait]
pub trait Handler: Send + Sync {
    type Request;
    type Response;
    type Error;

    async fn handle(&self, context: &SecurityContext, request: Self::Request) -> Result<Self::Response, Self::Error>;
    async fn validate(&self, request: &Self::Request) -> Result<ValidationResult, Self::Error>;
    async fn get_health(&self) -> Result<ComponentHealth, Self::Error>;
}

/// Generic request handler implementation
pub struct GenericHandler<Req, Res, E> {
    service: Arc<dyn Service<Item = Req, Error = E>>,
    metrics: Arc<RwLock<UnifiedMetrics>>,
    security: Arc<dyn SecurityManager>,
    _response: std::marker::PhantomData<Res>,
}

impl<Req, Res, E> GenericHandler<Req, Res, E>
where
    Req: Clone + Send + Sync + 'static,
    Res: From<Req> + Send + Sync + 'static,
    E: std::error::Error + Send + Sync + 'static,
{
    pub fn new(
        service: Arc<dyn Service<Item = Req, Error = E>>,
        metrics: Arc<RwLock<UnifiedMetrics>>,
        security: Arc<dyn SecurityManager>,
    ) -> Self {
        Self {
            service,
            metrics,
            security,
            _response: std::marker::PhantomData,
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
impl<Req, Res, E> Handler for GenericHandler<Req, Res, E>
where
    Req: Clone + Send + Sync + 'static,
    Res: From<Req> + Send + Sync + 'static,
    E: std::error::Error + Send + Sync + 'static,
{
    type Request = Req;
    type Response = Res;
    type Error = E;

    async fn handle(&self, context: &SecurityContext, request: Self::Request) -> Result<Self::Response, Self::Error> {
        let start_time = Utc::now();

        // Check security context
        self.check_security(context).await?;

        // Validate request
        self.validate(&request).await?;

        // Process through service layer
        let result = self.service.process(context, request).await?;

        // Convert to response type
        let response = Res::from(result);

        // Update metrics
        self.update_metrics("handle", start_time).await.map_err(|e| {
            error!("Failed to update metrics: {}", e);
            E::from(e)
        })?;

        Ok(response)
    }

    async fn validate(&self, request: &Self::Request) -> Result<ValidationResult, Self::Error> {
        // Delegate validation to service
        self.service.validate(request).await
    }

    async fn get_health(&self) -> Result<ComponentHealth, Self::Error> {
        // Delegate health check to service
        self.service.get_health().await
    }
}

/// Handler manager for coordinating multiple handlers
pub struct HandlerManager {
    handlers: HashMap<String, Arc<dyn Handler<Request = Box<dyn Any + Send + Sync>, Response = Box<dyn Any + Send + Sync>, Error = Box<dyn std::error::Error + Send + Sync>>>>,
    metrics: Arc<RwLock<UnifiedMetrics>>,
    service_manager: Arc<ServiceManager>,
    security_manager: Arc<dyn SecurityManager>,
}

impl HandlerManager {
    pub fn new(
        metrics: Arc<RwLock<UnifiedMetrics>>,
        service_manager: Arc<ServiceManager>,
        security_manager: Arc<dyn SecurityManager>,
    ) -> Self {
        Self {
            handlers: HashMap::new(),
            metrics,
            service_manager,
            security_manager,
        }
    }

    pub fn register_handler<H>(&mut self, name: &str, handler: H)
    where
        H: Handler<Request = Box<dyn Any + Send + Sync>, Response = Box<dyn Any + Send + Sync>, Error = Box<dyn std::error::Error + Send + Sync>> + 'static,
    {
        self.handlers.insert(name.to_string(), Arc::new(handler));
    }

    pub fn get_handler(&self, name: &str) -> Option<Arc<dyn Handler<Request = Box<dyn Any + Send + Sync>, Response = Box<dyn Any + Send + Sync>, Error = Box<dyn std::error::Error + Send + Sync>>>> {
        self.handlers.get(name).cloned()
    }

    pub async fn health_check(&self) -> HashMap<String, ComponentHealth> {
        let mut health = HashMap::new();
        for (name, handler) in &self.handlers {
            match handler.get_health().await {
                Ok(component_health) => {
                    health.insert(name.clone(), component_health);
                }
                Err(e) => {
                    error!("Health check failed for handler {}: {}", name, e);
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
    struct TestRequest {
        id: String,
        value: String,
    }

    #[derive(Clone, Debug, PartialEq)]
    struct TestResponse {
        id: String,
        value: String,
        processed: bool,
    }

    impl From<TestRequest> for TestResponse {
        fn from(req: TestRequest) -> Self {
            TestResponse {
                id: req.id,
                value: req.value,
                processed: true,
            }
        }
    }

    struct TestService;
    struct TestSecurityManager;

    #[async_trait]
    impl Service for TestService {
        type Item = TestRequest;
        type Error = Box<dyn std::error::Error + Send + Sync>;

        async fn process(&self, _context: &SecurityContext, item: Self::Item) -> Result<Self::Item, Self::Error> {
            Ok(item)
        }

        async fn validate(&self, _item: &Self::Item) -> Result<ValidationResult, Self::Error> {
            Ok(ValidationResult::Valid)
        }

        async fn get_health(&self) -> Result<ComponentHealth, Self::Error> {
            Ok(ComponentHealth {
                operational: true,
                health_score: 100.0,
                last_incident: None,
                error_count: 0,
                warning_count: 0,
            })
        }
    }

    #[async_trait]
    impl SecurityManager for TestSecurityManager {
        async fn validate_context(&self, _context: &SecurityContext) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_generic_handler() {
        let metrics = Arc::new(RwLock::new(UnifiedMetrics::default()));
        let service = Arc::new(TestService);
        let security = Arc::new(TestSecurityManager);
        
        let handler = GenericHandler::<TestRequest, TestResponse, Box<dyn std::error::Error + Send + Sync>>::new(
            service,
            metrics,
            security,
        );

        let context = SecurityContext::default();
        let request = TestRequest {
            id: "1".to_string(),
            value: "test".to_string(),
        };

        let response = handler.handle(&context, request.clone()).await.unwrap();
        assert_eq!(response, TestResponse {
            id: request.id,
            value: request.value,
            processed: true,
        });

        let health = handler.get_health().await.unwrap();
        assert!(health.operational);
        assert_eq!(health.health_score, 100.0);
    }
}
