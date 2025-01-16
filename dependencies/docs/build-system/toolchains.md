# Toolchains

This document details the toolchain configuration in Anya.

## Rust Toolchain

### 1. Channel Selection
```toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
targets = ["x86_64-unknown-linux-gnu", "x86_64-pc-windows-msvc"]
profile = "minimal"
```

### 2. Component Installation
```bash
# Install core components
rustup component add rustfmt
rustup component add clippy
rustup component add rust-src
rustup component add rust-analysis

# Install target support
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-msvc
rustup target add aarch64-apple-darwin
```

### 3. Tool Configuration
```toml
# rustfmt.toml
max_width = 100
tab_spaces = 4
edition = "2021"
merge_derives = true
use_small_heuristics = "Max"

# clippy.toml
cognitive-complexity-threshold = 30
too-many-arguments-threshold = 10
```

## External Tools

### 1. Build Tools
```bash
# Windows (PowerShell)
choco install cmake llvm visualstudio2019buildtools

# Linux
apt install build-essential cmake llvm

# macOS
brew install cmake llvm
```

### 2. Development Tools
```bash
# Install development tools
cargo install cargo-edit
cargo install cargo-watch
cargo install cargo-outdated
cargo install cargo-audit
```

### 3. Testing Tools
```bash
# Install testing tools
cargo install cargo-tarpaulin
cargo install cargo-nextest
cargo install cargo-criterion
```

## Best Practices

### 1. Toolchain Management
- Use rustup for toolchain management
- Keep toolchain updated
- Use consistent versions
- Document requirements

### 2. Development Workflow
- Use cargo-edit for dependency management
- Use cargo-watch for development
- Use cargo-outdated for updates
- Use cargo-audit for security

### 3. Testing Workflow
- Use cargo-tarpaulin for coverage
- Use cargo-nextest for testing
- Use cargo-criterion for benchmarks
- Use clippy for linting

## Related Documentation
- [Build Profiles](build-profiles.md)
- [Cross Compilation](cross-compilation.md)
- [Dependencies](dependencies.md)
