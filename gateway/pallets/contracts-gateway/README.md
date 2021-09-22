# Pallet Contracts Gateway 

This pallet should be used next to the [Contracts](https://github.com/paritytech/substrate/blob/master/frame/contracts) pallet as an extension that allows to execute `multistep_call` - contracts execution following t3rn protocol that differentiates the execution into 3 phases - `execute`, `revert`, `commit`. The execution is facilitated via escrow account that holds the execution proofs & deferred results as serves as a callee to operative contract calls. The deferred results are not commited to target accounts until the `commit` phase. The reversible code execution that integrates with contracts pallet is mitigated via `escrow-contracts-wrapper`, one of the dependencies.

## Installation

##### [Cargo.toml](https://github.com/t3rn/t3rn/blob/development/gateway/demo-runtime/runtime/Cargo.toml)

_Currently `v2.0.0` version of substrate is supported._

Installation of both `ContractsGateway` requires satisfying traits of other pallets which need to be added into the `Cargo.toml` of that parachain's runtime:
```rust,nofmt
[dependencies.pallet-sudo]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
version = '2.0.0'

[dependencies.pallet-contracts]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
version = '2.0.0'

[dependencies.gateway-escrow-engine]
default-features = false
git = 'https://github.com/t3rn/gateway-pallet.git'
version = "0.3.0"
```
##### Runtime [`lib.rs`](https://github.com/t3rn/t3rn/blob/development/gateway/demo-runtime/runtime/src/lib.rs)

###### Implement Traits
- __Parachains with Contracts__

Implement `Trait` for `contracts`, `sudo` and `contracts_gateway` (that comes with two traits: `EscrowTrait` for common escrow features - transfers and the actual `Trait` for contract-like behaviours:

```rust
// substrate-node-template implements sudo already.
impl pallet_sudo::Trait for Runtime {
	type Event = Event;
	type Call = Call;
}

pub const MILLICENTS: Balance = 1;
pub const CENTS: Balance = 1_000 * MILLICENTS;
pub const DOLLARS: Balance = 100 * CENTS;

parameter_types! {
    pub const TombstoneDeposit: Balance = 16 * MILLICENTS;
    pub const RentByteFee: Balance = 4 * MILLICENTS;
    pub const RentDepositOffset: Balance = 1000 * MILLICENTS;
    pub const SurchargeReward: Balance = 150 * MILLICENTS;
}

impl contracts::Trait for Runtime {
	type Time = Timestamp;
	type Randomness = RandomnessCollectiveFlip;
	type Currency = Balances;
	type Event = Event;
	type DetermineContractAddress = contracts::SimpleAddressDeterminer<Runtime>;
	type TrieIdGenerator = contracts::TrieIdFromParentCounter<Runtime>;
	type RentPayment = ();
	type SignedClaimHandicap = contracts::DefaultSignedClaimHandicap;
	type TombstoneDeposit = TombstoneDeposit;
	type StorageSizeOffset = contracts::DefaultStorageSizeOffset;
	type RentByteFee = RentByteFee;
	type RentDepositOffset = RentDepositOffset;
	type SurchargeReward = SurchargeReward;
	type MaxDepth = contracts::DefaultMaxDepth;
	type MaxValueSize = contracts::DefaultMaxValueSize;
	type WeightPrice = pallet_transaction_payment::Module<Self>;
}

pub use contracts::Schedule as ContractsSchedule;

impl contracts_gateway::EscrowTrait for Runtime {
	type Currency = Balances;
	type Time = Timestamp;
}

parameter_types! {
    pub const WhenStateChangedForceTry: bool = false;
}

impl contracts_gateway::Trait for Runtime {
	type Event = Event;
	type WhenStateChangedForceTry = WhenStateChangedForceTry;
}
```

You will see a similar error in case you missed an implementation of the contracts Trait:
```bash
  39  | pub trait Trait: contracts::Trait + system::Trait {
      |                  ---------------- required by this bound in `escrow_gateway::Trait`
```

###### Add pallets to "construct_runtime!"
Add both `EscrowGateway` and `Contracts` in your `construct_runtime!` macro:
```rust
Contracts: contracts::{Module, Call, Config, Storage, Event<T>},
ContractsGateway: contracts_gateway::{Module, Call, Storage, Event<T>},
```
In case you missed this step, you will probably see a similar error message:
```bash
 error[E0277]: the trait bound `Event: core::convert::From<pallet_contracts::RawEvent<u128, sp_runtime::AccountId32, sp_core::H256>>` is not satisfied
     --> /Users/macio/projects/substrate.dev/t3rn/gateway/demo-runtime/runtime/src/lib.rs:275:15
      |
  275 |     type Event = Event;
      |                  ^^^^^ the trait `core::convert::From<pallet_contracts::RawEvent<u128, sp_runtime::AccountId32, sp_core::H256>>` is not implemented for `Event`
      | 
     ::: /Users/macio/.cargo/git/checkouts/substrate-7e08433d4c370a21/e00d78c/frame/contracts/src/lib.rs:322:17
      |
  322 |     type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
      |                 ----------------- required by this bound in `pallet_contracts::Trait`
      |
```

_Please refer to https://substrate.dev/docs/en/tutorials/add-a-pallet-to-your-runtime in case you want to learn more details on adding any pallet to your runtime. Luckily, it's explained on the contracts pallet example :)_


## Interface
#### multistep_call
##### Parameters
```rust
fn multistep_call (
    origin: Origin,         // Origin source of a call; should be the escrow account owner.
    requester: T::AccountId,   // Account requesting the execution.
    target_dest: T::AccountId, // Target that the call is addressed to (can be an account or a contract)
    phase: u8,              // Current execution phase (execute = 0, revert = 1, commit = 2).
    code: Vec<u8>,          // Code of the package/contract as hex encode .wasm.
    input_data: Vec<u8>,    // Input data for a call to the instantiated code.
    value: BalanceOf<T>,    // Transferred value accompanying the call.
    gas_limit: Gas,         // Gas limit for processing the overall tx (instantiate + call).
) -> dispatch::DispatchResult
```
##### Description
Executes provided package within the multi-phase context. Invokes several methods of Contracts Pallet underneath: `put-code`, `instantiate` and `bare_call` in order to execute the package on the fly.
Depending on the `phase` invokes different behaviour: 
- Execute: Code results are stored on escrow account under corresponding to the call storage key.
- Revert:  Code results are removed out of escrow account.
- Commit:  Code results are moved from escrow account to target accounts.

#### rent_projection
##### Parameters
```
pub fn rent_projection(
    origin: Origin,         // Origin source of a call.
    address: T::AccountId   // Contract's address.
) -> dispatch::DispatchResult {
```
##### Description
Projects the time remaining for contract (accessible by its address) to exist. As of now, depends and calls solely `contracts::rent_projection`.
Target behaviour should estimate the proportional cost of the storage taken by the contract with corresponding address to the overall escrow account size.

#### get_storage
##### Parameters
```
pub fn get_storage(
    origin: Origin,         // Origin source of a call.
    address: T::AccountId   // Contract's address.
    key: [u8; 32]           // Key in 32-byte long hex value.
) -> dispatch::DispatchResult {
```
##### Description
Looks for a corresponig to the key value in the storage of contracts's address. As of now, depends and calls solely `contracts::get_storage`. This behaviour will probably remain with no changes.

## Related Modules

* [Contracts](https://github.com/paritytech/substrate/blob/master/frame/contracts)
* [escrow-contracts-wrapper](../../escrow-engine/escrow-contracts-wrapper)
* [escrow-engine](../../escrow-engine)

## License

---
Copyright 2020 Maciej Baj.

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0
    
---
