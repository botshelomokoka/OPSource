# Getting Started with OPSource

This guide will help you set up and start using OPSource for blockchain analytics.

## Prerequisites

- Python 3.11 or higher
- Git
- Virtual environment (recommended)

## Installation Steps

1. **Clone the Repository**
   ```bash
   git clone --recursive https://github.com/botshelomokoka/OPSource.git
   cd OPSource
   ```

2. **Set Up Python Environment**
   ```bash
   python -m venv venv
   source venv/bin/activate  # On Windows: venv\Scripts\activate
   pip install -r requirements.txt
   ```

3. **Configure Environment**
   ```bash
   cp config/example.env .env
   # Edit .env with your configuration settings
   ```

4. **Verify Installation**
   ```bash
   python scripts/track_development.py
   ```

## Basic Usage

1. **View Analytics Dashboard**
   ```bash
   python -m dash33.main
   ```

2. **Generate Reports**
   ```bash
   python scripts/generate_reports.py
   ```

## Next Steps

- Read the [API Documentation](api.md)
- Check out our [Contributing Guidelines](../CONTRIBUTING.md)
- Join our [Discord Community](https://discord.gg/opsource)
