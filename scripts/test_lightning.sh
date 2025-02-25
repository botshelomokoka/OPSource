#!/bin/bash
# Script to test Lightning Network implementation

# Ensure we're in the project root
cd "$(dirname "$0")/.." || exit

echo "===========================================" 
echo "⚡ OPSource Lightning Network Tester"
echo "===========================================" 

# Build with both implementations and Lightning support
echo -e "\n🔨 Building with Lightning support..."
cargo build --features full --bin lightning_test

if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

# Run the individual component tests
echo -e "\n🧪 Running individual component tests..."
cargo test --features full "lightning::tests::"

if [ $? -ne 0 ]; then
    echo "❌ Component tests failed!"
    exit 1
fi

# Run tests with Mock Lightning implementation
echo -e "\n🧪 Running tests with Mock Lightning implementation..."
LIGHTNING_IMPLEMENTATION=mock cargo run --features full --bin lightning_test

# Run comprehensive tests to ensure invoice and payment functionality works
echo -e "\n💸 Testing payment functionality..."
cargo test --features full "lightning::tests::test_invoice_manager"
cargo test --features full "lightning::tests::test_payment_router"
cargo test --features full "lightning::tests::test_payment_executor"

# Uncomment once LDK implementation is further along
# Run tests with LDK implementation
# echo -e "\n🧪 Running tests with LDK Lightning implementation..."
# LIGHTNING_IMPLEMENTATION=ldk cargo run --features full --bin lightning_test

echo -e "\n✅ All tests completed!" 