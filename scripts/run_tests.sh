#!/bin/bash

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
