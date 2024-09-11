use libp2p::{
    core::upgrade,
    floodsub::{Floodsub, FloodsubEvent, Topic},
    mdns::{Mdns, MdnsEvent},
    swarm::{NetworkBehaviourEventProcess, Swarm},
    NetworkBehaviour, PeerId,
};
use log::{error, info};
use std::error::Error;
use tokio::sync::mpsc;

#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
struct AnyadiscoveryBehaviour {
    floodsub: Floodsub,
    mdns: Mdns,
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for AnyadiscoveryBehaviour {
    fn inject_event(&mut self, event: FloodsubEvent) {
        if let FloodsubEvent::Message(message) = event {
            info!(
                "Received: '{:?}' from {:?}",
                String::from_utf8_lossy(&message.data),
                message.source
            );
        }
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for AnyadiscoveryBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer, _) in list {
                    self.floodsub.add_node_to_partial_view(peer);
                }
            }
            MdnsEvent::Expired(list) => {
                for (peer, _) in list {
                    if !self.mdns.has_node(&peer) {
                        self.floodsub.remove_node_from_partial_view(&peer);
                    }
                }
            }
        }
    }
}

pub struct NetworkDiscovery {
    swarm: Swarm<AnyadiscoveryBehaviour>,
}

impl NetworkDiscovery {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let local_key = libp2p::identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());

        let transport = libp2p::development_transport(local_key).await?;

        let mut behaviour = AnyadiscoveryBehaviour {
            floodsub: Floodsub::new(local_peer_id),
            mdns: Mdns::new(Default::default()).await?,
        };

        let topic = Topic::new("anya-network");
        behaviour.floodsub.subscribe(topic);

        let swarm = Swarm::new(transport, behaviour, local_peer_id);

        Ok(Self { swarm })
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let (tx, mut rx) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                println!("Received message: {}", message);
            }
        });

        loop {
            tokio::select! {
                event = self.swarm.next() => {
                    match event {
                        Some(event) => {
                            if let libp2p::swarm::SwarmEvent::Behaviour(event) = event {
                                // Handle the event
                            }
                        }
                        None => break,
                    }
                }
            }
        }

        Ok(())
    }
}