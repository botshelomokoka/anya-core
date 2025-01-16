# Target Platforms

This document details the supported target platforms in Anya.

## Platform Support

### 1. Windows
```toml
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "target-feature=+crt-static"]
linker = "link.exe"
ar = "lib.exe"

[target.x86_64-pc-windows-gnu]
rustflags = ["-C", "target-feature=+crt-static"]
linker = "x86_64-w64-mingw32-gcc"
ar = "x86_64-w64-mingw32-ar"
```

### 2. Linux
```toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
linker = "clang"
ar = "llvm-ar"

[target.aarch64-unknown-linux-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
linker = "aarch64-linux-gnu-gcc"
ar = "aarch64-linux-gnu-ar"
```

### 3. macOS
```toml
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
linker = "clang"
ar = "ar"

[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
linker = "clang"
ar = "ar"
```

## Platform Requirements

### 1. Windows Requirements
- Visual Studio Build Tools
- Windows SDK
- MSVC or MinGW toolchain
- Git for Windows

### 2. Linux Requirements
- GCC or Clang
- Build essentials
- OpenSSL development files
- SQLite development files

### 3. macOS Requirements
- Xcode Command Line Tools
- Homebrew (recommended)
- OpenSSL
- SQLite

## Cross Compilation

### 1. Windows to Linux
```bash
# Install cross toolchain
rustup target add x86_64-unknown-linux-gnu
# Install linker
apt install gcc-multilib
# Build
cargo build --target x86_64-unknown-linux-gnu
```

### 2. Linux to Windows
```bash
# Install cross toolchain
rustup target add x86_64-pc-windows-gnu
# Install linker
apt install mingw-w64
# Build
cargo build --target x86_64-pc-windows-gnu
```

### 3. Universal macOS
```bash
# Install cross toolchain
rustup target add aarch64-apple-darwin x86_64-apple-darwin
# Build universal binary
cargo build --target x86_64-apple-darwin
cargo build --target aarch64-apple-darwin
lipo -create target/*/release/anya -output anya-universal
```

## Best Practices

### 1. Development
- Use native toolchain
- Enable debug symbols
- Quick compilation
- Development profile

### 2. Release
- Cross compilation
- Optimization
- Static linking
- Release profile

### 3. Testing
- Platform-specific tests
- Integration tests
- Performance tests
- Security tests

## Related Documentation
- [Cross Compilation](cross-compilation.md)
- [Build Profiles](build-profiles.md)
- [Toolchains](toolchains.md)
