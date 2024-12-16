#!/bin/bash

# Common utility functions for Anya scripts

set -euo pipefail

# Configuration
CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/anya"
CORE_CONFIG_FILE="$CONFIG_DIR/core.conf"
ENTERPRISE_CONFIG_FILE="$CONFIG_DIR/enterprise.conf"
LOG_FILE="$CONFIG_DIR/setup.log"

# Ensure config directory exists
mkdir -p "$CONFIG_DIR"

# Logging function
log() {
    local DATE_FORMAT="+%Y-%m-%d %H:%M:%S"
    echo "[$(date "$DATE_FORMAT")] $*" | tee -a "$LOG_FILE"
}

# Error handling
error() {
    log "ERROR: $*" >&2
    exit 1
}

# Check if a command exists
command_exists() { 
    command -v "$1" &> /dev/null 
}

# Get project root directory
get_project_root() {
    git rev-parse --show-toplevel || error "Not in a git repository"
}

# Load environment variables
load_env() {
    local env_file="$1"
    if [ -f "$env_file" ]; then
        # shellcheck source=/dev/null
        source "$env_file"
    else
        log "Warning: $env_file not found"
    fi
}

# Save configuration
save_config() {
    local config_file="$1"
    shift
    printf "%s\n" "$@" > "$config_file"
}

# Load configuration
load_config() {
    local config_file="$1"
    if [ -f "$config_file" ]; then
        # shellcheck source=/dev/null
        source "$config_file"
    fi
}

# Check system requirements
check_system_requirements() {
    local min_space_gb="$1"
    local available_space
    available_space=$(df -BG . | awk 'NR==2 {print $4+0}')
    
    if [ "$available_space" -lt "$min_space_gb" ]; then
        error "Insufficient disk space. Required: ${min_space_gb}GB, Available: ${available_space}GB"
    fi
    
    # Check for required commands
    local required_commands=("git" "cargo" "rustc" "npm" "node")
    for cmd in "${required_commands[@]}"; do
        command_exists "$cmd" || error "$cmd is required but not installed"
    done
}

# Run cargo command with proper error handling
run_cargo() {
    local cmd="$1"
    shift
    cargo "$cmd" "$@" || error "Cargo $cmd failed"
}

# Run tests for a specific package
run_package_tests() {
    local package="$1"
    shift
    log "Running tests for $package..."
    run_cargo test --package "$package" "$@"
}

# Check if running in CI environment
is_ci() {
    [ -n "${CI:-}" ]
}

# Get the current platform
get_platform() {
    case "$(uname -s)" in
        Linux*)     echo "linux";;
        Darwin*)    echo "macos";;
        MINGW*)     echo "windows";;
        *)          echo "unknown";;
    esac
}
