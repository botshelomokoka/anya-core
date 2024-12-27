# Security Component

The Security component provides comprehensive security operations with a focus on authentication, authorization, encryption, and audit logging.

## Architecture

### Repository Layer (`repository.rs`)
- CRUD operations for security audits
- Audit trail management
- Event storage and querying
- Compliance tracking

### Service Layer (`service.rs`)
- Security operations processing
- Crypto operations
- Policy enforcement
- Audit logging
- Metrics collection

### Handler Layer (`handler.rs`)
- Request/response processing
- Input validation
- Error handling
- Security enforcement
- Metrics tracking

## Features

### Authentication & Authorization
- Multi-factor authentication
- Role-based access control
- Permission management
- Session handling
- Token management

### Cryptographic Operations
- Encryption/decryption
- Digital signatures
- Key management
- Hash functions
- Random number generation

### Audit & Compliance
- Comprehensive audit logs
- Event correlation
- Compliance reporting
- Risk assessment
- Policy enforcement

## Usage

```rust
// Example: Process security operation
let request = SecurityRequest {
    action: SecurityAction::Authenticate,
    resource_id: "resource-123",
    metadata: Some(SecurityRequestMetadata {
        user_id: Some("user-123"),
        ip_address: Some("192.168.1.1"),
        user_agent: Some("Mozilla/5.0"),
        additional_context: None,
    }),
};

let response = security_service.process(&context, request).await?;
```

## Configuration

```toml
[security]
token_expiry_seconds = 3600
max_failed_attempts = 5
lockout_duration_minutes = 30
audit_retention_days = 90
```

## Testing

```bash
# Run unit tests
cargo test --package anya-core --lib security

# Run integration tests
cargo test --package anya-core --test security_integration
```

## Metrics

The component exports the following metrics:
- `security_operation_time`: Histogram of operation execution times
- `security_audit_count`: Counter of audit events
- `security_errors`: Counter of security errors
- `security_auth_failures`: Counter of authentication failures

## Health Checks

Health monitoring includes:
- Service availability
- Operation performance
- Resource utilization
- Error rates
- Policy compliance

## Security Measures

Security implementation includes:
- Input sanitization
- Rate limiting
- Access control
- Audit logging
- Error handling
- Threat detection

## Dependencies

```toml
[dependencies]
tokio = { version = "1.34", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
tracing = { version = "0.1", features = ["attributes"] }
metrics = "0.21"
ring = "0.17"
```
