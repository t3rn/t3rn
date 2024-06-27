---
sidebar_position: 4
---

# Circuit

Circuit is the blockchain that powers t3rn. It is the core component that handles every event and is in charge of keeping the state of every on-and offchain transaction happening in t3rn.
To perform multichain transactions, Circuit checks, and manages the state of all involved chains to enable composable and fail-safe multichain transactions.

The Circuit is also responsible for storing Side Effect, which will then be picked up by Executors based on reward set by the creator of the Side Effect.
Side Effects can be created on Circuit directly via Extrinsics, incoming XBI Messages, or 3VM, making Circuit the Hub handling all incoming and outcoming messages.
This allows XBI-enabled Parachains to create multichain transactions outside of the Polkadot Ecosystem.

<p align="center">
    <img height="150" src="/img/t3rn_circuit.png?raw=true"/>
</p>
