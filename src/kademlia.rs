use std::error::Error;
use std::time::Duration;
use libp2p::{
    core::upgrade,
    futures::StreamExt,
    kad::{
        Kademlia, KademliaConfig, KademliaEvent, QueryResult, Record, RecordStore,
        store::MemoryStore,
    },
    mplex, noise,
    swarm::{Swarm, SwarmBuilder},
    tcp::TokioTcpConfig,
    Transport,
};
use tokio::time::timeout;
use log::{info, error};

pub struct KademliaServer {
    swarm: Swarm<Kademlia<MemoryStore>>,
}

impl KademliaServer {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());

        let transport = TokioTcpConfig::new()
            .nodelay(true)
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::NoiseConfig::xx(local_key).into_authenticated())
            .multiplex(mplex::MplexConfig::new())
            .boxed();

        let store = MemoryStore::new(local_peer_id);
        let kademlia = Kademlia::new(local_peer_id, store);

        let mut swarm = SwarmBuilder::new(transport, kademlia, local_peer_id)
            .executor(Box::new(|fut| {
                tokio::spawn(fut);
            }))
            .build();

        Ok(KademliaServer { swarm })
    }

    pub async fn start(&mut self, addr: &str) -> Result<(), Box<dyn Error>> {
        self.swarm.listen_on(addr.parse()?)?;
        info!("Kademlia server listening on {}", addr);

        loop {
            match self.swarm.next().await {
                Some(event) => self.handle_event(event).await?,
                None => break,
            }
        }

        Ok(())
    }

    async fn handle_event(&mut self, event: KademliaEvent) -> Result<(), Box<dyn Error>> {
        match event {
            KademliaEvent::OutboundQueryCompleted { result, .. } => {
                match result {
                    QueryResult::GetRecord(Ok(ok)) => {
                        for PeerRecord { record, .. } in ok.records {
                            info!("Got record: {:?}", record);
                        }
                    }
                    QueryResult::PutRecord(Ok(_)) => {
                        info!("Successfully put record");
                    }
                    QueryResult::GetClosestPeers(Ok(ok)) => {
                        info!("Got closest peers: {:?}", ok.peers);
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
        let quorum = 1;
        match timeout(
            Duration::from_secs(60),
            self.swarm.behaviour_mut().put_record(record, quorum),
        )
        .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn get_record(&mut self, key: Vec<u8>) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
        let quorum = 1;
        match timeout(
            Duration::from_secs(60),
            self.swarm.behaviour_mut().get_record(&key, quorum),
        )
        .await
        {
            Ok(Ok(ok)) => Ok(ok.records.into_iter().next().map(|r| r.record.value)),
            Ok(Err(e)) => Err(Box::new(e)),
            Err(e) => Err(Box::new(e)),
        }
    }
}
