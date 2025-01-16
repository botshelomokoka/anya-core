#!/bin/bash

echo "Running integration tests..."

# Run workspace tests
echo "Running workspace tests..."
cargo test --all-features --workspace

# Run specific integration tests
echo "Running integration tests..."
cargo test --test '*' --features integration-test

# Run cross-component tests
echo "Running cross-component tests..."
for crate in anya-core anya-enterprise dash33; do
    echo "Testing $crate..."
    (cd $crate && cargo test --all-features)
done

# Run benchmarks
echo "Running benchmarks..."
cargo bench --all-features

# Run security checks
echo "Running security checks..."
./scripts/security_check.sh

# Run license checks
echo "Running license checks..."
./scripts/license_check.sh

# Generate test coverage report
echo "Generating test coverage report..."
cargo tarpaulin --all-features --workspace --out Html

echo "Integration tests complete. See coverage/tarpaulin-report.html for details."
