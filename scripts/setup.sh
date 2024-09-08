#!/bin/bash

# Setup script for Anya Core project

# Update system packages
echo "Updating system packages..."
sudo apt-get update
sudo apt-get upgrade -y

# Install required dependencies
echo "Installing required dependencies..."
sudo apt-get install -y build-essential curl libssl-dev pkg-config

# Install Rust
echo "Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# Clone the repository
echo "Cloning the Anya Core repository..."
git clone https://github.com/botshelomokoka/anya-core.git
cd anya-core

# Build the project
echo "Building the project..."
cargo build --release

# Set up environment variables
echo "Setting up environment variables..."
cp .env.example .env
# TODO: Prompt user to fill in necessary values in .env file

# Set up database
echo "Setting up database..."
# TODO: Add database setup commands

# Install additional tools
echo "Installing additional tools..."
cargo install cargo-watch
cargo install cargo-audit

echo "Setup complete! You can now run the project using 'cargo run'"