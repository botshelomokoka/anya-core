# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
