mod support;
mod error;

pub use support::{Web5Support, Web5Operations};
pub use error::Web5Error;

use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use ring::{rand, signature};
use base64::{Engine as _, engine::general_purpose};
use chrono::{DateTime, Utc};

// Re-export types from the web5 crate
pub use web5::{
    did::{DID, KeyMethod},
    dids::methods::key::DIDKey,
    credentials::{Credential, CredentialSubject},
    data_model::{Record, RecordQuery},
    protocols::{Protocol, ProtocolDefinition},
};

// Import and re-export functionality from @web5/common
pub mod common {
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Jwk {
        pub kty: String,
        pub crv: String,
        pub x: String,
        pub y: Option<String>,
        pub d: Option<String>,
    }

    pub fn base64url_encode(input: &[u8]) -> String {
        general_purpose::URL_SAFE_NO_PAD.encode(input)
    }

    pub fn base64url_decode(input: &str) -> Result<Vec<u8>, Web5Error> {
        general_purpose::URL_SAFE_NO_PAD.decode(input)
            .map_err(|e| Web5Error::DecodingError(e.to_string()))
    }
}

// Import and re-export functionality from @web5/credentials
pub mod credentials {
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct VerifiableCredential {
        pub context: Vec<String>,
        pub id: String,
        pub type_: Vec<String>,
        pub issuer: String,
        pub issuance_date: String,
        pub credential_subject: CredentialSubject,
        pub proof: Option<Proof>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Proof {
        pub type_: String,
        pub created: String,
        pub verification_method: String,
        pub proof_purpose: String,
        pub proof_value: String,
    }

    pub async fn issue_credential(credential: &VerifiableCredential, key: &common::Jwk) -> Result<VerifiableCredential, Web5Error> {
        let mut issued_credential = credential.clone();
        let signing_key = crypto::jwk_to_signing_key(key)?;
        let signature = crypto::sign(&serde_json::to_vec(&issued_credential)?, &signing_key)?;
        
        issued_credential.proof = Some(Proof {
            type_: "Ed25519Signature2018".to_string(),
            created: Utc::now().to_rfc3339(),
            verification_method: format!("{}#keys-1", credential.issuer),
            proof_purpose: "assertionMethod".to_string(),
            proof_value: common::base64url_encode(&signature),
        });

        Ok(issued_credential)
    }

    pub async fn verify_credential(credential: &VerifiableCredential) -> Result<bool, Web5Error> {
        if let Some(proof) = &credential.proof {
            let verification_key = dids::resolve_verification_key(&credential.issuer).await?;
            let signature = common::base64url_decode(&proof.proof_value)?;
            let credential_without_proof = {
                let mut c = credential.clone();
                c.proof = None;
                c
            };
            crypto::verify(&serde_json::to_vec(&credential_without_proof)?, &signature, &verification_key)
        } else {
            Err(Web5Error::VerificationError("Credential has no proof".to_string()))
        }
    }
}

// Import and re-export functionality from @web5/crypto
pub mod crypto {
    use super::*;

    pub async fn generate_key_pair(key_type: &str, curve: &str) -> Result<common::Jwk, Web5Error> {
        match (key_type, curve) {
            ("EC", "P-256") => {
                let rng = rand::SystemRandom::new();
                let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng)
                    .map_err(|e| Web5Error::KeyGenerationError(e.to_string()))?;
                let key_pair = signature::Ed25519KeyPair::from_pkcs8(&pkcs8_bytes)
                    .map_err(|e| Web5Error::KeyGenerationError(e.to_string()))?;
                
                Ok(common::Jwk {
                    kty: "EC".to_string(),
                    crv: "P-256".to_string(),
                    x: common::base64url_encode(key_pair.public_key().as_ref()),
                    y: None,
                    d: Some(common::base64url_encode(&pkcs8_bytes)),
                })
            },
            _ => Err(Web5Error::UnsupportedKeyType),
        }
    }

    pub fn jwk_to_signing_key(jwk: &common::Jwk) -> Result<signature::Ed25519KeyPair, Web5Error> {
        let pkcs8_bytes = common::base64url_decode(&jwk.d.as_ref().ok_or(Web5Error::InvalidKey)?)?;
        signature::Ed25519KeyPair::from_pkcs8(&pkcs8_bytes)
            .map_err(|e| Web5Error::KeyConversionError(e.to_string()))
    }

    pub fn sign(data: &[u8], key: &signature::Ed25519KeyPair) -> Result<Vec<u8>, Web5Error> {
        Ok(key.sign(data).as_ref().to_vec())
    }

