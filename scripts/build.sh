#!/bin/bash
set -e

echo "Building Linux Control Center..."

# Build Rust crates
echo "Building Rust workspace..."
cargo build --release

# Build Tauri app
echo "Building Tauri application..."
cd apps/desktop
npm install
npm run tauri build

echo "Build complete!"
