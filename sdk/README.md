<p align="center">
    <img height="150" src="./assets/t3rn_Logo_Black.png?raw=true"/>
</p>
<h1 align="center">
Composable smart contract hosting with interoperable, multi-chain execution.
</h1>


[![Documentation](https://docs.rs/t3rn-sdk-primitives/badge.svg)](https://docs.rs/t3rn-sdk-primitives/)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![ink!](https://img.shields.io/badge/ink!-3.0.0-red)](https://github.com/paritytech/ink/tree/v3.0.0)

## [API Documentation](https://docs.rs/t3rn-sdk-primitives/)


# t3rn Smart Contracts SDK

The t3rn SDK is a set of libraries that enable smart contract developers to utilise the building blocks of t3rn and provide novel access to 3VM(t3rn's smart contract VMs).

## Feature Overview

Here we set out what building blocks are available through the t3rn SDK and for what developers.

### User Function

The user function is a function provided to t3rn_sdk::execute. This function is the key to the t3rn SDK and acts as a state transition function for your contract. On each step iteration, a developer will provide business logic that will determine if new side effects are to be submitted. By default, if no new side effects are submitted, then the circuit will treat execution as completed and return.

The default characteristics are as follows:
```
- On UserF Error:
    The error is emitted to the circuit to stop execution
- On no new side effects returned:
    Execution is finished
- on Side effects returned:
    A new step will be created to submit more side effects
```

### Local state

Local state is the representation of a smart contracts [ExecutationState](https://docs.rs/t3rn-sdk-primitives/latest/t3rn_sdk_primitives/state/struct.ExecutionState.html) in the circuit. On each incremental step, the circuit will update the state accordingly. For each step, the provided user function(add link) is applied over the current steps state.

### Side effects

[Side effects](https://docs.rs/t3rn-sdk-primitives/latest/t3rn_sdk_primitives/xc/enum.Chain.html) are your cross-chain slices of transactions that help you build your business logic.

### Signalling

A [signal](https://docs.rs/t3rn-sdk-primitives/latest/t3rn_sdk_primitives/signal/struct.ExecutionSignal.html) is almost exactly like a linux signal. When we provide this to the circuit, depending on the situation, execution will either stop, or continue.

## Supported Virtual Machines

At the moment, we support WASM contracts via [ink!](https://paritytech.github.io/ink/) and Solidity via [EVM precompiles](https://www.evm.codes/precompiled) which needs extensive documentation(TODO).

### Note on solidity

We are working to have generated support for Solidity contracts so that types can be better represented.


