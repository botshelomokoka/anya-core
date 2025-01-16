# Anya Dependencies

[![Build Status](https://github.com/botshelomokoka/anya-core/workflows/CI/badge.svg)](https://github.com/botshelomokoka/anya-core/actions)
[![Security Audit](https://github.com/botshelomokoka/anya-core/workflows/Security/badge.svg)](https://github.com/botshelomokoka/anya-core/security)

Core dependencies and shared components for the Anya Bitcoin Platform ecosystem.

## Repository Structure

### Core Dependencies
- `anya-core/` - Core Bitcoin functionality and utilities
- `anya-enterprise/` - Enterprise-grade features and security components
- `dash33/` - Bitcoin Intelligence Platform integration

## Shared Components

### Bitcoin Core
- Bitcoin RPC client wrappers
- Transaction utilities
- Network interfaces
- Block processing utilities

### Security
- Encryption utilities
- Key management
- HSM interfaces
- Authentication modules

### Analytics
- Data processing utilities
- Model interfaces
- Caching mechanisms
- Performance optimizations

## Version Management

### Current Versions
- anya-core: v0.2.7
- anya-enterprise: v0.2.0
- dash33: v0.2.0

### Compatibility Matrix
| Component | Min Version | Max Version | Notes |
|-----------|-------------|-------------|-------|
| Bitcoin Core | 24.0.0 | - | Taproot support required |
| Rust | 1.70.0 | - | Async traits needed |
| PostgreSQL | 14.0 | - | JSONB support required |

## Development

### Prerequisites
```bash
# Core build tools
rustup component add rustfmt clippy

# Development dependencies
cargo install cargo-audit cargo-deny cargo-watch
```

### Building
```bash
# Build all components
cargo build --all-features

# Run tests
cargo test --all-features

# Check dependencies
cargo audit
cargo deny check
```

## CI/CD

### Workflows
- `rust_combined.yml` - Main CI pipeline for all components
- Security audit checks
- Dependency verification
- Cross-platform testing

## Documentation
- [Architecture](docs/ARCHITECTURE.md)
- [Security](docs/SECURITY.md)
- [Integration Guide](docs/INTEGRATION.md)
- [Dependency Management](docs/DEPENDENCIES.md)

## Contributing
Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License
Licensed under either:
- Apache License, Version 2.0
- MIT License
