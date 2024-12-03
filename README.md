# Anya Core Platform

A powerful platform combining Bitcoin/crypto functionality, ML-based analytics, and Web5 decentralized data management.

> For Enterprise features and capabilities, please see our [Enterprise Platform Documentation](./enterprise/README.md)

![Anya Architecture](docs/images/anya_architecture.png)

## Licensing

This core platform is released under the [MIT License](LICENSE.md), allowing for free use, modification, and distribution. However, please note that the [Enterprise features](./enterprise/README.md) are subject to a separate proprietary license with different terms, including revenue sharing requirements. See the [Enterprise License](./enterprise/LICENSE) for details.

## Core Features

### Hexagonal Architecture
- Clean separation of concerns with ports and adapters
- Domain-driven design principles
- Advanced error handling and telemetry
- Circuit breaker pattern implementation
- Comprehensive health monitoring
- Thread-safe caching layer

### Blockchain Integration
- Bitcoin Core & Lightning Network support
- DLC (Discreet Log Contracts)
- Taproot/Schnorr signatures
- Layer 2 solutions
- Cross-chain capabilities
- Custom chain support

### Machine Learning & AI
- Model optimization
- Federated learning
- Pipeline optimization
- Basic analytics
- Prediction models

### Web5 Integration & Storage
- Decentralized Web Nodes (DWN)
- Decentralized data storage
- Protocol-based data management
- Identity-centric storage
- Secure data encryption
- Record-based storage
- Automated data replication
- Protocol optimization
- Identity management
- Custom protocols

### Decentralized Communication
- Nostr protocol integration (NIPs 01, 02, 04, 05, 13, 15, 20)
- End-to-end encrypted messaging
- Multi-relay support with health monitoring
- Automatic relay selection and load balancing
- Simple key subscription system
- Secure key management and backup

### Monitoring & Metrics
- Distributed tracing
- Performance metrics
- Resource monitoring
- Health checks
- Basic dashboards

## Technical Stack

### Prerequisites
- Rust 1.70+
- Bitcoin Core 24.0+
- Web5 DWN Node

### Core Dependencies
```toml
[dependencies]
tokio = { version = "1.34", features = ["full"] }
bitcoin = { version = "0.31.0", features = ["rand"] }
tracing = { version = "0.1", features = ["attributes"] }
metrics = "0.21"
web5 = { version = "0.1.0", features = ["storage"] }
ml-core = { version = "0.1.0" }
```

## Quick Start

1. **Clone and Setup**
```bash
# Clone the repository
git clone https://github.com/anya/anya-core.git
cd anya-core

# Install dependencies
./scripts/setup.sh

# Build the project
cargo build --release
```

2. **Configuration**
```env
# Web5 Settings
WEB5_DWN_URL=http://localhost:3000
WEB5_STORAGE_PATH=/path/to/web5/data

# Bitcoin Settings
BITCOIN_RPC_URL=http://localhost:8332
BITCOIN_RPC_USER=user
BITCOIN_RPC_PASS=password

# ML Settings
ML_MODEL_PATH=/path/to/models
NPU_ENABLED=true

# Monitoring
METRICS_ENDPOINT=http://localhost:9090
TRACING_ENDPOINT=http://localhost:4317
```

## Documentation

### System Architecture
- [Architecture Overview](docs/ARCHITECTURE.md)
- [Hexagonal Design](docs/HEXAGONAL.md)
- [Error Handling](docs/ERROR_HANDLING.md)
- [ML System](docs/ML_SYSTEM_ARCHITECTURE.md)
- [Web5 Integration](docs/WEB5_INTEGRATION.md)

### Development
- [API Reference](docs/API.md)
- [Contributing Guide](CONTRIBUTING.md)
- [Security Guidelines](docs/SECURITY.md)
- [Testing Strategy](docs/TESTING.md)

### Deployment
- [Deployment Guide](docs/DEPLOYMENT.md)
- [Configuration Guide](docs/CONFIGURATION.md)
- [Monitoring Setup](docs/MONITORING.md)

## Configuration

The Anya platform uses a flexible configuration system that supports multiple configuration sources:

1. **Configuration Files** (`config/`)
   - `default.yaml`: Default configuration values
   - Environment-specific configs (e.g., `development.yaml`, `production.yaml`)

