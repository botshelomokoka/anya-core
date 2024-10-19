# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<<<<<<< HEAD
## [Unreleased] - 2024-05-16

*Current development stage with ongoing updates, including new MLCore structure, fee management integration, and dynamic fee adjustment methods.*
## [Unreleased] - 2024-05-16

*Current development stage with ongoing updates, including new MLCore structure, fee management integration, and dynamic fee adjustment methods.*

### Added

- New MLCore structure for centralized machine learning operations
- Integrated fee management functionality in `mlfee.rs`
- New `adjust_fee` method in DAORules for dynamic fee adjustment

### Updated `federated_learning.rs` to use new MLCore components
### Updated `federated_learning.rs` to use new MLCore components

- Updated `system_evaluation.rs` to work with new MLCore and FederatedLearning structures
- Modified `lib.rs` to reflect new module organization
- Updated `Cargo.toml` with necessary dependencies for new structure

### Removed `ml_fee_manager.rs`, with functionality merged into `mlfee.rs`
### Removed `ml_fee_manager.rs`, with functionality merged into `mlfee.rs`

## [1.0.0] - 2024-05-18

*Major release with core functionality and integration improvements.*
*Major release with core functionality and integration improvements.*

### Added (v0.0.9)
### Added (v0.0.9)

- Implemented core functionality for Bitcoin, Lightning, DLC, and Stacks integration
- Added basic ML models and federated learning capabilities
- Implemented network discovery using libp2p
- Added integration tests
- Set up CI/CD pipeline with GitHub Actions

### Updated `system_evaluation.rs` to work with new MLCore and FederatedLearning structures

- Modified `lib.rs` to reflect new module organization
- Updated `Cargo.toml` with necessary dependencies for new structure

### Removed

- Removed `ml_fee_manager.rs`, with functionality merged into `mlfee.rs`

## [0.1.0] - 2024-05-01

*Initial release with foundational features and user management.*
### Updated `system_evaluation.rs` to work with new MLCore and FederatedLearning structures

- Modified `lib.rs` to reflect new module organization
- Updated `Cargo.toml` with necessary dependencies for new structure

### Removed

- Removed `ml_fee_manager.rs`, with functionality merged into `mlfee.rs`

## [0.1.0] - 2024-05-01

*Initial release with foundational features and user management.*

### Added (Pre-release)

- Initial project structure
- Basic user management system
- STX, DLC, Lightning, and Bitcoin support
- Kademlia-based network discovery

### Changed (v0.1.0)

- Updated `system_evaluation.rs` to work with new MLCore and FederatedLearning structures
- Modified `lib.rs` to reflect new module organization
- Updated `Cargo.toml` with necessary dependencies for new structure

### Removed `ml_fee_manager.rs` Functionality

- Removed `ml_fee_manager.rs`, with functionality merged into `mlfee.rs`

---

**Note:** The `ml_logic` component is responsible for corrective dating checks and updates across the project.
=======
## [Unreleased]

### Added (7)

- Consolidated development plan into ROADMAP.md
- Detailed breakdown of Phase 2 tasks for both Anya Core and Anya Enterprise
- Next steps and ongoing tasks in the roadmap
- Implemented decentralized identifiers (DIDs) and Verifiable Credentials
- Added support for WebAssembly in smart contracts module
- Integrated InterBlockchain Communication (IBC) protocol
- Implemented zero-knowledge proofs using bulletproofs library

### Changed (6)  // Incremented the count

- Updated project structure to remove DEVPLAN.md
- Expanded ROADMAP.md to include more comprehensive development information
- Updated dependencies to latest versions
- Refactored project structure to align with the new rewrite plan
- Aligned date formats across all documentation files (README.md, CHANGELOG.md, ROADMAP.md)

### Removed (1)

- DEVPLAN.md (content merged into ROADMAP.md)

## [1.0.0] - 2023-05-18

### Added (9)

- Core Architecture: Modular, plugin-based with Rust-based Hexagonal Architecture pattern
- Networking: libp2p for peer-to-peer communications, Kademlia DHT for peer discovery and routing
- Blockchain Integration: Bitcoin Core RPC interface, Lightning Network with LND gRPC API, Stacks blockchain support, DLC support using latest Rust DLC library
- Machine Learning: Federated Learning with self-research capabilities, Internal AI engine with model aggregation and optimization
- Identity and Authentication: DIDs using W3C DID specification, Verifiable Credentials
- Smart Contracts: Clarity support, WebAssembly integration for execution
- Interoperability: IBC protocol for cross-chain interactions
- Privacy and Security: Zero-knowledge proofs using bulletproofs
- User Interface: Basic CLI implementation

## [0.2.0] - 2023-05-20

### Added (3)

- Data Storage: IPFS integration for decentralized storage, OrbitDB support for peer-to-peer databases
- Advanced Cryptography: Homomorphic encryption module, Secure multi-party computation module
- AI Enhancements: Natural language processing capabilities, Improved federated learning with OpenFL

### Changed (2)

- Updated all dependencies to their latest versions
- Refactored project structure to support new modules

## [0.1.0] - 2023-05-01

### Added (4) (Pre-release)

- Initial project structure
- Basic user management system
- Blockchain Support: STX, DLC, Lightning, and Bitcoin
- Networking: Kademlia-based network discovery
>>>>>>> 73719fd69dc5deae81358f465a7c0b572919e2d3
