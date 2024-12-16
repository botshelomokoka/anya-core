#!/bin/bash
set -euo pipefail

# Source common utilities
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=./lib/common.sh
source "$SCRIPT_DIR/lib/common.sh"

# Project configuration
readonly INSTANCE_ID="9111727350091981557"
readonly PROJECT_ID="anya-433919"
readonly USERNAME="botshelomokoka@gmail.com"
readonly MIN_DISK_SPACE=50  # GB
readonly REQUIRED_PACKAGES=(
    "build-essential"
    "curl"
    "libssl-dev"
    "pkg-config"
)

# Function to check disk space
check_disk_space() {
    local drive=$1
    df -B1G "$drive" | awk 'NR==2 {print $4+0}'
}

# Function to find a drive with more than MIN_DISK_SPACE available
find_suitable_drive() {
    local drives
    drives=$(lsblk -ndo NAME,TYPE | awk '$2=="disk" {print "/dev/"$1}')
    local drive space
    
    for drive in $drives; do
        space=$(check_disk_space "$drive")
        if (( space > MIN_DISK_SPACE )); then
            echo "$drive"
            return 0
        fi
    done
    return 1
}

# Function to move data to a new drive
move_data() {
    local old_drive=$1
    local new_drive=$2
    log_info "Moving data from $old_drive to $new_drive"
    
    # Mount new drive
    sudo mkdir -p /mnt/newdrive
    sudo mount "$new_drive" /mnt/newdrive
    
    # Backup fstab
    sudo cp /etc/fstab /etc/fstab.bak
    
    # Copy data with progress
    sudo rsync -avz --progress --exclude='/mnt/newdrive' / /mnt/newdrive/
    
    # Update fstab
    sudo sed -i "s|$old_drive|$new_drive|g" /etc/fstab
    
    log_info "Data moved successfully. Please reboot to apply changes."
    exit 0
}

# Function to fetch metadata
get_metadata() {
    local key=$1
    local value
    
    value=$(curl -sf "http://metadata.google.internal/computeMetadata/v1/instance/$key" \
        -H "Metadata-Flavor: Google")
    
    if [[ $? -ne 0 ]]; then
        log_error "Unable to retrieve $key from metadata server"
        return 1
    fi
    
    echo "$value"
}

# Function to install Rust
install_rust() {
    log_info "Installing Rust..."
    if ! command -v rustc &> /dev/null; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        # shellcheck source=/dev/null
        source "$HOME/.cargo/env"
    else
        log_info "Rust is already installed"
    fi
}

# Function to setup Web5
setup_web5() {
    log_info "Setting up Web5..."
    
    # Install CLI
    cargo install web5-cli
    
    # Initialize database
    web5 db init
    web5 db migrate
    
    # Setup protocols
    mkdir -p .web5/protocols
    cat > .web5/protocols/anya.json << 'EOL'
{
  "protocol": "https://anya.ai/protocol",
  "published": true,
  "types": {
    "proposal": {
      "schema": "https://anya.ai/schemas/proposal",
      "dataFormats": ["application/json"]
    },
    "vote": {
      "schema": "https://anya.ai/schemas/vote",
      "dataFormats": ["application/json"]
    },
    "configuration": {
      "schema": "https://anya.ai/schemas/config",
      "dataFormats": ["application/json"]
    }
  }
}
EOL
}

# Function to setup environment
setup_environment() {
    log_info "Setting up environment..."
    
    # Copy example env
    cp .env.example .env
    
    # Update env variables
    {
        echo "PROJECT_ID=$PROJECT_ID"
        echo "INSTANCE_NAME=$INSTANCE_NAME"
        echo "ZONE=$ZONE"
        echo "INSTANCE_ID=$INSTANCE_ID"
        echo "USERNAME=$USERNAME"
        echo "WEB5_DID_METHOD=key"
        echo "WEB5_CREDENTIAL_STATUS_TYPE=RevocationList2020"
        echo "WEB5_STORAGE_PATH=.web5/data"
        echo "WEB5_PROTOCOL_URL=http://localhost:3000"
    } >> .env
}

cleanup() {
    log_info "Cleaning up..."
    if [[ -d .web5/data/tmp ]]; then
        rm -rf .web5/data/tmp
    fi
}

main() {
    trap cleanup EXIT
    
    log_info "Project setup initiated"
    log_info "Username: $USERNAME"
    
    # Get instance metadata
    INSTANCE_NAME=$(get_metadata "name")
    ZONE=$(get_metadata "zone" | awk -F/ '{print $NF}')
    
    log_info "Instance: $INSTANCE_NAME"
    log_info "Zone: $ZONE"
    
    # Check disk space
    CURRENT_DRIVE=$(df / | awk 'NR==2 {print $1}')
    AVAILABLE_SPACE=$(check_disk_space "$CURRENT_DRIVE")
    
    log_info "Current drive: $CURRENT_DRIVE"
    log_info "Available space: ${AVAILABLE_SPACE}GB"
    
    if (( AVAILABLE_SPACE <= MIN_DISK_SPACE )); then
        log_info "Available space is less than ${MIN_DISK_SPACE}GB. Searching for a drive with more space..."
        NEW_DRIVE=$(find_suitable_drive)
        if [[ $? -eq 0 ]]; then
            log_info "Found suitable drive: $NEW_DRIVE"
            move_data "$CURRENT_DRIVE" "$NEW_DRIVE"
        else
            log_error "No suitable drive found. Please add more storage to continue."
            exit 1
        fi
    fi
    
    # Update system
    log_info "Updating system packages..."
    sudo apt-get update
    sudo apt-get upgrade -y
    
    # Install dependencies
    log_info "Installing required dependencies..."
    sudo apt-get install -y "${REQUIRED_PACKAGES[@]}"
    
    # Install Rust
    install_rust
    
    # Clone repository
    log_info "Cloning the Anya Core repository..."
    git clone https://github.com/botshelomokoka/anya-core.git
    cd anya-core || exit
    
    # Build project
    log_info "Building the project..."
    cargo build --release
    
    # Setup environment and Web5
    setup_environment
    setup_web5
    
    # Install additional tools
    log_info "Installing additional tools..."
    cargo install cargo-watch cargo-audit
    
    # Run tests
    log_info "Running integration tests..."
    cargo test --test integration_tests
    
    log_info "Full project setup and installation complete!"
    log_info "You can now run the project using 'cargo run'"
}

# Run main if script is executed directly
if [[ "${BASH_SOURCE[0]}" = "$0" ]]; then
    main
fi
