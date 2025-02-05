#!/bin/bash

set -e

# Color codes for output
BLUE='\033[0;34m'
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

echo "Starting AI Core Project installation..."

# Check prerequisites
echo -e "${BLUE}==>${NC} Checking prerequisites..."

# Check Git version
if ! command -v git &> /dev/null; then
    echo -e "${RED}Error:${NC} Git is not installed"
    exit 1
fi

# Check Rust version
if ! command -v rustc &> /dev/null; then
    echo -e "${RED}Error:${NC} Rust is not installed"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check Node.js version
if ! command -v node &> /dev/null; then
    echo -e "${RED}Error:${NC} Node.js is not installed"
    echo "Please install Node.js from https://nodejs.org/"
    exit 1
fi

# Check Flutter version (for mobile)
if ! command -v flutter &> /dev/null; then
    echo -e "${YELLOW}Warning:${NC} Flutter is not installed (required for mobile development)"
    echo "Please install Flutter from https://flutter.dev/docs/get-started/install"
fi

# Setup submodules
echo -e "${BLUE}==>${NC} Setting up submodules..."
./scripts/setup_submodules.sh

# Install Rust dependencies
echo -e "${BLUE}==>${NC} Installing Rust dependencies..."
cargo check

# Create git hooks
echo -e "${BLUE}==>${NC} Setting up git hooks..."
if [ ! -d ".git/hooks" ]; then
    mkdir -p .git/hooks
fi

# Pre-commit hook
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
set -e

echo "Running pre-commit checks..."

# Run cargo fmt
cargo fmt -- --check

# Run cargo clippy
cargo clippy -- -D warnings

# Run tests
cargo test
EOF

chmod +x .git/hooks/pre-commit

echo -e "${GREEN}Success:${NC} Installation complete!"
echo "
Next steps:
1. Run 'cargo build' to build the project
2. Check the README.md for more information
3. Start developing!"
