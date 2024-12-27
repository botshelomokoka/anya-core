# Protocol Component

The Protocol component provides advanced transaction handling capabilities with support for various Bitcoin script types and operations.

## Architecture

### Repository Layer (`repository.rs`)
- CRUD operations for transactions
- Transaction validation
- Status tracking
- Metrics storage

### Service Layer (`service.rs`)
- Transaction processing
- Script operations
- Fee management
- Security integration
- Metrics collection

### Handler Layer (`handler.rs`)
- Request/response processing
- Input validation
- Error handling
- Security enforcement
- Metrics tracking

## Features

### Transaction Operations
- Creation
- Signing
- Broadcasting
- Validation
- Status tracking

### Script Support
- P2PKH (Pay to Public Key Hash)
- P2SH (Pay to Script Hash)
- P2WPKH (Pay to Witness Public Key Hash)
- P2WSH (Pay to Witness Script Hash)
- P2TR (Pay to Taproot)

### Advanced Features
- Fee estimation
- PSBT support
- Mempool monitoring
- Multi-signature operations
- Transaction batching

## Usage

```rust
// Example: Create transaction
let request = TransactionRequest {
    operation: TransactionOperationType::Create,
    protocol: ProtocolType::Bitcoin,
    transaction_details: TransactionDetails {
        inputs: vec![input],
        outputs: vec![output],
        transaction_options: options,
    },
};

let response = protocol_service.process(&context, request).await?;
```

## Configuration

```toml
[protocol]
min_fee_rate = 1
max_fee_rate = 100
mempool_expiry_hours = 72
rbf_sequence = 0xffffffff - 2
```

## Testing

```bash
# Run unit tests
cargo test --package anya-core --lib protocol

# Run integration tests
cargo test --package anya-core --test protocol_integration
```

## Metrics

The component exports the following metrics:
- `protocol_transaction_time`: Histogram of transaction processing times
- `protocol_fee_rates`: Histogram of fee rates
- `protocol_errors`: Counter of transaction errors
- `protocol_confirmations`: Histogram of confirmation times

## Health Checks

Health monitoring includes:
- Service availability
- Transaction performance
- Fee rate analysis
- Error rates
- Network status

## Security

Security measures include:
- Input validation
- Fee limits
- Access control
- Audit logging
- Error handling

## Dependencies

```toml
[dependencies]
tokio = { version = "1.34", features = ["full"] }
bitcoin = { version = "0.31.0", features = ["rand"] }
serde = { version = "1.0", features = ["derive"] }
tracing = { version = "0.1", features = ["attributes"] }
metrics = "0.21"
```
