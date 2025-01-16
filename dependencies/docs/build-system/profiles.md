# Build Profiles

This document details the build profiles configuration in Anya.

## Profile Types

### 1. Development Profile
```toml
[profile.dev]
opt-level = 0
debug = true
debug-assertions = true
overflow-checks = true
lto = false
panic = 'unwind'
incremental = true
codegen-units = 256
rpath = false
```

### 2. Release Profile
```toml
[profile.release]
opt-level = 3
debug = false
debug-assertions = false
overflow-checks = false
lto = true
panic = 'abort'
incremental = false
codegen-units = 16
rpath = false
```

### 3. Test Profile
```toml
[profile.test]
opt-level = 0
debug = 2
debug-assertions = true
overflow-checks = true
lto = false
panic = 'unwind'
incremental = true
codegen-units = 256
rpath = false
```

## Profile Configuration

### 1. Optimization Settings
- opt-level: Optimization level (0-3)
- lto: Link Time Optimization
- codegen-units: Code generation units
- panic: Panic strategy

### 2. Debug Settings
- debug: Debug symbol level
- debug-assertions: Debug assertions
- overflow-checks: Integer overflow checks
- incremental: Incremental compilation

### 3. Platform Settings
- rpath: Runtime path
- target-cpu: Target CPU architecture
- target-features: CPU feature selection

## Best Practices

### 1. Development
- Fast compilation times
- Debug information
- Runtime checks
- Easy debugging

### 2. Release
- Maximum optimization
- Minimal binary size
- Best performance
- Production ready

### 3. Testing
- Quick compilation
- Debug information
- Test coverage
- Profiling support

## Related Documentation
- [Dependencies](dependencies.md)
- [Features](features.md)
- [Cross Compilation](cross-compilation.md)
