# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- New MLCore structure for centralized machine learning operations
- Integrated fee management functionality in `mlfee.rs`
- New `adjust_fee` method in DAORules for dynamic fee adjustment

### Changed

- Refactored `federated_learning.rs` to use new MLCore components
- Updated `system_evaluation.rs` to work with new MLCore and FederatedLearning structures
- Modified `lib.rs` to reflect new module organization
- Updated `Cargo.toml` with necessary dependencies for new structure

### Removed

- Removed `ml_fee_manager.rs`, with functionality merged into `mlfee.rs`

## [0.2.0] - 2023-05-15

### Added

- Implemented core functionality for Bitcoin, Lightning, DLC, and Stacks integration
- Added basic ML models and federated learning capabilities
- Implemented network discovery using libp2p
- Added integration tests
- Set up CI/CD pipeline with GitHub Actions

### Changed

- Updated dependencies to latest versions
- Refactored module structure for better organization
- Improved error handling and logging in main application

## [0.1.0] - 2023-05-01

### Added (Pre-release)

- Initial project structure
- Basic user management system
- STX, DLC, Lightning, and Bitcoin support
- Kademlia-based network discovery

## [Unreleased]

### Changed
- Completed all planned features, achieving 100% progress and production readiness

## [1.0.0] - 2023-XX-XX
