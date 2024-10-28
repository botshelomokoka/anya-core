use super::*;
use anya_core::auth::web5::protocols::identity::credentials::Credential;

#[tokio::test]
async fn test_credential_revocation() {
    let db = setup_test_db().await;
    let (issuer_key, _) = create_test_identity().await;
    let (holder_key, _) = create_test_identity().await;
    
    let protocol = IdentityProtocol::new();
    
    // Issue credential
    let credential = protocol.credentials
        .issue_credential(
            &issuer_key,
            &holder_key.get_did().to_string(),
            serde_json::json!({"access": "admin"}),
            vec!["AccessCredential".to_string()],
            None,
        )
        .await
        .expect("Failed to issue credential");
        
    // Verify before revocation
    let verification = protocol.verification
        .verify_credential(&credential, &issuer_key)
        .await
        .expect("Failed to verify credential");
    assert!(verification.is_valid);
    
    // Revoke credential
    protocol.credentials
        .revoke_credential(&credential.id, &issuer_key)
        .await
        .expect("Failed to revoke credential");
        
    // Verify after revocation
    let verification = protocol.verification
        .verify_credential(&credential, &issuer_key)
        .await
        .expect("Failed to verify credential");
    assert!(!verification.is_valid);
}

#[tokio::test]
async fn test_credential_update() {
    let db = setup_test_db().await;
    let (issuer_key, _) = create_test_identity().await;
    let (holder_key, _) = create_test_identity().await;
    
    let protocol = IdentityProtocol::new();
    
    // Issue initial credential
    let mut credential = protocol.credentials
        .issue_credential(
            &issuer_key,
            &holder_key.get_did().to_string(),
            serde_json::json!({"level": 1}),
            vec!["LevelCredential".to_string()],
            None,
        )
        .await
        .expect("Failed to issue credential");
        
    // Update credential
    credential = protocol.credentials
        .update_credential(
            &credential.id,
            &issuer_key,
            serde_json::json!({"level": 2}),
        )
        .await
        .expect("Failed to update credential");
        
    assert_eq!(
        credential.claims.get("level").unwrap().as_i64().unwrap(),
        2
    );
}
