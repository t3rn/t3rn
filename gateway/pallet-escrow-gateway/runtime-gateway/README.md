# Pallet Runtime Gateway 

This pallet can be used as an extension to any parachain that comes along with its own sandboxed wasm execution engine - `versatile-wasm`. 

The pallet allows for the `multistep_call` - execution following t3rn protocol that differentiates the execution contracts into 3 phases - `execute`, `revert`, `commit`. The execution is facilitated via escrow account that holds the execution proofs & deferred results as serves as a callee to operative contract calls. The deferred results are not commited to target accounts until the `commit` phase. 

The mechanics and which authorising certain operations to be dispatched via gateway on a given parachain are facilitated by `verstaile-wasm`. Parachain can also implements a `DispatchRuntimeCalls` trait to open up the resources of of internal modules for external execution via contract-like calls. That's demonstrated in [storage_runtime_demo.wat](../contracts-gateway/fixtures/32b-account-and-u128-balance/storage_runtime_demo.wat) and [demo-runtime](../../demo-runtime). It can also be disabled by importing and using ``versatile_wasm::DisableStorageDispatchRuntimeCall``.


## Installation

##### [Cargo.toml](https://github.com/t3rn/t3rn/blob/development/gateway/demo-runtime/runtime/Cargo.toml)

_Currently `v2.0.0` version of substrate is supported._

Installation of `RuntimeGateway` requires satisfying traits of other pallets which need to be added into the `Cargo.toml` of that parachain's runtime:
```rust,nofmt
[dependencies.pallet-sudo]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
version = '2.0.0'

[dependencies.versatile-wasm]
default-features = false
git = 'https://github.com/t3rn/gateway-pallet.git'
version = "0.3.0"
```

##### Runtime [`lib.rs`](https://github.com/t3rn/t3rn/blob/development/gateway/demo-runtime/runtime/src/lib.rs)

##### Implement Traits

Implement `Trait` for `sudo` and `escrow_gateway_balances` (that comes with two traits: `EscrowTrait` for common escrow features - transfers and the actual `Trait` that provides sandboxed runtime execution of WASM code:

```rust
// substrate-node-template implements sudo already.
impl pallet_sudo::Trait for Runtime {
	type Event = Event;
	type Call = Call;
}

impl runtime_gateway::EscrowTrait for Runtime {
	type Currency = Balances;
	type Time = Timestamp;
}

impl versatile_wasm::VersatileWasm for Runtime {
    type DispatchRuntimeCall = versatile_wasm::DisableStorageDispatchRuntimeCall;
    type Event = Event;
    type Call = Call;
    type Randomness = RandomnessCollectiveFlip;
}

impl runtime_gateway::Trait for Runtime {
    type Event = Event;
}
```

###### Add pallets to "construct_runtime!"
Add both `VerastileWasm` and `RuntimeGateway` in your `construct_runtime!` macro:
```rust
RuntimeGateway: runtime_gateway::{Module, Call, Storage, Event<T>},
VersatileWasm: versatile_wasm::{Module, Call, Event<T>},
```


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

## Related Modules

* [versatile-wasm](../escrow-engine/versatile-wasm)
* [escrow-engine](../escrow-engine)

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
