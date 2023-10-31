#!/bin/bash

# Prepare tests
docker-compose -f rudp2p/tests/compose.yml up -d

# Build
cargo build --verbose

# Run tests with default features
cargo test
# Run tests with feature ""ssl"
cargo test --features="ssl"

# Clear tests
docker-compose -f rudp2p/tests/compose.yml down

# Compatibility WASM
cd wasm
rustup target add wasm32-unknown-unknown
cargo build --lib --target wasm32-unknown-unknown
