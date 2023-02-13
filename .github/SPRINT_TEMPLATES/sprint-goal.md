name: Sprint goal
about: Open a new Sprint Goal
title: 'Release X.Y.Z-rcN'
labels: ''
---
## Description

Describe the sprint goal as detailed as possible in terms of materialized outcomes and end user value.


### Goal and Acceptance Criteria
Release X.Y.Z-rc

[ReleaseVersion] - [DATE] - [Sprint Goal]
- [DeliveryType][DeliveryDomain] ..delivery description (actor -> action)
   - Acceptance criteria: // list of acceptance criteria to check on zombienet in Gurken format
        - [ ] Acceptance criteria 1
        - [ ] Acceptance criteria 2
        - [ ] Acceptance criteria 3

### Terminology

```rust
enum DeliveryType { 
    Refactor,
    Feature,
    Bugfix,
    Chore,
}

enum DeliveryDomain {
    Frontend,
    Interoperability {
        Intrinsic,
        Extrinsic,
    },
    Assets,
    Governance,
    Contracts,
    Execution,
    DevOps,
    Design,
    Documentation,
    Testing,
    Security,
    DepsUpdate,
    Other,
}
```

> Example:
> [v1.3.0-rc] - [14-02-2023] - [Bootstrap the fail-safe execution between Parachains in Foreign assets ]
- [Feature][Interoperability::Intrinsic] executors submit fail-safe execution of SFX::transfer_asset via XBI
    - Acceptance criteria:
        - [ ] executors submit SFX::transfer over XBI via dedicated pallet executors 
        - [ ] executors submit SFX::transfer_assets over XBI via dedicated pallet executors 
        - [ ] executors submit SFX::call_evm over XBI via dedicated pallet executors 
- [Feature][Assets] users pay for any tx in foreign assets (ROC/DOT)
    - Acceptance criteria:
        - [ ] users pay for tx::transfer in ROC
        - [ ] users still pay for tx::transfer in T0RN
        - [ ] users pay for sfx execution in ROC 
        - [ ] users pay for sfx execution in T0RN
- [Feature][Assets] requesters submit SFX for bidding and execution 
    - Acceptance criteria:
        - [ ] requesters submit SFX transfers to registered target - Self
        - [ ] requesters submit SFX transfers to registered XBI target - HydraDX
        - [ ] SFX without bids are killed after 3 blocks and funds are returned to the requester
        - [ ] SFX without confirmations are reverted after 400 blocks and funds are returned to the requester and executors slashed
        - [ ] SFX with confirmations finalized and rewards reach executors
