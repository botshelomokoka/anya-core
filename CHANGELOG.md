# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

Ongoing updates include:

- New MLCore structure
- Fee management integration
- Dynamic fee adjustment methods

### Added

- Added new MLCore structure for centralized machine learning operations
- Integrated fee management functionality in `mlfee.rs`
- Added new `adjust_fee` method in DAORules for dynamic fee adjustment
- Initial release with foundational project structure and basic user management system

### Changed

- Refactored `system_evaluation.rs` to integrate with the new MLCore structure, including changes to the evaluation logic and methods
- Updated `Cargo.toml` with necessary dependencies for the new structure

### Removed

- Removed `ml_fee_manager.rs`, with functionality merged into `mlfee.rs`

## [1.0.0] - 2024-05-15

### Added

- Implemented core functionality for Bitcoin, Lightning, DLC, and Stacks integration
- Added basic ML models and federated learning capabilities
- Implemented network discovery using libp2p
- Added integration tests
- Set up CI/CD pipeline with GitHub Actions

### Changed

- Modified `lib.rs` to reflect new module organization
- Updated `Cargo.toml` with necessary dependencies for new structure

## [0.1.0] - 2024-05-01

*Initial release with foundational features and user management.*

- Initial project structure
- Basic user management system
