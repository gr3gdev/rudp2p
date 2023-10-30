#!/bin/bash

# Prepare tests
docker-compose -f rudp2p/tests/compose.yml up -d

# Build
cargo build --verbose

# Compatibility WASM
rustup target add wasm32-unknown-unknown
cargo build --lib --target wasm32-unknown-unknown

cd rudp2p
# Run tests with default features
cargo test
# Run tests with feature ""ssl"
cargo test --features="ssl"
cd ..

# Clear tests
docker-compose -f rudp2p/tests/compose.yml down