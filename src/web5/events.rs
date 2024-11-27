use std::sync::Arc;
use tokio::sync::broadcast;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub event_type: EventType,
    pub timestamp: DateTime<Utc>,
    pub data: serde_json::Value,
    pub metadata: EventMetadata,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventMetadata {
    pub source: String,
    pub correlation_id: Option<String>,
    pub user_id: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum EventType {
    // Data Events
    RecordCreated,
    RecordUpdated,
    RecordDeleted,
    BatchOperationCompleted,
    
    // Schema Events
    SchemaRegistered,
    SchemaUpdated,
    
    // Cache Events
    CacheHit,
    CacheMiss,
    CacheEviction,
    
    // System Events
    SystemStartup,
    SystemShutdown,
    HealthCheck,
    Error,
}

pub struct EventBus {
    tx: broadcast::Sender<Event>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self { tx }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.tx.subscribe()
    }

    pub fn publish(&self, event: Event) -> Result<(), broadcast::error::SendError<Event>> {
        self.tx.send(event)
            .map(|_| ())
    }
}

#[derive(Clone)]
pub struct EventPublisher {
    bus: Arc<EventBus>,
    source: String,
}

impl EventPublisher {
    pub fn new(bus: Arc<EventBus>, source: impl Into<String>) -> Self {
        Self {
            bus,
            source: source.into(),
        }
    }

    pub fn publish_event(
        &self,
        event_type: EventType,
        data: impl Serialize,
        correlation_id: Option<String>,
        user_id: Option<String>,
        tags: Vec<String>,
    ) -> Result<(), broadcast::error::SendError<Event>> {
        let event = Event {
            id: uuid::Uuid::new_v4().to_string(),
            event_type,
            timestamp: Utc::now(),
            data: serde_json::to_value(data).unwrap_or_default(),
            metadata: EventMetadata {
                source: self.source.clone(),
                correlation_id,
                user_id,
                tags,
            },
        };
        
        self.bus.publish(event)
    }
}

pub struct EventSubscriber {
    rx: broadcast::Receiver<Event>,
    filters: Vec<Box<dyn Fn(&Event) -> bool + Send + Sync>>,
}

impl EventSubscriber {
    pub fn new(bus: &EventBus) -> Self {
        Self {
            rx: bus.subscribe(),
            filters: Vec::new(),
        }
    }

    pub fn filter<F>(mut self, filter: F) -> Self
    where
        F: Fn(&Event) -> bool + Send + Sync + 'static,
    {
        self.filters.push(Box::new(filter));
        self
    }

    pub fn filter_by_type(self, event_type: EventType) -> Self {
        self.filter(move |event| event.event_type == event_type)
    }

    pub fn filter_by_source(self, source: String) -> Self {
        self.filter(move |event| event.metadata.source == source)
    }

    pub async fn receive(&mut self) -> Option<Event> {
        loop {
            match self.rx.recv().await {
                Ok(event) => {
                    if self.filters.iter().all(|f| f(&event)) {
                        return Some(event);
                    }
                }
                Err(_) => return None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;
    use std::time::Duration;

    #[tokio::test]
    async fn test_event_system() {
        let bus = Arc::new(EventBus::new(100));
        let publisher = EventPublisher::new(Arc::clone(&bus), "test_source");
        
        let mut subscriber = EventSubscriber::new(&bus)
            .filter_by_type(EventType::RecordCreated)
            .filter_by_source("test_source".to_string());

        // Publish test event
        let data = serde_json::json!({
            "record_id": "test123",
            "content": "test data"
        });
        
        publisher.publish_event(
            EventType::RecordCreated,
            data,
            Some("correlation123".to_string()),
            Some("user123".to_string()),
            vec!["test".to_string()]
        ).unwrap();

        // Receive and verify event
        if let Some(event) = subscriber.receive().await {
            assert_eq!(event.event_type, EventType::RecordCreated);
            assert_eq!(event.metadata.source, "test_source");
            assert_eq!(event.metadata.correlation_id, Some("correlation123".to_string()));
        } else {
            panic!("Expected to receive event");
        }
    }
}
