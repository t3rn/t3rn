---
sidebar_position: 2
---

# Circuit
The Circuit is the blockchain that powers t3rn. It is the core component which handles every event and is in charge to keep the state of every on-and offchain transaction happening in t3rn. 
In order to perform multichain transactions, Circuit checks and manages state of all involved chains to enable composable and fail safe multichain transactions.


The Circuit is also responsible for storing Sideeffect, which will then be picket up by Executors based on [reward](sfx-overview#max_reward) set by the creator of the Sideeffect.
Sideeffects can be created on Circuit directly via Extrinsics, incomming XBI Messages or 3VM. This makes Circuit to the Hub handeling all incomming and outcomming messages.
This allows XBI enabled Parachains to create multi chain transactions outside of the Polkadot Ecosystem!

<p align="center">
    <img height="150" src="/img/t3rn_circuit.png?raw=true"/>
</p>


