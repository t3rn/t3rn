# Executor Breakdown

Executors in the t3rn network facilitate cross-chain interoperability by executing user-defined intents across different blockchain networks. Their primary responsibilities include executing transactions, processing claims, and ensuring seamless communication across multiple chains.

To fully understand how Executors operate, it's important to first grasp the concepts of interoperability and cross-chain intents in the t3rn ecosystem.

## Interoperability and Cross-Chain Intents

t3rn enables cross-chain transactions through an intent-based model. This means users express an "intent" (a desired action, such as a token swap or staking), and the network takes care of executing it across multiple chains. This intent-based approach contrasts with traditional step-by-step transaction models, providing a more user-centric and efficient way to perform multi-chain operations.

Executors play a crucial role in this model, as they handle the execution of these intents by interacting with the different blockchain networks involved. They are also responsible for batching, bidding, and claiming rewards for completed transactions.

## Step-by-Step Process for Cross-Chain Transactions

This section outlines the lifecycle of a cross-chain transaction within the t3rn ecosystem, with a focus on the role of Executors.

### Step 1: Executor Initialization

Log Sequence:

```
{"level":"info","time":1729093072668,"msg":"🚀 Starting Executor"}
```

The Executor process starts, initializing the necessary components for interacting with the blockchain networks. This involves setting up the environment, loading configurations, and preparing for transaction execution.

### Step 2: Enabling Networks and Connecting to RPC Providers

Log Sequence:

```
{"level":"info","time":1729093072670,"msg":"Enabled networks: arbitrum-sepolia, base-sepolia, blast-sepolia, optimism-sepolia, l1rn"}
{"level":"info","time":1729093072670,"networkId":"l1rn","msg":"✅ L3 network initialization"}
{"level":"info","time":1729093072671,"providersUrls":["https://rpc.brn.t3rn.io/http"],"networkId":"l1rn","msg":"🔌 Initializing RPC provider"}
{"level":"info","time":1729093072671,"networkId":"l1rn","provider":"https://rpc.brn.t3rn.io/http","msg":"🔌 Connected to RPC provider"}
{"level":"info","time":1729093075158,"providersUrls":["https://sepolia-rollup.arbitrum.io/rpc"],"networkId":"arbt","msg":"🔌 Initializing RPC provider"}
{"level":"info","time":1729093075159,"networkId":"arbt","provider":"https://sepolia-rollup.arbitrum.io/rpc","msg":"🔌 Connected to RPC provider"}
{"level":"info","time":1729093075164,"msg":"🔗 Connected to network arbt at {\"chainId\":6636130,\"name\":\"unknown\"}"}
{"level":"info","time":1729093075190,"providersUrls":["https://base-sepolia.blockpi.network/v1/rpc/public"],"networkId":"bssp","msg":"🔌 Initializing RPC provider"}
{"level":"info","time":1729093075190,"networkId":"bssp","provider":"https://base-sepolia.blockpi.network/v1/rpc/public","msg":"🔌 Connected to RPC provider"}
{"level":"info","time":1729093075192,"msg":"🔗 Connected to network bssp at {\"chainId\":6636130,\"name\":\"unknown\"}"}
{"level":"info","time":1729093075203,"providersUrls":["https://sepolia.blast.io"],"networkId":"blss","msg":"🔌 Initializing RPC provider"}
{"level":"info","time":1729093075203,"networkId":"blss","provider":"https://sepolia.blast.io","msg":"🔌 Connected to RPC provider"}
{"level":"info","time":1729093075205,"msg":"🔗 Connected to network blss at {\"chainId\":6636130,\"name\":\"unknown\"}"}
{"level":"info","time":1729093075214,"providersUrls":["https://sepolia.optimism.io"],"networkId":"opsp","msg":"🔌 Initializing RPC provider"}
{"level":"info","time":1729093075214,"networkId":"opsp","provider":"https://sepolia.optimism.io","msg":"🔌 Connected to RPC provider"}
{"level":"info","time":1729093075215,"msg":"🔗 Connected to network opsp at {\"chainId\":6636130,\"name\":\"unknown\"}"}
```

The Executor connects to various networks, including Layer 3 networks and testnets. RPC providers for each network are initialized, allowing the Executor to interact with the blockchain and fetch relevant data.

