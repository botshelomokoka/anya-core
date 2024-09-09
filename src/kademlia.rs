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