2. **Environment Variables**
   - All configuration can be overridden using environment variables
   - Variables are prefixed with `ANYA_`
   - Example: `ANYA_NETWORK_CAPACITY=2000`

3. **Secure Credentials**
   - Sensitive data is stored securely using encryption
   - Credentials are managed through the `CredentialManager`
   - Never commit `.env` files containing secrets

### Configuration Structure

```yaml
network:
  capacity: 1000
  node_connection_limit: 100
  performance_threshold: 0.6

dao:
  contract_name: "anya-dao"
  proposal_threshold: 100000000
  voting_period_blocks: 1008

features:
  experimental_ml: false
  advanced_optimization: false
  quantum_resistant: false
```

### Dynamic Configuration

The platform supports dynamic configuration updates:
- Network limits adjust based on system resources
- Timelock periods scale with network activity
- Performance thresholds adapt to usage patterns

### Security

- Sensitive configuration is encrypted at rest
- Credentials are stored securely using the `SecureStorage` module
- Environment-specific secrets are managed via `.env` files (not committed to VCS)

## Decentralized Governance (DAO)

### Governance Token (AGT)
- **Total Supply**: 21,000,000 AGT
- **Emission Model**: Bitcoin-inspired halving mechanism
- **Voting Mechanism**: 
  - Quadratic voting
  - Time-weighted participation
  - Expertise-based multipliers

### Governance Features
- **Proposal Framework**
  - Low barrier to entry (100 AGT proposal threshold)
  - Multi-dimensional proposal evaluation
  - ML-driven proposal scoring
  - Adaptive governance parameters

### Governance Intelligence
- **Machine Learning Enhanced**
  - Predictive proposal outcome analysis
  - Risk assessment modeling
  - Sentiment analysis integration
  - Dynamic governance optimization

### Cross-Platform Governance
- **Multi-Chain Compatibility**
  - Stacks Blockchain Integration
  - Web5 Decentralized Identity Support
  - Interoperability Protocols

### Governance Security
- **Advanced Protection Mechanisms**
  - Multi-signature proposal execution
  - Intelligent threat detection
  - Automated security audits
  - Zero-knowledge proof governance

### Compliance and Ethics
- **Governance Principles**
  - Transparent decision-making
  - Privacy-preserving technologies
  - Ethical AI governance
  - Continuous improvement mechanisms

### Technical Specifications
- **Supported Platforms**:
  - Rust (Core Implementation)
  - Dart (Mobile/Web Interfaces)
  - Web5 Decentralized Infrastructure

### Version Information
- **Current Version**: 3.1.0
- **Last Updated**: 2024-02-15
- **Compatibility**: 
  - Stacks v2.4
  - Web5 Protocol v1.0
  - Bitcoin Core Compatibility

### Governance Manifesto
*"Intelligence is our governance, decentralization is our method, and human potential is our ultimate goal."*

## Storage Architecture

Anya uses Web5's Decentralized Web Nodes (DWN) for all data storage, providing:

### Features
- **Decentralized Storage**: Data is stored across the DWN network
- **Identity-Based Access**: Data access is controlled by DIDs
- **Protocol-Driven**: Data schemas and interactions defined by protocols
- **Encrypted by Default**: All data and communications are encrypted
- **Automatic Replication**: Data is replicated across nodes
- **Flexible Querying**: Rich query capabilities for data retrieval

### Data Types
- User profiles and preferences
- Transaction records
- Analytics data
- Machine learning models
- System configurations
- Audit logs

### Benefits
- No central database dependency
- Built-in encryption and security
- Automatic data replication
- Identity-based access control
- Protocol-based data validation
- Offline-first capability

## Contributing

We welcome contributions! See our [Contributing Guide](CONTRIBUTING.md) for details.

## Project Status

- **Current Version**: 1.0.0
- **Status**: Production/Stable
- **Last Update**: 2024-01-05

## License

This project is licensed under either of:
- Apache License, Version 2.0
- MIT License

at your option.

## Links

- [Documentation](https://docs.anya.ai)
- [API Reference](https://api.anya.ai)
- [Community Forum](https://community.anya.ai)
- [Development Blog](https://blog.anya.ai)

## Acknowledgments

Special thanks to our contributors and the following projects:
- Bitcoin Core
- Lightning Network
- Web5
- TBD
- Block
