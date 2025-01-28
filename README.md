# Anya Core Library

Core library for the Anya AI IDE system, providing fundamental functionality for machine learning, Web5 integration, and Bitcoin operations.

## Features

- Advanced ML capabilities with PyTorch integration
- Web5 protocol implementation for decentralized data management
- Bitcoin and Lightning Network support
- Comprehensive security and privacy features
- Memory-safe and thread-safe implementation
- Zero-cost abstractions
- Formal verification support
- Bitcoin Core coding standards compliance

## Architecture

The library is organized into several main modules:

- `ml`: Machine learning components and AI agent system
- `web5`: Web5 protocol integration and decentralized identity
- `bitcoin`: Bitcoin and Lightning Network functionality
- `utils`: Common utilities and helper functions

## Getting Started

Add this to your `Cargo.toml`:

```toml
[dependencies]
anya-core = { git = "https://github.com/botshelomokoka/anya-core.git" }
```

## Example Usage

```rust
use anya_core::{ml, web5, bitcoin};

async fn example() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize ML system
    let ml_system = ml::MLSystem::new()?;

    // Set up Web5 DID
    let did = web5::identity::DID::new()?;

    // Create Bitcoin wallet
    let wallet = bitcoin::wallet::HDWallet::new()?;

    Ok(())
}
```

## Development

### Prerequisites

- Rust 1.70.0 or later
- CMake (for PyTorch)
- CUDA toolkit (optional, for GPU support)

### Building

```bash
cargo build --all-features
```

### Testing

```bash
cargo test --all-features
```

### Documentation

```bash
cargo doc --no-deps --document-private-items
```

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

### Code Style

- Follow Rust standard formatting (rustfmt)
- Use clippy for linting
- Follow Bitcoin Core coding standards
- Maintain comprehensive documentation
- Include unit tests for all new features

## Security

- OWASP compliance
- Regular security audits
- Formal verification
- Memory safety guarantees
- Thread safety guarantees
- Supply chain attack prevention

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Bitcoin Core team for their coding standards
- PyTorch team for their ML framework
- Web5 community for protocol specifications
- Rust community for their excellent tools and libraries
