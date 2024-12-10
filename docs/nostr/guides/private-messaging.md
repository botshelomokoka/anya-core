# Private Messaging Integration Guide

This guide demonstrates how to implement secure private messaging using Anya Core's Nostr integration.

## Basic Implementation

### 1. Setup
```rust
use anya_core::enterprise::{NostrConfig, NostrClient, NostrUserProfile};

// Initialize with user's key
let profile = NostrUserProfile::subscribe_with_key(
    "nsec1...", // User's private key
    None, // Use default relays
).await?;

// Create client
let client = NostrClient::new(NostrConfig {
    private_key: profile.to_nsec()?,
    relays: vec![
        "wss://relay.damus.io".to_string(),
        "wss://relay.nostr.info".to_string(),
    ],
    ..Default::default()
}).await?;
```

### 2. Sending Messages
```rust
impl MessageHandler {
    async fn send_private_message(
        &self,
        client: &NostrClient,
        recipient: &str,
        content: &str,
    ) -> Result<(), CoreError> {
        // Send encrypted message
        client.send_encrypted_message(recipient, content).await?;
        
        // Store message locally (optional)
        self.store_message(MessageType::Sent {
            recipient: recipient.to_string(),
            content: content.to_string(),
            timestamp: chrono::Utc::now(),
        })?;
        
        Ok(())
    }
}
```

### 3. Receiving Messages
```rust
impl MessageHandler {
    async fn start_message_listener(
        &self,
        client: &NostrClient,
    ) -> Result<(), CoreError> {
        // Subscribe to encrypted messages
        let subscription = client.subscribe(vec![
            Filter::new()
                .kinds(vec![4]) // kind 4 = encrypted DM
                .since(Timestamp::now())
        ]);

        // Handle incoming messages
        while let Some(event) = subscription.next().await {
            if let Ok(content) = client.decrypt_message(&event).await {
                self.handle_new_message(
                    event.pubkey.clone(),
                    content,
                    event.created_at,
                ).await?;
            }
        }
        
        Ok(())
    }
    
    async fn handle_new_message(
        &self,
        sender: String,
        content: String,
        timestamp: i64,
    ) -> Result<(), CoreError> {
        // Store message
        self.store_message(MessageType::Received {
            sender,
            content: content.clone(),
            timestamp: timestamp.into(),
        })?;
        
        // Notify user (if configured)
        if self.config.notifications_enabled {
            self.notify_user(&content).await?;
        }
        
        Ok(())
    }
}
```

## Advanced Features

### 1. Message Threading
```rust
impl MessageThread {
    async fn create_thread(
        &self,
        client: &NostrClient,
        recipient: &str,
        thread_id: &str,
    ) -> Result<(), CoreError> {
        let content = json!({
            "type": "thread_start",
            "thread_id": thread_id,
        }).to_string();
        
        client.send_encrypted_message(recipient, &content).await
    }
    
    async fn reply_in_thread(
        &self,
        client: &NostrClient,
        recipient: &str,
        thread_id: &str,
        content: &str,
    ) -> Result<(), CoreError> {
        let threaded_content = json!({
            "type": "thread_reply",
            "thread_id": thread_id,
            "content": content,
        }).to_string();
        
        client.send_encrypted_message(recipient, &threaded_content).await
    }
}
```

### 2. Read Receipts
```rust
impl MessageHandler {
    async fn send_read_receipt(
        &self,
        client: &NostrClient,
        sender: &str,
        message_id: &str,
    ) -> Result<(), CoreError> {
        let receipt = json!({
            "type": "read_receipt",
            "message_id": message_id,
            "timestamp": chrono::Utc::now(),
        }).to_string();
        
        client.send_encrypted_message(sender, &receipt).await
    }
}
```

