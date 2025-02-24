## sBTC Preparation

1. Implement sBTC mint/burn listeners [@stacks-team]
   - Deadline: 2025-Q1
   - Depends: Nakamoto testnet

2. Develop wrapped sBTC ↔ RGB bridge
   - Points: 8
   - Requires: Bitcoin SPV proofs

3. Create sBTC liquidity monitor
   - Metric: Reserve ratio alerts
   - Threshold: <90% collateral

## Web5-Bitcoin Bridge

1. Implement DID:BTCR resolver [@web5-team]
   - Depends: Bitcoin Core 25+
   - Points: 3

2. Create DWN schema for PSBT
   - Versioned PSBT templates
   - Multi-sig coordination

3. Develop DID auth for Lightning
   - LNURL-auth integration
   - Web5 credential assertions

gantt
    title Anya Integration Timeline
    dateFormat  YYYY-MM-DD
    section Core
    Hexagonal Interfaces       :2025-01-01, 30d
    DID Rotation System         :2025-02-01, 21d
    section DAO
    Voting Mechanism Upgrade   :2025-01-15, 45d
    Proposal Hooks             :2025-03-01, 14d
    section ML
    Federated Learning v2      :2025-02-15, 60d
    Model Registry             :2025-04-01, 30d

## Hexagonal Architecture
1. [ ] Formalize port interfaces [@arch-team]
   - Deadline: 2025-Q1
   - Points: 13

2. [ ] Audit adapter compliance [@security]
   - Requires: Interface specs
   - Critical: Bitcoin protocol adapters

## Web5/DWN
1. [ ] Implement DID rotation [@web-team]
   - Interval: 90 days
   - Method: did:key rotation

2. [ ] DWN schema registry [@protocol]
   - Types: Metrics, Proposals, PSBTs

## DAO
1. [ ] Quadratic voting setup [@dao-team]
   - Depends: Token distribution
   - Snapshot integration

2. [ ] Proposal lifecycle hooks [@devs]
   - Pre/post-execution checks
   - Time-locked changes

## ML Ops
1. [ ] Federated node setup [@ml-team]
   - Nodes: 5 geo-distributed
   - TEE verification

2. [ ] Model version registry [@devops]
   - IPFS-based storage
   - ZK-proofs of training