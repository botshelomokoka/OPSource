# OPSource

Research and development platform for the Anya Core Bitcoin implementation.

## Latest Updates

- **February 25, 2025**: First package release (v0.1.0) now available in the [packages](./packages) directory
- SHA256 checksum verification included for security
- Updated documentation with packaging instructions

## Project Structure

```
OPSource/
├── src/
│   └── core/
│       ├── dao/          # DAO implementation
│       │   ├── contracts/
│       │   ├── tests/
│       │   └── traits/
│       └── shared/       # Shared utilities
│           └── utils/
├── packages/            # Distribution packages
├── scripts/             # Build and deployment scripts
├── docs/               # Documentation
└── tests/              # Integration tests
```

## Key Features

- Bitcoin protocol research
- Experimental feature development
- Performance analysis tools
- Security testing frameworks
- Development utilities

## Development

### Prerequisites

- Node.js 18+
- Git
- PowerShell 7+

### Setup

1. Clone with submodules:

```powershell
git clone --recursive https://github.com/botshelomokoka/OPSource.git
cd OPSource
```

2. Install dependencies:

```powershell
npm install
```

3. Initialize environment:

```powershell
.\scripts\setup-project.ps1
```

### Available Scripts

- `npm test` - Run tests
- `npm run check` - Check contracts
- `npm run build` - Build project
- `npm run dev` - Watch mode
- `npm run package` - Create a distribution package (auto-detects platform)
- `npm run package:win` - Create a package on Windows
- `npm run package:unix` - Create a package on Unix-based systems

### Packaging

You can create a distribution package using:

```powershell
# On Windows
npm run package:win

# On Unix-based systems
npm run package:unix
```

This will:
1. Run tests to verify everything works
2. Build the project
3. Create a package in the `packages` directory
4. Generate a SHA256 checksum

#### Packaging Options

The packaging scripts accept several options:

- `--version` - Set custom version (default: from VERSION file)
- `--output-dir` - Set output directory (default: ./packages)
- `--name` - Set package name (default: opsource-[version])
- `--debug` - Build in debug mode (default: release)
- `--no-docs` - Exclude documentation
- `--no-examples` - Exclude examples

For example:

```powershell
# On Windows
.\scripts\package.ps1 -version "1.0.0" -outputDir "dist"

# On Unix-based systems
./scripts/package.sh --version "1.0.0" --output-dir "dist"
```

### Using the Packaged Version

See [Getting Started](./docs/GETTING_STARTED.md) for instructions on using the packaged version.

## Related Projects

- [Anya Core](https://github.com/botshelomokoka/anya-core) - Main Bitcoin implementation

## Documentation

- [System Map](./docs/system_map.md) - System architecture and roadmap
- [Getting Started](./docs/GETTING_STARTED.md) - Detailed setup instructions
- [Changelog](./CHANGELOG.md) - Version history and updates

## Contributing

Please read our [Contributing Guidelines](CONTRIBUTING.md) before submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
