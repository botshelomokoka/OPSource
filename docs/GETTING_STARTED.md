# Getting Started with OPSource

## Prerequisites

### Required Tools

- Node.js v18 or later
- Git 2.x or later
- VS Code with extensions:
  - GitLens
  - Clarity for VS Code
  - npm
  - ESLint

### Optional Tools

- Docker Desktop for Windows
- PowerShell 7+ (recommended)

## Installation

1.Clone the repository:

```powershell
git clone https://github.com/botshelomokoka/OPSource.git
cd OPSource
```

2.Set up environment:

```powershell
# Create environment files
./scripts/setup_env.sh development

# Install dependencies
npm install
```

3.Initialize project:

```powershell
# Setup VS Code configuration
./scripts/create-vscode-settings.ps1

# Setup development environment
npm run setup
```

## Project Structure

```plaintext
OPSource/
├── .vscode/                    # VS Code configuration
├── scripts/                   # Setup and utility scripts
├── src/                      # Source code
│   └── core/                # Core implementations
│       ├── dao/            # DAO implementation
│       └── shared/        # Shared utilities
├── docs/                     # Documentation
└── tests/                    # Integration tests
```

## Quick Start

### Development Setup

```typescript
// Start development server
npm run dev

// Run tests
npm test

// Check contracts
npm run check
```

### DAO Contract Integration

```clarity
;; Initialize DAO contract
(contract-call? .dao-core initialize
  "OPSource DAO"
  u1000000
  u100)
```

## User Roles

### Administrator

- Environment configuration
- Contract deployment
- System monitoring

### Developer

- Contract development
- Integration testing
- Documentation updates

### User

- Contract interaction
- Proposal submission
- Voting participation

## Common Tasks

### Managing Environment

```powershell
# Switch to development environment
./scripts/setup_env.sh development

# Switch to production
./scripts/setup_env.sh production
```

### Running Tests

```powershell
# Unit tests
npm run test:unit

# Integration tests
npm run test:integration

# Contract tests
npm run test:contracts
```

### Documentation Updates

```powershell
# Generate documentation
npm run docs

# Serve documentation locally
npm run docs:serve
```

## Troubleshooting

### Common Issues

1. Environment Setup
   - Verify Node.js installation: `node --version`
   - Check npm installation: `npm --version`
   - Confirm environment files exist

2. Build Issues
   - Clear npm cache: `npm cache clean --force`
   - Remove node_modules: `rm -r node_modules`
   - Reinstall dependencies: `npm install`

3. Contract Errors
   - Verify Clarity version
   - Check contract syntax
   - Review error messages in VS Code

## Next Steps

1. Review [system_map.md](./system_map.md)
2. Study [Integration Patterns](./INTEGRATION_PATTERNS.md)
3. Follow [Development Roadmap](./roadmap.md)
4. Join developer discussions
5. Start contributing
