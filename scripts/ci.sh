#!/bin/bash
set -e

echo "Checking code formatting..."
cargo fmt --all -- --check

echo "Running Clippy for code linting..."
cargo clippy --all-targets --all-features -- -D warnings

echo "Running security audit..."
cargo audit

echo "Running tests..."
./scripts/test.sh

echo "CI checks completed successfully." 