### 3. Message Status Tracking
```rust
#[derive(Debug, Clone)]
enum MessageStatus {
    Sent,
    Delivered,
    Read,
    Failed(String),
}

impl MessageTracker {
    async fn track_message(
        &self,
        client: &NostrClient,
        message_id: &str,
        recipient: &str,
    ) -> Result<(), CoreError> {
        let mut status = MessageStatus::Sent;
        
        // Wait for delivery confirmation
        if let Ok(confirmation) = self.wait_for_confirmation(message_id).await {
            status = MessageStatus::Delivered;
            
            // Wait for read receipt
            if let Ok(receipt) = self.wait_for_read_receipt(message_id).await {
                status = MessageStatus::Read;
            }
        }
        
        self.update_message_status(message_id, status)?;
        Ok(())
    }
}
```

## Security Best Practices

### 1. Message Validation
```rust
impl MessageValidator {
    fn validate_message(
        &self,
        event: &NostrEvent,
        expected_sender: Option<&str>,
    ) -> Result<(), CoreError> {
        // Verify signature
        if !event.verify_signature()? {
            return Err(CoreError::InvalidSignature);
        }
        
        // Check sender if specified
        if let Some(sender) = expected_sender {
            if event.pubkey != sender {
                return Err(CoreError::InvalidSender);
            }
        }
        
        // Validate content format
        self.validate_content_format(&event.content)?;
        
        Ok(())
    }
}
```

### 2. Rate Limiting
```rust
impl RateLimiter {
    async fn check_rate_limit(
        &self,
        sender: &str,
    ) -> Result<(), CoreError> {
        let key = format!("ratelimit:{}:{}", 
            sender, 
            chrono::Utc::now().date_naive()
        );
        
        let count = self.increment_counter(&key).await?;
        if count > self.max_messages_per_day {
            return Err(CoreError::RateLimitExceeded);
        }
        
        Ok(())
    }
}
```

### 3. Content Filtering
```rust
impl ContentFilter {
    fn filter_content(
        &self,
        content: &str,
    ) -> Result<String, CoreError> {
        // Remove potentially harmful content
        let filtered = content
            .replace('<', "&lt;")
            .replace('>', "&gt;");
            
        // Check for blocked patterns
        if self.contains_blocked_pattern(&filtered) {
            return Err(CoreError::BlockedContent);
        }
        
        Ok(filtered)
    }
}
```

## Error Handling

```rust
impl ErrorHandler {
    async fn handle_message_error(
        &self,
        error: CoreError,
        context: MessageContext,
    ) -> Result<(), CoreError> {
        match error {
            CoreError::ConnectionFailed => {
                // Retry with exponential backoff
                self.retry_with_backoff(context).await
            }
            CoreError::InvalidSignature => {
                // Log security warning
                log::warn!("Invalid signature from {}", context.sender);
                Err(error)
            }
            CoreError::RateLimitExceeded => {
                // Notify user
                self.notify_rate_limit(context.sender).await?;
                Err(error)
            }
            _ => {
                // Log error and notify user
                log::error!("Message error: {}", error);
                self.notify_error(error, context).await?;
                Err(error)
            }
        }
    }
}
```

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_private_messaging() {
        // Create test clients
        let alice = create_test_client("alice_key").await?;
        let bob = create_test_client("bob_key").await?;
        
        // Send test message
        let content = "Hello, Bob!";
        alice.send_encrypted_message(&bob.public_key(), content).await?;
        
        // Verify message receipt
        let received = bob.wait_for_message().await?;
        assert_eq!(received.content, content);
        assert_eq!(received.sender, alice.public_key());
    }
    
    #[tokio::test]
    async fn test_message_threading() {
        let thread = MessageThread::new();
        let thread_id = "test_thread";
        
        // Start thread
        thread.create_thread(&client, recipient, thread_id).await?;
        
        // Send replies
        thread.reply_in_thread(&client, recipient, thread_id, "Reply 1").await?;
        thread.reply_in_thread(&client, recipient, thread_id, "Reply 2").await?;
        
        // Verify thread
        let messages = thread.get_thread_messages(thread_id).await?;
        assert_eq!(messages.len(), 3);
    }
}
```

## Related Resources
- [NIP-04 Specification](../nips/nip-04.md)
- [Security Best Practices](../security/best-practices.md)
- [API Reference](../api/client.md)

*Last updated: 2024-12-07*
