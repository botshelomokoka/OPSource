# OPSource

Research and development platform for the Anya Core Bitcoin implementation.

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

## Related Projects

- [Anya Core](https://github.com/botshelomokoka/anya) - Main Bitcoin implementation

## Contributing

Please read our [Contributing Guidelines](CONTRIBUTING.md) before submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
