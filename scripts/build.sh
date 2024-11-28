#!/bin/bash

# Main build script for Anya

# Source common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=common/utils.sh
source "$SCRIPT_DIR/common/utils.sh"

# Build targets
TARGETS=(
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu"
)

build_all() {
    log "Building Anya..."
    
    # Load environment
    load_env "$(get_project_root)/.env"
    
    # Build core
    log "Building Anya Core..."
    for target in "${TARGETS[@]}"; do
        log "Building for $target..."
        run_cargo build --target "$target" --release
    done
    
    # Build enterprise if it exists
    if [ -d "$(get_project_root)/enterprise" ]; then
        log "Building Anya Enterprise..."
        for target in "${TARGETS[@]}"; do
            log "Building enterprise for $target..."
            run_cargo build --manifest-path enterprise/Cargo.toml --target "$target" --release
        done
    fi
    
    log "Build completed successfully"
}

# Run build if script is executed directly
if [ "${BASH_SOURCE[0]}" = "$0" ]; then
    build_all
fi
