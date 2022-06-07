---
sidebar_position: 2
---

# Types

On the t3rn circuit, a gateway can be registered as one of three different types. We will go through the different ones in this section, explaining the functionality they offer and their requirements for connecting a target blockchain.

In general, the gateway type defines how a target chain connects to the circuit and what type of functionalities are supported. As blockchains differ in the functionality they offer, a generalizable classification is useful. A simple and clear classification is the separation into programmable and non-programmable blockchains. A simple example of this is Ethereum vs. Bitcoin. Ethereum is programmable, as it allows the deployment and execution of custom logic with smart contracts. On the Bitcoin blockchain, this is different. While `P2SH` addresses allow the creation of some custom logic, it can’t be seen as programmable when compared to Ethereum.

![](/img/gateway_types.png)

## Non-Programmable Blockchains

In t3rn the `transfer-only` type is used for target blockchains that aren’t programmable. As the functionality of these blockchains is rather limited, this type suffices to enable the functionality of these blockchains. The `transfer-only` type is also permissionless, meaning anyone can connect a target blockchain via this gateway type.

### Transfer-Only Gateway

The `transfer-only` is the loosest type and only requires that the target blockchain allows transactions that result in a state-change of some sort. This pretty much includes every blockchain in existence, one prominent example being the Bitcoin blockchain. On `transfer-only` blockchains, the t3rn circuit is able to trigger dirty transfers. A dirty transfer is executed on the target blockchain, and is not revert-or committable. Once finalized, there is no way to revert the state changes. For `transfer-only` blockchains, this should make sense, as it's not feasible to build revert/commit logic on a blockchain that is by nature not programmable. It must be noted that every target blockchain fulfilling the `transfer-only` requirements can be connected via a `transfer-only` gateway. This also includes programmable blockchains like Ethereum.

## Programmable Blockchains

First, let's give a loose definition of what we mean by programmable. Essentially, we consider a blockchain programmable, if it allows the addition of custom logic to transactions. This can be achieved with smart contracts, but also with other approaches like the integration of custom pallets in the Polkadot ecosystem. For these blockchains, the t3rn circuit offers two different types that a chain can integrate with, namely `extrinsic-programmable` and `intrinsic-programmable`. At the time of writing, the `intrinsic-programmable` type exists, but is currently not the focus of development in that area.

However, we want to roughly explain it here, as it makes the separation of the different programmable types clearer. The `extrinsic-programmable` is a permissionless type, that can be integrated with any permissionless programmable blockchain. The `intrinsic-programmable` on the other hand, requires the integration of the t3rn gateway pallet on the target blockchain. For this reason, it can be seen as a permissioned integration, requiring the integration of code written by the t3rn team. With this separation being clear, let's dive into the different types.

### Extrinsic Programmable

The `extrinsic-programmable` is a stricter type, requiring that the target blockchain allows permissionless deployment and custom logic. One example of this could be the Ethereum blockchain. To make transactions revertable, transactions must be routed through smart-contract that contain the reversibility logic and apply it to incoming transactions. For example, to make a transfer revertable, the smart contract receives the funds, along with some data describing the transaction. As the last step, the transfer can be `COMMITED` or `REVERTED`. On commit, the funds are sent from the smart contract to the receiver. On revert, they are sent back to the sender. The release of funds can be achieved in a trustless manner, with two main approaches being available. These are explained in detail in section XX. (escrow vs reversible)

### Intrinsic Programmable

The `intrinsic-programmable` is a much narrower type. Essentially, this type is limited to substrate-based target blockchains that integrate the t3rn gateway pallet. The core idea of this pallet is to make the reversibility of transactions more efficient and generic by integrating it via a native substrate-pallet. The specifications of this type are not finalized and are currently not at the center of development in that area. For now, the focus is on the `extrinsic-programmable` type, as we believe the majority of blockchains will integrate with this type. This section will be updated once we have a better idea of how this implementation will look like.

The three types presented here enable the generic connection of a wide range of blockchains. One aspect to reiterate is that the types are not exclusive. Ethereum could also be connected via a `transfer-only` gateway, enabling non-reversible asset transfers. The gateway type is a rough classification, enabling different functionalities. At t3rn, we expect the majority of blockchains to connect via the `extrinsic-programmable` gateway, at least for the time being. It's the most flexible type and can be added to any programmable blockchains offering smart contracts in a permissionless manner.
