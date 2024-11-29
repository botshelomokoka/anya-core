---
layout: default
title: Cross-Platform Installation
description: Install Anya on any platform using Dart
---

# Cross-Platform Installation Guide

Anya now supports cross-platform installation using Dart SDK, making it easier to deploy and run on any operating system.

## Prerequisites

### Windows
```powershell
# Install Chocolatey (if not installed)
Set-ExecutionPolicy Bypass -Scope Process -Force
[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))

# Install Dart SDK
choco install dart-sdk -y
```

### macOS
```bash
# Install using Homebrew
brew tap dart-lang/dart
brew install dart
```

### Linux (Ubuntu/Debian)
```bash
# Add Google's apt repository
sudo apt-get update
sudo apt-get install apt-transport-https
wget -qO- https://dl-ssl.google.com/linux/linux_signing_key.pub | sudo gpg --dearmor -o /usr/share/keyrings/dart.gpg
echo 'deb [signed-by=/usr/share/keyrings/dart.gpg arch=amd64] https://storage.googleapis.com/download.dartlang.org/linux/debian stable main' | sudo tee /etc/apt/sources.list.d/dart_stable.list

# Install Dart SDK
sudo apt-get update
sudo apt-get install dart
```

## Installing Anya

Once Dart SDK is installed, you can install Anya using:

```bash
dart pub global activate anya
```

## Verifying Installation

Verify your installation:

```bash
anya --version
```

## Configuration

Create a new Anya project:

```bash
anya init my_project
cd my_project
```

Configure your Bitcoin network settings in `config.yaml`:

```yaml
network:
  type: mainnet  # or testnet
  rpc_url: "http://localhost:8332"
  rpc_user: "your_username"
  rpc_password: "your_password"

web5:
  enabled: true
  did_method: "key"  # or "ion"
```

## Running Anya

Start the Anya service:

```bash
anya serve
```

## Development Setup

For development, you'll need additional tools:

```bash
# Install development dependencies
dart pub get

# Run tests
dart test

# Build for production
dart compile exe bin/anya.dart
```

## Troubleshooting

### Common Issues

1. **Dart SDK not found**
   ```bash
   # Add Dart to PATH
   export PATH="$PATH:/usr/lib/dart/bin"  # Linux
   # or
   refreshenv  # Windows (after Chocolatey installation)
   ```

2. **Permission Issues**
   ```bash
   # Linux/macOS
   sudo chown -R $(whoami) ~/.pub-cache
   ```

3. **Network Configuration**
   ```bash
   # Test Bitcoin RPC connection
   anya test-connection
   ```

## Next Steps

- [Quick Start Guide](../getting-started/quick-start)
- [API Reference](../api/)
- [Security Best Practices](../security/)
