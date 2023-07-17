# Circuit Clock

The global clock calculating basis for claims applicable for entire t3rn platform.

Enforces global round time for all of the pallets plugged to t3rn Circuit. 

Tick fires every `RoundDuration` via `on_finalize` hook.

Each round tick the clock collects claimable artifacts for t3rn actors, i.e:

## On Collect Claimable
Pallets feed the claimable artifacts to the circuit clock via unified interface:
```rust

pub enum BenefitSource {
    ExecutorRewards,
    ExecutorStakingRewards,
    LiquidityRewards,
    ContractsStaking,
    AmbassadorsStaking,
    ExecutorInflation,
    BuildersDAOInflation,
    CollatorsInflation,
}

pub struct ClaimableArtifacts<T> {
    beneficiary: T::AccountId,
    role: CircuitRole,
    total_round_claim: T::Balance,
    benefit_source: BenefitSource,
    non_native_asset_id: Option<u32>,
}
```


Current claimable candidates at t3rn (accounted every round):
- For Executors: rewards for Xtx executions
  - source: account-manager collected from pallet-staking
- For Stakers: rewards for Xtx executions executed by all nominated executors
  - source: account-manager collected from pallet-staking
- For Stakers on Contract: rewards for Contracts Xtx executions executed by all nominated contracts
    - source: account-manager collected from pallet-3vm
- ...
  
## Claim
Circuit Clock implements a single endpoint where every eligble account collects all of the accounted rewards that 
the clock is aware of - collection of rewards from all of the possible staking: `executors`, `liquidity`, `contracts`, `ambassadors`. 
But also the rewards dynamic ecosystem incentives from inflation: `executors`, `builders`, `collators` - those are exposed to the clock through the treasury.    


License: Apache-2.0
