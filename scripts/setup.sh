#!/usr/bin/env bash

set -euo pipefail

# Setup script for Anya Core project

CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/anya-core"
CONFIG_FILE="$CONFIG_DIR/config"
LOG_FILE="$CONFIG_DIR/setup.log"

# Ensure config directory exists
mkdir -p "$CONFIG_DIR"

# Function to log messages
log() {
    local DATE_FORMAT="+%Y-%m-%d %H:%M:%S"
    echo "[$(date "$DATE_FORMAT")] $*" | tee -a "$LOG_FILE"
}

# Function to check if a command exists
command_exists() { command -v "$1" &> /dev/null; }

# Function to save configuration
save_config() {
    cat > "$CONFIG_FILE" <<EOF
save_config() {
    cat > "$CONFIG_FILE" << "EOF"
USER_ROLE=$USER_ROLE
ENVIRONMENT=$ENVIRONMENT
EOF
} Function to load configuration
load_config() {
    if [ -f "$CONFIG_FILE" ]; then
        # shellcheck source=/dev/null
        source "$CONFIG_FILE"
    fi
}

# Load existing configuration if available
load_config

# Determine user role if not already set
if [ -z "${USER_ROLE:-}" ]; then
    log "Select your user role:"
    select USER_ROLE in "developer" "user" "owner"; do
        PS3="Please enter your choice: "
        case $USER_ROLE in
            developer|user|owner) break ;;
            *)                    log "Invalid selection. Please try again." ;;
        esac
    done
fi

# Determine environment if not already set
if [ -z "${ENVIRONMENT:-}" ]; then
    if [ "$USER_ROLE" = "user" ]; then
        ENVIRONMENT="live"
    elif [ "$USER_ROLE" = "owner" ]; then
        ENVIRONMENT="all"
    else
        PS3="Please select the environment: "
        select ENVIRONMENT in "testnet" "live"; do
            case $ENVIRONMENT in
                testnet|live) break ;;
                *)            log "Invalid selection. Please try again." ;;
            esac
        doneesac
        done
    fi
fi

# Save configuration
save_config

log "Setting up for $USER_ROLE in $ENVIRONMENT environment"

# Install Rust if not already installed
if ! command -v rustc &> /dev/null
then
    source "$HOME/.cargo/env"-tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
fi

# Install system dependencies
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev

# Build the project
cargo build --release

# Set up environment variables
{
    echo "export ANYA_LOG_LEVEL=info"
    echo "export ANYA_NETWORK_TYPE=testnet"
# Source the updated bashrc
source "~/.bashrc"
# Source the updated bashrc
source ~/.bashrc

echo "Anya Core setup complete!"