:::info Custom RPC URLs
These are the default RPC providers. Executors can add their custom RPC URLs. Find out how to set them up for [Binary](./become-an-executor/binary-setup.md) and [GUI](./become-an-executor/gui-setup.md) in our docs.
:::

### Step 3: Configuring Attestation and Batching

Log Sequence:

```
{"level":"info","time":1729093072719,"networkId":"l1rn","singleAttestationBookContract":"0x450392B796A65FE542C0CDdBeb802f832e54Ac64","attestationBatchMakerContract":"0x9bf6931F78524c7c93Bcba6be4f99c4f5236Db81","msg":"🚧 Using V4 contracts for Attesting and Batching on L3"}
```

The Executor configures the necessary contracts for attestation and batching on the networks. These include the `SingleAttestationBook` and `LongRangeAttestationsBook` contracts, which are pivotal in managing the attestation and batching processes.

**Attestation Configuration:** Attestations ensure the correctness and validity of cross-chain transactions. They involve submitting proofs that a transaction occurred as intended. In the `SingleAttestationBookV4` contract, attestations are submitted by committee members who provide signatures and message payloads associated with specific orders. The attestation data is stored in a linked list structure within the `LongRangeAttestationsBook` contract.

**Batching:** The contracts enable batching of multiple attestations to reduce gas costs and optimize efficiency. The `LongRangeAttestationsBook` manages long-range batching by combining multiple attestations into a single batch. Each batch is assigned a `batchHash` for tracking and is constructed using the attestation data from multiple orders. The batching process includes anti-duplicate checks, quorum validation, and payload encoding to ensure that only valid and unique attestations are included.

**Key Features of Attestation and Batching Contracts:**

- The `SingleAttestationBookV4` contract tracks individual attestations, while the `LongRangeAttestationsBook` focuses on batching them.
- The system includes mechanisms for dispute resolution, where conflicting attestations are tracked, and the most popular attestation (based on votes) is selected.
- `Batch` structures include attestation data, the current and next committee roots, and payloads, which are used to verify attestation validity across networks.

### Step 4: Monitoring and Metrics Setup

Log Sequence:

```
{"level":"debug","time":1729093072725,"msg":"Prometheus started on 9090"}
```

Monitoring tools such as Prometheus are started to track the Executor's activity, system metrics, and performance.

### Step 5: Wallet Initialization and Configuration

Log Sequence:

```
{"level":"info","time":1729093075189,"address":"0x4e2F...","networkId":"arbitrum-sepolia","msg":"✅ Wallet loaded"}
```

The Executor loads the wallet used for submitting transactions to the connected networks. This wallet will be responsible for executing the transactions, paying gas fees, and handling refunds or claims.

### Step 6: Initializing Transaction Processing

Log Sequence:

```
{"level":"info","time":1729093075229,"networkId":"arbt","excludedLifecyclesFromBatching":["Bid"],"msg":"🚀 Start tx processing in Transmitter..."}
```

The Executor initializes transaction processing for various networks, preparing to handle incoming orders. The lifecycle of a transaction includes multiple stages: bidding, execution, attestation, and claiming. Setting up these processes ensures that each stage is handled in sequence for successful cross-chain transaction execution.

### Step 7: Setting Up Listeners and Event Handlers

Log Sequence:

```
{"level":"info","time":1729093075229,"address":"0x05C9e2dDfBa9120565c9588fd5d4464B85E77285","networkId":"l1rn","msg":"⏳ Listening for InsuranceClaimed, InflationDistributed, BRNRewardsDistributed events on BiddingBook contract"}
```

The Executor sets up event listeners to monitor specific events related to insurance claims, reward distributions, and other activities on the blockchain. These listeners help automate responses to relevant events, such as processing claims or handling refunds.

**InsuranceClaimed:** Triggered when insurance for a bid is refunded to the previous bidder. It ensures that the losing bidder gets back their insurance if they are outbid.
**BRNRewardsDistributed:** Occurs when BRN rewards are allocated to users based on their participation, such as executing orders or attesting.

### Step 8: Fetching Pending Orders and Claims

Log Sequence:

```
{"level":"info","time":1729093075233,"msg":"🔃 Processing pending orders..."}
{"level":"info","time":1729093075234,"msg":"🔃 Processing pending claims..."}
```

