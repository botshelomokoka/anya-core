# Cross Compilation

## Overview

Cross compilation is the process of compiling code for one type of computer system (the target) on a different type of computer system (the host). This is useful when the target system is not capable of compiling code itself, or when the host system has more resources available.

## Supported Targets

The following targets are supported:

- `x86_64-unknown-linux-gnu` (64-bit Linux)
- `x86_64-unknown-linux-musl` (64-bit Linux, statically linked)
- `x86_64-apple-darwin` (64-bit macOS)
- `i686-unknown-linux-gnu` (32-bit Linux)
- `i686-unknown-linux-musl` (32-bit Linux, statically linked)
- `i686-apple-darwin` (32-bit macOS)
- `arm-unknown-linux-gnueabihf` (ARMv7, hard float)
- `aarch64-unknown-linux-gnu` (AArch64, Linux)
- `wasm32-unknown-unknown` (WebAssembly)

## Using Cross Compilation

To cross compile, you can use the `--target` flag when running `cargo build`. For example, to build for 64-bit Linux:
