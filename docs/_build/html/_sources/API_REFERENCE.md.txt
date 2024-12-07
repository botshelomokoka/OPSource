# OPSource API Reference

## Core Components

### Anya Enterprise

#### Main System APIs
- `initialize_modules()`: Initializes core system modules including network, ML, cryptocurrency, and analytics components
- `ProjectSetup`: Main configuration and setup interface for the Anya system

#### Advanced Analytics
- `AdvancedAnalytics`: Core analytics engine for processing blockchain and market data
- `AdvancedBitcoinPricePredictor`: ML-based price prediction system

#### Blockchain Integration
- `BitcoinSupport`: Bitcoin network integration
- `LightningSupport`: Lightning Network capabilities  
- `DLCSupport`: Discreet Log Contract functionality
- `STXSupport`: Stacks blockchain integration

#### Machine Learning
- `MLCore`: Core machine learning engine
- `MLInput/MLOutput`: Standard ML data interfaces
- `AIModelUpgrader`: Automated model improvement system

#### Networking
- `NetworkDiscovery`: P2P network discovery and management
- `UnifiedNetworkManager`: Cross-chain network coordination
- `Libp2pSupport`: libp2p protocol integration

### Dash33

[Documentation in progress]

### Enterprise

[Documentation in progress]

### Mobile

[Documentation in progress]

## Common Interfaces

### User Management
- `UserManagement`: User authentication and permission management
- `UserType`: User role definitions (Creator, Developer, Normal)
- `UserMetrics`: User activity and performance tracking

### Data Management  
- `DataFeed`: Standard data feed interface
- `DataSource`: Data source configuration
- `MarketDataFetcher`: Market data acquisition

### Development Tools
- `LibraryVersionManager`: Dependency version management
- `ResearchPaperDatabase`: ML research tracking
- `HistoricalDataAnalyzer`: Historical data analysis tools

## Configuration

### Environment Variables
- `GITHUB_TOKEN`: GitHub API authentication
- `STX_CONFIG`: Stacks blockchain configuration
- `GIT_AUTH`: Git authentication settings

### Project Structure
```
anya-core/
├── src/
│   ├── ml_logic/
│   ├── network_discovery/
│   ├── main_system/
│   ├── stx_support/
│   ├── dlc_support/
│   ├── lightning_support/
│   ├── bitcoin_support/
│   └── web5_support/
├── tests/
├── admin_tools/
├── dev_env/
└── wallets/
    ├── stx/
    ├── dlc/
    ├── lightning/
    └── bitcoin/
```
