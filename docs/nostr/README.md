# Nostr Integration Guide

## Overview
Anya Core provides comprehensive support for the Nostr protocol, enabling decentralized communication with end-to-end encryption, multi-relay support, and advanced key management features.

## Key Features

### 1. Decentralized Communication
- End-to-end encrypted messaging (NIP-04)
- Multi-relay support with health monitoring
- Automatic relay selection and load balancing
- Simple key subscription system
- Secure key management and backup

### 2. NIP Compliance
- NIP-01: Basic protocol flow
- NIP-02: Contact list and petnames
- NIP-04: Encrypted direct messages
- NIP-05: Mapping Nostr keys to DNS identifiers
- NIP-13: Proof of Work
- NIP-15: End of Stored Events Notice
- NIP-20: Command Results

### 3. Key Management
```rust
// Subscribe with existing nsec key
let profile = NostrUserProfile::subscribe_with_key(
    "nsec1...", // Your private key
    None // Use default relays
).await?;

// Export key to nsec format
let nsec = profile.to_nsec()?;
```

### 4. Relay Management
- Automatic relay selection based on health metrics
- Load balancing across multiple relays
- Connection pooling for improved performance
- Retry mechanisms with exponential backoff
- Health monitoring and metrics

### 5. Security Features
- ChaCha20-Poly1305 encryption for messages
- Secure key storage and backup
- Shared secret computation
- Privacy controls
- Encrypted notifications

## Getting Started

### Installation
Add to your `Cargo.toml`:
```toml
[dependencies]
anya-core = "1.3.0"
```

### Basic Usage
```rust
// Initialize Nostr client
let config = NostrConfig {
    private_key: "your_key",
    relays: vec!["wss://relay.damus.io"],
    ..Default::default()
};

let client = NostrClient::new(config).await?;

// Send encrypted message
client.send_encrypted_message(
    "recipient_pubkey",
    "Secret message"
).await?;
```

### Relay Configuration
```rust
// Custom relay setup
let relays = vec![
    "wss://relay.damus.io",
    "wss://relay.nostr.info",
    "wss://nostr-pub.wellorder.net"
];

let profile = NostrUserProfile::subscribe_with_key(
    "nsec1...",
    Some(relays)
).await?;
```

## Best Practices

### Key Management
1. Always backup your keys securely
2. Use environment variables for sensitive data
3. Implement key rotation policies
4. Enable secure key recovery mechanisms

### Relay Selection
1. Use multiple relays for redundancy
2. Monitor relay health metrics
3. Implement fallback mechanisms
4. Configure geographic distribution

### Security
1. Always use encryption for private messages
2. Validate all incoming messages
3. Implement rate limiting
4. Monitor for suspicious activity

## Future Enhancements
- Additional NIP implementations
- Advanced relay features
- Enhanced key management
- Group messaging support
- Social features
- File sharing capabilities
- Voice/video support

## Contributing
We welcome contributions! Please see our [Contributing Guide](../../CONTRIBUTING.md) for details.

## License
This integration is part of Anya Core and is released under the MIT License. See [LICENSE.md](../../LICENSE.md) for details.
