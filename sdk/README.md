# Welcome to our tiny SDK!

SDK contains demonstration showcasing execution of composable contracts with t3rn & tools to build, deploy and execute your own ones. 


## Setup
To build composable smart contracts with t3rn you need to install [compiler](./compiler). Follow its installation process or just type `cargo install --git https://github.com/MaciejBaj/cargo-contract cargo-t3rn-contract --features extrinsics` if you're short on time.

You should end up with `cargo-t3rn-contract` shell command installed.


## Run Demo
There is a `mock_circuit.sh` script that demonstrate building, deploying & execution of composable contracts against a [demo_runtime](../gateway/demo-runtime) substrate node, with `pallet-contract`, `pallet-contracts-gateway` & `pallet-runtime-gateway` installed. 
Pre-requisites to running a script are:
- `cargo-t3rn-contract` installed - either as a shell command or from source at `./compiler` location. Script checks for the shell command first.
- `npm.js` installed - script starts [demo_runtime](../gateway/demo-runtime) in an additional terminal using npm package `ttab`.

Run a demo with a command: `bash mock_circuit.sh`.

## Create
Create a new contract with `cargo-t3rn-contract contract new`.

From there you can start writing your composable contracts. 

### Composable? 
Within a single contract file you can define several smart contracts components that will be seperately compiled and deployed (possibly to different blockchains). To define your components add an extra `package.metadata` tag to `Cargo.toml` of your contract and define `composables` array entry. Each component needs to be also added as a cargo feature. For example: 
```rust
[package.metadata]
composables = ["my_component", "flipper_contract"]
deploy = [ { compose = "flipper_contract", vm = "pallet-contracts", url = "ws://localhost:9944" } ]

[features]
my_component = []
flipper_contract = []
```
You can also specify configuration for `composable-deploy` like shown above: `flipper_contract` will be deployed to a default substrate node url `ws://localhost:9944` via `pallet-contracts::put_code`. 

There is currently no limit on number of components you can create. 

### Write contracts

Start each contract component with a built-in [conditional compilation macro](https://doc.rust-lang.org/reference/conditional-compilation.html). 
`composable-build` starts a separate compilation for the contract with all of the features (components) specified in `Cargo.toml` and builds them separately. 

You can write your smart contract components as:
- regular in [!ink](https://github.com/paritytech/ink) contracts:
```rust
use ink_lang as ink;
#[cfg(feature = "flipper_contract")]
#[ink::contract]
pub mod flipper { ... }
```
- [WebAssembly text format](https://github.com/WebAssembly/wabt) by assigning your WAT to a static variable:
```rust
#[cfg(feature = "my_component")]
static MY_COMPONENT_WAT_CONTRACT: &str = r#"
    (module (
        ...
    )"#;
```
###### Warning! Compilation of WebAssembly text format is regexp-based. Therefore you must follow the naming convention and name the constant prepending your exact component name to "_WAT_CONTRACT" and place code between  `r#"` and `)"#` tags. There can be multiple WAT contracts in the same file.

- and soon in [Solidity](https://github.com/hyperledger-labs/solang). 

## Execute
Deploy all of your components to different target chains using `composable-deploy`. 

Unfortunately, there is no equivalent commands for composable execution, as interoperable execution of multiple generic complex and synchronizing them together is complex - we therefore develop the [Gateway Circuit](../README.md). We will be continuously updating this doc as the development progresses.

For now you can have a sneak peek on how we see the circuit working in [`mock_circuit.sh`](./mock_circuit.sh). You could try to implement your own execution schedule in a similar vein. 

## Example 

You can find here an exhaustive [example](./examples). The example shows how to deploy several scattered contracts (components) into Substrate based chains. Demonstrates that chains without Contracts pallet (demo runtime module with storage calculations) can also be integrated into the execution circuit with regular smart contracts on different chains. The example composes out of three contracts: 
- A) !ink flipper contract (being deployed to substrate runtime **with** Contracts Pallet)
- B) WASM demo storage contract, demonstrating calculations on runtime storage (being deployed to substrate runtime  **without** Contracts Pallet)
- C)!ink call flipper contract and depending on random flip result start calculations (being deployed to substrate runtime **with** Contracts Pallet) 
 
The execution schedule goes as followed:
1. Deploy `A) !ink flipper contract`
2. Instantiate `A) !ink flipper contract`
3. EXEC - Step 1: Execute `B) WASM demo storage contract` via `runtime-gateway`. 
4. EXEC - Step 2: Execute `C) !ink call flipper contract` as a regular contract call. 
5. COMMIT: If the flip result of 4. was TRUE, commit results of `B) WASM demo storage contract` via `runtime-gateway` which completes the transaction.
6. REVERT: If the flip result of 4. was FALSE, revert results of `B) WASM demo storage contract` via `runtime-gateway` which completes the transaction.

In the near future composable contracts will be running on a semi-decentralised Circuit. As of now, the oversimplified replacement functions as a [mock circuit bash script](./mock_circuit.sh). 

## Compiler
Compiler is a command line tool written in Rust. It's a fork of original [paritytech/cargo-contract](https://github.com/paritytech/cargo-contract) with additional commands, useful for  contracts execution using t3rn. More details at the [compiler's README](./compiler).

Compiler composes contracts according to the schedule. The schedule can be (regular smart contracts with no schedule are also compatible) added into Cargo.toml of original contracts under "metadata" key:
```rust
# Cargo.toml of smart contract
[package]
name = "..."
...
[package.metadata]
composables = ["runtime_demo_storage", "flipper_contract", "call_flipper"]
deploy = [ { compose = "flipper_contract", vm = "pallet-contracts", url = "ws://localhost:9944" } ]

[features]
default = ["std"]
runtime_demo_storage = []
flipper_contract = []
call_flipper = []
```

## subxt
 SDK contains a [forked subxt](./subxt) library, which signs, dispatches and watches for a resulting event at the t3rn gateways. As of now it's only used by the compiler, but will be integrated as part of the Gateway Circuit pallet in the next development phases.
