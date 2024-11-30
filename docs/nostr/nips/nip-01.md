# NIP-01: Basic Protocol Flow

## Overview
NIP-01 defines the basic protocol flow in Nostr, including event format, relay communication, and message types. This document explains how Anya Core implements these fundamental features.

## Event Format

### Basic Event Structure
```rust
pub struct NostrEvent {
    pub id: String,
    pub pubkey: String,
    pub created_at: i64,
    pub kind: u32,
    pub tags: Vec<NostrTag>,
    pub content: String,
    pub sig: String,
}
```

### Creating Events
```rust
// Create a text note
let event = NostrEvent::new(
    1, // kind 1 = text note
    "Hello Nostr!".to_string(),
    vec![], // no tags
);

// Sign the event
let signed_event = client.sign_event(event)?;
```

## Relay Communication

### Connecting to Relays
```rust
// Connect to multiple relays
let relays = vec![
    "wss://relay.damus.io",
    "wss://relay.nostr.info",
];

// Create client with relay list
let config = NostrConfig {
    relays: relays.iter().map(|s| s.to_string()).collect(),
    ..Default::default()
};

let client = NostrClient::new(config).await?;
```

### Publishing Events
```rust
// Publish to best relays
client.publish_event_to_best_relays(signed_event).await?;

// Or publish to specific relay
client.publish_event("wss://relay.damus.io", &signed_event).await?;
```

## Subscription

### Basic Subscription
```rust
// Subscribe to specific kinds of events
let subscription = client.subscribe(vec![
    Filter::new()
        .kinds(vec![1]) // text notes
        .since(Timestamp::now() - 3600) // last hour
        .limit(10) // max 10 events
]);

// Handle incoming events
while let Some(event) = subscription.next().await {
    println!("Received event: {:?}", event);
}
```

## Error Handling

### Relay Connection Errors
```rust
match client.connect_relay("wss://relay.example.com").await {
    Ok(_) => println!("Connected successfully"),
    Err(e) => match e {
        CoreError::ConnectionFailed => {
            // Handle connection failure
        }
        CoreError::Timeout => {
            // Handle timeout
        }
        _ => {
            // Handle other errors
        }
    }
}
```

## Best Practices

1. **Event Creation**
   - Always validate event data before creation
   - Use appropriate event kinds
   - Keep content size reasonable

2. **Relay Management**
   - Connect to multiple relays for redundancy
   - Monitor relay health
   - Implement reconnection logic

3. **Subscription**
   - Use specific filters to reduce load
   - Implement pagination where needed
   - Handle subscription errors gracefully

4. **Error Handling**
   - Implement proper error recovery
   - Log connection issues
   - Use exponential backoff for retries

## Common Issues

1. **Connection Problems**
   ```rust
   // Implement retry logic
   let max_retries = 3;
   for attempt in 0..max_retries {
       match client.connect_relay(relay_url).await {
           Ok(_) => break,
           Err(e) if attempt < max_retries - 1 => {
               tokio::time::sleep(Duration::from_secs(2_u64.pow(attempt))).await;
               continue;
           }
           Err(e) => return Err(e),
       }
   }
   ```

2. **Event Validation Failures**
   ```rust
   // Validate event before publishing
   if !event.is_valid() {
       return Err(CoreError::InvalidEvent);
   }
   ```

## Related NIPs
- [NIP-02: Contact List](./nip-02.md)
- [NIP-04: Encrypted Direct Messages](./nip-04.md)
- [NIP-15: End of Stored Events Notice](./nip-15.md)
- [NIP-20: Command Results](./nip-20.md)
