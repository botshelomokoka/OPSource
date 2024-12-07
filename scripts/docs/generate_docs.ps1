# PowerShell script for documentation management
param (
    [string]$rootDir = (Get-Location),
    [string]$component = "all",
    [switch]$verify = $false,
    [switch]$update = $false
)

# Function to create directory if it doesn't exist
function Ensure-Directory {
    param (
        [string]$path
    )
    if (-not (Test-Path $path)) {
        New-Item -ItemType Directory -Path $path -Force | Out-Null
        Write-Host "Created directory: $path"
    }
}

# Function to create documentation structure
function Create-DocStructure {
    param (
        [string]$basePath,
        [string]$componentName
    )
    
    $docPaths = @(
        "docs/architecture",
        "docs/api",
        "docs/guides",
        "docs/examples",
        "docs/security",
        "docs/development",
        "docs/deployment"
    )

    foreach ($path in $docPaths) {
        Ensure-Directory (Join-Path $basePath $path)
    }

    # Create basic documentation files
    $files = @{
        "README.md" = "# $componentName`n`n## Overview`n`nDescription of $componentName component.`n`n## Quick Start`n`n## Features`n`n## Documentation`n`n*Last updated: $(Get-Date -Format 'yyyy-MM-dd')*"
        "INDEX.md" = "# $componentName Documentation Index`n`n## Guides`n`n## API Reference`n`n## Architecture`n`n*Last updated: $(Get-Date -Format 'yyyy-MM-dd')*"
        "CHANGELOG.md" = "# Changelog`n`n## [Unreleased]`n`n### Added`n`n### Changed`n`n### Deprecated`n`n### Removed`n`n### Fixed`n`n### Security`n`n*Last updated: $(Get-Date -Format 'yyyy-MM-dd')*"
    }

    foreach ($file in $files.GetEnumerator()) {
        $filePath = Join-Path $basePath $file.Key
        if (-not (Test-Path $filePath)) {
            Set-Content -Path $filePath -Value $file.Value
            Write-Host "Created file: $filePath"
        }
    }
}

# Function to verify documentation
function Verify-Documentation {
    param (
        [string]$path
    )
    
    $issues = @()
    
    # Check for required files
    $requiredFiles = @("README.md", "INDEX.md", "CHANGELOG.md")
    foreach ($file in $requiredFiles) {
        $filePath = Join-Path $path $file
        if (-not (Test-Path $filePath)) {
            $issues += "Missing required file: $file in $path"
        }
    }
    
    # Check for broken internal links
    Get-ChildItem -Path $path -Filter *.md -Recurse | ForEach-Object {
        $content = Get-Content $_.FullName -Raw
        $matches = [regex]::Matches($content, '\[([^\]]+)\]\(([^)]+)\)')
        foreach ($match in $matches) {
            $link = $match.Groups[2].Value
            if (-not $link.StartsWith("http")) {
                $linkedPath = Join-Path (Split-Path $_.FullName) $link.TrimStart("./")
                if (-not (Test-Path $linkedPath)) {
                    $issues += "Broken link in $($_.Name): $link"
                }
            }
        }
    }
    
    # Check for last updated timestamp
    Get-ChildItem -Path $path -Filter *.md -Recurse | ForEach-Object {
        $content = Get-Content $_.FullName -Raw
        if (-not ($content -match "Last updated: \d{4}-\d{2}-\d{2}")) {
            $issues += "Missing last updated timestamp in $($_.Name)"
        }
    }
    
    return $issues
}

# Function to update documentation
function Update-Documentation {
    param (
        [string]$path
    )
    
    $currentDate = Get-Date -Format "yyyy-MM-dd"
    
    Get-ChildItem -Path $path -Filter *.md -Recurse | ForEach-Object {
        $content = Get-Content $_.FullName -Raw
        if ($content -match "Last updated: \d{4}-\d{2}-\d{2}") {
            $updatedContent = $content -replace "Last updated: \d{4}-\d{2}-\d{2}", "Last updated: $currentDate"
            Set-Content -Path $_.FullName -Value $updatedContent
            Write-Host "Updated timestamp in: $($_.Name)"
        }
    }
}

# Main execution
Write-Host "Documentation Management Script"
Write-Host "============================="

$components = @("anya", "dash33", "enterprise", "mobile")
if ($component -ne "all") {
    $components = @($component)
}

foreach ($comp in $components) {
    $compPath = Join-Path $rootDir $comp
    Write-Host "`nProcessing component: $comp"
    
    if ($verify) {
        Write-Host "Verifying documentation..."
        $issues = Verify-Documentation -path $compPath
        if ($issues.Count -gt 0) {
            Write-Host "Found issues:"
            $issues | ForEach-Object { Write-Warning $_ }
        } else {
            Write-Host "No issues found."
        }
    }
    
    if ($update) {
        Write-Host "Updating documentation..."
        Update-Documentation -path $compPath
    } else {
        Write-Host "Creating documentation structure..."
        Create-DocStructure -basePath $compPath -componentName $comp
    }
}

Write-Host "`nDocumentation management complete."