    pub fn verify(data: &[u8], signature: &[u8], key: &common::Jwk) -> Result<bool, Web5Error> {
        let public_key_bytes = common::base64url_decode(&key.x)?;
        let public_key = signature::UnparsedPublicKey::new(&signature::ED25519, &public_key_bytes);
        public_key.verify(data, signature)
            .map(|_| true)
            .map_err(|e| Web5Error::VerificationError(e.to_string()))
    }
}

// Import and re-export functionality from @web5/dids
pub mod dids {
    use super::*;

    #[async_trait]
    pub trait DidMethod {
        async fn create(&self) -> Result<DID, Web5Error>;
        async fn resolve(&self, did: &str) -> Result<DidDocument, Web5Error>;
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DidDocument {
        pub id: String,
        pub verification_method: Vec<VerificationMethod>,
        pub authentication: Vec<String>,
        pub assertion_method: Vec<String>,
        pub key_agreement: Vec<String>,
        pub capability_invocation: Vec<String>,
        pub capability_delegation: Vec<String>,
        pub service: Vec<Service>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct VerificationMethod {
        pub id: String,
        pub type_: String,
        pub controller: String,
        pub public_key_jwk: common::Jwk,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Service {
        pub id: String,
        pub type_: String,
        pub service_endpoint: String,
    }

    pub struct DidKey;

    #[async_trait]
    impl DidMethod for DidKey {
        async fn create(&self) -> Result<DID, Web5Error> {
            let key_pair = crypto::generate_key_pair("EC", "P-256").await?;
            let did = format!("did:key:{}", key_pair.x);
            Ok(DID::parse(&did).map_err(|e| Web5Error::DIDCreationError(e.to_string()))?)
        }

        async fn resolve(&self, did: &str) -> Result<DidDocument, Web5Error> {
            if !did.starts_with("did:key:") {
                return Err(Web5Error::InvalidDID);
            }
            let key_base64 = did.strip_prefix("did:key:").ok_or(Web5Error::InvalidDID)?;
            let key_bytes = common::base64url_decode(key_base64)?;
            
            let verification_method = VerificationMethod {
                id: format!("{}#keys-1", did),
                type_: "Ed25519VerificationKey2018".to_string(),
                controller: did.to_string(),
                public_key_jwk: common::Jwk {
                    kty: "EC".to_string(),
                    crv: "P-256".to_string(),
                    x: key_base64.to_string(),
                    y: None,
                    d: None,
                },
            };

            Ok(DidDocument {
                id: did.to_string(),
                verification_method: vec![verification_method.clone()],
                authentication: vec![verification_method.id.clone()],
                assertion_method: vec![verification_method.id.clone()],
                key_agreement: vec![],
                capability_invocation: vec![],
                capability_delegation: vec![],
                service: vec![],
            })
        }
    }

    pub async fn resolve_verification_key(did: &str) -> Result<common::Jwk, Web5Error> {
        let did_key = DidKey;
        let did_doc = did_key.resolve(did).await?;
        did_doc.verification_method.first()
            .map(|vm| vm.public_key_jwk.clone())
            .ok_or(Web5Error::VerificationKeyNotFound)
    }
}

pub const WEB5_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn initialize_web5() -> Result<Web5Support, Web5Error> {
    log::info!("Initializing Web5 version {}", WEB5_VERSION);
    Web5Support::new()
}

pub trait Web5Identifiable {
    fn get_did(&self) -> &DID;
}

#[cfg(feature = "web5_setup")]
pub mod setup {
    use super::*;

    pub async fn setup_web5_environment() -> Result<Web5Support, Web5Error> {
        initialize_web5()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web5_version() {
        assert!(!WEB5_VERSION.is_empty());
    }

    #[tokio::test]
    async fn test_initialize_web5() {
        let result = initialize_web5();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_and_verify_credential() {
        let issuer_key = crypto::generate_key_pair("EC", "P-256").await.unwrap();
        let credential = credentials::VerifiableCredential {
            context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            id: "http://example.edu/credentials/3732".to_string(),
            type_: vec!["VerifiableCredential".to_string()],
            issuer: "did:example:123".to_string(),
            issuance_date: Utc::now().to_rfc3339(),
            credential_subject: CredentialSubject::new(serde_json::json!({"id": "did:example:456", "degree": {"type": "BachelorDegree", "name": "Bachelor of Science and Arts"}})),
            proof: None,
        };

        let issued_credential = credentials::issue_credential(&credential, &issuer_key).await.unwrap();
        let verification_result = credentials::verify_credential(&issued_credential).await.unwrap();
        assert!(verification_result);
    }

    #[tokio::test]
    async fn test_did_key_creation_and_resolution() {
        let did_key = dids::DidKey;
        let did = did_key.create().await.unwrap();
        let did_doc = did_key.resolve(did.as_str()).await.unwrap();
        assert_eq!(did_doc.id, did.to_string());
    }
}