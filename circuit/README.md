# Circuit
Circuit is a backend backbone of t3rn protocol. Maintains networks data as a decentralised contracts repository, current set of active validators, relayers, fisherman and all connected Gateways. Smart contracts are re-executed on the Circuit and results are compared to the Gateway execution, verifying successful execution.

🚧 Circuit is currently under construction 🚧

## Installation
To get up and running you need both stable and nightly Rust. Rust nightly is used to build the Web
Assembly (WASM) runtime for the node. You can configure the WASM support as so:

```
rustup install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```
### Node 
Installing the main Circuit's directory runs the actual blockchain as a Substrate-based Node.
 ```
 cargo build
 ```

You can then run the circuit with a locally stored data for development purposes with 
```
 cargo build --dev -lruntime=debug
```

#### [RPC API](./src/rpc)
RPC methods for interaction with Circuit.

#### [Runtime](./src/runtime)
The `modules` which are used to build the blockchain's logic, runtime pallets are available at [./src/runtime](./src/runtime). For relying messages - block headers and related to execution events from connected Gateways we intend to use [Parity Bridges](https://github.com/paritytech/parity-bridges-common). Integration is currently WIP.

#### [Primitives](./src/primitives)
A crate that hosts common definitions that are relevant for the Circuit.

## Circuit's Relevant Roadmap Phases
#### Phase 2: Live Circuit Prototype
#### Phase 3: Feature-Complete PoA Circuit
#### Phase 4: Trustless PoS Circuit

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



