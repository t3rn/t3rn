<p align="center">
    <img height="150" src="./specification/assets/t3rn_Logo_Black.png?raw=true"/>
</p>
<h1 align="center">
Composable smart contract hosting with interoperable, multi-chain execution.
</h1>

[![CI](https://github.com/t3rn/t3rn/workflows/Circuit%20Build%20%26%20Test%20CI/badge.svg)](https://github.com/t3rn/t3rn/actions) [![Latest tag](https://badgen.net/github/tag/t3rn/t3rn)](https://github.com/t3rn/t3rn/tags/) [![Telegram Group](https://img.shields.io/endpoint?color=neon&style=flat-square&url=https%3A%2F%2Ftg.sumanjay.workers.dev%2FT3RN_official)](https://telegram.dog/T3RN_official) [![Twitter handle](https://img.shields.io/badge/Twitter-1DA1F2?style=for-the-badge&logo=twitter&logoColor=white)](https://twitter.com/t3rn_io) [![Discord Chat](https://img.shields.io/badge/Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white)](https://discord.gg/kfVX6k3cNp) [![codecov](https://codecov.io/gh/t3rn/t3rn/branch/development/graph/badge.svg?token=PKR69NFU6U)](https://codecov.io/gh/t3rn/t3rn)


t3rn is a hosting platform for smart contracts, that enables trustless, multi-chain execution and composable collaboration.

t3rn renders smart contracts blockchain agnostic, meaning they can instantly execute on multiple blockchains. The smart contracts can be uploaded as they are and the t3rn protocol will host and execute them across different blockchains, breaking the barrier to serving users across industries and blockchain platforms.


## Interoperability plugin - integrate with Gateway
The protocol works well with Parachains regardless of whether they support smart contracts or not. It's designed to be highly compatible with different blockchain architectures and easy integration using one of three kind of Gateway: Intrinsic Programmable, Extrinsic Programmable or Transaction-Only.

t3rn emphasizes the existent decentralised-solutions and allows multiple blockchains to collaborate on the same contracts repository. By re-using the whole decentralised application blocks, fosters building the decentralised solutions with the freedom to operate on multiple chains.
t3rn facilitates building interoperable solutions in familiar smart contract languages like Solidity, !ink or directly in Web Assembly. Smart contracts or Modules for Runtime which are hosted on a decentralised execution platform, Circuit and can be executed on multiple integrated blockchains. These smart contracts are shared and are being collaboratively added by community. 

[Learn more about Gateways and how to integrate with t3rn.](./gateway)

## Multi-chain execution platform - execute on Circuit
Gateway Circuit shares the context of the overall Interoperable Transaction and passes it over to the Gateways. The context contains all of the necessary data base on the Parachains can decide whether to not proceed with the execution. 
Gateway Circuit has an access to all of the ongoing Interoperable Transactions and before the execution is started the circuit checks if there is no two transactions changing the same account at the same time. 

[Work on the Circuit Prototype is currently in progress.](./pallets/circuit)

## On-chain contracts repository - share composable contracts.
<p align="center">
  <img width="400" src="./specification/assets/circuit_gateways_contracts.png?raw=true"/>
</p>

Each successful compilation of Composable Smart Contracts is immediately available for the network to use. The on-chain contracts hosting can be compared with decentralised package manager created by the community of t3rn developers.

All the newly created code for interoperable programming is automatically shared with other developers to reuse:
•  projects can easily collaborate by sharing and re-using the business logic 
•  developers can contribute code for free or expect remuneration per usage. This opens up a way for developers of earning money for writing the Open Source code.

Smart contracts can be written in familiar languages like !ink, Solidity, WebAssembly. Existent smart contracts can be uploaded as they are, no rewriting required. 

[Learn more about writing composable contracts in our SDK.](./client/packages/sdk)

## Motivation

Creating safe solutions operating and synchronizing multiple blockchains comes with new challenges.
 
### Synchronisation
Multiple blockchains means that accounts and the storage allocated by them is located on different chains. Without the overarching synchronisation mechanism there is no guarantee that the state of accounts won't change while the interoperable transaction is executed. t3rn offers the interoperable execution protocol performed by Circuit, which manages the multi-chain transactions.

### Non-reversible
Once a transaction is applied on a blockchain it's non-reversible, which constitutes a problem for transactions only considered useful if they succeed on multiple blockchains simultaneously. t3rn introduces multiple phases to execution of interoperable transactions and implements the safety mechanisms which are able to revert the execution on affected chains in case the overall interoperable transaction fails. 

### Complexity
Designing interoperable solutions is complex and requires developers and users to operate on multiple blockchains, possibly creating multiple different accounts, acquiring multiple currencies, providing a way for different blockchains to communicate. t3rn brings that complexity down and offers services and packages that facilitate interoperable execution securely.


### Repository setup
Follow the steps below if you're interested building and running the Circuit on your local machine:

```bash
git clone https://github.com/t3rn/t3rn
git checkout origin/development
cargo build --release
cargo run --bin t0rn-collator
```

#### Git Config

We have a way of utilizing the build system (cargo) to reason about dependencies. However, this requires some changes to your configs. Ensure you have your SSH key added to github.

`~/.gitconfig`
```
[url "git@github.com:"]
  insteadOf = https://github.com/
```

`~/.cargo/config`
```
[net]
git-fetch-with-cli = true
```

#### Running benchmarks

1. Build the circuit node with ```runtime-benchmarks``` feature:
```bash
cargo build --release --features with-standalone-runtime,runtime-benchmarks
```
2. Run the command to execute the benchmarks for desirable pallet:
```bash
./target/release/t0rn-collator benchmark --chain dev --execution wasm --wasm-execution compiled --pallet pallet_you_want --extrinsic '*' --steps 50 --repeat 20 --raw --template=./benchmarking/frame-weight-template.hbs --output .
```
3. After executing following commands a file called ```pallet_you_want.rs``` will be generated. The file contains weights for the desirable pallet, copy that file into the pallets src directory, and rename it to ```weights.rs```. 

This concludes the process of generating weights for the pallet.

## License

---
Copyright 2020-2023 t3rn Ltd.

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

