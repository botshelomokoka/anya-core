use super::*;
use anya_core::auth::web5::{
    protocols::identity,
    data_manager::{Web5DataManager, DataRecord},
};
use did_key::Ed25519KeyPair;
use chrono::Utc;

#[tokio::test]
async fn test_identity_credential_creation() {
    let key_pair = Ed25519KeyPair::generate();
    let data_manager = Web5DataManager::new(key_pair).await.unwrap();
    
    let test_credential = serde_json::json!({
        "did": "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK",
        "verificationMethod": [{
            "id": "#key-1",
            "type": "Ed25519VerificationKey2020",
            "controller": "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK",
            "publicKeyMultibase": "z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK"
        }]
    });
    
    let record = DataRecord {
        protocol_id: identity::IDENTITY_PROTOCOL_ID.to_string(),
        schema: "IdentityCredential".to_string(),
        data: test_credential,
        timestamp: Utc::now(),
    };
    
    let record_id = data_manager.store_data(record).await.unwrap();
    assert!(!record_id.is_empty());
}

#[tokio::test]
async fn test_verifiable_claim_validation() {
    let protocol = identity::get_identity_protocol();
    
    let test_claim = serde_json::json!({
        "id": "http://example.edu/credentials/3732",
        "type": ["VerifiableCredential", "UniversityDegreeCredential"],
        "issuer": "did:web:example.edu",
        "issuanceDate": "2023-01-01T19:23:24Z",
        "credentialSubject": {
            "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
            "degree": {
                "type": "BachelorDegree",
                "name": "Bachelor of Science and Arts"
            }
        }
    });
    
    let schema = &protocol.types[1].schema;
    let validation = jsonschema::validate(schema, &test_claim);
    assert!(validation.is_ok());
}
