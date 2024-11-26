//! This module provides the network adapter for interacting with the Kademlia DHT.

use crate::kademlia::KademliaModule;
use libp2p::PeerId;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct NetworkAdapter {
    kademlia: Arc<Mutex<KademliaModule>>,
    // Other fields...
}

impl NetworkAdapter {
    pub fn new() -> Self {
        Self {
            kademlia: Arc::new(Mutex::new(KademliaModule::new())),
            // Initialize other fields...
        }
    }
        pub async fn find_nodes(&self) -> Vec<PeerId> {
            let locked_kademlia = self.kademlia.lock().await;
            locked_kademlia.find_nodes().await
        }
    pub async fn store_value(&self, key: &[u8], value: &[u8]) {
        let locked_kademlia = self.kademlia.lock().await;
        let key = key.to_vec();
        let value = value.to_vec();
        let kademlia_clone = locked_kademlia.clone();
        drop(locked_kademlia); // Release the lock

        kademlia_clone.put_value(&key, &value).await;
    }> {
        let kademlia = self.kademlia.lock().await;
        let value = kademlia.get_value(key).await;
        drop(kademlia); // Explicitly drop the lock
        value
    }

    pub async fn get_value(&self, key: &[u8]) -> Option<Vec<u8>> {
        let mut kademlia = self.kademlia.lock().await;
        kademlia.get_value(key).await
    }

    // Other methods...
}

pub mod discovery;
pub mod p2p;
pub mod unified;

pub use discovery::NetworkDiscovery;
pub use p2p::P2PNetwork;
pub use unified::UnifiedNetworkManager;

use thiserror::Error;
use metrics::{counter, gauge};
use log::{info, error};

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("Discovery error: {0}")]
    DiscoveryError(String),
    #[error("P2P error: {0}")]
    P2PError(String),
    #[error("Connection error: {0}")]
    ConnectionError(String),
}

/// Core network metrics for monitoring and observability
struct NetworkMetrics {
    peer_count: gauge::Gauge,
    message_count: counter::Counter,
    bandwidth_usage: gauge::Gauge,
    latency: gauge::Gauge,
}

impl NetworkMetrics {
    fn new() -> Self {
        Self {
            peer_count: gauge!("network_peers_total"),
            message_count: counter!("network_messages_total"),
            bandwidth_usage: gauge!("network_bandwidth_bytes"),
            latency: gauge!("network_latency_ms"),
        }
    }
}
