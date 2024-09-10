#!/bin/bash

<<<<<<< HEAD
# Run all tests for Anya Core

# Set the project root directory
PROJECT_ROOT=$(git rev-parse --show-toplevel)

# Change to the project root directory
cd "$PROJECT_ROOT" || exit

# Run cargo tests
echo "Running cargo tests..."
cargo test --all

# Run integration tests
echo "Running integration tests..."
cargo test --test '*' --features integration

# Run ML logic tests
echo "Running ML logic tests..."
cargo test --package anya-core --lib ml_logic

# Run specific module tests
echo "Running specific module tests..."
cargo test --package anya-core --lib network_discovery
cargo test --package anya-core --lib user_management
cargo test --package anya-core --lib stx_support
cargo test --package anya-core --lib bitcoin_support
cargo test --package anya-core --lib lightning_support
cargo test --package anya-core --lib dlc_support
cargo test --package anya-core --lib kademlia
cargo test --package anya-core --lib setup_project
cargo test --package anya-core --lib setup_check

# Run Web5 integration tests
echo "Running Web5 integration tests..."
cargo test --package anya-core --test web5_integration

# Run DAO governance tests
echo "Running DAO governance tests..."
cargo test --package anya-core --lib dao_governance

# Run developer ecosystem tests
echo "Running developer ecosystem tests..."
cargo test --package anya-core --lib developer_ecosystem

# Run privacy enhancement tests
echo "Running privacy enhancement tests..."
cargo test --package anya-core --lib privacy_enhancements

# Run libp2p integration tests
echo "Running libp2p integration tests..."
cargo test --package anya-core --test libp2p_integration

# Run any additional custom tests
echo "Running custom tests..."
# Add any custom test commands here

echo "All tests completed."
=======
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
>>>>>>> b706d7c49205d3634e6b11d0309d8911a18a435c
