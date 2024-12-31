#!/bin/bash

# Test script for Anya Enterprise

# Source common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../common/utils.sh
source "$SCRIPT_DIR/common/utils.sh"

run_enterprise_tests() {
    log "Running Anya Enterprise tests..."
    
    # Load environment
    load_env "$(get_project_root)/enterprise/.env"
    
    # Run enterprise integration tests
    log "Running enterprise integration tests..."
    run_cargo test --manifest-path enterprise/Cargo.toml --test '*' --features integration
    
    # Run enterprise module tests
    local enterprise_modules=(
        "advanced_analytics"
        "enterprise_security"
        "multi_tenant"
        "audit_logging"
        "compliance"
        "enterprise_api"
        "enterprise_auth"
    )
    
    for module in "${enterprise_modules[@]}"; do
        run_package_tests "anya-enterprise" --lib "$module"
    done
    
    # Run enterprise-specific integration tests
    log "Running enterprise integration tests..."
    run_package_tests "anya-enterprise" --test enterprise_integration
    
    log "All enterprise tests completed successfully"
}

# Run tests if script is executed directly
if [ "${BASH_SOURCE[0]}" = "$0" ]; then
    run_enterprise_tests
fi
