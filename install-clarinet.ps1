# Install Clarinet SDK and tools
$ErrorActionPreference = 'Stop'

Write-Host "Installing Clarinet and dependencies..." -ForegroundColor Cyan

# First, ensure npm is in PATH
$env:PATH = $env:PATH + ";$env:USERPROFILE\AppData\Roaming\npm"

# Install Clarinet SDK globally
npm install -g @hirosystems/clarinet-sdk
npm install -g @hirosystems/clarity-lsp

# Verify installations
Write-Host "
Verifying installations..." -ForegroundColor Cyan
npm list -g @hirosystems/clarinet-sdk
npm list -g @hirosystems/clarity-lsp

# Initialize Clarinet project
Write-Host "
Initializing Clarinet project..." -ForegroundColor Cyan
mkdir anya-dao
cd anya-dao
npx @hirosystems/clarinet-sdk init

Write-Host "
Setup complete!" -ForegroundColor Green
