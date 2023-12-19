---
sidebar_position: 1
---

# Smart Contract Hub

The t3rn Smart Contract Hub represents a pivotal innovation in the realm of blockchain interoperability and secure cross-chain execution. It is designed to address the complexities and security challenges associated with bridging disparate blockchain networks.

## Challenges in Bridging Security

When bridging between different blockchains, several security challenges arise:

- **Isolation of Blockchains:** Each blockchain operates independently, making secure communication across chains difficult.
- **Diverse Consensus Mechanisms:** Different blockchains use varied consensus mechanisms, which complicates the trustless validation of transactions across networks.
- **Asset Security:** Ensuring the safety of assets during cross-chain transfers is paramount, yet challenging due to the different security protocols across blockchains.

Usually, bridges have to hold a large amount of tokens on the native and target chain to perform bridging operations, attracting malicious actors that are trying to exploit the contracts holding those funds.

Another type of risk concerns wrapped tokens. These are a copy of the value of a native token (supposed to hold a one-to-one peg) allowing it to be transferred from one blockchain to another. An underlying asset (native asset) is sent to a custodian (an entity holding the same value of the native asset and newly-minted asset) who keeps it in a digital vault using a smart contract. Once the asset is locked, custodians mint the new wrapped asset to be used on another blockchain.

The risk lies in the smart contracts, representing those wrapped assets. If those smart contracts are not audited or not well designed, there is a risk that malicious actors can exploit the contract to mint wrapped assets at will and then exchange them for the native assets.

## t3rn Smart Contract Hub: A Solution

The t3rn Smart Contract Hub is engineered to tackle these challenges, providing a secure and trustless environment for cross-chain interactions.

It is a hosting platform for smart contracts written in the most widely adopted programming languages including Solidity, ink!, WebAssembly or anything that compiles to WASM.

Smart contracts stored on the t3rn platform can be used by anyone, while the developers that contribute smart contracts to the open-source repository may choose to get remunerated anytime their code is executed. Additionally, the t3rn protocol distributes a significant part of the gas fees the smart contract generates back to the original contract author.

### Trustless Cross-Chain Execution

- **Mechanism:** Utilizes a unique execution model that allows smart contracts to operate across multiple blockchains in a trustless manner.
- **Fail-Safe Protocol Design:** The architecture of t3rn is structured to offer fail-safe operations for cross-chain transactions. This is achieved through a combination of advanced smart contract logic and network protocols that monitor and validate each step of a cross-chain transaction. If a transaction fails at any point, the protocol ensures that effects are reverted to the pre-transaction state, similar to how gas exhaustion triggers reversion on Ethereum.
- **Circuit Mechanism:** Employs a 'Circuit' system, which acts as a decentralized hub coordinating cross-chain communications and validations. When a smart contract execution request is initiated, the Circuit interprets the request and translates it into a format understandable by the target blockchain. This involves converting function calls, parameters, and data structures to be compatible with the target chain's protocol.

### Cross-Chain Communication

XBI is a standard for fail-safe and trustless cross-chain executions. It sits inside XCM, which is the native cross-consensus messaging format of Polkadot.

Where XBI shows to be useful is adding additional standards over the transactions, specifically on setting the standards between cross-chain smart contract execution.

Read more about [XCM and XBI](/xcm/xbi-overview).
