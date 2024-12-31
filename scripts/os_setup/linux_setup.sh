#!/bin/bash
set -euo pipefail

# Linux-specific setup for Anya Project
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Source common utilities
# shellcheck source=../lib/common.sh
source "$SCRIPT_DIR/../lib/common.sh"

setup_linux_environment() {
    log_info "Setting up Linux environment..."
    
    # Set ulimit values for development
    if [ -w "/etc/security/limits.conf" ]; then
        cat >> /etc/security/limits.conf << EOF
* soft nofile 65535
* hard nofile 65535
* soft nproc 32768
* hard nproc 32768
EOF
        log_info "Updated system limits"
    else
        log_warn "Cannot update system limits - requires root access"
    fi
    
    # Set swappiness for better performance
    if [ -w "/proc/sys/vm/swappiness" ]; then
        echo 10 > /proc/sys/vm/swappiness
        echo "vm.swappiness = 10" >> /etc/sysctl.conf
        log_info "Set VM swappiness to 10"
    else
        log_warn "Cannot set VM swappiness - requires root access"
    fi
}

optimize_linux_performance() {
    log_info "Optimizing Linux performance settings..."
    
    # Enable performance CPU governor if available
    if [ -w "/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor" ]; then
        for governor in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor; do
            echo "performance" > "$governor"
        done
        log_info "Set CPU governor to performance mode"
    else
        log_warn "Cannot set CPU governor - requires root access"
    fi
    
    # Configure transparent hugepages
    if [ -w "/sys/kernel/mm/transparent_hugepage/enabled" ]; then
        echo "always" > /sys/kernel/mm/transparent_hugepage/enabled
        log_info "Enabled transparent hugepages"
    else
        log_warn "Cannot configure transparent hugepages - requires root access"
    fi
}

setup_development_tools() {
    log_info "Setting up development tools..."
    
    # Detect package manager
    if command -v apt-get >/dev/null; then
        PKG_MANAGER="apt-get"
        PKG_INSTALL="apt-get install -y"
    elif command -v dnf >/dev/null; then
        PKG_MANAGER="dnf"
        PKG_INSTALL="dnf install -y"
    elif command -v yum >/dev/null; then
        PKG_MANAGER="yum"
        PKG_INSTALL="yum install -y"
    elif command -v pacman >/dev/null; then
        PKG_MANAGER="pacman"
        PKG_INSTALL="pacman -S --noconfirm"
    else
        log_error "No supported package manager found"
        exit 1
    fi
    
    # Install common development tools
    local packages=(
        "build-essential"
        "pkg-config"
        "libssl-dev"
        "cmake"
        "llvm"
        "clang"
    )
    
    if [ "$PKG_MANAGER" = "pacman" ]; then
        packages=(
            "base-devel"
            "openssl"
            "cmake"
            "llvm"
            "clang"
        )
    fi
    
    log_info "Installing development packages..."
    if ! sudo $PKG_INSTALL "${packages[@]}"; then
        log_error "Failed to install development packages"
        exit 1
    fi
}

configure_git() {
    log_info "Configuring Git for Linux..."
    
    # Configure Git to handle line endings
    git config --global core.autocrlf input
    
    # Enable Git credential cache
    git config --global credential.helper cache
    git config --global credential.helper 'cache --timeout=3600'
    
    log_info "Git configuration completed"
}

setup_systemd_services() {
    log_info "Setting up systemd services..."
    
    # Create systemd service for Anya
    local service_file="/etc/systemd/system/anya.service"
    if [ -w "$(dirname "$service_file")" ]; then
        cat > "$service_file" << EOF
[Unit]
Description=Anya Web5 Service
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$PROJECT_ROOT
ExecStart=$HOME/.cargo/bin/cargo run --release
Restart=always
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
EOF
        
        systemctl daemon-reload
        systemctl enable anya.service
        log_info "Created and enabled Anya systemd service"
    else
        log_warn "Cannot create systemd service - requires root access"
    fi
}

main() {
    log_info "Starting Linux-specific setup..."
    
    # Check if running as root
    if [ "$EUID" -ne 0 ]; then
        log_warn "Some operations may require root privileges"
    fi
    
    setup_linux_environment
    optimize_linux_performance
    setup_development_tools
    configure_git
    setup_systemd_services
    
    log_info "Linux-specific setup completed successfully"
}

# Run main if script is executed directly
if [[ "${BASH_SOURCE[0]}" = "$0" ]]; then
    main
fi
