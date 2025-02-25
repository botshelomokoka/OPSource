#!/bin/bash
set -e

# Variables
VERSION=$(cat VERSION 2>/dev/null || echo "0.1.0")
PACKAGE_NAME="opsource-$VERSION"
OUTPUT_DIR="./packages"
BUILD_MODE="release"
INCLUDE_DOCS=true
INCLUDE_EXAMPLES=true

# Parse command-line arguments
while [[ $# -gt 0 ]]; do
  case "$1" in
    --version)
      VERSION="$2"
      shift 2
      ;;
    --output-dir)
      OUTPUT_DIR="$2"
      shift 2
      ;;
    --name)
      PACKAGE_NAME="$2"
      shift 2
      ;;
    --debug)
      BUILD_MODE="debug"
      shift
      ;;
    --no-docs)
      INCLUDE_DOCS=false
      shift
      ;;
    --no-examples)
      INCLUDE_EXAMPLES=false
      shift
      ;;
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
done

echo "=== OPSource Packaging Script ==="
echo "Version: $VERSION"
echo "Package name: $PACKAGE_NAME"
echo "Output directory: $OUTPUT_DIR"
echo "Build mode: $BUILD_MODE"
echo "Include docs: $INCLUDE_DOCS"
echo "Include examples: $INCLUDE_EXAMPLES"
echo "=================================="

# Make sure we're in the project root
if [ ! -f "package.json" ]; then
  echo "Error: This script must be run from the project root directory."
  exit 1
fi

# Run tests first
echo "Running tests..."
npm test || { echo "Tests failed, aborting packaging process."; exit 1; }

# Build the project
echo "Building project..."
npm run build || { echo "Build failed, aborting packaging process."; exit 1; }

# Create output directory
mkdir -p "$OUTPUT_DIR"
PACKAGE_DIR="$OUTPUT_DIR/$PACKAGE_NAME"
mkdir -p "$PACKAGE_DIR"

# Copy essential files
echo "Copying files to package directory..."
cp -r dist "$PACKAGE_DIR/"
cp package.json "$PACKAGE_DIR/"
cp README.md "$PACKAGE_DIR/"
cp LICENSE "$PACKAGE_DIR/" 2>/dev/null || echo "No LICENSE file found, skipping."
cp CHANGELOG.md "$PACKAGE_DIR/" 2>/dev/null || echo "No CHANGELOG.md file found, skipping."

# Optional docs
if [ "$INCLUDE_DOCS" = true ]; then
  echo "Including documentation..."
  if [ -d "docs" ]; then
    mkdir -p "$PACKAGE_DIR/docs"
    cp -r docs/* "$PACKAGE_DIR/docs/"
  else
    echo "Warning: No docs directory found."
    # Generate API docs if possible
    if command -v typedoc &>/dev/null; then
      echo "Generating API documentation with TypeDoc..."
      npx typedoc --out "$PACKAGE_DIR/docs/api" src/
    fi
  fi
fi

# Optional examples
if [ "$INCLUDE_EXAMPLES" = true ]; then
  echo "Including examples..."
  if [ -d "examples" ]; then
    mkdir -p "$PACKAGE_DIR/examples"
    cp -r examples/* "$PACKAGE_DIR/examples/"
  else
    echo "Warning: No examples directory found."
  fi
fi

# Create package archive
echo "Creating package archive..."
(cd "$OUTPUT_DIR" && tar -czf "${PACKAGE_NAME}.tar.gz" "$PACKAGE_NAME")

# Generate SHA256 checksum
echo "Generating checksum..."
if command -v sha256sum &>/dev/null; then
  (cd "$OUTPUT_DIR" && sha256sum "${PACKAGE_NAME}.tar.gz" > "${PACKAGE_NAME}.tar.gz.sha256")
elif command -v shasum &>/dev/null; then
  (cd "$OUTPUT_DIR" && shasum -a 256 "${PACKAGE_NAME}.tar.gz" > "${PACKAGE_NAME}.tar.gz.sha256")
else
  echo "Warning: Neither sha256sum nor shasum found, skipping checksum generation."
fi

echo "Package created at $OUTPUT_DIR/${PACKAGE_NAME}.tar.gz"
echo "Packaging completed successfully." 