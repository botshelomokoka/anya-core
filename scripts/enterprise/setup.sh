#!/bin/bash

# Setup script for Anya Enterprise

# Source common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../common/utils.sh
source "$SCRIPT_DIR/common/utils.sh"

# Initialize enterprise-specific variables
ENTERPRISE_FEATURES=(
    "advanced_analytics"
    "enterprise_security"
    "multi_tenant"
    "audit_logging"
    "compliance"
)

setup_enterprise() {
    log "Setting up Anya Enterprise..."
    
    # Check system requirements (enterprise needs more space)
    check_system_requirements 50
    
    # Load environment
    load_env "$(get_project_root)/enterprise/.env"
    
    # Initialize enterprise configuration
    if [ ! -f "$ENTERPRISE_CONFIG_FILE" ]; then
        log "Initializing enterprise configuration..."
        
        # Prompt for deployment type if not in CI
        if ! is_ci; then
            select DEPLOY_TYPE in "on-premise" "cloud" "hybrid"; do
                case $DEPLOY_TYPE in
                    on-premise|cloud|hybrid) break;;
                    *) log "Invalid selection. Please try again.";;
                esac
            done
            
            # Save enterprise configuration
            save_config "$ENTERPRISE_CONFIG_FILE" \
                "DEPLOY_TYPE=$DEPLOY_TYPE" \
                "SETUP_DATE=$(date +%Y-%m-%d)"
        fi
    fi
    
    # Install enterprise dependencies
    log "Installing enterprise dependencies..."
    run_cargo build --manifest-path enterprise/Cargo.toml
    
    # Initialize enterprise features
    log "Initializing enterprise features..."
    for feature in "${ENTERPRISE_FEATURES[@]}"; do
        log "Setting up $feature..."
        run_cargo test --manifest-path enterprise/Cargo.toml --lib "$feature" || log "Warning: $feature setup incomplete"
    done
    
    log "Anya Enterprise setup completed successfully"
}

# Run setup if script is executed directly
if [ "${BASH_SOURCE[0]}" = "$0" ]; then
    setup_enterprise
fi
