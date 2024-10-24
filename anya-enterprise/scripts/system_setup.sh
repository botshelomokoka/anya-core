#!/bin/bash

set -e

# Function to print status messages
print_status() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') - $1"
}

# Function to check disk space
check_disk_space() {
    local drive=$1
    local available_space=$(df -BG $drive | awk 'NR==2 {print $4}' | sed 's/G//')
    echo $available_space
}

# Function to find a drive with more than 50GB available
find_suitable_drive() {
    local drives=$(lsblk -ndo NAME,TYPE | awk '$2=="disk" {print "/dev/"$1}')
    for drive in $drives; do
        local space=$(check_disk_space $drive)
        if [ $space -gt 50 ]; then
            echo $drive
            return 0
        fi
    done
    return 1
}

# Function to move data to a new drive
move_data() {
    local old_drive=$1
    local new_drive=$2
    print_status "Moving data from $old_drive to $new_drive"
    
    # Mount new drive
    sudo mkdir -p /mnt/newdrive
    sudo mount $new_drive /mnt/newdrive
    
    # Copy data
    sudo rsync -avz --exclude='/mnt/newdrive' / /mnt/newdrive/
    
    # Update fstab
    sudo sed -i "s|$old_drive|$new_drive|g" /etc/fstab
    
    print_status "Data moved successfully. Please reboot to apply changes."
    exit 0
}

# Specific project details
INSTANCE_ID="9111727350091981557"
PROJECT_ID="anya-433919"
USERNAME="botshelomokoka@gmail.com"

# Detect instance name and zone
INSTANCE_NAME=$(curl -s "http://metadata.google.internal/computeMetadata/v1/instance/name" -H "Metadata-Flavor: Google")
ZONE=$(curl -s "http://metadata.google.internal/computeMetadata/v1/instance/zone" -H "Metadata-Flavor: Google" | awk -F/ '{print $NF}')

print_status "Project setup initiated"
print_status "Username: $USERNAME"
print_status "Detected instance name: $INSTANCE_NAME"
print_status "Detected zone: $ZONE"
print_status "Instance ID: $INSTANCE_ID"
print_status "Project ID: $PROJECT_ID"

# Check current disk space
CURRENT_DRIVE=$(df / | awk 'NR==2 {print $1}')
AVAILABLE_SPACE=$(check_disk_space $CURRENT_DRIVE)

print_status "Current drive: $CURRENT_DRIVE"
print_status "Available space: ${AVAILABLE_SPACE}GB"

if [ $AVAILABLE_SPACE -le 50 ]; then
    print_status "Available space is less than 50GB. Searching for a drive with more space..."
    NEW_DRIVE=$(find_suitable_drive)
    if [ $? -eq 0 ]; then
        print_status "Found suitable drive: $NEW_DRIVE"
        move_data $CURRENT_DRIVE $NEW_DRIVE
    else
        print_status "No suitable drive found. Please add more storage to continue."
        exit 1
    fi
fi

# Update system packages
print_status "Updating system packages..."
sudo apt-get update
sudo apt-get upgrade -y

# Install required dependencies
print_status "Installing required dependencies..."
sudo apt-get install -y build-essential curl libssl-dev pkg-config

# Install Rust
print_status "Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# Clone the repository
print_status "Cloning the Anya Core repository..."
git clone https://github.com/botshelomokoka/anya-core.git
cd anya-core

# Build the project
print_status "Building the project..."
cargo build --release

# Set up environment variables
print_status "Setting up environment variables..."
cp .env.example .env
sed -i "s/PROJECT_ID=.*/PROJECT_ID=$PROJECT_ID/" .env
sed -i "s/INSTANCE_NAME=.*/INSTANCE_NAME=$INSTANCE_NAME/" .env
sed -i "s/ZONE=.*/ZONE=$ZONE/" .env
sed -i "s/INSTANCE_ID=.*/INSTANCE_ID=$INSTANCE_ID/" .env
sed -i "s/USERNAME=.*/USERNAME=$USERNAME/" .env

# Set up database
print_status "Setting up database..."
sudo apt-get install -y postgresql postgresql-contrib
sudo -u postgres createdb anya_core
sudo -u postgres psql -c "ALTER USER postgres WITH PASSWORD 'anya_core_password';"
sed -i "s/DATABASE_URL=.*/DATABASE_URL=postgres:\/\/postgres:anya_core_password@localhost\/anya_core/" .env

# Run database migrations
print_status "Running database migrations..."
cargo install diesel_cli --no-default-features --features postgres
diesel setup
diesel migration run

# Install additional tools
print_status "Installing additional tools..."
cargo install cargo-watch
cargo install cargo-audit

# Install dependencies for ZK proofs, STX, DLC, Lightning, and Bitcoin support
print_status "Installing dependencies for advanced features..."
sudo apt-get install -y libgmp-dev libsodium-dev

# Set up Kademlia and libp2p
print_status "Setting up Kademlia and libp2p..."
cargo install libp2p-cli
echo "KADEMLIA_BOOTSTRAP_NODES=<bootstrap_node_addresses>" >> .env
echo "LIBP2P_LISTENING_ADDRESS=/ip4/0.0.0.0/tcp/4001" >> .env

# Set up Web5 support
print_status "Setting up Web5 support..."
cargo install web5-cli
echo "WEB5_DID_METHOD=key" >> .env
echo "WEB5_CREDENTIAL_STATUS_TYPE=RevocationList2020" >> .env

# Set up ML logic
print_status "Setting up ML logic..."
sudo apt-get install -y python3-pip
pip3 install tensorflow numpy pandas scikit-learn
echo "ML_MODEL_PATH=/path/to/ml/model" >> .env
echo "ML_DATA_DIR=/path/to/ml/data" >> .env

# Run integration tests
print_status "Running integration tests..."
cargo test --test integration_tests

print_status "Full project setup and installation on GCloud complete!"
print_status "Instance: $INSTANCE_NAME"
print_status "Zone: $ZONE"
print_status "Username: $USERNAME"
print_status "Instance ID: $INSTANCE_ID"
print_status "Project ID: $PROJECT_ID"
print_status "You can now run the project using 'cargo run'"
