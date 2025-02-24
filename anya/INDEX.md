# Anya Core

## Overview

Anya Core is the main framework of the project, providing essential services and integrations.

## Components

- [Bitcoin Integration](./anya-bitcoin/README.md)
- [Enterprise Core](./anya-enterprise/README.md)
- [Extensions](./anya-extensions/README.md)

## Documentation

- [System Architecture](./SYSTEM_MAP.md)
- [API Reference](./docs/api/README.md)
- [Development Guide](./docs/development/README.md)

## Configuration

- [Environment Setup](./.env.template)
- [Docker Configuration](./docker-compose.yml)
- [Build Configuration](./build.rs)

## Testing

- [Test Suite](./tests/README.md)
- [Benchmarks](./benches/README.md)
- [Integration Tests](./tests/integration/README.md)

## Scripts

- [Development Scripts](./scripts/README.md)
- [Build Scripts](./scripts/build/README.md)
- [Deployment Scripts](./scripts/deploy/README.md)

## Related Components

- [Dashboard (Dash33)](../dash33/INDEX.md)
- [Enterprise Implementation](../enterprise/INDEX.md)
- [Mobile Application](../mobile/INDEX.md)

## Core Modules (Enhanced)

- **Bitcoin Protocol Stack**
  - [Base Layer](./anya-bitcoin/README.md) (rust-bitcoin + BDK)
  - [Lightning Network](./anya-bitcoin/lightning.md) (LDK with DLC support)
  - [Taproot Services](./anya-bitcoin/taproot.md) (P2TR & Taproot Assets)
  - [Sidechain Bridges](./anya-bitcoin/sidechains.md) (RSK merge-mining)

## AI Systems (Enhanced)

- **Network Agents**
  - [Transaction Graph Analyzer](./ai/network/tx_analysis.md) (UTXO clustering)
  - [Fee Market Predictor](./ai/network/fee_engine.md) (Mempool CNN-LSTM)
  - [Privacy Guardian](./ai/security/privacy.md) (CoinJoin simulator)

- **Financial Agents**
  - [DLC Oracle Engine](./ai/finance/dlc_oracle.md) (Federated learning)
  - [RGB Asset Manager](./ai/finance/rgb_agent.md) (Contract-state ML)

## Enhanced Integration Points

1. **Taproot-DLC Workflow**:
   - Schnorr-based oracle signatures
   - Adaptor signatures for contract execution
   - CTV-encoded settlement conditions

2. **RSK Bridge**:
   - Powpeg monitoring agent
   - Two-way peg transaction optimizer
   - EVM contract ↔ Bitcoin DLC mapper

3. **Mobile Integration**:
   - React Native HSM module
   - PSBT co-signing workflow
   - Lightning microservices

*Last updated:  2025-02-24*
