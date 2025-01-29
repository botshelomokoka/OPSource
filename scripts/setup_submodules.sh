#!/bin/bash

set -e

# Color codes for output
BLUE='\033[0;34m'
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}==>${NC} Setting up submodules..."

# Function to create directory if it doesn't exist
create_dir_if_not_exists() {
    if [ ! -d "$1" ]; then
        mkdir -p "$1"
    fi
}

# Setup core submodules
echo "Setting up AI Core submodules..."
git submodule update --init --recursive anya-core/dash33
git submodule update --init --recursive anya-core/enterprise
git submodule update --init --recursive anya-core/mobile

# Create dependency directories
echo "Creating dependency directories..."
create_dir_if_not_exists "anya-core/enterprise/dependencies"
create_dir_if_not_exists "anya-core/mobile/dependencies"

# Setup enterprise dependencies
echo "Setting up enterprise dependencies..."
cd anya-core/enterprise/dependencies
git submodule update --init --recursive web5-rs
cd ../../..

# Setup mobile dependencies
echo "Setting up mobile dependencies..."
cd anya-core/mobile/dependencies
git submodule update --init --recursive web5-rs
git submodule update --init --recursive flutter-rust-bridge
cd ../../..

echo "Submodule setup complete!"
echo -e "${GREEN}Success:${NC} Submodules initialized successfully"
