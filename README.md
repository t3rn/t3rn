<p align="center">
    <img height="150" src="./docs/main/static/img/readme-banner.png?raw=true"/>
</p>
<h1 align="center">
Universal Execution: Coordinate smart contracts, capital, and computation across ecosystems.
</h1>

<p align="center">
  <a href="https://github.com/t3rn/t3rn/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/t3rn/t3rn/circuit-build-test-ci.yml?branch=development&style=flat-square&label=CI&logo=github-actions&logoColor=white" alt="CI Status" />
  </a>
  <a href="https://codecov.io/gh/t3rn/t3rn">
    <img src="https://img.shields.io/codecov/c/github/t3rn/t3rn/development?style=flat-square&logo=codecov&logoColor=white" alt="Code Coverage" />
  </a>
  <a href="https://github.com/t3rn/t3rn/tags/">
    <img src="https://img.shields.io/github/v/tag/t3rn/t3rn?style=flat-square&label=Latest%20Tag&logo=git&logoColor=white" alt="Latest Tag" />
  </a>
  <a href="https://telegram.dog/T3RN_official">
    <img src="https://img.shields.io/endpoint?color=neon&style=flat-square&url=https%3A%2F%2Ftg.sumanjay.workers.dev%2FT3RN_official&logo=telegram&logoColor=white" alt="Telegram Group" />
  </a>
  <a href="https://discord.gg/t3rn">
    <img src="https://img.shields.io/badge/Discord-5865F2?style=flat-square&logo=discord&logoColor=white" alt="Discord Chat" />
  </a>
  <a href="https://twitter.com/t3rn_io">
    <img src="https://img.shields.io/badge/Twitter-1DA1F2?style=flat-square&logo=twitter&logoColor=white" alt="Twitter Handle" />
  </a>
</p>

t3rn is the Universal Execution Protocol where smart contracts and executions have no boundaries. To achieve this we are building the Universal Execution Protocol where smart contracts operate across all chains, atomically, and with full composability, enabling decentralized applications, agents, and protocols to coordinate without fragmentation or custom integration overhead.

## Interoperability: Universal Execution Across EVM and Beyond

The t3rn Universal Execution Protocol is designed for true cross-ecosystem compatibility - supporting both EVM and non-EVM chains, as well as emerging execution environments. We don’t just move tokens or messages, we execute smart contract logic across chains, using a network of decentralized Executors to handle the heavy lifting.

This architecture allows smart contracts on one chain to synchronously trigger calls on another, with full verification and rollback guarantees.

t3rn’s interoperable design makes it simple for developers to compose crosschain applications, where a single action can interact with multiple chains in one transaction. This reduces fragmentation, eliminates manual overhead, and brings us closer to a unified crosschain developer experience.

## Protocol Architecture

t3rn is building a decentralized system built to coordinate smart contract logic across blockchains. Instead of just moving tokens or passing messages, t3rn enables contracts to execute across multiple networks as a single, unified process. No more partial states, manual bridging, or siloed applications.

**Universal Executions:** At the heart of the t3rn protocol lies Executors (solvers), key participants in the t3rn ecosystem, are rewarded for successfully completing crosschain transactions. They bid to execute user requests, and the most cost-effective bid is chosen, ensuring competitive transaction fees.

**The Settlement Mechanism:** The magic behind t3rn's ability to execute complex crosschain operations reliably lies in its Settlement mechanism. This acts as the coordination engine and source of truth for every multi-step transaction that runs through the protocol. Its purpose is to solve one of the challenges to interoperability: how do you safely complete operations that span multiple chains without leaving behind partial states or vulnerable funds? Whether it’s a three-way swap or a contract call followed by a transfer, t3rn ensures that everything either goes through as intended, or doesn’t go through at all.

[Read more about our Protocol Architecture.](https://docs.t3rn.io/protocol-architecture)

## License

---

Copyright 2020 - 2025 t3rn Limited.
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

---
