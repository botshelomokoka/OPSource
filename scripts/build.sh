#!/bin/bash
set -e

# Check for Rust installation
if ! command -v cargo &>/dev/null; then
  echo "Cargo is not installed. Please install Rust and Cargo to proceed."
  exit 1
fi

echo "Building the project..."
cargo build --release

echo "Build completed successfully." 