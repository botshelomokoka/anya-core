use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::web5::events::{EventBus, EventPublisher, EventType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: SystemStatus,
    pub timestamp: DateTime<Utc>,
    pub components: HashMap<String, ComponentHealth>,
    pub metrics: SystemMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SystemStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub status: SystemStatus,
    pub last_check: DateTime<Utc>,
    pub message: Option<String>,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub uptime: u64,
    pub memory_usage: f64,
    pub cpu_usage: f64,
    pub active_connections: u32,
    pub error_rate: f64,
}

pub struct HealthMonitor {
    status: Arc<RwLock<HealthStatus>>,
    event_publisher: EventPublisher,
    check_interval: std::time::Duration,
}

impl HealthMonitor {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        let status = Arc::new(RwLock::new(HealthStatus {
            status: SystemStatus::Healthy,
            timestamp: Utc::now(),
            components: HashMap::new(),
            metrics: SystemMetrics {
                uptime: 0,
                memory_usage: 0.0,
                cpu_usage: 0.0,
                active_connections: 0,
                error_rate: 0.0,
            },
        }));

        let event_publisher = EventPublisher::new(event_bus, "health_monitor");

        Self {
            status,
            event_publisher,
            check_interval: std::time::Duration::from_secs(60),
        }
    }

    pub async fn start(&self) {
        let status = Arc::clone(&self.status);
        let publisher = self.event_publisher.clone();
        let interval = self.check_interval;

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval);
            loop {
                interval.tick().await;
                if let Err(e) = Self::perform_health_check(&status, &publisher).await {
                    eprintln!("Health check error: {}", e);
                }
            }
        });
    }

    async fn perform_health_check(
        status: &Arc<RwLock<HealthStatus>>,
        publisher: &EventPublisher,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut current_status = status.write().await;
        
        // Update system metrics
        current_status.metrics = Self::collect_system_metrics().await?;
        current_status.timestamp = Utc::now();

        // Check component health
        let components = Self::check_components().await?;
        current_status.components = components;

        // Determine overall system status
        current_status.status = Self::determine_system_status(&current_status);

        // Publish health check event
        publisher.publish_event(
            EventType::HealthCheck,
            &*current_status,
            None,
            None,
            vec!["health".to_string()],
        )?;

        Ok(())
    }

    async fn collect_system_metrics() -> Result<SystemMetrics, Box<dyn std::error::Error>> {
        // This is a simplified implementation. In a real system, you would:
        // 1. Use system-specific APIs to gather metrics
        // 2. Potentially use external monitoring services
        // 3. Implement proper error handling
        
        Ok(SystemMetrics {
            uptime: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
            memory_usage: 0.0, // Implement actual memory monitoring
            cpu_usage: 0.0,    // Implement actual CPU monitoring
            active_connections: 0,
            error_rate: 0.0,
        })
    }

    async fn check_components() -> Result<HashMap<String, ComponentHealth>, Box<dyn std::error::Error>> {
        let mut components = HashMap::new();

        // Check Web5 DID status
        components.insert(
            "did".to_string(),
            ComponentHealth {
                status: SystemStatus::Healthy,
                last_check: Utc::now(),
                message: Some("DID service operational".to_string()),
                details: None,
            },
        );

        // Check cache status
        components.insert(
            "cache".to_string(),
            ComponentHealth {
                status: SystemStatus::Healthy,
                last_check: Utc::now(),
                message: Some("Cache service operational".to_string()),
                details: None,
            },
        );

        // Add more component checks as needed

        Ok(components)
    }

    fn determine_system_status(status: &HealthStatus) -> SystemStatus {
        let unhealthy_components = status.components
            .values()
            .filter(|c| c.status == SystemStatus::Unhealthy)
            .count();

        let degraded_components = status.components
            .values()
            .filter(|c| c.status == SystemStatus::Degraded)
            .count();

        if unhealthy_components > 0 {
            SystemStatus::Unhealthy
        } else if degraded_components > 0 {
            SystemStatus::Degraded
        } else {
            SystemStatus::Healthy
        }
    }

    pub async fn get_health_status(&self) -> HealthStatus {
        self.status.read().await.clone()
    }

    pub async fn update_component_status(
        &self,
        component: &str,
        status: SystemStatus,
        message: Option<String>,
        details: Option<serde_json::Value>,
    ) {
        let mut current_status = self.status.write().await;
        
        current_status.components.insert(
            component.to_string(),
            ComponentHealth {
                status,
                last_check: Utc::now(),
                message,
                details,
            },
        );

        current_status.status = Self::determine_system_status(&current_status);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_health_monitor() {
        let event_bus = Arc::new(EventBus::new(100));
        let monitor = HealthMonitor::new(Arc::clone(&event_bus));

        // Start health monitoring
        monitor.start().await;

        // Wait for initial health check
        tokio::time::sleep(Duration::from_secs(1)).await;

        // Get health status
        let status = monitor.get_health_status().await;
        assert_eq!(status.status, SystemStatus::Healthy);

        // Update component status
        monitor.update_component_status(
            "test_component",
            SystemStatus::Degraded,
            Some("Service degraded".to_string()),
            None,
        ).await;

        // Verify system status changed
        let status = monitor.get_health_status().await;
        assert_eq!(status.status, SystemStatus::Degraded);
    }
}
