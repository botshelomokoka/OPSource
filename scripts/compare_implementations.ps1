# PowerShell script to compare Python and Rust Bitcoin implementations

# Ensure we're in the project root
$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location (Join-Path $scriptPath "..")

Write-Host "==========================================="
Write-Host "🔍 Bitcoin Implementation Comparison Tool"
Write-Host "==========================================="

# Build with both implementations
Write-Host "`n🔨 Building with both implementations..."
cargo build --features full

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Build failed!"
    exit 1
}

# Create log directory
New-Item -ItemType Directory -Force -Path logs | Out-Null

# Run in shadow mode with Python as primary
Write-Host "`n🐍 Running in shadow mode with Python as primary..."
$env:SHADOW_MODE = "true"
$env:PRIMARY_IMPL = "python"
$env:LOG_FILE = "logs/shadow_python_$(Get-Date -Format 'yyyyMMdd_HHmmss').log"
cargo run -- test

# Run in shadow mode with Rust as primary
Write-Host "`n🦀 Running in shadow mode with Rust as primary..."
$env:SHADOW_MODE = "true"
$env:PRIMARY_IMPL = "rust"
$env:LOG_FILE = "logs/shadow_rust_$(Get-Date -Format 'yyyyMMdd_HHmmss').log"
cargo run -- test

# Analyze the logs
Write-Host "`n📊 Analyzing comparison logs..."
python scripts/analyze_comparison_logs.py logs/

Write-Host "`n✅ Comparison completed!"
Write-Host "Check the logs directory for detailed comparison results." 