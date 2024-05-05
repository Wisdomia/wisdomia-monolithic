#!/bin/bash

# Build worker worker_wisdoms_checker
cd worker_wisdoms_checker
cargo build --release
cd ..

# Build worker worker_wisdoms_daily_rotator
cd worker_wisdoms_daily_rotator
cargo build --release
cd ..

# Build server
cd server
cargo build --release
cd ..

# Start worker worker_wisdoms_checker
./target/release/worker_wisdoms_checker &

# Start worker worker_wisdoms_daily_rotator
./target/release/worker_wisdoms_daily_rotator &

# Start server
./target/release/wisdomia-monolithic
