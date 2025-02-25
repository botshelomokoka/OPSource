#!/bin/bash
# Script to test both Python and Rust Bitcoin implementations

# Ensure we're in the project root
cd "$(dirname "$0")/.." || exit

echo "===========================================" 
echo "🧪 OPSource Bitcoin Implementation Tester"
echo "===========================================" 

# Build with both features
echo -e "\n🔨 Building with both implementations..."
cargo build --features full

if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

# Run tests
echo -e "\n🧪 Running tests for both implementations..."
cargo run -- test

if [ $? -ne 0 ]; then
    echo "❌ Tests failed!"
    exit 1
fi

# Run Python implementation demo
echo -e "\n🐍 Running Python implementation demo..."
USE_RUST_BITCOIN=false cargo run -- python

# Run Rust implementation demo
echo -e "\n🦀 Running Rust implementation demo..."
USE_RUST_BITCOIN=true cargo run -- rust

echo -e "\n✅ All tests completed!" 