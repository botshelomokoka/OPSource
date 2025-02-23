#!/bin/bash
set -e

# Script location
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( cd "$SCRIPT_DIR/.." && pwd )"

# Load environment variables based on the environment
ENVIRONMENT=${1:-"development"}
ENV_FILE="$PROJECT_ROOT/.env.${ENVIRONMENT}"

# Print step information
print_step() {
    echo -e "\n🔧 $1"
}

print_step "Setting up environment: $ENVIRONMENT"

if [ -f "$ENV_FILE" ]; then
    # Load and export environment variables
    set -a
    source "$ENV_FILE"
    set +a
    echo "Loaded environment from $ENV_FILE"
else
    echo "❌ Environment file $ENV_FILE not found!"
    exit 1
fi

# Check and export BITCOIN_NETWORK if not set
if [ -z "${BITCOIN_NETWORK}" ]; then
    export BITCOIN_NETWORK="mainnet"
    echo "Set default BITCOIN_NETWORK = $BITCOIN_NETWORK"
fi

# Check and export NODE_ENV if not set
if [ -z "${NODE_ENV}" ]; then
    export NODE_ENV="production"
    echo "Set default NODE_ENV = $NODE_ENV"
fi

# Verify Node.js installation
if ! command -v node >/dev/null 2>&1; then
    echo "❌ Node.js not found! Please install Node.js"
    exit 1
fi

# Verify npm installation
if ! command -v npm >/dev/null 2>&1; then
    echo "❌ npm not found! Please install npm"
    exit 1
fi

echo -e "\n✅ Environment setup complete!"

# Print current environment
print_step "Current environment:"
echo "BITCOIN_NETWORK = $BITCOIN_NETWORK"
echo "NODE_ENV = $NODE_ENV"
# Development Environment Configuration
BITCOIN_NETWORK=testnet
NODE_ENV=development
DEBUG=true
API_PORT=3000
CLARITY_VERSION=2
DAO_CONTRACT_PATH=src/core/dao/contracts
VSCODE_SETTINGS_PATH=.vscode/settings.json