The Executor fetches any pending orders, executions, or claims from the network. These include cross-chain transactions that need to be processed, orders awaiting execution, or claims for rewards.

The `RemoteOrderCreated` event is emitted when a new cross-chain order is created. This marks the starting point for Executors to bid on the order. The event provides information such as the source and destination networks, asset details, and the reward offered for executing the order.

```
{"level":"info","time":1729093087825,"msg":"📝 RemoteOrderCreated event received"}
networkId: "l1rn"
id: "0xcff4..."
sourceNetwork: "arbt"
destinationNetwork: "bssp"
asset: 0
rewardAsset: "0x0000..."
amount: "0.099308703944814615"
maxReward: "0.1"
sourceAccount: "0x27CA..."
targetAccount: "0x27ca..."
orderTimestamp: 1729585761
txHash: "0x502a..."
```

### Step 9: Processing Orders and Bidding

In this step, the Executor processes an order, checking timestamps and evaluating if the wallet has enough funds to complete the transaction. Let’s break down these logs:

### Successful Execution

```
{"level":"debug","time":1729093079936,"orderId":"0x30c7...","timestamp":1729092828,"orderTimestampOfObject":1729092828,"msg":"🕒 Order timestamp"}
{"level":"debug","time":1729093081678,"id":"0x30c7...","sourceNetwork":"bssp","destinationNetwork":"arbt","asset":0,"assetToSpend":"eth","assetToReceive":"eth","strategy":{"minProfitPerOrder":"0","minProfitRate":0,"maxAmountPerOrder":"1000000000000000000","minAmountPerOrder":"40000000000000","maxShareOfMyBalancePerOrder":25},"profit":0.000282126214544651,"msg":"📈️ Order is profitable. Trade will have a profit denominated in \"assetToSpend\""}
{"level":"info","time":1729093081678,"profit":"0.000282126214544651","loss":"0.0","isProfitable":true,"networkId":"l1rn","executor":"0x4e2F...","msg":"📝🧡 Processing new profitable Order"}
{"level":"info","time":1729093088745,"id":"0x30c7...","sourceNetwork":"bssp","destinationNetwork":"arbt","asset":0,"amount":0.09971787378545534,"maxReward":0.1,"isUnderbidEnabled":false,"underbidPercentage":0,"msg":"✅ Bid successful. Execute the order..."}
{"level":"info","time":1729093095960,"id":"0x30c7...","sourceNetwork":"bssp","destinationNetwork":"arbt","txHash":"0xae22...","amount":0.09971787378545534,"targetAccount":"0xf002...","asset":0,"reward":0.1,"lifecycle":"Execute","msg":"✅ Execution successful"}
```

**Order Timestamp:** The `timestamp` and `orderTimestampOfObject` both show `1729092828`, indicating the moment when the order was created. This check ensures that the order is still valid for execution, preventing the processing of outdated or expired orders.<br /><br />
**Profitability:** The system calculates a potential profit of `0.000282126214544651` ETH, verifying that the trade aligns with the Executor's strategy for minimum profit.<br /><br />
**Order ID:** `0x30c7...`<br /><br />
**Source Network:** `bssp` (Base Sepolia)<br /><br />
**Destination Network:** `arbt` (Arbitrum Sepolia)<br /><br />
**Amount:** `0.09971787378545534` ETH<br /><br />
**Bidding and Execution:** The Executor places a bid for the specified amount and confirms that the bid was successful. This step is a precursor to transferring the funds.<br /><br />
**Successful Execution:** The transaction completes with the transfer of funds from the source network to the destination network. The transaction is validated by the recorded transaction hash `0xae22...`, confirming the order's successful completion.

At this step, the Executor first verifies the order's timestamp to confirm that the order is still within the valid time window for processing. Then, the system evaluates the profitability of the trade by calculating the potential gains based on the specified strategy parameters, such as minimum profit and maximum share of balance allowed per order. Since the order is determined to be profitable, the Executor proceeds to place a bid for the specified amount on the source network. After successfully bidding, the order is executed, transferring the funds to the target account on the destination network. The transaction is completed and confirmed with a recorded transaction hash, ensuring the process was successfully finalized without errors.

After execution, the cross-chain order must undergo an attestation step to confirm its validity across networks. The attestation process verifies that the transaction details are consistent and recognized on all relevant networks. The `TransferCommitApplied` event serves as an inclusion proof, indicating that the transaction has been successfully validated and included in the network state. This step is crucial for ensuring the correctness of the executed transaction and securing the reward distribution process.

