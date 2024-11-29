# API Reference

## Overview

Anya provides a comprehensive REST and WebSocket API for integrating Bitcoin infrastructure into enterprise applications. This reference covers all available endpoints, authentication, error handling, and best practices.

## Authentication

### API Keys
```rust
// Request with API key
GET /api/v1/transactions
Authorization: Bearer YOUR_API_KEY
```

### OAuth2
```rust
// OAuth2 token request
POST /oauth/token
Content-Type: application/x-www-form-urlencoded

grant_type=client_credentials
&client_id=YOUR_CLIENT_ID
&client_secret=YOUR_CLIENT_SECRET
```

## REST API

### Transaction Endpoints

#### Create Transaction
```rust
POST /api/v1/transactions
Content-Type: application/json

{
    "recipients": [{
        "address": "bc1q...",
        "amount": "0.1"
    }],
    "fee_rate": "5",
    "rbf": true
}
```

#### Get Transaction
```rust
GET /api/v1/transactions/{txid}
```

#### List Transactions
```rust
GET /api/v1/transactions?limit=10&offset=0
```

### Wallet Endpoints

#### Create Wallet
```rust
POST /api/v1/wallets
Content-Type: application/json

{
    "name": "main",
    "type": "segwit",
    "backup_type": "encrypted"
}
```

#### Get Wallet
```rust
GET /api/v1/wallets/{wallet_id}
```

#### List Wallets
```rust
GET /api/v1/wallets?limit=10&offset=0
```

### Contract Endpoints

#### Create Contract
```rust
POST /api/v1/contracts
Content-Type: application/json

{
    "type": "dlc",
    "oracle": "oracle_id",
    "outcomes": ["true", "false"],
    "collateral": "1.0"
}
```

#### Get Contract
```rust
GET /api/v1/contracts/{contract_id}
```

#### Execute Contract
```rust
PUT /api/v1/contracts/{contract_id}/execute
Content-Type: application/json

{
    "outcome": "true"
}
```

## WebSocket API

### Connection
```rust
// Connect to WebSocket
ws://api.anya.com/v1/ws

// Authentication message
{
    "type": "auth",
    "api_key": "YOUR_API_KEY"
}
```

### Subscriptions

#### Transaction Updates
```rust
// Subscribe
{
    "type": "subscribe",
    "channel": "transactions"
}

// Update message
{
    "type": "transaction",
    "data": {
        "txid": "...",
        "status": "confirmed",
        "block_height": 700000
    }
}
```

#### Block Updates
```rust
// Subscribe
{
    "type": "subscribe",
    "channel": "blocks"
}

// Update message
{
    "type": "block",
    "data": {
        "height": 700000,
        "hash": "...",
        "timestamp": 1631234567
    }
}
```

#### Contract Updates
```rust
// Subscribe
{
    "type": "subscribe",
    "channel": "contracts"
}

// Update message
{
    "type": "contract",
    "data": {
        "contract_id": "...",
        "status": "executed",
        "outcome": "true"
    }
}
```

## Error Handling

### Error Format
```json
{
    "error": {
        "code": "invalid_request",
        "message": "Invalid transaction parameters",
        "details": {
            "field": "amount",
            "reason": "insufficient_funds"
        }
    }
}
```

### Common Error Codes
- `invalid_request`: Invalid request parameters
- `unauthorized`: Authentication failed
- `forbidden`: Permission denied
- `not_found`: Resource not found
- `rate_limited`: Too many requests
- `internal_error`: Server error

## Rate Limiting

### Headers
```
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1631234567
```

### Limits
- REST API: 1000 requests per minute
- WebSocket: 100 messages per second
- Bulk operations: 10 requests per minute

## Pagination

### Request
```rust
GET /api/v1/transactions?limit=10&offset=0
```

### Response
```json
{
    "data": [...],
    "pagination": {
        "total": 100,
        "limit": 10,
        "offset": 0,
        "has_more": true
    }
}
```

## Versioning

### API Versions
- v1: Current stable version
- v2: Beta version (if available)
- v0: Deprecated version

### Headers
```
Accept: application/json; version=1
```

## Examples

### Creating a Transaction
```rust
use anya_sdk::{Client, TransactionBuilder};

let client = Client::new(api_key);
let tx = TransactionBuilder::new()
    .add_recipient("bc1q...", "0.1")
    .set_fee_rate(5)
    .enable_rbf()
    .build()?;

let result = client.send_transaction(tx).await?;
```

### Managing Contracts
```rust
use anya_sdk::{Client, ContractBuilder};

let client = Client::new(api_key);
let contract = ContractBuilder::new()
    .set_type(ContractType::DLC)
    .set_oracle("oracle_id")
    .add_outcomes(vec!["true", "false"])
    .set_collateral("1.0")
    .build()?;

let result = client.create_contract(contract).await?;
```

### WebSocket Subscription
```rust
use anya_sdk::{WebSocketClient, Subscription};

let ws = WebSocketClient::new(api_key);
ws.subscribe(vec![
    Subscription::Transactions,
    Subscription::Blocks,
    Subscription::Contracts,
])?;

while let Some(msg) = ws.next().await {
    match msg {
        Message::Transaction(tx) => println!("New transaction: {}", tx.txid),
        Message::Block(block) => println!("New block: {}", block.height),
        Message::Contract(contract) => println!("Contract update: {}", contract.id),
    }
}
```

## Best Practices

### 1. Error Handling
- Always check error responses
- Implement exponential backoff
- Handle rate limiting
- Log errors appropriately

### 2. Performance
- Use WebSocket for real-time updates
- Implement caching
- Batch operations when possible
- Monitor API usage

### 3. Security
- Secure API keys
- Use HTTPS
- Implement timeouts
- Validate responses

## SDK Support

### Official SDKs
- Rust: `anya-sdk`
- Python: `anya-python`
- JavaScript: `anya-js`
- Go: `anya-go`

### Installation
```bash
# Rust
cargo add anya-sdk

# Python
pip install anya-python

# JavaScript
npm install anya-js

# Go
go get github.com/anya/anya-go
```

## Support

For API support:
- API documentation
- SDK documentation
- Support channels
- Status page
