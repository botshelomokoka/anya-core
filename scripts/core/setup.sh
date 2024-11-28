#!/bin/bash

# Setup script for Anya Core

# Source common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../common/utils.sh
source "$SCRIPT_DIR/common/utils.sh"

# Initialize core-specific variables
CORE_FEATURES=(
    "bitcoin_support"
    "lightning_support"
    "dlc_support"
    "kademlia"
    "ml_logic"
    "web5_integration"
    "dao_governance"
)

setup_core() {
    log "Setting up Anya Core..."
    
    # Check system requirements
    check_system_requirements 20
    
    # Load environment
    load_env "$(get_project_root)/.env"
    
    # Initialize core configuration
    if [ ! -f "$CORE_CONFIG_FILE" ]; then
        log "Initializing core configuration..."
        
        # Prompt for user role if not in CI
        if ! is_ci; then
            select USER_ROLE in "developer" "user" "owner"; do
                case $USER_ROLE in
                    developer|user|owner) break;;
                    *) log "Invalid selection. Please try again.";;
                esac
            done
            
            # Save core configuration
            save_config "$CORE_CONFIG_FILE" \
                "USER_ROLE=$USER_ROLE" \
                "SETUP_DATE=$(date +%Y-%m-%d)"
        fi
    fi
    
    # Install core dependencies
    log "Installing core dependencies..."
    run_cargo build --workspace
    
    # Initialize core features
    log "Initializing core features..."
    for feature in "${CORE_FEATURES[@]}"; do
        log "Setting up $feature..."
        run_cargo test --package anya-core --lib "$feature" || log "Warning: $feature setup incomplete"
    done
    
    log "Anya Core setup completed successfully"
}

# Run setup if script is executed directly
if [ "${BASH_SOURCE[0]}" = "$0" ]; then
    setup_core
fi
