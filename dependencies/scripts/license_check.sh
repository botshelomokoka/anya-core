#!/bin/bash

echo "Running license compliance checks..."

# Check licenses using cargo-deny
echo "Checking licenses with cargo-deny..."
cargo deny check licenses

# Generate license report
echo "Generating license report..."
cargo license > license_report.txt

# Check for GPL dependencies
echo "Checking for GPL dependencies..."
grep -i "GPL" license_report.txt

# Check for unknown licenses
echo "Checking for unknown licenses..."
grep -i "Unknown" license_report.txt

# Generate NOTICE file
echo "Generating NOTICE file..."
echo "Third-party licenses" > NOTICE.txt
echo "===================" >> NOTICE.txt
echo "" >> NOTICE.txt
cat license_report.txt >> NOTICE.txt

echo "License check complete. See license_report.txt and NOTICE.txt for details."
