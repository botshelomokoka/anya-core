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

mod discovery;
mod connection;

pub use discovery::NetworkDiscovery;
pub use connection::ConnectionManager;