```
{"level":"info","time":1729093087825,"msg":"🌺➡️ TransferCommitApplied event received"}
```

### Outbid By Other Executor

```
{"level":"info","time":1729093083057,"msg":"💸 OutbidReceived event received"}
```

**OutbidReceived:** This event occurs if another Executor places a higher bid, indicating that the original bid has been outdone. The previous bidder may receive an insurance claim back if applicable. This kicks off the `InsuranceClaimed` event which is emitted when an Executor who has been outbid on an order claims back the insurance amount they deposited. This step is crucial for refunding Executors who lost their bids.

### Not Enough Funds

```
{"level":"debug","time":1729093076286,"orderId":"0x6815...","timestamp":1729092942,"orderTimestampOfObject":1729092942,"msg":"🕒 Order timestamp"}
{"level":"warn","time":1729093076288,"id":"0x6815...","sourceNetwork":"bssp","destinationNetwork":"opsp","asset":0,"rewardAsset":"0x0000000000000000000000000000000000000000","amount":"0.099653228275073908","maxReward":"0.1","sourceAccount":"0xC1Ce...","targetAccount":"0xc1ce...","txHash":"0xdc8e...","wallet":"0x4e2F...","balance":{"bssp":0.15678578604873383,"opsp":0.004362630213705954},"msg":"📝🍋 Wallet balance has not enough funds. Skip order"}
```

**Timestamp:** The `timestamp` and `orderTimestampOfObject` values reflect the moment this order was added to the system. In this case, both values are `1729092942`.<br /><br />
**Purpose:** This timestamp is crucial to ensure the order is within the time limit for processing, avoiding expired orders.<br /><br />
**Order ID:** `0x6815...`<br /><br />
**Source Network:** `bssp`<br /><br />
**Destination Network:** `opsp` (Optimism Sepolia)<br /><br />
**Amount:** `0.099653228275073908`<br /><br />
**Source Account:** `0xC1Ce...` (wallet that is sending the funds)<br /><br />
**Target Account:** `0xc1ce...` (wallet that is receiving the funds)<br /><br />

At this step, the Executor checks the order's timestamp to ensure it's still valid for execution. The system then retrieves wallet balances for both the source and destination networks. In this case, the Executor wallet on the destination network does not have sufficient funds to complete the transaction. As a result, the order is skipped to prevent failed transactions or gas fees for orders that can't be completed.

### Step 10: Fetch and Processing Claims

In this step, rewards are being distributed. The Executor handles claimable orders, verifying their status and determining whether they are ready for claims. Events like `BRNRewardsDistributed` handle the distribution of rewards to Executors based on their successful execution of orders and claims. These events ensure that rewards are fairly allocated according to the Executor's contribution. The following logs provide insights into how the Executor processes these claims:

**Pending Claims Fetch Completed**

```
{"level":"info","time":1729093081299,"orderIds":["0x8673...","0x7f1a...","0xe63a...","0x7b1a...","0x86d8...","0xbd92...","0xd234...","0x4572...","0x7385...","0x92e..."],"count":10,"executor":"0x4e2F...","msg":"🔃 Pending claims fetch completed"}
```

The Executor completes fetching the list of orders pending claims, retrieving 10 orders ready for further processing.

**Order Status Recovery**

```
{"level":"debug","time":1729093083057,"id":"0x8673...","sourceNetwork":"arbt","networkId":"arbt","executor":"0x4e2F...","orderId":"0x8673...","debugContext":"executor::processClaim","msg":"📦 recoverOrderStatus"}
```

The system attempts to recover the status of the order with ID `0x8673...` on the Arbitrum network.

**Claimable Order Confirmation**

```
{"level":"info","time":1729093087825,"id":"0x8673...","sourceNetwork":"arbt","networkId":"arbt","executor":"0x4e2F...","gmpPayloadL2":"0x6f0c...","msg":"🌺🏆 Order is claimable. Transmit claim..."}
```

The system confirms that the order is claimable. The Executor is prepared to transmit the claim, with the `gmpPayloadL2` indicating the relevant payload for the claim process. This log signifies that the order meets the conditions for claiming, allowing the Executor to proceed with the claim transmission.
