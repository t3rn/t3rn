## Gateway

t3rn comes with the standalone Gateway pallet for multi-steps transaction processing that brings a possibility of safely reverting the changes based on external of parachain factors and spread over time. 

The standalone version of the gateway brings additional phases over the regular runtime execution - commit and revert, due to which effects of the execution can be reverted and never committed to the target accounts. This is possible, as after the execution phase the changes are made only to the child storage of dedicated on that parachain Escrow Account, which acts as a regular account from the perspective of a parachain, therefore all of the changes to its storage are secured by the consensus effective in that parachain and can be relied on by already integrated services. 

### User Guide
#### Architecture overview
`EscrowGateway` pallet is modelled to be similar to [`Contracts`](https://github.com/paritytech/substrate/tree/master/frame/contracts) pallet, therefore giving the same developer experience and only extending functionalities over standard smart contracts. `EscrowGateway` internally imports the `Contracts` module and calls following methods: 
- Inside `multistep_call`: `put_code`, `instantiate`, `bare_call`, `terminate`. 
- Inside `rent_projection`: `rent_projection`.
- Inside `get_storage`: `get_storage`.
`EscrowGateway` is intended to work within the [Gateway Circuit](https://github.com/t3rn/t3rn#gateway-circuit) which oversees, synchronises and secures the interoperable execution between Parachains involved and communicates with Gateway API.


#### Use
In this repository, `escrow_pallet` is installed twofold: 
- as `tiny-node`, where the Escrow Gateway is one very few connected pallets. This is extended after [substrate-node-template](https://github.com/substrate-developer-hub/substrate-node-template)
- as `full-node`, where the Escrow Gateway is connected alongside with all the other pallets. Full node comes as a git submodule of [substrate](https://github.com/paritytech/substrate.git). 

To use a full node you will have to initialize the repository as a git submodule first:
```
git submodule init 
git submodule update
```

Run either a tiny or full node with `bash run-node-tiny.sh` or `bash run-node-full.sh`. 

The node runs on a default for Substrate `ws-port = 9944` & `http-port = 9933`. 

##### Front-end
You can verify the Escrow Gateway running demo frontend app from [front-end](./front-end) directory, or using the `Extrinsics` tool of [@polkadot/apps](https://github.com/polkadot-js/apps) GUI which is hosted on https://polkadot.js.org/apps/#/extrinsics. Remember to select a "local node" from the left-menu dropdown.


#### Installation

Gateway comes with the `EscrowGateway` pallet, which can be integrated into existing and new parachains and their runtimes:

##### Runtime `Cargo.toml`
Add `EscrowGateway` pallet by specifying the dependency in `Cargo.toml` file of your parachain's node runtime (please note, that because of the dependencies, contracts pallet needs to be installed as well):
```rust,nofmt
[dependencies.escrow-gateway]
default_features = false
git = 'https://github.com/t3rn/pallet-escrow-gateway.git'
package = 'pallet-escrow-gateway'
version = '0.1.0-rc5'

[dependencies.contracts]
git = 'https://github.com/paritytech/substrate.git'
default-features = false
package = 'pallet-contracts'
tag = 'v2.0.0-rc5'
version = '2.0.0-rc5'
```
_Currently supported substrate version is `2.0.0-rc5`._


##### Runtime `lib.rs`

###### Implement Traits
Implement `Trait` for both `contracts` and `escrow_gateway`:
```rust
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
    type WeightPrice = transaction_payment::Module<Self>;
}

impl escrow_gateway::Trait for Runtime {
    type Event = Event;
}
```

You will see a similar error in case you missed an implementation of the contracts Trait:
```bash
  39  | pub trait Trait: contracts::Trait + system::Trait {
      |                  ---------------- required by this bound in `escrow_gateway::Trait`
```

###### Expose Contracts Schedule
Expose `Schedule` from `contracts` pallet so it can be consumed by other modules (like `node/src/chain_spec.rs`) without introducing a dependency to `contracts`.
```rust
pub use contracts::Schedule as ContractsSchedule;
```

###### Add pallets to "construct_runtime!"
Add both `EscrowGateway` and `Contracts` in your `construct_runtime!` macro:
```rust
Contracts: contracts::{Module, Call, Config, Storage, Event<T>},
EscrowGateway: escrow_gateway::{Module, Call, Storage, Event<T>},
```
In case you missed this step, you will probably see a similar error message:
```bash
 error[E0277]: the trait bound `Event: core::convert::From<pallet_contracts::RawEvent<u128, sp_runtime::AccountId32, sp_core::H256>>` is not satisfied
     --> /Users/macio/projects/substrate.dev/t3rn/gateway/node-tiny/runtime/src/lib.rs:275:15
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

###### Add contracts pallet to Genesis Configuration

Genesis configurations are controlled in `src/chain_spec.rs`. We need to modify this file to include the ContractsConfig type and the contract price units at the top:
```rust
use node_template_runtime::{ContractsConfig, ContractsSchedule};
```

Then inside the testnet_genesis function we need to add the contract configuration to the returned GenesisConfig object as followed:

```rust
fn testnet_genesis(initial_authorities: Vec<(AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    _enable_println: bool) -> GenesisConfig {

    GenesisConfig {
        /* --snip-- */

        /*** Add This Block ***/
        contracts: Some(ContractsConfig {
            current_schedule: ContractsSchedule {
                    enable_println,
                    ..Default::default()
            },
        }),
        /*** End Added Block ***/
    }
}
```
In case you missed this step, you will probably see a similar error message:
```bash
error[E0063]: missing field `contracts` in initializer of `node_template_runtime::GenesisConfig`
   --> node/src/chain_spec.rs:136:2
    |
136 |     GenesisConfig {
    |     ^^^^^^^^^^^^^ missing `contracts`

```

_Please refer to https://substrate.dev/docs/en/tutorials/add-a-pallet-to-your-runtime in case you want to learn more details on adding any pallet to your runtime. Luckily, it's explained on the contracts pallet example :)_

#### API
The `escrow-gateway-api` pallet is intended to provide the way for a parachain to connect with the t3rn network operated by Gateway Circuit (scheduled to be implemented in [following development phases](../roadmap/following_development_phases.md)) or any other trusted service holding the 
authorisation keys.

As of now, `escrow_gateway` doesn't implement [Custom RPC](https://substrate.dev/recipes/3-entrees/custom-rpc.html). This might change in the next milestones. 

##### multistep_call
###### Parameters
```rust
fn multistep_call (
    origin: Origin,         // Origin source of a call; should be the escrow account owner.
    phase: u8,              // Current execution phase (execute = 0, revert = 1, commit = 2).
    code: Vec<u8>,          // Code of the package/contract as hex encode .wasm.
    input_data: Vec<u8>,    // Input data for a call to the instantiated code.
    value: BalanceOf<T>,    // Transferred value accompanying the call.
    gas_limit: Gas,         // Gas limit for processing the overall tx (instantiate + call).
) -> dispatch::DispatchResult
```
###### Description
Executes provided package within the multi-phase context. Invokes several methods of Contracts Pallet underneath: `put-code`, `instantiate` and `bare_call` in order to execute the package on the fly.
Depending on the `phase` invokes different behaviour: 
- Execute: Code results are stored on escrow account under corresponding to the call storage key.
- Revert:  Code results are removed out of escrow account.
- Commit:  Code results are moved from escrow account to target accounts.

##### rent_projection
###### Parameters
```
pub fn rent_projection(
    origin: Origin,         // Origin source of a call.
    address: T::AccountId   // Contract's address.
) -> dispatch::DispatchResult {
```
###### Description
Projects the time remaining for contract (accessible by its address) to exist. As of now, depends and calls solely `contracts::rent_projection`.
Target behaviour should estimate the proportional cost of the storage taken by the contract with corresponding address to the overall escrow account size.

##### get_storage
###### Parameters
```
pub fn get_storage(
    origin: Origin,         // Origin source of a call.
    address: T::AccountId   // Contract's address.
    key: [u8; 32]           // Key in 32-byte long hex value.
) -> dispatch::DispatchResult {
```
###### Description
Looks for a corresponig to the key value in the storage of contracts's address. As of now, depends and calls solely `contracts::get_storage`. This behaviour will probably remain with no changes.

### Testing Guide
_As of now only the behaviour of `execute` phase is checked._

#### Unit Tests
`EscrowGateway` comes with unit tests. Module instantiation is complex as the gateway introduces a depencency on the contracts and takes place in `src/mock.rs`.  
Unit tests consist mainly of the tests of `multistep_call`, passing the valid (`returns_from_start_fn.wasm`) and invalid (`empty`) code  for execution. The unit tests are in `src/test.rs`. 

To execute the unit test, type: 
```shell script
cargo test -- --nocapture
```

Make sure, you're in `pallet-escrow-engine` directory.

_While running tests, you may want to change the `debug::info!` to `println!` messages, like for `multistep_call` message:_

```rust
/// Change debug::info! to println! for test debugging.
// debug::info!("DEBUG multistep_call -- escrow_engine.execute  {:?}", exec_res);
println!("DEBUG multistep_call -- escrow_engine.execute  {:?}", exec_res);
```


#### Integration Tests
`EscrowGateway` comes with the integration tests. 

Integration tests run different integration scenarios against running Substrate node (either `tiny-node` or `full-node`) connecting with its Call API dedicated for extrinsics. 

For example to run the integration tests against the tiny node:
1. Build & run a `tiny-node` with `bash run-node-tiny.sh`.
1. Execute integration tests against `ws:9944` default port: `cd test-integration && npm test:tiny` or `cd test-integration && npm test:full`.

So far, only the following scenario has been implemented:
###### - [Execute multi-step transaction](./test-integration/multistep_call.spec.js)
Uses Alices account as the one that already has some positive balance on it allowing to put_code and instantiate contract. Alice's account from the perspective on the Gateway becomes an Escrow Account. That Escrow Account sends the signed transaction against the `multistep_call` API containing valid example code (`returns_from_start_fn.wasm`) and checks the correct results of that execution by retreving the data about the events - there should be one from the contracts pallet (code is stored_ and one from escrow gateway pallet - mutlistep_call result. 

### Please refer to the [Gateway specification](../specification/gateway_standalone.md) to find details on the future offer, intended shape and [Development Roadmap](../roadmap/initial_development_phase.md). 

