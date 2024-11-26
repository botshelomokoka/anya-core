use did_key::{DIDKey, KeyMaterial, CONFIG_LD_PUBLIC};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemIdentity {
    did: String,
    verification_methods: Vec<VerificationMethod>,
    services: Vec<Service>,
}

pub struct DIDManager {
    identities: HashMap<String, SystemIdentity>,
    key_manager: KeyManager,
    dwn_client: Arc<DWNClient>,
}

impl DIDManager {
    pub async fn create_system_identity(&self) -> Result<SystemIdentity> {
        // Generate key material
        let key_pair = self.key_manager.generate_key_pair()?;
        
        // Create DID
        let did = DIDKey::new(&key_pair)
            .set_verification_method("key-1")
            .set_service("dwn", "DWNService")
            .build()?;
            
        // Create system identity
        let identity = SystemIdentity {
            did: did.to_string(),
            verification_methods: vec![
                VerificationMethod::new("key-1", &key_pair.public_key()),
            ],
            services: vec![
                Service::new("dwn", "DWNService", &self.dwn_client.endpoint()),
            ],
        };
        
        // Store identity
        self.store_identity(&identity).await?;
        
        Ok(identity)
    }
}

