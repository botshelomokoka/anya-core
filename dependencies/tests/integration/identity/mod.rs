use anya_core::auth::web5::{
    protocols::identity::{
        IdentityProtocol,
        credentials::{Credential, CredentialProof},
        verification::VerificationResult,
        resolution::ResolutionResult,
    },
    data_manager::Web5DataManager,
};
use did_key::Ed25519KeyPair;
use sqlx::PgPool;
use chrono::{Utc, Duration};

mod credentials;
mod verification;
mod resolution;

async fn setup_test_db() -> PgPool {
    let db_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:test@localhost:5432/anya_test".to_string());
    
    let pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to test database");
        
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
        
    pool
}

async fn create_test_identity() -> (Ed25519KeyPair, Web5DataManager) {
    let key_pair = Ed25519KeyPair::generate();
    let data_manager = Web5DataManager::new(key_pair.clone())
        .await
        .expect("Failed to create data manager");
    
    (key_pair, data_manager)
}

#[tokio::test]
async fn test_full_identity_lifecycle() {
    let db = setup_test_db().await;
    let (issuer_key, data_manager) = create_test_identity().await;
    let (holder_key, _) = create_test_identity().await;
    
    let protocol = IdentityProtocol::new();
    protocol.initialize().await.expect("Failed to initialize protocol");
    
    // Issue credential
    let claims = serde_json::json!({
        "name": "Test User",
        "permissions": ["read", "write"]
    });
    
    let credential = protocol.credentials
        .issue_credential(
            &issuer_key,
            &holder_key.get_did().to_string(),
            claims,
            vec!["TestCredential".to_string()],
            Some(Utc::now() + Duration::days(30)),
        )
        .await
        .expect("Failed to issue credential");
    
    // Verify credential
    let verification = protocol.verification
        .verify_credential(&credential, &issuer_key)
        .await
        .expect("Failed to verify credential");
    
    assert!(verification.is_valid);
    
    // Resolve DID
    let resolution = protocol.resolution
        .resolve_did(&holder_key.get_did().to_string())
        .await
        .expect("Failed to resolve DID");
    
    assert_eq!(resolution.did, holder_key.get_did().to_string());
}
