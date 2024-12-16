#!/bin/bash
set -euo pipefail

# macOS-specific setup for Anya Project
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Source common utilities
# shellcheck source=../lib/common.sh
source "$SCRIPT_DIR/../lib/common.sh"

setup_macos_environment() {
    log_info "Setting up macOS environment..."
    
    # Set ulimit values for development
    if ! grep -q "ulimit" ~/.zshrc 2>/dev/null; then
        cat >> ~/.zshrc << EOF
# Increase file descriptor limits
ulimit -n 65536
ulimit -u 2048
EOF
        log_info "Updated shell limits in .zshrc"
    fi
    
    # Configure performance settings
    sudo sysctl -w kern.maxfiles=65536
    sudo sysctl -w kern.maxfilesperproc=65536
    sudo sysctl -w kern.maxvnodes=262144
    
    # Make settings permanent
    if [ ! -f "/etc/sysctl.conf" ]; then
        sudo touch /etc/sysctl.conf
    fi
    
    cat > /tmp/sysctl.conf << EOF
kern.maxfiles=65536
kern.maxfilesperproc=65536
kern.maxvnodes=262144
EOF
    
    sudo mv /tmp/sysctl.conf /etc/sysctl.conf
    log_info "Updated system limits"
}

setup_homebrew() {
    log_info "Setting up Homebrew..."
    
    # Install Homebrew if not present
    if ! command -v brew >/dev/null; then
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
        log_info "Installed Homebrew"
    fi
    
    # Update Homebrew
    brew update
    
    # Install essential packages
    local packages=(
        "openssl"
        "pkg-config"
        "cmake"
        "llvm"
        "node"
        "git"
    )
    
    log_info "Installing development packages..."
    for package in "${packages[@]}"; do
        if ! brew list "$package" &>/dev/null; then
            brew install "$package"
            log_info "Installed $package"
        fi
    done
}

configure_xcode() {
    log_info "Configuring Xcode Command Line Tools..."
    
    # Install Xcode Command Line Tools if not present
    if ! xcode-select -p &>/dev/null; then
        xcode-select --install
        log_info "Installing Xcode Command Line Tools..."
        # Wait for installation to complete
        until xcode-select -p &>/dev/null; do
            sleep 5
        done
    fi
    
    # Accept Xcode license
    if ! sudo xcodebuild -license status 2>/dev/null | grep -q "accepted"; then
        sudo xcodebuild -license accept
        log_info "Accepted Xcode license"
    fi
}

configure_git() {
    log_info "Configuring Git for macOS..."
    
    # Configure Git to handle line endings
    git config --global core.autocrlf input
    
    # Use Keychain for credential storage
    git config --global credential.helper osxkeychain
    
    # Enable Git CLI colors
    git config --global color.ui auto
    
    log_info "Git configuration completed"
}

setup_launchd_service() {
    log_info "Setting up launchd service..."
    
    local plist_file="$HOME/Library/LaunchAgents/ai.anya.service.plist"
    mkdir -p "$(dirname "$plist_file")"
    
    cat > "$plist_file" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>ai.anya.service</string>
    <key>ProgramArguments</key>
    <array>
        <string>$HOME/.cargo/bin/cargo</string>
        <string>run</string>
        <string>--release</string>
    </array>
    <key>WorkingDirectory</key>
    <string>$PROJECT_ROOT</string>
    <key>EnvironmentVariables</key>
    <dict>
        <key>RUST_LOG</key>
        <string>info</string>
    </dict>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>$HOME/Library/Logs/anya.log</string>
    <key>StandardErrorPath</key>
    <string>$HOME/Library/Logs/anya.error.log</string>
</dict>
</plist>
EOF
    
    # Load service
    launchctl load -w "$plist_file"
    log_info "Created and loaded Anya launchd service"
}

optimize_macos_performance() {
    log_info "Optimizing macOS performance settings..."
    
    # Disable spotlight indexing for the project directory
    sudo mdutil -i off "$PROJECT_ROOT"
    
    # Disable sleep mode during development
    sudo pmset -a sleep 0
    sudo pmset -a hibernatemode 0
    
    # Disable App Nap for Terminal and IDE
    defaults write com.apple.Terminal NSAppSleepDisabled -bool YES
    defaults write com.microsoft.VSCode NSAppSleepDisabled -bool YES
    
    log_info "Performance optimizations completed"
}

main() {
    log_info "Starting macOS-specific setup..."
    
    # Check if running with sudo
    if [ "$EUID" -ne 0 ]; then
        log_warn "Some operations may require sudo privileges"
    fi
    
    configure_xcode
    setup_homebrew
    setup_macos_environment
    configure_git
    setup_launchd_service
    optimize_macos_performance
    
    log_info "macOS-specific setup completed successfully"
    log_info "Please restart your terminal to apply all changes"
}

# Run main if script is executed directly
if [[ "${BASH_SOURCE[0]}" = "$0" ]]; then
    main
fi
