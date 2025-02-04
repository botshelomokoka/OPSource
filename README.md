# OPSource Development Environment

This is a development environment configured for AI and Bitcoin development following core principles and best practices.

## Setup Instructions

1. Ensure Python 3.11+ is installed
2. Run the setup script:
   ```bash
   python setup_dev.py
   ```

## Port Configuration

The development environment uses the following ports:

- **API Server**: 8000
- **Model Service**: 8001
- **Inference Service**: 8002
- **Development Server**: 3000
- **Bitcoin RPC (testnet)**: 18332
- **RSK Node**: 4444
- **RGB Node**: 3000
- **Database**: 5432
- **Test Server**: 5000

## Environment Structure

```
OPSource-Dev/
├── .venv/                 # Virtual environment
├── certs/                 # SSL certificates for development
├── logs/                  # Application logs
├── requirements.txt       # Python dependencies
├── dev_config.py         # Development configuration
├── setup_dev.py          # Setup script
└── README.md             # This file
```

## Development Guidelines

- All development follows Bitcoin core principles
- Uses established libraries (python-bitcoinlib, rust-bitcoin)
- Supports Taproot and DLC implementations
- Integrates with RGB and Stacks for smart contracts
- Implements comprehensive testing
- Maintains security and privacy best practices

## Security Notes

- Development SSL certificates are for testing only
- Bitcoin testnet is used by default
- All ports are configured for local development only
