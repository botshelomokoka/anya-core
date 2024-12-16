use std::sync::Once;
use tokio::runtime::Runtime;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

static INIT: Once = Once::new();

pub fn setup_test_env() {
    INIT.call_once(|| {
        // Initialize logging for tests
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::DEBUG)
            .with_test_writer()
            .init();

        // Set up test environment variables
        std::env::set_var("ENVIRONMENT", "test");
        std::env::set_var("DATABASE_URL", "sqlite::memory:");
    });
}

pub fn get_test_runtime() -> Runtime {
    Runtime::new().expect("Failed to create Tokio runtime")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::CONFIG;
    use crate::security::SecurityManager;
    use crate::cache::CacheManager;
    use std::time::Duration;

    #[test]
    fn test_environment_setup() {
        setup_test_env();
        assert_eq!(std::env::var("ENVIRONMENT").unwrap(), "test");
    }

    #[tokio::test]
    async fn test_config_loading() {
        setup_test_env();
        let config = CONFIG.read().await;
        assert!(config.get_string("environment").is_some());
    }

    #[tokio::test]
    async fn test_security_manager() {
        setup_test_env();
        let security = SecurityManager::new().await.unwrap();
        
        // Test password hashing
        let password = "test_password";
        let hash = security.hash_password(password).unwrap();
        assert!(security.verify_password(password, &hash).unwrap());
        
        // Test JWT
        let token = security.generate_jwt("test_user", "user").unwrap();
        let claims = security.verify_jwt(&token).unwrap();
        assert_eq!(claims.sub, "test_user");
    }

    #[tokio::test]
    async fn test_cache_manager() {
        setup_test_env();
        let cache = CacheManager::new(Default::default());
        
        // Test basic cache operations
        cache.set("test_key".to_string(), vec![1, 2, 3]).await.unwrap();
        let value = cache.get("test_key").await.unwrap();
        assert_eq!(value, vec![1, 2, 3]);
        
        // Test expiration
        tokio::time::sleep(Duration::from_secs(2)).await;
        cache.cleanup().await;
        let stats = cache.get_stats().await;
        assert!(stats.expired_entries == 0);
    }

    #[tokio::test]
    async fn test_resource_limits() {
        setup_test_env();
        let resource_manager = ResourceManager::new().await;
        
        // Test connection acquisition
        let conn = resource_manager.acquire_connection().await.unwrap();
        assert!(resource_manager.check_resource_health().await.is_healthy);
        
        // Test memory allocation
        assert!(resource_manager.allocate_memory(1024).await.is_ok());
        let health = resource_manager.check_resource_health().await;
        assert!(health.memory_usage_percent > 0.0);
    }

    #[tokio::test]
    async fn test_performance_monitoring() {
        setup_test_env();
        let monitor = PerformanceMonitor::new();
        
        // Record some test metrics
        monitor.record_request(Duration::from_millis(100), true).await;
        monitor.update_system_metrics(50.0, 30.0).await;
        
        let health = monitor.get_health_check().await;
        assert_eq!(health.status, "healthy");
        
        let report = monitor.generate_performance_report().await;
        assert_eq!(report.total_requests, 1);
        assert_eq!(report.total_errors, 0);
    }
}
