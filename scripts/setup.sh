#!/bin/bash

set -e

echo "Setting up OPSource development environment..."

# Check system requirements
check_requirements() {
    echo "Checking system requirements..."
    
    # Check for required commands
    commands=("git" "python3" "pip3" "cargo" "rustc" "flutter" "node" "npm")
    for cmd in "${commands[@]}"; do
        if ! command -v "$cmd" &> /dev/null; then
            echo "Error: $cmd is required but not installed."
            exit 1
        fi
    done
    
    # Check Python version
    python_version=$(python3 -c 'import sys; print(".".join(map(str, sys.version_info[:2])))')
    if (( $(echo "$python_version < 3.12" | bc -l) )); then
        echo "Error: Python 3.12 or higher is required."
        exit 1
    fi
    
    # Check Rust version
    rust_version=$(rustc --version | cut -d ' ' -f 2)
    if (( $(echo "$rust_version < 1.70.0" | bc -l) )); then
        echo "Error: Rust 1.70.0 or higher is required."
        exit 1
    fi
    
    # Check Flutter version
    flutter_version=$(flutter --version | head -n 1 | cut -d ' ' -f 2)
    if (( $(echo "$flutter_version < 3.16.0" | bc -l) )); then
        echo "Error: Flutter 3.16.0 or higher is required."
        exit 1
    fi
}

# Install Python dependencies
setup_python() {
    echo "Setting up Python environment..."
    
    # Create virtual environment
    python3 -m venv venv
    source venv/bin/activate
    
    # Install core dependencies
    pip install --upgrade pip
    pip install -r requirements.txt
    
    # Install development tools
    pip install pytest pytest-cov mypy black isort bandit safety
}

# Install Rust tools
setup_rust() {
    echo "Setting up Rust environment..."
    
    # Install additional components
    rustup component add rustfmt clippy
    
    # Install development tools
    cargo install cargo-audit
    cargo install cargo-tarpaulin
    cargo install cargo-watch
    cargo install cargo-expand
    cargo install cargo-udeps
}

# Install Flutter dependencies
setup_flutter() {
    echo "Setting up Flutter environment..."
    
    # Enable desktop support
    flutter config --enable-linux-desktop
    flutter config --enable-macos-desktop
    flutter config --enable-windows-desktop
    
    # Install dependencies
    flutter pub get
    
    # Run doctor
    flutter doctor
}

# Set up Git hooks
setup_git_hooks() {
    echo "Setting up Git hooks..."
    
    # Create pre-commit hook
    cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash

# Run formatters
echo "Running formatters..."
cargo fmt -- --check
black .
isort .

# Run linters
echo "Running linters..."
cargo clippy -- -D warnings
flake8
mypy .

# Run tests
echo "Running tests..."
cargo test
pytest
flutter test

# Check security
echo "Running security checks..."
cargo audit
safety check
EOF
    
    chmod +x .git/hooks/pre-commit
}

# Initialize metrics collection
setup_metrics() {
    echo "Setting up metrics collection..."
    
    # Create metrics directory
    mkdir -p metrics
    
    # Set up cron job for metrics collection
    (crontab -l 2>/dev/null; echo "0 * * * * $(pwd)/scripts/metrics.py") | crontab -
}

# Main setup process
main() {
    check_requirements
    
    # Update submodules
    git submodule update --init --recursive
    
    # Component-specific setup
    pushd anya-core/dash33
    setup_python
    setup_rust
    popd
    
    pushd anya-core/enterprise
    setup_rust
    popd
    
    pushd anya-core/mobile
    setup_flutter
    popd
    
    pushd anya-core/web5-rs
    setup_rust
    popd
    
    # Set up development tools
    setup_git_hooks
    setup_metrics
    
    echo "Setup complete! Development environment is ready."
}

main "$@"
