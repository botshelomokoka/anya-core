use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use tokio::time;
use serde::{Deserialize, Serialize};
use log::{info, error};
use async_trait::async_trait;
use libp2p::{
    core::upgrade,
    floodsub::{Floodsub, FloodsubEvent, Topic},
    identity,
    kad::{
        record::store::MemoryStore,
        Kademlia, KademliaEvent, QueryResult, Record, RecordKey,
        GetClosestPeersOk, GetProvidersOk,
    },
    noise,
    swarm::{NetworkBehaviourEventProcess, Swarm, SwarmBuilder, SwarmEvent},
    tcp::TokioTcpConfig,
    NetworkBehaviour, PeerId, Transport,
};
use libp2p::core::multiaddr::MultiAddr;
use crate::state_management::Node;
use crate::user_management::UserManagement;
use crate::ml_logic::MLLogic;
use crate::stx_support::STXSupport;
use crate::dlc_support::DLCSupport;
use crate::lightning_support::LightningSupport;
use crate::bitcoin_support::BitcoinSupport;
use crate::web5_support::Web5Support;

const K: usize = 20; // Maximum number of nodes in a k-bucket
const ALPHA: usize = 3; // Number of parallel lookups
const BUCKET_SIZE: usize = 160; // Number of buckets (for 160-bit node IDs)

#[derive(NetworkBehaviour)]
pub struct KademliaBehaviour {
    kademlia: Kademlia<MemoryStore>,
    floodsub: Floodsub,
}

pub struct KademliaServer {
    swarm: Swarm<KademliaBehaviour>,
    user_management: UserManagement,
    ml_logic: MLLogic,
    stx_support: STXSupport,
    dlc_support: DLCSupport,
    lightning_support: LightningSupport,
    bitcoin_support: BitcoinSupport,
    web5_support: Web5Support,
}

impl KademliaServer {
    pub async fn new(
        user_management: UserManagement,
        ml_logic: MLLogic,
        stx_support: STXSupport,
        dlc_support: DLCSupport,
        lightning_support: LightningSupport,
        bitcoin_support: BitcoinSupport,
        web5_support: Web5Support,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let id_keys = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(id_keys.public());
        info!("Local peer id: {:?}", peer_id);

        let transport = TokioTcpConfig::new()
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::NoiseConfig::xx(id_keys).into_authenticated())
            .boxed();

        let store = MemoryStore::new(peer_id);
        let kademlia = Kademlia::new(peer_id, store);
        let floodsub = Floodsub::new(peer_id);

        let behaviour = KademliaBehaviour { kademlia, floodsub };
        let swarm = SwarmBuilder::new(transport, behaviour, peer_id).build();

        Ok(Self {
            swarm,
            user_management,
            ml_logic,
            stx_support,
            dlc_support,
            lightning_support,
            bitcoin_support,
            web5_support,
        })
    }

    pub async fn run(&mut self) {
        loop {
            match self.swarm.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } => {
                    info!("Listening on {:?}", address);
                },
                SwarmEvent::Behaviour(KademliaBehaviourEvent::Kademlia(KademliaEvent::OutboundQueryCompleted { result, .. })) => {
                    match result {
                        QueryResult::GetClosestPeers(Ok(ok)) => {
                            self.handle_closest_peers(ok).await;
                        }
                        QueryResult::GetProviders(Ok(ok)) => {
                            self.handle_providers(ok).await;
                        }
                        _ => {}
                    }
                },
                SwarmEvent::Behaviour(KademliaBehaviourEvent::Floodsub(FloodsubEvent::Message(message))) => {
                    self.handle_floodsub_message(message).await;
                },
                _ => {}
            }
        }
    }

    async fn handle_closest_peers(&mut self, peers: GetClosestPeersOk) {
        for peer in peers.peers {
            self.swarm.behaviour_mut().kademlia.add_address(&peer, "/ip4/0.0.0.0/tcp/0".parse().unwrap());
        }
    }

    async fn handle_providers(&mut self, providers: GetProvidersOk) {
        for peer in providers.providers {
            if let Some(addr) = self.swarm.behaviour_mut().kademlia.addresses_of_peer(&peer).next() {
                self.swarm.behaviour_mut().kademlia.add_address(&peer, addr.clone());
            }
        }
    }

    async fn handle_floodsub_message(&mut self, message: FloodsubEvent) {
        if let FloodsubEvent::Message(msg) = message {
            if let Ok(content) = String::from_utf8(msg.data) {
                info!("Received message: {:?} from {:?}", content, msg.source);
                // Process the message using other components
                self.user_management.process_message(&content).await;
                self.ml_logic.process_data(&content).await;
                self.stx_support.handle_stx_operation(&content).await;
                self.dlc_support.handle_dlc_operation(&content).await;
                self.lightning_support.handle_lightning_operation(&content).await;
                self.bitcoin_support.handle_bitcoin_operation(&content).await;
                self.web5_support.handle_web5_operation(&content).await;
            }
        }
    }

    pub async fn store(&mut self, key: Vec<u8>, value: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let record = Record {
            key: RecordKey::new(&key),
            value,
            publisher: None,
            expires: None,
        };
        self.swarm.behaviour_mut().kademlia.put_record(record, libp2p::kad::Quorum::One)?;
        Ok(())
    }

    pub async fn get(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>> {
        let record_key = RecordKey::new(key);
        self.swarm.behaviour_mut().kademlia.get_record(&record_key, libp2p::kad::Quorum::One);
        // Note: This is a simplified example. In a real-world scenario, you'd need to wait for and process the query result.
        Ok(None)
    }
}

#[async_trait]
pub trait KademliaInterface {
    async fn store(&mut self, key: Vec<u8>, value: Vec<u8>) -> Result<(), Box<dyn std::error::Error>>;
    async fn get(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>>;
}

#[async_trait]
impl KademliaInterface for KademliaServer {
    async fn store(&mut self, key: Vec<u8>, value: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        self.store(key, value).await
    }

    async fn get(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>> {
        self.get(key).await
    }
}

use libp2p::kad::{Kademlia, KademliaEvent};
use crate::core::NetworkNode;

pub struct KademliaModule {
    kademlia: Kademlia<MemoryStore>,
}

impl KademliaModule {
    pub fn new() -> Self {
        // Initialize Kademlia DHT
    }

    pub async fn put_value(&mut self, key: &[u8], value: &[u8]) {
        // Implement value storage in DHT
    }

    pub async fn get_value(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        // Implement value retrieval from DHT
    }

    pub async fn find_node(&mut self, peer_id: &PeerId) -> Vec<PeerId> {
        // Implement node discovery
    }
}
