#!/bin/bash

# Run all tests for the Anya Core project

# Set up environment variables
source .env

# Run unit tests
echo "Running unit tests..."
cargo test --lib

# Run integration tests
echo "Running integration tests..."
cargo test --test '*'

# Run specific module tests
echo "Running user management tests..."
cargo test --test user_management_tests
echo "Running blockchain integration tests..."
cargo test --test blockchain_integration_tests
echo "Running ML logic tests..."
cargo test --test ml_logic_tests

# Run new test categories
echo "Running blockchain interoperability tests..."
cargo test --test blockchain_interoperability
echo "Running privacy and security tests..."
cargo test --test privacy_and_security
echo "Running smart contracts tests..."
cargo test --test smart_contracts
echo "Running user interface tests..."
cargo test --test user_interface

# Run code formatting check
echo "Running code formatting check..."
cargo fmt -- --check

# Run linter
echo "Running linter..."
cargo clippy -- -D warnings

# Run security audit
echo "Running security audit..."
cargo audit

# Check for outdated dependencies
echo "Checking for outdated dependencies..."
cargo outdated

# Run code coverage
echo "Running code coverage..."
cargo tarpaulin --ignore-tests

# Run benchmarks
echo "Running benchmarks..."
cargo bench

# New module tests
echo "Running identity tests..."
cargo test --test identity_tests
echo "Running data storage tests..."
cargo test --test data_storage_tests
echo "Running smart contracts tests..."
cargo test --test smart_contracts_tests
echo "Running interoperability tests..."
cargo test --test interoperability_tests
echo "Running privacy tests..."
cargo test --test privacy_tests
echo "Running UI tests..."
cargo test --test ui_tests

echo "All tests completed successfully!"
