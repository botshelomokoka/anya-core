use anya_core::enterprise::{NostrConfig, NostrClient, NostrUserProfile};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Subscribe with existing key
    let profile = NostrUserProfile::subscribe_with_key(
        "nsec1...", // Replace with your private key
        None, // Use default relays
    ).await?;

    // Initialize Nostr client
    let config = NostrConfig {
        private_key: profile.to_nsec()?,
        relays: vec![
            "wss://relay.damus.io".to_string(),
            "wss://relay.nostr.info".to_string(),
            "wss://nostr-pub.wellorder.net".to_string(),
        ],
        default_kind: 1,
        pow_difficulty: 0,
    };

    let client = NostrClient::new(config).await?;

    // Send encrypted message
    client.send_encrypted_message(
        "recipient_pubkey",
        "Hello, this is an encrypted message!",
    ).await?;

    // Publish public note
    let event = client.create_text_note("Hello Nostr world!")?;
    client.publish_event_to_best_relays(event).await?;

    // Monitor relay health
    for relay in client.get_healthy_relays().await? {
        println!("Healthy relay: {}", relay);
        println!("Health score: {}", client.get_relay_health_score(&relay).await?);
    }

    Ok(())
}
