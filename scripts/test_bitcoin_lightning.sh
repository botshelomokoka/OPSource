#!/bin/bash
# Script to test Bitcoin-Lightning bridge implementation

# Ensure we're in the project root
cd "$(dirname "$0")/.." || exit

echo "===========================================" 
echo "⚡ OPSource Bitcoin-Lightning Bridge Tester"
echo "===========================================" 

# Build with Lightning support
echo -e "\n🔨 Building with Bitcoin-Lightning bridge support..."
cargo build --features full --bin bitcoin_lightning_test

if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

# Run the bridge tests
echo -e "\n🧪 Running Bitcoin-Lightning bridge tests..."
cargo run --features full --bin bitcoin_lightning_test

if [ $? -ne 0 ]; then
    echo "❌ Bridge tests failed!"
    exit 1
fi

# Run unit tests for the bridge component
echo -e "\n🧪 Running Bitcoin-Lightning bridge unit tests..."
cargo test --features full lightning::bitcoin_bridge::tests

if [ $? -ne 0 ]; then
    echo "❌ Unit tests failed!"
    exit 1
fi

echo -e "\n✅ All tests completed successfully!" 