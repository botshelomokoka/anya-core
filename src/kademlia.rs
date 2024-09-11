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
use std::error::Error;
use libp2p::{
    core::upgrade,
    futures::StreamExt,
    kad::{Kademlia, KademliaEvent, QueryResult, Record, store::MemoryStore},
    swarm::{Swarm, SwarmEvent},
    identity, PeerId, Multiaddr,
};
use log::{info, error};

pub struct KademliaServer {
    swarm: Swarm<Kademlia<MemoryStore>>,
}

impl KademliaServer {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        let store = MemoryStore::new(local_peer_id.clone());
        let behaviour = Kademlia::new(local_peer_id.clone(), store);
        let transport = libp2p::development_transport(local_key).await?;
        let swarm = Swarm::new(transport, behaviour, local_peer_id);

        Ok(Self { swarm })
    }

    pub async fn start(&mut self, addr: Multiaddr) -> Result<(), Box<dyn Error>> {
        self.swarm.listen_on(addr)?;
        info!("Kademlia server started on {:?}", addr);

        loop {
            match self.swarm.next().await {
                Some(event) => self.handle_event(event).await?,
                None => break,
            }
        }

        Ok(())
    }

    async fn handle_event(&mut self, event: SwarmEvent<KademliaEvent>) -> Result<(), Box<dyn Error>> {
        match event {
            SwarmEvent::Behaviour(KademliaEvent::OutboundQueryCompleted { result, .. }) => {
                match result {
                    QueryResult::GetRecord(Ok(ok)) => {
                        for PeerRecord { record, .. } in ok.records {
                            info!("Got record: {:?}", record);
                        }
                    }
                    QueryResult::PutRecord(Ok(_)) => {
                        info!("Successfully put record");
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub async fn put_record(&mut self, key: Vec<u8>, value: Vec<u8>) -> Result<(), Box<dyn Error>> {
        let record = Record {
            key,
            value,
            publisher: None,
            expires: None,
        };
<<<<<<< HEAD
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
=======
        self.swarm.behaviour_mut().put_record(record, libp2p::kad::Quorum::One)?;
        Ok(())
    }

    pub async fn get_record(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.swarm.behaviour_mut().get_record(key, libp2p::kad::Quorum::One);
        // ... (implement logic to receive and return the record)
        Ok(None)
    }
}
>>>>>>> b706d7c49205d3634e6b11d0309d8911a18a435c
