use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use crate::architecture::errors::{HexagonalError, HexagonalResult};
use crate::architecture::types::{
    HealthStatus, HealthState, ComponentHealth, HealthCheck,
};

pub struct HealthChecker {
    checks: Vec<Box<dyn HealthCheck>>,
    status_history: Arc<RwLock<Vec<HealthStatus>>>,
    max_history: usize,
}

impl Default for HealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl HealthChecker {
    pub fn new() -> Self {
        Self {
            checks: Vec::new(),
            status_history: Arc::new(RwLock::new(Vec::new())),
            max_history: 100,
        }
    }

    pub fn with_max_history(max_history: usize) -> Self {
        Self {
            checks: Vec::new(),
            status_history: Arc::new(RwLock::new(Vec::new())),
            max_history,
        }
    }

    pub fn register_check(&mut self, check: Box<dyn HealthCheck>) {
        self.checks.push(check);
    }

    pub async fn check_health(&self) -> HealthStatus {
        let mut results = Vec::new();

        for check in &self.checks {
            match check.check().await {
                Ok(result) => results.push(result),
                Err(e) => results.push(ComponentHealth {
                    state: HealthState::Unhealthy,
                    message: Some(format!("Check failed: {}", e)),
                    last_check: chrono::Utc::now(),
                    metrics: std::collections::HashMap::new(),
                }),
            }
        }

        let status = HealthStatus::from_results(results);
        self.update_history(status.clone()).await;
        status
    }

    async fn update_history(&self, status: HealthStatus) {
        let mut history = self.status_history.write().await;
        history.push(status);
        if history.len() > self.max_history {
            history.remove(0);
        }
    }

    pub async fn get_history(&self) -> Vec<HealthStatus> {
        self.status_history.read().await.clone()
    }

    pub async fn get_component_history(&self, component: &str) -> Vec<ComponentHealth> {
        self.status_history
            .read()
            .await
            .iter()
            .filter_map(|status| status.components.get(component).cloned())
            .collect()
    }

    pub async fn get_state_distribution(&self) -> std::collections::HashMap<HealthState, usize> {
        let mut distribution = std::collections::HashMap::new();
        let history = self.status_history.read().await;

        for status in history.iter() {
            *distribution.entry(status.status).or_insert(0) += 1;
        }

        distribution
    }
}

pub struct HealthMonitor {
    checker: Arc<HealthChecker>,
    check_interval: Duration,
    subscribers: Arc<RwLock<Vec<Box<dyn HealthSubscriber>>>>,
}

#[async_trait::async_trait]
pub trait HealthSubscriber: Send + Sync {
    async fn on_health_change(&self, old_status: Option<&HealthStatus>, new_status: &HealthStatus);
}

impl HealthMonitor {
    pub fn new(checker: Arc<HealthChecker>, check_interval: Duration) -> Self {
        Self {
            checker,
            check_interval,
            subscribers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn subscribe(&self, subscriber: Box<dyn HealthSubscriber>) {
        let mut subscribers = self.subscribers.blocking_write();
        subscribers.push(subscriber);
    }

    pub async fn start(self) {
        let mut interval = tokio::time::interval(self.check_interval);
        let mut last_status: Option<HealthStatus> = None;

        loop {
            interval.tick().await;
            let new_status = self.checker.check_health().await;

            if last_status.as_ref().map(|s| s.status) != Some(new_status.status) {
                for subscriber in self.subscribers.read().await.iter() {
                    subscriber.on_health_change(last_status.as_ref(), &new_status).await;
                }
            }

            last_status = Some(new_status);
        }
    }
}

// Example health checks
pub struct SystemHealthCheck {
    threshold: f64,
}

impl SystemHealthCheck {
    pub fn new(threshold: f64) -> Self {
        Self { threshold }
    }
}

#[async_trait::async_trait]
impl HealthCheck for SystemHealthCheck {
    async fn check(&self) -> HexagonalResult<ComponentHealth> {
        let sys_info = sysinfo::System::new_all();
        let cpu_usage = sys_info.global_cpu_info().cpu_usage() as f64;
        let memory_usage = sys_info.used_memory() as f64 / sys_info.total_memory() as f64 * 100.0;

        let mut metrics = std::collections::HashMap::new();
        metrics.insert("cpu_usage".to_string(), cpu_usage);
        metrics.insert("memory_usage".to_string(), memory_usage);

        let state = if cpu_usage > self.threshold || memory_usage > self.threshold {
            HealthState::Degraded
        } else {
            HealthState::Healthy
        };

        Ok(ComponentHealth {
            state,
            message: Some(format!(
                "CPU: {:.1}%, Memory: {:.1}%",
                cpu_usage, memory_usage
            )),
            last_check: chrono::Utc::now(),
            metrics,
        })
    }
}

pub struct DatabaseHealthCheck {
    connection_string: String,
}

impl DatabaseHealthCheck {
    pub fn new(connection_string: String) -> Self {
        Self { connection_string }
    }
}

#[async_trait::async_trait]
impl HealthCheck for DatabaseHealthCheck {
    async fn check(&self) -> HexagonalResult<ComponentHealth> {
        // Simulate database check
        let start = std::time::Instant::now();
        tokio::time::sleep(Duration::from_millis(10)).await;
        let latency = start.elapsed();

        let mut metrics = std::collections::HashMap::new();
        metrics.insert("latency_ms".to_string(), latency.as_millis() as f64);

        Ok(ComponentHealth {
            state: HealthState::Healthy,
            message: Some("Database connection successful".to_string()),
            last_check: chrono::Utc::now(),
            metrics,
        })
    }
}

// Example subscriber
pub struct LoggingHealthSubscriber;

#[async_trait::async_trait]
impl HealthSubscriber for LoggingHealthSubscriber {
    async fn on_health_change(&self, old_status: Option<&HealthStatus>, new_status: &HealthStatus) {
        match old_status {
            Some(old) => log::info!(
                "Health status changed from {:?} to {:?}",
                old.status,
                new_status.status
            ),
            None => log::info!("Initial health status: {:?}", new_status.status),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_health_checker() {
        let mut checker = HealthChecker::new();
        checker.register_check(Box::new(SystemHealthCheck::new(90.0)));
        checker.register_check(Box::new(DatabaseHealthCheck::new(
            "postgres://localhost/test".to_string(),
        )));

        let status = checker.check_health().await;
        assert_eq!(status.components.len(), 2);
        assert!(matches!(status.status, HealthState::Healthy));
    }

    #[tokio::test]
    async fn test_health_monitor() {
        let checker = Arc::new(HealthChecker::new());
        let monitor = HealthMonitor::new(checker, Duration::from_secs(1));
        monitor.subscribe(Box::new(LoggingHealthSubscriber));

        // Start monitoring in background
        let monitor_handle = tokio::spawn(async move {
            monitor.start().await;
        });

        // Let it run for a bit
        tokio::time::sleep(Duration::from_secs(2)).await;
        monitor_handle.abort();
    }

    #[tokio::test]
    async fn test_health_history() {
        let mut checker = HealthChecker::with_max_history(2);
        checker.register_check(Box::new(SystemHealthCheck::new(90.0)));

        // Check health multiple times
        checker.check_health().await;
        checker.check_health().await;
        checker.check_health().await;

        let history = checker.get_history().await;
        assert_eq!(history.len(), 2);
    }
}
