#!/bin/bash
set -euo pipefail

# Main build script for Anya

# Source common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=common/utils.sh
source "$SCRIPT_DIR/common/utils.sh"

# Build targets
readonly TARGETS=(
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu"
)

check_dependencies() {
    local deps=("cargo" "rustc" "rustup")
    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            log_error "Required tool not found: $dep"
            exit 1
        fi
    done
}

setup_toolchain() {
    for target in "${TARGETS[@]}"; do
        if ! rustup target list | grep -q "installed" | grep -q "$target"; then
            log_info "Adding target $target..."
            rustup target add "$target"
        fi
    done
}

build_component() {
    local component=$1
    local manifest_path=$2
    local target=$3
    
    log_info "Building $component for $target..."
    if ! run_cargo build --manifest-path "$manifest_path" --target "$target" --release; then
        log_error "Failed to build $component for $target"
        return 1
    fi
}

cleanup() {
    log_info "Cleaning up build artifacts..."
    run_cargo clean
}

build_all() {
    log_info "Building Anya..."
    
    # Check dependencies
    check_dependencies
    
    # Setup toolchain
    setup_toolchain
    
    # Load environment
    load_env "$(get_project_root)/.env"
    
    # Trap cleanup
    trap cleanup EXIT
    
    # Build core
    log_info "Building Anya Core..."
    for target in "${TARGETS[@]}"; do
        build_component "core" "Cargo.toml" "$target"
    done
    
    # Build enterprise if it exists
    if [[ -d "$(get_project_root)/enterprise" ]]; then
        log_info "Building Anya Enterprise..."
        for target in "${TARGETS[@]}"; do
            build_component "enterprise" "enterprise/Cargo.toml" "$target"
        done
    fi
    
    log_info "Build completed successfully"
}

# Run build if script is executed directly
if [[ "${BASH_SOURCE[0]}" = "$0" ]]; then
    build_all
fi
