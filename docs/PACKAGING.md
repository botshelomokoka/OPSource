# OPSource Packaging Guide

This document explains how to package OPSource for distribution and how to use the packaged version.

## Package Structure

The OPSource package is a self-contained distribution of the project with all necessary files to run the application. The package follows this structure:

```
opsource-[version]/
├── dist/               # Compiled JavaScript/TypeScript
├── docs/               # Documentation
├── examples/           # Example code and usage
├── package.json        # Package metadata
└── README.md           # Basic information
```

## Creating Packages

### Automatic Packaging

The easiest way to create a package is using the npm scripts:

```bash
# Auto-detect platform and create package
npm run package

# Create package explicitly for Windows
npm run package:win

# Create package explicitly for Unix systems
npm run package:unix
```

### Manual Packaging

You can run the packaging scripts directly with additional options:

#### Windows (PowerShell)

```powershell
.\scripts\package.ps1 [options]
```

Options:
- `-version "x.x.x"` - Set package version (default: from VERSION file)
- `-outputDir "dir"` - Set output directory (default: ./packages)
- `-name "custom-name"` - Set custom package name
- `-debug` - Build in debug mode
- `-noDocs` - Skip including documentation
- `-noExamples` - Skip including examples

#### Unix (Bash)

```bash
./scripts/package.sh [options]
```

Options:
- `--version "x.x.x"` - Set package version
- `--output-dir "dir"` - Set output directory
- `--name "custom-name"` - Set custom package name
- `--debug` - Build in debug mode
- `--no-docs` - Skip including documentation
- `--no-examples` - Skip including examples

## Verifying Packages

### Verifying Package Checksums

Each package includes a SHA256 checksum file (.sha256) that can be used to verify the integrity of the package:

#### Windows (PowerShell)

```powershell
# Generate hash for downloaded package
$downloadedHash = Get-FileHash -Path "opsource-0.1.0.zip" -Algorithm SHA256
$downloadedHash.Hash

# Compare with expected hash
$expectedHash = Get-Content "opsource-0.1.0.zip.sha256"
$downloadedHash.Hash -eq $expectedHash
```

#### Unix (Bash)

```bash
# Generate hash for downloaded package
sha256sum opsource-0.1.0.tar.gz

# Compare with expected hash
cat opsource-0.1.0.tar.gz.sha256
```

## Using Packaged Version

### Windows

```powershell
# Extract package
Expand-Archive -Path "opsource-0.1.0.zip" -DestinationPath "opsource"

# Navigate to extracted directory
cd opsource
```

### Unix

```bash
# Extract package
tar -xzf opsource-0.1.0.tar.gz

# Navigate to extracted directory
cd opsource-0.1.0
```

## Packaging Process

The packaging process performs these steps:

1. **Test** - Runs the test suite to ensure everything works
2. **Build** - Compiles TypeScript code to JavaScript
3. **Copy** - Gathers all necessary files
4. **Archive** - Creates a zip/tar.gz archive
5. **Checksum** - Generates a SHA256 checksum

## Troubleshooting

### Common Packaging Issues

- **Missing Dependencies**: Ensure all dependencies are installed with `npm install`
- **Build Failures**: Fix any TypeScript errors before packaging
- **Permission Issues**: Ensure you have write access to the output directory
- **Path Too Long**: Use shorter package names if encountering path length limits

### Package Verification Failures

If checksum verification fails:
1. Check if the download completed properly
2. Verify you're using the correct checksum file for the package
3. Download the package again from a trusted source

## Release Process

1. Update version in VERSION file
2. Update CHANGELOG.md with new version information
3. Run the package script
4. Verify the package by extracting and testing it
5. Commit the package and push to GitHub
6. Update documentation to point to the new package

## Related Documentation

- [Getting Started Guide](./GETTING_STARTED.md)
- [System Map](./system_map.md)
- [Release Procedures](./DEPLOYMENT.md) 