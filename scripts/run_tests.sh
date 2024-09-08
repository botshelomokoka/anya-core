#!/bin/bash

# Run all tests for the Anya Core project

# Set up environment variables
source .env

# Run unit tests
echo "Running unit tests..."
cargo test --lib

# Run integration tests
echo "Running integration tests..."
cargo test --test integration_tests

# Run specific module tests
echo "Running user management tests..."
cargo test --test user_management_tests
echo "Running blockchain integration tests..."
cargo test --test blockchain_integration_tests
echo "Running ML logic tests..."
cargo test --test ml_logic_tests

# Run code formatting check
echo "Checking code formatting..."
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

echo "All tests and checks completed."
