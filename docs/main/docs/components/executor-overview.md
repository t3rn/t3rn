---
sidebar_position: 1
---

# Executor

At t3rn, we aim to abstract away the complexity of cross-chain transactions. Being one of the Polkadot Parachains, we can utilize XCM to perform cross-chain transactions with other Polkadot parachains. In order to connect to other blockchains, outside of the Polkadot ecosystem, we need to rely on other network participants to facilitate these transactions for us, in t3rn nomenclature we call them 'executors'. 

In simple terms, executors generate yield by executing transactions invoked by users on the t3rn blockchain. For example, a user submits a transaction, requesting to send ETH to a specific address on the Ethereum blockchain. An executor is now able to execute the transaction on the target chain (Ethereum) and submit an inclusion proof back to the Circuit, verifying that the transaction was executed correctly. The executor is now rewarded for their service with the reward payment, set by the user.

Besides finding efficient ways to source assets to send to users, executors also need to balance their funds internally. Since they operate on multiple chains, they periodically need to move funds across chains. In most cases, they will be paid on the t3rn blockchain for their services. When executing transactions, the executors spend funds on the target blockchain, while bringing assets to the target chains they operate on. How the executor moves funds across chains internally is not specified and can be freely chosen. An executor could utilize a standard bridge, a centralized exchange, OTC, etc. 
	
To incentivize efficient fees, which are paid by the user, executors are in competition with each other. Users set a maximum reward they are looking to pay, which executors can undercut, bringing the fees down to a value they are happy with. The main goal behind this mechanism is to have efficient executors, that are able to turn a profit, without charging excessive fees. 
