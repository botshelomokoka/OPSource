# PowerShell script for documentation maintenance
param (
    [string]$rootDir = (Get-Location),
    [switch]$verify = $false,
    [switch]$update = $false,
    [switch]$createMissing = $false
)

# Template content for different documentation types
$templates = @{
    "README.md" = @"
# Component Name

## Overview
Brief description of the component and its purpose.

## Quick Start
Steps to get started with this component.

## Features
List of key features.

## Documentation
Links to detailed documentation.

*Last updated: $(Get-Date -Format 'yyyy-MM-dd')*
"@

    "INDEX.md" = @"
# Documentation Index

## Getting Started
- [Installation Guide](./docs/getting-started/installation.md)
- [Quick Start Guide](./docs/getting-started/quick-start.md)
- [Configuration Guide](./docs/getting-started/configuration.md)

## User Guide
- [Basic Usage](./docs/guides/basic-usage.md)
- [Advanced Features](./docs/guides/advanced-features.md)
- [Troubleshooting](./docs/guides/troubleshooting.md)

## API Reference
- [API Overview](./docs/api/overview.md)
- [Endpoints](./docs/api/endpoints.md)
- [Authentication](./docs/api/authentication.md)

## Development
- [Architecture](./docs/architecture/overview.md)
- [Contributing](./CONTRIBUTING.md)
- [Security](./SECURITY.md)

*Last updated: $(Get-Date -Format 'yyyy-MM-dd')*
"@

    "SECURITY.md" = @"
# Security Policy

## Supported Versions
List of currently supported versions with security updates.

## Reporting a Vulnerability
Instructions for reporting security vulnerabilities.

## Security Features
Overview of security features and best practices.

*Last updated: $(Get-Date -Format 'yyyy-MM-dd')*
"@
}

# Required documentation files for each component
$requiredFiles = @(
    "README.md",
    "INDEX.md",
    "CHANGELOG.md",
    "CONTRIBUTING.md",
    "SECURITY.md"
)

# Required documentation directories
$requiredDirs = @(
    "docs/architecture",
    "docs/api",
    "docs/guides",
    "docs/features",
    "docs/security",
    "docs/development",
    "docs/integration"
)

function Create-RequiredStructure {
    param (
        [string]$componentPath
    )
    
    # Create directories
    foreach ($dir in $requiredDirs) {
        $path = Join-Path $componentPath $dir
        if (-not (Test-Path $path)) {
            New-Item -ItemType Directory -Path $path -Force | Out-Null
            Write-Host "Created directory: $path"
        }
    }
    
    # Create files
    foreach ($file in $requiredFiles) {
        $path = Join-Path $componentPath $file
        if (-not (Test-Path $path)) {
            if ($templates.ContainsKey($file)) {
                Set-Content -Path $path -Value $templates[$file]
            }
            else {
                Set-Content -Path $path -Value "# $file`n`n*Last updated: $(Get-Date -Format 'yyyy-MM-dd')*"
            }
            Write-Host "Created file: $path"
        }
    }
}

function Update-Timestamps {
    param (
        [string]$path
    )
    
    $currentDate = Get-Date -Format "yyyy-MM-dd"
    Get-ChildItem -Path $path -Filter *.md -Recurse | ForEach-Object {
        $content = Get-Content $_.FullName -Raw
        if ($content -match "Last updated: \d{4}-\d{2}-\d{2}") {
            $updatedContent = $content -replace "Last updated: \d{4}-\d{2}-\d{2}", "Last updated: $currentDate"
            Set-Content -Path $_.FullName -Value $updatedContent
            Write-Host "Updated timestamp in: $($_.FullName)"
        }
        elseif ($content -notmatch "Last updated:") {
            Add-Content -Path $_.FullName -Value "`n*Last updated: $currentDate*"
            Write-Host "Added timestamp to: $($_.FullName)"
        }
    }
}

function Verify-Documentation {
    param (
        [string]$path
    )
    
    $issues = @()
    
    # Check required files
    foreach ($file in $requiredFiles) {
        $filePath = Join-Path $path $file
        if (-not (Test-Path $filePath)) {
            $issues += "Missing required file: $file in $path"
        }
    }
    
    # Check required directories
    foreach ($dir in $requiredDirs) {
        $dirPath = Join-Path $path $dir
        if (-not (Test-Path $dirPath)) {
            $issues += "Missing required directory: $dir in $path"
        }
    }
    
    # Check for timestamps
    Get-ChildItem -Path $path -Filter *.md -Recurse | ForEach-Object {
        $content = Get-Content $_.FullName -Raw
        if (-not ($content -match "Last updated: \d{4}-\d{2}-\d{2}")) {
            $issues += "Missing timestamp in: $($_.FullName)"
        }
    }
    
    return $issues
}

# Main execution
$components = @("anya", "dash33", "enterprise", "mobile")
foreach ($component in $components) {
    $componentPath = Join-Path $rootDir $component
    if (Test-Path $componentPath) {
        Write-Host "`nProcessing component: $component"
        
        if ($createMissing) {
            Write-Host "Creating required documentation structure..."
            Create-RequiredStructure -componentPath $componentPath
        }
        
        if ($verify) {
            Write-Host "Verifying documentation..."
            $issues = Verify-Documentation -path $componentPath
            if ($issues.Count -gt 0) {
                Write-Host "Found issues:"
                $issues | ForEach-Object { Write-Warning $_ }
            }
            else {
                Write-Host "No issues found."
            }
        }
        
        if ($update) {
            Write-Host "Updating documentation timestamps..."
            Update-Timestamps -path $componentPath
        }
    }
    else {
        Write-Warning "Component directory not found: $componentPath"
    }
}

Write-Host "`nDocumentation maintenance complete."
