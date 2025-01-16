#!/bin/bash

echo "Running security checks..."

# Update security advisories database
cargo audit update

# Run security audit
echo "Running cargo audit..."
cargo audit

# Run dependency check
echo "Running cargo deny check..."
cargo deny check

# Check for unused dependencies
echo "Checking for unused dependencies..."
cargo udeps

# Check for outdated dependencies
echo "Checking for outdated dependencies..."
cargo outdated

# Generate dependency report
echo "Generating dependency report..."
cargo tree > dependency_report.txt

echo "Security check complete. See dependency_report.txt for details."
