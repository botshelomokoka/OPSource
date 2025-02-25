# OPSource Project TODO

Last Updated: March 1, 2025

## Core Bitcoin Infrastructure

### Bitcoin Implementation
- [x] Create dual implementation architecture 
- [x] Implement Python wrapper (python-bitcoinlib)
- [x] Implement basic rust-bitcoin functionality
- [x] Create common interface for both implementations
- [x] Complete BDK wallet for key management
- [x] Make Rust the default implementation
- [ ] Implement local storage for wallet data
- [ ] Add PSBT support for cold storage

### Lightning Network
- [ ] Integrate LDK for Lightning support
- [ ] Implement Lightning channels management
- [ ] Create Lightning payment interfaces
- [ ] Add BOLT 12 offers support
- [ ] Implement route finding algorithms

### DLC Support
- [ ] Add basic Discrete Log Contract support
- [ ] Implement oracle interfaces
- [ ] Create contract templates

## Web5 Integration
- [ ] Implement DID methods
- [ ] Create DWAS support
- [ ] Add decentralized data storage
- [ ] Build secure messaging protocol

## ML/AI Components
- [ ] Implement federated learning models
- [ ] Create privacy-preserving data structures
- [ ] Build model distribution mechanism

## Enterprise Integration
- [ ] Create authentication interfaces
- [ ] Implement compliance reporting
- [ ] Add multi-signature governance
- [ ] Build audit trail functionality

## Next Steps Priority List
1. Complete Lightning Network integration with LDK
2. Implement persistent storage for wallet data
3. Add PSBT support for hardware wallet integration
4. Develop DLC oracle integration
5. Add Taproot support for enhanced privacy

## Integration Timeline
| Component | Target Date | Status |
|-----------|-------------|--------|
| Bitcoin Core Interface | 2025-02-15 | COMPLETED |
| Rust Implementation | 2025-03-01 | COMPLETED |
| Lightning Network | 2025-04-15 | IN PROGRESS |
| DLC Implementation | 2025-06-01 | PLANNED |
| Web5 Integration | 2025-07-15 | PLANNING |
| ML Components | 2025-09-01 | RESEARCH |
| Enterprise Features | 2025-10-15 | RESEARCH |

## Component Status Tracking
| Component | Operational | Development | Testing | Documentation |
|-----------|-------------|-------------|---------|---------------|
| Bitcoin Core | ✅ | ✅ | ✅ | ✅ |
| Lightning | 🔄 | 🔄 | ❌ | 🔄 |
| BDK Wallet | ✅ | ✅ | ✅ | ✅ |
| Web5 | ❌ | 🔄 | ❌ | 🔄 |
| ML/AI | ❌ | ❌ | ❌ | 🔄 |
| Mobile | ❌ | 🔄 | ❌ | ❌ |
| Enterprise | ❌ | ❌ | ❌ | 🔄 |
| DAO | ❌ | ❌ | ❌ | 🔄 |

## Priority Tasks for anya-core Operational Status

### Critical (Immediate Implementation)
1. [x] Complete project infrastructure setup
2. [x] Implement core Bitcoin protocol interfaces
   - [x] Connect Bitcoin module stubs with implementation
   - [x] Add UTXO management and coin selection
   - [x] Implement transaction signing flow
   - [ ] Replace python-bitcoinlib dependencies with Rust libraries
3. [ ] Implement Web5 DWN integration
   - [ ] Implement DID resolver
   - [ ] Create DWN schema for blockchain data
4. [ ] Setup ML pipeline foundations
   - [ ] Implement ML service connectors
   - [ ] Setup secure model data flow
5. [ ] Create minimal viable enterprise features
   - [ ] Implement repository layer
   - [ ] Setup metrics collection

### High Priority (Required for MVP)
1. [ ] Formalize port interfaces for hexagonal architecture [@arch-team]
   - Deadline: 2025-Q1
   - Points: 13
   - Focus: Bitcoin protocol adapters

2. [ ] Implement DID rotation system [@web-team]
   - Interval: 90 days
   - Method: did:key rotation
   - Critical for security compliance

3. [ ] Set up federated node environment [@ml-team]
   - Nodes: 5 geo-distributed
   - TEE verification
   - Required for decentralized ML

4. [x] Migrate Bitcoin functionality to Rust libraries [@bitcoin-team]
   - [x] Create dual implementation architecture
   - [x] Implement python-bitcoinlib wrapper
   - [x] Implement rust-bitcoin basic functionality
   - [x] Create common interface for both implementations
   - [ ] Complete BDK wallet for key management
   - [ ] Integrate with LDK for Lightning support
   - Deadline: 2025-Q1

### Medium Priority (Post-MVP Enhancement)
1. [ ] Implement sBTC mint/burn listeners [@stacks-team]
   - Deadline: 2025-Q1
   - Depends: Nakamoto testnet

2. [ ] Develop wrapped sBTC ↔ RGB bridge
   - Points: 8
   - Requires: Bitcoin SPV proofs

3. [ ] Create sBTC liquidity monitor
   - Metric: Reserve ratio alerts
   - Threshold: <90% collateral

4. [ ] Create DWN schema registry [@protocol]
   - Types: Metrics, Proposals, PSBTs

5. [ ] Setup quadratic voting system [@dao-team]
   - Depends: Token distribution
   - Snapshot integration

6. [ ] Implement proposal lifecycle hooks [@devs]
   - Pre/post-execution checks
   - Time-locked changes

7. [ ] Develop model version registry [@devops]
   - IPFS-based storage
   - ZK-proofs of training

8. [ ] Complete DLC implementation [@bitcoin-team]
   - Implement oracle functionality
   - Add adaptor signature support
   - Integrate with Taproot contracts
   - Deadline: 2025-Q2

## Deployment and Packaging Tasks
1. [x] Create comprehensive test suite
   - [x] Unit tests for core functionality
   - [x] Integration tests for component interaction
   - [ ] Performance benchmarks
   - [x] Bitcoin-specific test vectors

2. [ ] Setup CI/CD pipeline
   - [ ] Build automation
   - [ ] Test automation
   - [ ] Deployment automation

3. [x] Create packaging scripts
   - [x] Generate release artifacts
   - [x] Create distribution packages
   - [x] Setup version management

4. [x] Documentation completion
   - [x] API reference
   - [x] Integration guides
   - [x] Installation instructions
   - [x] Bitcoin library usage examples
   - [x] Rust crate documentation

## Bitcoin-Web5 Bridge
1. [ ] Implement DID:BTCR resolver [@web5-team]
   - Depends: Bitcoin Core 25+
   - Points: 3

2. [ ] Create DWN schema for PSBT
   - Versioned PSBT templates
   - Multi-sig coordination

3. [ ] Develop DID auth for Lightning
   - LNURL-auth integration
   - Web5 credential assertions

## Bitcoin Rust Libraries Implementation
1. [x] Complete integration of main Rust libraries
   - [x] rust-bitcoin (v0.32.5): Core Bitcoin data structures
   - [x] BDK (v0.30.2): Basic wallet implementation
   - [ ] LDK (v0.0.116): Lightning Network implementation
   - [ ] RGB-core: Asset issuance on Bitcoin
   - [ ] Taproot: Advanced contract support

2. [x] Develop FFI bindings for legacy components
   - [x] Create Python ↔ Rust bridge functions
   - [x] Implement gradual migration strategy

*Last updated: 2025-03-05*