#!/bin/bash
set -e

echo "Running tests for all packages..."

# Run tests with coverage
cargo tarpaulin --workspace --out Html

echo "Tests completed successfully. Coverage report generated at ./tarpaulin-report.html." 