#!/bin/bash

# Test script for Anya Core

# Source common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../common/utils.sh
source "$SCRIPT_DIR/common/utils.sh"

run_core_tests() {
    log "Running Anya Core tests..."
    
    # Load environment
    load_env "$(get_project_root)/.env"
    
    # Run integration tests
    log "Running integration tests..."
    run_cargo test --test '*' --features integration
    
    # Run core module tests
    local core_modules=(
        "ml_logic"
        "network_discovery"
        "user_management"
        "stx_support"
        "bitcoin_support"
        "lightning_support"
        "dlc_support"
        "kademlia"
        "setup_project"
        "setup_check"
        "dao_governance"
    )
    
    for module in "${core_modules[@]}"; do
        run_package_tests "anya-core" --lib "$module"
    done
    
    # Run Web5 integration tests
    log "Running Web5 integration tests..."
    run_package_tests "anya-core" --test web5_integration
    
    log "All core tests completed successfully"
}

# Run tests if script is executed directly
if [ "${BASH_SOURCE[0]}" = "$0" ]; then
    run_core_tests
fi
