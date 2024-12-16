# Environment Variables Documentation

All environment variables in the Anya platform are prefixed with `ANYA_` to avoid conflicts with other applications.

## Required Variables

### Bitcoin Configuration
- `ANYA_BITCOIN_RPC_URL`: Bitcoin Core RPC URL (e.g., `http://localhost:8332`)
- `ANYA_BITCOIN_RPC_USER`: Bitcoin Core RPC username
- `ANYA_BITCOIN_RPC_PASS`: Bitcoin Core RPC password

### Web5 Configuration
- `ANYA_WEB5_DWN_URL`: Web5 DWN endpoint URL
- `ANYA_WEB5_STORAGE_PATH`: Path to Web5 storage directory
- `ANYA_WEB5_DID`: Web5 Decentralized Identifier

### Security Configuration
- `ANYA_ENCRYPTION_KEY`: Master encryption key for secure storage
- `ANYA_JWT_SECRET`: Secret for JWT token generation
- `ANYA_API_KEY`: API key for external service authentication

## Optional Variables

### Feature Flags
- `ANYA_FEATURES_EXPERIMENTAL_ML`: Enable experimental ML features (default: false)
- `ANYA_FEATURES_ADVANCED_OPTIMIZATION`: Enable advanced optimization (default: false)
- `ANYA_FEATURES_QUANTUM_RESISTANT`: Enable quantum-resistant algorithms (default: false)
- `ANYA_FEATURES_ENHANCED_SECURITY`: Enable enhanced security features (default: true)

### Network Configuration
- `ANYA_NETWORK_CAPACITY`: Maximum network capacity (default: 1000)
- `ANYA_NETWORK_NODE_CONNECTION_LIMIT`: Maximum node connections (default: 100)
- `ANYA_NETWORK_PERFORMANCE_THRESHOLD`: Performance threshold (default: 0.6)

### NPU Configuration
- `ANYA_NPU_CAPACITY_GB`: NPU memory capacity in GB (default: 4.5)
- `ANYA_NPU_PIPELINE_DEPTH`: NPU pipeline depth (default: 24)

### Metrics Configuration
- `ANYA_METRICS_COLLECTION_INTERVAL_MS`: Metrics collection interval (default: 5000)

## Security Notes

1. Never commit `.env` files containing real credentials
2. Use secure credential storage for production environments
3. Rotate secrets regularly
4. Use strong, unique values for all security-related variables

## Dynamic Configuration

Some configuration values can be dynamically adjusted based on system resources and network activity:

1. Network limits scale with available system resources
2. Timelock periods adjust based on network activity
3. Performance thresholds adapt to usage patterns

## Environment-Specific Configuration

Different environments (development, staging, production) should use different configuration values:

### Development
```env
ANYA_BITCOIN_RPC_URL=http://localhost:8332
ANYA_WEB5_DWN_URL=http://localhost:3000
ANYA_FEATURES_EXPERIMENTAL_ML=true
```

### Production
```env
ANYA_FEATURES_EXPERIMENTAL_ML=false
ANYA_FEATURES_ENHANCED_SECURITY=true
ANYA_NETWORK_CAPACITY=5000
```

## Validation

The platform includes built-in validation for all configuration values. See `src/config/validator.rs` for validation rules.

*Last updated: 2024-12-07*
