#!/bin/bash
# Build for Linux
cross build --target x86_64-unknown-linux-gnu --release

# Optional: Build for multiple architectures
cross build --target aarch64-unknown-linux-gnu --release

