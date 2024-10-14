# Protocol Architecture

<img src="/img/protocol-architecture.png"/>

t3rn is a cutting-edge, modular, and decentralized protocol designed to address the challenges of cross-chain interoperability, security, and scalability. By enabling seamless communication and interaction between diverse blockchain ecosystems, t3rn facilitates the transfer of assets and the execution of smart contracts across multiple networks.

The t3rn protocol operates on a layered architecture that divides key functionalities into three layers: the **Settlement Layer**, the **Economic Layer**, and the **Interoperability Layer**. This structure ensures a highly modular, scalable, and efficient environment for cross-chain operations, while also maintaining the highest levels of security and decentralization.

## The t3rn Settlement Layer

At the heart of the t3rn protocol lies the **Settlement Layer**, which is fundamental for executing cross-chain smart contracts and maintaining transaction finality across interconnected blockchains. The Settlement Layer serves as the backbone for cross-chain execution, ensuring that assets and contract states remain synchronized across multiple chains.

### Key Features:

- **Finality of Transactions**: Ensures that all cross-chain transactions are completed or reverted in their entirety, preventing partial executions that could lead to inconsistent states across networks.
- **Smart Contract Flexibility**: Enables developers to execute complex smart contracts across multiple blockchain environments without the need to rewrite or port them for each ecosystem.
- **Multi-Chain Integration**: Supports interoperability between different blockchain networks, including EVM-compatible chains and non-EVM chains.

The **Settlement Layer** has been strengthened with the release of our testnet and will be further enhanced by the upcoming EVM bridging mainnet launch and the Token Generation Event (TGE). These milestones solidify the foundation for t3rn’s cross-chain execution and settlement infrastructure.

---

## The Economic Layer

The **Economic Layer** underpins the incentives and value distribution mechanisms in the t3rn protocol. This layer is designed to encourage participation from **Executors**, **Validators**, and the broader community by offering rewards for executing cross-chain transactions and maintaining the integrity of the network.

### Key Features:

- **Executor Incentives**: Executors, key participants in the t3rn ecosystem, are rewarded for successfully completing cross-chain transactions. They bid to execute user requests, and the most cost-effective bid is chosen, ensuring competitive transaction fees.
- **Transaction Fees**: A dynamic fee structure that adjusts based on network conditions, transaction complexity, and liquidity availability across connected blockchains.
- **Reward Distribution**: t3rn employs a fair and transparent reward system, providing BRN tokens to Executors, users, and community members who contribute to the platform’s success. Active participants can earn rewards by executing transactions, staking, or providing valuable feedback.

As the network grows, t3rn will continue to expand the range of Executors, allowing more participants to join the ecosystem and earn rewards. Future updates to the Economic Layer will include more sophisticated staking mechanisms and economic models to maintain the long-term sustainability of the platform.

---

## The Interoperability Layer

The **Interoperability Layer** ensures seamless interaction between a diverse set of blockchain networks. By supporting a wide range of chains, including Ethereum, Substrate-based chains, and other modular ecosystems like **Celestia**, the t3rn Interoperability Layer extends beyond traditional cross-chain operations.

### Key Features:

- **Support for Multiple Chains**: The Interoperability Layer connects multiple blockchains, ensuring that smart contracts and transactions can be executed across both EVM and non-EVM chains.
- **Modular Expansion**: t3rn is built with modularity in mind, allowing for easy integration of new blockchain ecosystems and technologies.
- **Decentralized Security**: Through bonded Attestors and advanced security mechanisms like fault proofs, t3rn ensures that cross-chain transactions are secure and resilient. This ensures that the user's assets are never at risk, even during complex cross-chain operations.

The layer also focuses on decentralizing the t3rn tech stack to ensure robustness. Future updates will see the expansion of support for new blockchain environments, enhancing the protocol’s ability to seamlessly interact with a broader range of ecosystems.

---

## Cross-Chain Execution

<img src="/img/cross-chain-execution.png"/>

One of the standout features of t3rn is its **Cross-Chain Orders (XCO)**, which utilize intent-based execution to process transactions. This system ensures atomic execution, meaning that transactions either fully complete across all chains or fail entirely—eliminating the risk of partial executions.

The **Executors** play a vital role in ensuring the success of these transactions. They compete through a bidding process to execute user orders, ensuring cost-efficiency and fairness. Executors are also responsible for maintaining liquidity and ensuring that transactions are processed swiftly and accurately.
