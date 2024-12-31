use anya_core::auth::web5::{
    protocols::identity::{IdentityProtocol, security::SecurityManager},
    data_manager::Web5DataManager,
};
use did_key::Ed25519KeyPair;
use chrono::{Utc, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup
    let db = setup_database().await?;
    let protocol = IdentityProtocol::new();
    let security = SecurityManager::new(Arc::new(KeyEncryption::new(&[0u8; 32])?));

    // Create identities
    let issuer = create_identity("issuer").await?;
    let holder = create_identity("holder").await?;
    let verifier = create_identity("verifier").await?;

    // Issue an encrypted credential
    let credential = issue_encrypted_credential(&protocol, &security, &issuer, &holder).await?;

    // Verify the credential
    verify_credential(&protocol, &security, &credential, &verifier).await?;

    // Example of credential sharing
    share_credential(&protocol, &security, &credential, &holder, "recipient-did").await?;

    Ok(())
}

async fn issue_encrypted_credential(
    protocol: &IdentityProtocol,
    security: &SecurityManager,
    issuer: &Ed25519KeyPair,
    holder: &Ed25519KeyPair,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Create credential
    let credential = protocol.credentials
        .issue_credential(
            issuer,
            &holder.get_did().to_string(),
            serde_json::json!({
                "type": "AccessCredential",
                "access_level": "admin",
                "permissions": ["read", "write", "execute"]
            }),
            vec!["AccessCredential".to_string()],
            Some(Utc::now() + Duration::days(365)),
        )
        .await?;

    // Encrypt credential
    let encrypted = security.encrypt_credential(&credential)?;
    Ok(encrypted)
}

async fn verify_credential(
    protocol: &IdentityProtocol,
    security: &SecurityManager,
    encrypted: &[u8],
    verifier: &Ed25519KeyPair,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Decrypt credential
    let credential = security.decrypt_credential(encrypted)?;

    // Verify
    let result = protocol.verification
        .verify_credential(&credential, verifier)
        .await?;

    Ok(result.is_valid)
}
