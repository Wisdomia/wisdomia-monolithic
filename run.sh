#!/bin/bash

# Build worker
cd worker
cargo build --release
cd ..

# Build server
cd server
cargo build --release
cd ..

# Start worker
./target/release/worker &

# Start server
./target/release/wisdomia-monolithic
