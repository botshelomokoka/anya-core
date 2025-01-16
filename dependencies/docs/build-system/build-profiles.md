# Build Profiles

## Overview

Anya's build system uses a set of build profiles to control the compilation process. Build profiles are used to specify the optimization level, code generation, and other settings for the build. The build profiles are divided into three categories: `dev`, `release`, and `benchmark`.

## Profile Details

### `dev` Profile

The `dev` profile is used for development builds. It is optimized for fast compilation and debugging. The profile settings are as follows:

* `opt-level = 0`: Disables optimizations
* `debug = true`: Enables debug information
* `lto = false`: Disables link-time optimization
* `codegen-units = 1`: Sets the number of code generation units to 1
* `panic = 'unwind'`: Sets the panic strategy to unwind

### `release` Profile

The `release` profile is used for release builds. It is optimized for performance and size. The profile settings are as follows:

* `opt-level = 3`: Enables aggressive optimizations
* `debug = false`: Disables debug information
* `lto = true`: Enables link-time optimization
* `codegen-units = 16`: Sets the number of code generation units to 16
* `panic = 'abort'`: Sets the panic strategy to abort

### `benchmark` Profile

The `benchmark` profile is used for benchmarking builds. It is optimized for performance and size. The profile settings are as follows:

* `opt-level = 3`: Enables aggressive optimizations
* `debug = false`: Disables debug information
* `lto = true`: Enables link-time optimization
* `codegen-units = 16`: Sets the number of code generation units to 16
* `panic = 'abort'`: Sets the panic strategy to abort
