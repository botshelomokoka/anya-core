use log::info;
use crate::blockchain::BlockchainPort;
use crate::networking::NetworkingPort;
use crate::identity::IdentityPort;

pub struct HexagonalArchitecture {
    domain: Domain,
    ports: Ports,
    adapters: Adapters,
}

pub struct Domain {
    // Core business logic components
    blockchain: Box<dyn BlockchainPort>,
    networking: Box<dyn NetworkingPort>,
    identity: Box<dyn IdentityPort>,
}

pub struct Ports {
    // Input and output ports (interfaces)
    blockchain: Box<dyn BlockchainPort>,
    networking: Box<dyn NetworkingPort>,
    identity: Box<dyn IdentityPort>,
}

pub struct Adapters {
    // Primary (driving) and secondary (driven) adapters
    blockchain_adapter: Box<dyn BlockchainPort>,
    networking_adapter: Box<dyn NetworkingPort>,
    identity_adapter: Box<dyn IdentityPort>,
}

impl HexagonalArchitecture {
    pub fn new(
        blockchain: Box<dyn BlockchainPort>,
        networking: Box<dyn NetworkingPort>,
        identity: Box<dyn IdentityPort>,
    ) -> Self {
        HexagonalArchitecture {
            domain: Domain {
                blockchain: blockchain.clone(),
                networking: networking.clone(),
                identity: identity.clone(),
            },
            ports: Ports {
                blockchain: blockchain.clone(),
                networking: networking.clone(),
                identity: identity.clone(),
            },
            adapters: Adapters {
                blockchain_adapter: blockchain,
                networking_adapter: networking,
                identity_adapter: identity,
            },
        }
    }

    pub fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Initializing Hexagonal Architecture");
        self.domain.blockchain.init()?;
        self.domain.networking.init()?;
        self.domain.identity.init()?;
        Ok(())
    }
}

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up Hexagonal Architecture");
    // Hexagonal architecture will be initialized in main.rs
    Ok(())
}