# Anya Project Web5 Setup Script
# Run as Administrator for system-wide installations

$ErrorActionPreference = "Stop"

function Write-Status {
    param($Message)
    Write-Host "==> $Message" -ForegroundColor Cyan
}

function Test-CommandExists {
    param($Command)
    $null -ne (Get-Command -Name $Command -ErrorAction SilentlyContinue)
}

Write-Status "Setting up Anya Web5 Development Environment..."

# Install Visual Studio Build Tools if not present
if (-not (Test-Path "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools")) {
    Write-Status "Installing Visual Studio Build Tools..."
    # Download VS Build Tools
    $vsUrl = "https://aka.ms/vs/17/release/vs_buildtools.exe"
    $vsInstaller = "$env:TEMP\vs_buildtools.exe"
    Invoke-WebRequest -Uri $vsUrl -OutFile $vsInstaller
    
    # Install with required components
    Start-Process -Wait -FilePath $vsInstaller -ArgumentList "--quiet", "--wait", "--norestart", "--nocache", `
        "--installPath", "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools", `
        "--add", "Microsoft.VisualStudio.Component.VC.Tools.x86.x64"
    
    Remove-Item $vsInstaller
}

# Install Node.js if not present (required for Web5)
if (-not (Test-CommandExists "node")) {
    Write-Status "Installing Node.js..."
    $nodeUrl = "https://nodejs.org/dist/v20.10.0/node-v20.10.0-x64.msi"
    $nodeInstaller = "$env:TEMP\node_installer.msi"
    Invoke-WebRequest -Uri $nodeUrl -OutFile $nodeInstaller
    Start-Process -Wait msiexec -ArgumentList "/i", $nodeInstaller, "/quiet", "/norestart"
    Remove-Item $nodeInstaller
}

# Install Rust if not present
if (-not (Test-CommandExists "rustc")) {
    Write-Status "Installing Rust..."
    Invoke-WebRequest -Uri "https://win.rustup.rs" -OutFile "rustup-init.exe"
    Start-Process -Wait -FilePath ".\rustup-init.exe" -ArgumentList "-y", "--default-toolchain", "nightly"
    Remove-Item "rustup-init.exe"
}

# Update Rust and add components
Write-Status "Updating Rust and adding components..."
rustup update
rustup default nightly
rustup component add rustfmt clippy

# Install Web5 CLI tools
Write-Status "Installing Web5 development tools..."
npm install -g @web5/cli

# Install Rust dependencies
Write-Status "Installing Rust dependencies..."
cargo install cargo-watch
cargo install cargo-edit

# Create Web5 configuration
Write-Status "Setting up Web5 configuration..."
$web5Config = @{
    did = @{
        method = "key"
    }
    dwn = @{
        uri = "https://dwn.anya.blockchain"
    }
} | ConvertTo-Json

$web5Config | Out-File -FilePath ".web5.json" -Encoding UTF8

# Set up environment variables
Write-Status "Setting up environment variables..."
[Environment]::SetEnvironmentVariable("RUST_BACKTRACE", "1", "User")
[Environment]::SetEnvironmentVariable("RUST_LOG", "debug", "User")
[Environment]::SetEnvironmentVariable("WEB5_ENV", "development", "User")

Write-Status "Installation complete! Next steps:"
Write-Host "1. Run 'cargo build' to build the project"
Write-Host "2. Run 'cargo test' to verify installation"
Write-Host "3. Start developing with Web5!"
