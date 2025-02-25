param(
    [string]$version = "",
    [string]$outputDir = "",
    [string]$name = "",
    [switch]$debug,
    [switch]$noDocs,
    [switch]$noExamples
)

# OPSource Packaging Script for Windows PowerShell

# Default variables
$DefaultVersion = if (Test-Path "VERSION") { Get-Content "VERSION" } else { "0.1.0" }
$DefaultPackageName = "opsource-$DefaultVersion"
$DefaultOutputDir = "./packages"
$BuildMode = "release"
$IncludeDocs = $true
$IncludeExamples = $true

# Apply parameter values or use defaults
$Version = if ([string]::IsNullOrEmpty($version)) { $DefaultVersion } else { $version }
$PackageName = if ([string]::IsNullOrEmpty($name)) { "opsource-$Version" } else { $name }
$OutputDir = if ([string]::IsNullOrEmpty($outputDir)) { $DefaultOutputDir } else { $outputDir }

if ($debug) { $BuildMode = "debug" }
if ($noDocs) { $IncludeDocs = $false }
if ($noExamples) { $IncludeExamples = $false }

Write-Host "=== OPSource Packaging Script ===" -ForegroundColor Cyan
Write-Host "Version: $Version" -ForegroundColor Cyan
Write-Host "Package name: $PackageName" -ForegroundColor Cyan
Write-Host "Output directory: $OutputDir" -ForegroundColor Cyan
Write-Host "Build mode: $BuildMode" -ForegroundColor Cyan
Write-Host "Include docs: $IncludeDocs" -ForegroundColor Cyan
Write-Host "Include examples: $IncludeExamples" -ForegroundColor Cyan
Write-Host "==================================" -ForegroundColor Cyan

# Make sure we're in the project root
if (-not (Test-Path "package.json")) {
    Write-Host "Error: This script must be run from the project root directory." -ForegroundColor Red
    exit 1
}

# Run tests first
Write-Host "Running tests..." -ForegroundColor Yellow
npm test
if ($LASTEXITCODE -ne 0) {
    Write-Host "Tests failed, aborting packaging process." -ForegroundColor Red
    exit 1
}

# Build the project
Write-Host "Building project..." -ForegroundColor Yellow
npm run build
if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed, aborting packaging process." -ForegroundColor Red
    exit 1
}

# Create output directory
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir | Out-Null
}
$PackageDir = Join-Path -Path $OutputDir -ChildPath $PackageName
if (-not (Test-Path $PackageDir)) {
    New-Item -ItemType Directory -Path $PackageDir | Out-Null
}

# Copy essential files
Write-Host "Copying files to package directory..." -ForegroundColor Yellow
Copy-Item -Path "dist" -Destination "$PackageDir\" -Recurse -Force
Copy-Item -Path "package.json" -Destination "$PackageDir\" -Force
Copy-Item -Path "README.md" -Destination "$PackageDir\" -Force
if (Test-Path "LICENSE") {
    Copy-Item -Path "LICENSE" -Destination "$PackageDir\" -Force
} else {
    Write-Host "No LICENSE file found, skipping." -ForegroundColor Yellow
}
if (Test-Path "CHANGELOG.md") {
    Copy-Item -Path "CHANGELOG.md" -Destination "$PackageDir\" -Force
} else {
    Write-Host "No CHANGELOG.md file found, skipping." -ForegroundColor Yellow
}

# Optional docs
if ($IncludeDocs) {
    Write-Host "Including documentation..." -ForegroundColor Yellow
    if (Test-Path "docs") {
        if (-not (Test-Path "$PackageDir\docs")) {
            New-Item -ItemType Directory -Path "$PackageDir\docs" | Out-Null
        }
        Copy-Item -Path "docs\*" -Destination "$PackageDir\docs\" -Recurse -Force
    } else {
        Write-Host "Warning: No docs directory found." -ForegroundColor Yellow
        # Generate API docs if possible
        if (Get-Command "npx" -ErrorAction SilentlyContinue) {
            Write-Host "Generating API documentation with TypeDoc..." -ForegroundColor Yellow
            npx typedoc --out "$PackageDir\docs\api" src\
        }
    }
}

# Optional examples
if ($IncludeExamples) {
    Write-Host "Including examples..." -ForegroundColor Yellow
    if (Test-Path "examples") {
        if (-not (Test-Path "$PackageDir\examples")) {
            New-Item -ItemType Directory -Path "$PackageDir\examples" | Out-Null
        }
        Copy-Item -Path "examples\*" -Destination "$PackageDir\examples\" -Recurse -Force
    } else {
        Write-Host "Warning: No examples directory found." -ForegroundColor Yellow
    }
}

# Create package archive
Write-Host "Creating package archive..." -ForegroundColor Yellow
$ArchivePath = Join-Path -Path $OutputDir -ChildPath "$PackageName.zip"
Compress-Archive -Path "$PackageDir\*" -DestinationPath $ArchivePath -Force

# Generate SHA256 checksum
Write-Host "Generating checksum..." -ForegroundColor Yellow
$FileHash = Get-FileHash -Path $ArchivePath -Algorithm SHA256
$FileHash.Hash | Out-File "$ArchivePath.sha256" -Encoding utf8

Write-Host "Package created at $ArchivePath" -ForegroundColor Green
Write-Host "Packaging completed successfully." -ForegroundColor Green 