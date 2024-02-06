---
sidebar_position: 1
---

# XBI Overview
XBI Format is an XCM-based high-level interface that each Parachain can optionally implement and enable others to use, while defining the error and result handling using an asynchronous, promise-like solution. XBI specifically focuses on setting the standards between cross-chain smart contract execution.


**XBI Format consists of two parts:**
1. XBI Instruction ordered on destination Parachain to execute, over XCM Transact.
2. XBI Metadata specifies details of interoperable execution and receives asynchronous results.

All XBI traffic goes over XCM Transact, therefore deriving its security from XCM.

## Motivation
Set high-level format XBI standard for interfaces implementing interactions between Parachains, specifically EVM and WASM based contracts.

XBI focuses on usability. It will recognise the difference between WASM and EVM, the most popular smart contract byte code in the Polkadot ecosystem today.

The XBI interface offers contingencies against runtime upgrades while allowing Parachains to define and expose their functionalities without needing runtime upgrades upon introducing new XBI Instructions distinct for selected Parachains.

XBI Metadata provides coherent controls over cross-chain execution, leaving dispatching origin in complete control over the execution costs and results format.

:::info
Since XBI's goal is to agree on a standart to be adopted from as many parachains as possible, we are more then welcome to see contrinutions on our public [repository](https://github.com/t3rn/xbi).
:::
