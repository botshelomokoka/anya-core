# NIP-04: Encrypted Direct Messages

## Overview
NIP-04 defines the protocol for end-to-end encrypted direct messages in Nostr. Anya Core implements this using ChaCha20-Poly1305 encryption with shared secret computation.

## Implementation Details

### Message Encryption
```rust
pub struct EncryptedMessage {
    shared_secret: [u8; 32],
    content: String,
    nonce: [u8; 24],
}

impl NostrClient {
    /// Send an encrypted direct message
    pub async fn send_encrypted_message(
        &self,
        recipient_pubkey: &str,
        content: &str,
    ) -> Result<(), CoreError> {
        // Compute shared secret
        let shared_secret = self.compute_shared_secret(recipient_pubkey)?;
        
        // Encrypt message
        let encrypted = self.encrypt_content(content, &shared_secret)?;
        
        // Create and publish event
        let event = NostrEvent::new(
            4, // kind 4 = encrypted direct message
            encrypted,
            vec![vec!["p", recipient_pubkey]], // tag recipient
        );
        
        self.publish_event_to_best_relays(event).await
    }
}
```

### Shared Secret Computation
```rust
impl NostrClient {
    fn compute_shared_secret(&self, recipient_pubkey: &str) -> Result<[u8; 32], CoreError> {
        // Decode recipient's public key
        let pub_key = hex::decode(recipient_pubkey)
            .map_err(|_| CoreError::InvalidInput("Invalid public key".into()))?;
            
        // Compute shared secret using x25519
        let shared_point = self.keypair.private_key.diffie_hellman(&pub_key);
        
        Ok(shared_point.to_bytes())
    }
}
```

## Usage Examples

### Sending Encrypted Messages
```rust
// Initialize client
let client = NostrClient::new(config).await?;

// Send encrypted message
client.send_encrypted_message(
    "recipient_pubkey_hex",
    "This is a secret message",
).await?;
```

### Receiving Encrypted Messages
```rust
// Subscribe to encrypted messages
let subscription = client.subscribe(vec![
    Filter::new()
        .kinds(vec![4]) // kind 4 = encrypted DM
        .pubkey(sender_pubkey) // optional: filter by sender
]);

// Handle incoming messages
while let Some(event) = subscription.next().await {
    match client.decrypt_message(&event).await {
        Ok(content) => println!("Decrypted message: {}", content),
        Err(e) => eprintln!("Failed to decrypt: {}", e),
    }
}
```

## Security Considerations

### 1. Key Management
```rust
// GOOD: Store private key securely
let encrypted_key = encrypt_with_password(private_key, user_password)?;
secure_storage.store("nostr_key", encrypted_key)?;

// BAD: Don't store private key in plaintext
let private_key = "nsec1..."; // Never do this!
```

### 2. Message Validation
```rust
// Validate message before decryption
fn validate_encrypted_message(&self, event: &NostrEvent) -> Result<(), CoreError> {
    // Check event kind
    if event.kind != 4 {
        return Err(CoreError::InvalidEventKind);
    }
    
    // Verify signature
    if !event.verify_signature()? {
        return Err(CoreError::InvalidSignature);
    }
    
    // Check recipient tag
    if !event.has_recipient_tag(self.pubkey())? {
        return Err(CoreError::InvalidRecipient);
    }
    
    Ok(())
}
```

### 3. Nonce Management
```rust
impl NostrClient {
    fn generate_nonce() -> [u8; 24] {
        let mut nonce = [0u8; 24];
        getrandom::getrandom(&mut nonce)
            .expect("Failed to generate random nonce");
        nonce
    }
}
```

## Best Practices

1. **Key Security**
   - Store private keys securely
   - Use key rotation when needed
   - Implement key backup mechanisms

2. **Message Handling**
   - Validate all messages before processing
   - Implement proper error handling
   - Use timeouts for operations

3. **Privacy**
   - Clear message content after use
   - Implement message expiry
   - Use secure random number generation

## Common Issues and Solutions

### 1. Decryption Failures
```rust
match client.decrypt_message(&event).await {
    Ok(content) => {
        // Handle decrypted content
    }
    Err(CoreError::InvalidKey) => {
        // Handle invalid key error
        log::error!("Invalid key for message decryption");
    }
    Err(CoreError::DecryptionFailed) => {
        // Handle decryption failure
        log::error!("Message decryption failed");
    }
    Err(e) => {
        // Handle other errors
        log::error!("Unexpected error: {}", e);
    }
}
```

### 2. Key Exchange Issues
```rust
// Implement key verification
fn verify_key_exchange(&self, pubkey: &str) -> Result<(), CoreError> {
    // Send test message
    let test_content = "key_verification";
    self.send_encrypted_message(pubkey, test_content).await?;
    
    // Wait for echo
    let timeout = Duration::from_secs(5);
    tokio::time::timeout(timeout, async {
        // Wait for verification response
    }).await?;
    
    Ok(())
}
```

## Related NIPs
- [NIP-01: Basic Protocol](./nip-01.md)
- [NIP-02: Contact List](./nip-02.md)
- [NIP-05: DNS Mapping](./nip-05.md)
- [NIP-13: Proof of Work](./nip-13.md)

*Last updated: 2024-12-07*
