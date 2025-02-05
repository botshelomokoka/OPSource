#!/bin/bash
set -e

# Load environment variables based on the environment
ENVIRONMENT=${1:-"development"}

if [ -f ".env.${ENVIRONMENT}" ]; then
  export $(grep -v '^#' .env.${ENVIRONMENT} | xargs)
else
  echo "Environment file .env.${ENVIRONMENT} not found!"
  exit 1
fi

# Check and export BITCOIN_NETWORK if not set
if [ -z "${BITCOIN_NETWORK}" ]; then
  export BITCOIN_NETWORK="mainnet"
fi

# Check and export NODE_ENV if not set
if [ -z "${NODE_ENV}" ]; then
  export NODE_ENV="production"
fi 