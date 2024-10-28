# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- New MLCore structure for centralized machine learning operations
- Integrated fee management functionality in `mlfee.rs`
- New `adjust_fee` method in DAORules for dynamic fee adjustment

### Updated

- Updated `system_evaluation.rs` to work with new MLCore and FederatedLearning structures
- Updated `Cargo.toml` with necessary dependencies for new structure

### Removed

- `ml_fee_manager.rs`, with functionality merged into `mlfee.rs`

## [1.0.0] - 2024-05-15

### Updated

- v0.0.9 integration improvements.

### Added

- Implemented core functionality for Bitcoin, Lightning, DLC, and Stacks integration
- Added basic ML models and federated learning capabilities
- Implemented network discovery using libp2p
- Added integration tests
- Set up CI/CD pipeline with GitHub Actions
- Modified `lib.rs` to reflect new module organization
- Updated `Cargo.toml` with necessary dependencies for new structure

## [0.1.0] - 2024-05-01

*Initial release with foundational features and user management.*

- Initial project structure
- Basic user management system