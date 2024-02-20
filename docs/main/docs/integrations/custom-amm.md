---
sidebar_position: 1
---

# Integrate t3rn into a Custom AMM

Follow distinct sections of the [lifecycle](#lifecycle-of-the-cross-chain-order) dedicated to integrators of custom Automated Market Makers (AMMs):

1. [Order submission: by User](#1-order-submission-by-user)
2. [Order evaluation: by Executors](#2-order-evaluation-by-executors)
3. [Order auction: by Executors](#3-order-auction-by-executors)
4. [Order execution: by Executors](#4-order-execution-by-executors)
5. [Order reward claim: by Executors](#5-order-reward-claim-by-executors)
6. [Order refund: by User](#6-order-refund-by-user)

<a id="lifecycle"/>

## Lifecycle of the cross-chain order

### 1. Order submission: by User

This is typically done by a user, who submits a transaction to the t3rn Circuit. The transaction contains all the relevant information for the Executor to be able to execute & evaluate the transaction. This includes the target chain, the action to be executed, the arguments to be passed, and the reward for the Executor.

To interact with the order function of our smart contract, users must provide parameters including destination (bytes4), asset (uint32), targetAccount (bytes32), amount (uint256), rewardAsset (address), insurance (uint256), and maxReward (uint256). This function encodes these parameters and calls orderMemoryData, creating an order with the specified details. Upon successful execution, the contract emits an OrderCreated event, which includes a unique order ID, details of the order, and the block number. Users can call this function through a web3 provider in their environment, passing the required parameters to initiate and track their order transactions effectively.

```solidity
    function order(bytes4 destination, uint32 asset, bytes32 targetAccount, uint256 amount, address rewardAsset, uint256 insurance, uint256 maxReward) public payable {
        bytes memory input = abi.encode(destination, asset, targetAccount, amount, rewardAsset, insurance, maxReward);
        orderMemoryData(input);
        emit OrderCreated(generateId(msg.sender, uint32(block.number)), destination, asset, targetAccount, amount, rewardAsset, insurance, maxReward, uint32(block.number));
    }
```

```typescript
// Example - call remoteOrder::order as a user
const txPromise: Promise<TxResponse> = contract.order(
  params.destination,
  params.asset,
  params.targetAccount,
  params.amountWei,
  params.rewardAsset,
  params.insurance,
  params.maxRewardWei,
  {
    value: params.maxRewardWei,
    gasPrice: txExecParams.gasPrice,
    nonce: txExecParams.nonce,
    gasLimit: txExecParams.gasLimit,
  }
);
```

Output: `event OrderCreated(
bytes32 indexed id,
bytes4 indexed destination,
uint32 asset,
bytes32 targetAccount,
uint256 amount,
address rewardAsset,
uint256 insurance,
uint256 maxReward,
uint32 nonce
)`

### Run orderer - fast writer

To test out orders submission, we've created an app called FastWriter. It's a simple web app that allows users to submit orders to the t3rn Circuit. Contact the team if you'd like to test it out or customize to your own needs.

<img src="/img/fast-writer-order-sent.png"/>

#### (Coming soon) 3D Orders - Submitting orders via 3rd party, non-EVM compatible, chains

The introduction of Dynamic Destination Deal (3D) marks a revolutionary stride in addressing liquidity challenges within the t3rn ecosystem. 3D offers a novel approach to asset liquidity, transforming the way assets are brought from target chains to t3rn.

3D simplifies the process of asset bridging. Assets can be swapped directly from the user's wallet on any integrated network, enhancing the user experience significantly.

3D stands out for its ability to allow assets from various chains to be used directly as rewards for orders, bypassing the need to bridge assets to t3rn first. This not only simplifies the process but also opens up new avenues for asset utilization. For example, swapping DOT for ETH no longer requires a two-step process involving t3rn but can be done directly from a user’s wallet on Polkadot.

In essence, 3D represents a leap forward in blockchain interoperability and liquidity management. It's not just a tool; it's a paradigm shift in how we approach asset transactions across multiple chains.

Submission of 3D orders is done via the t3rn Circuit. 3D will come as a part of the Execution flow once the EVM's support is implemented on t3rn's Parachains.

### 2. Order evaluation: by Executors

t3rn protocol is designed to be a free market, where Executors compete with each other to execute orders.

The Executor's architecture assumes Executor **listen** for orders out of all of the chains, that executor selects to provide the liquidity for.

#### 2.1 Listen for orders emitted by RemoteOrder.sol: by Executors

Listen & Receive the OrderCreated event on one of the networks (see #supported chains and the list of deployed RemoteOrders.sol smart contracts).

```solidity
event OrderCreated(
    bytes32 indexed id,
    bytes4 indexed destination,
    uint32 asset,
    bytes32 targetAccount,
    uint256 amount,
    address rewardAsset,
    uint256 insurance,
    uint256 maxReward,
    uint32 nonce
);
```

```typescript
// Example - listen for OrderCreated event on EVM-compatible chain in Typescript
  async function listenRemoteOrderEvents(name: string, network: NetworkConfigWithPrivKey) {
    logger.info(
        { address: network.contracts.remoteOrder, network: network.id },
        `⏳ Listening for OrderCreated events on contract RemoteOrder`,
    )

    this.remoteOrderContract[network.id].on('OrderCreated', async (...args: NewOrderEventData) => {
        const order: Order = constructOrderObj(args, network.id);
        evaluateOrder(order, network);
```

#### 2.2. Profitability evaluation: by Executors

Once the order is received, the Executor evaluates the profitability of the order.
From the t3rn's perspective, we cannot decide whether the order is profitable or not - this must be done by the Executor.
**We advocate for and support a secure arbitrage strategy, where executors compare execution costs and the ordered amount against the offered reward. This approach ensures orders are filled efficiently and profitably.**

<a id="arbitrage-strategy"/>

##### 2.2.1 Arbitrage strategy: by Executors

- **`OrderArbitrageStrategy`** interface is central to our system, allowing executors to define their individualized strategy for order execution. An example of a strategy configuration is as follows:

```jsx
// Example strategy for arbitrage settings on Base for Ethereum asset
export interface OrderArbitrageStrategy {
    minProfitPerOrder: BigNumber // in target asset
    minProfitRate: number // in %
    maxAmountPerOrder: BigNumber // in target asset
    minAmountPerOrder: BigNumber // in target asset
    maxShareOfMyBalancePerOrder: number // in %
}

const defaultSettings: OrderArbitrageStrategy = {
    minProfitPerOrder: BigNumber.from('1'),
    minProfitRate: 0.1,
    maxAmountPerOrder: BigNumber.from('1000'),
    minAmountPerOrder: BigNumber.from('5'),
    maxShareOfMyBalancePerOrder: 25,
}
```

** All of the amount parameters are denoted on 18 decimals precision! **

** That means, that e.g. setting `minProfitPerOrder` to 1, sets it with value = 1 Wei (0.000000000000000001 ETH) **

Each field in this strategy configuration plays a specific role:

1. **`asset` (string)**: Specifies the target asset for the arbitrage strategy, such as 'eth' in the given example. This defines the type of asset the executor will be dealing with.
2. **`minProfitPerOrder` (BigNumber)**: The minimum profit, in the target asset, that an executor aims to achieve for each order. In the example, '1' indicates the executor expects at least 1 ETH of profit per order.
3. **`minProfitRate` (number)**: This is the minimum profit rate, expressed as a percentage, that the executor seeks to achieve. A '0.1%' rate implies that the profit must be at least 0.1% of the total order value.
4. **`maxAmountPerOrder` (BigNumber)**: This sets the upper limit on the amount, in the target asset, that the executor is willing to handle in a single order. The example sets this limit at 1000 ETH.
5. **`minAmountPerOrder` (BigNumber)**: Conversely, this is the minimum order amount, in the target asset, that the executor is willing to handle. In our example, the minimum is 5 ETH.
6. **`maxShareOfMyBalancePerOrder` (number)**: This represents the maximum percentage of the executor's total balance in the target asset that can be used for a single order. The example strategy limits this to 25% of the executor's total ETH balance.

When the **`evaluateDeal`** function is called on each received order, it uses these parameters to assess the profitability of an order. It calculates the potential profit and checks it against these strategy thresholds. If the order meets or exceeds the specified minimum profit, profit rate, and amount constraints while staying within the maximum amount and balance share limits, it is deemed profitable. This strategy ensures executors can optimize their returns while managing risks according to their predefined preferences.

### 3. Order auction: by Executors

If the order is profitable, the Executor submits a bid to the t3rn Circuit. The bid is a transaction that contains the Executor's address, the order ID, and the bid amount. The bid amount is the amount the Executor is willing to accept as a reward for executing the order. The Executor with the lowest bid is selected to execute the order. The Executor is then responsible for executing the order (by sending ordered amount of target asset to orderer's target account - step 4.)

```solidity
function bidFifo(bytes32 sfxId) public {
    // Check if the payload is already stored and return false if it is
    require(isKnownId(sfxId), "Payload not found");
    require(orderWinners[sfxId] == address(0), "Order already won");
    orderWinners[sfxId] = msg.sender;
    emit BidReceived(sfxId, msg.sender);
}
```

```typescript
    // Example - call remoteOrder::bidFifo as an executor
    const txPromise: Promise<TxResponse> = contract.bidFifo(
        params.sfxId
    )
)
```

Output: `event BidReceived(
    bytes32 indexed id,
    address indexed winner
)`

Since bidding is available to all executors, it is crucial to ensure, that executor's account is assigned as a winner. This is done by the `orderWinners` mapping, which stores the executor's address for each order ID. If the executor's address is already stored, the bid is rejected.

```typescript
// Ensure that the bid was successful by reading the storage of the remoteOrder contract on source
const assignedBidder = await this.remoteOrderContract[
  order.source
].orderWinners(order.id);
if (!bidTransmitterReceipt.success || assignedBidder !== walletAddressSource) {
  this.prometheus.bidFailed.inc({ sourceNetwork: order.source });
  // If bid was reverted then decode the revert reason and log it. Possibly it was to other executor
  logger.warn(
    {
      id: order.id,
      sourceNetwork: order.source,
      destinationNetwork: order.destination,
      err: bidTransmitterReceipt.error,
      assignedBidder,
      myAccount: walletAddressSource,
    },
    `🥀 Bid lost. Skip order`
  );
  return;
}
```

### 4. Order execution: by Executors

Execution phase follows after successfully won Bid for a given order.
Executor has 128 blocks measured on Source chain to fulfil the order. Assuming 12s block time, it's about 25 minutes. Otherwise, the Executor loses the certainty to score the offered reward - since user can request a refund of the max reward deposit.

Execution is done by sending the ordered amount of the target asset to the orderer's target account on **destination** chain. We recommend using the dedicated `execute` function of remoteOrder smart contract for this purpose. This function takes the order ID as a parameter and transfers the ordered amount of the target asset to the orderer's target account. It also emits an `Confirmation` event, which includes the order ID, the target's address, amount and asset. This event is used to track the execution of the order.

- supports execution in both Native currency of destination chain and ERC20 tokens.
- supports batching - multiple orders can be executed in a single transaction, this is especially useful for Ethereum, where the gas costs are high (`function confirmBatchOrder(ConfirmBatchOrderEntry[] calldata confirmBatch) public payable`)

```solidity
function confirmOrder(bytes32 id, address payable target, uint256 amount, address asset) public payable returns (bool) {
    // Send the amount to the target
    if (asset == address(0)) {
        if (msg.value < amount) {
            return false;
        }
        (bool sent, bytes memory _data) = target.call{value: amount}("");
        if (!sent) {
            return false;
        }
    } else {
        // Send the amount from users balance to the target; check if the allowance is enough
        if (IERC20(asset).allowance(msg.sender, address(this)) < amount) {
            return false;
        }
        IERC20(asset).safeTransferFrom(msg.sender, target, amount);
    }
    emit Confirmation(id, target, amount, asset);
    return true;
}
```

Output: `event Confirmation(
    bytes32 indexed id,
    address indexed target,
    uint256 indexed amount,
    address asset
)`

```typescript
// Example - call remoteOrder::confirmOrder as an executor
const txPromise: Promise<TxResponse> = contract.confirmOrder(
  params.id,
  params.target,
  params.amount,
  params.asset,
  {
    value: params.amount,
    gasPrice: txExecParams.gasPrice,
    nonce: txExecParams.nonce,
    gasLimit: txExecParams.gasLimit,
  }
);
```

### 5. Order reward claim: by Executors

Once the order is executed, the Executor can claim the reward. Similar to the execution, the reward claim is done by calling the dedicated `claimPayout` function of the remoteOrder smart contract. This function takes the order ID as a parameter and transfers the reward amount to the Executor's address. It also emits a `Claim` event, which includes the order ID, the Executor's address, and the reward amount. This event is used to track the reward claim.

- supports batching - multiple orders can be claimed in a single transaction, this is especially useful for Ethereum, where the gas costs are high (`function claimPayoutBatch(Payout[] memory payouts) public payable`)

```typescript
// Example - call remoteOrder::claimPayout as an executor
const nativeAssetToClaim = ADDRESS_ZERO; // address(0) for native currency of destination chain
const erc20TokenAssetToClaim = "0x6b175474e89094c44da98b954eedeac495271d0f"; // example DAI token address
const isClaimable = await this.remoteOrderContract[network.id].checkIsClaimable(
  orderId,
  orderToClaim.isNative ? nativeAssetToClaim : erc20TokenAssetToClaim,
  orderToClaim.maxReward
);

if (isClaimable) {
  const txPromise: Promise<TxResponse> = this.remoteOrderContract[
    network.id
  ].claimPayout(
    orderId,
    orderToClaim.isNative ? nativeAssetToClaim : erc20TokenAssetToClaim,
    orderToClaim.maxReward,
    {
      gasPrice: txExecParams.gasPrice,
      nonce: txExecParams.nonce,
      gasLimit: txExecParams.gasLimit,
    }
  );
}
```

Optionally, executors can listen to the TransferCommitApplied emitted by Attesters contracts on each source chain. This event is emitted when the reward is transferred to the Executor's address. This event can be used to track the reward claim.

```solidity
event TransferCommitApplied(bytes32 indexed sfxId, address indexed executor);
```

```typescript
// Listen for TransferCommitApplied event on EVM-compatible (source of orders) chain in Typescript
async function listenToTransferCommitAppliedEventsAndClaim(name: string, network: NetworkConfigWithPrivKey) {
logger.info(
    { address: network.contracts.remoteOrder, sourceNetwork: network.id },
    `⏳ Listening for TransferCommitApplied events on ${name} for contract Attesters contract`,
)
this.attestersContract[network.id].on('TransferCommitApplied', async (...args: any[]) => {
    const [orderId, committedBeneficiary] = args
    logger.info(
        {
            id: orderId,
            committedBeneficiary: committedBeneficiary,
        },
        `📝 TransferCommitApplied event received`,
    )
```

### 6. Order refund: by User

This part is irrelevant for Executors, but it's important to understand the full lifecycle of the order.
If the order is not executed within 128 blocks, the user can request a refund of the max reward deposit. This is done by calling the dedicated `claimRefund` function of the remoteOrder smart contract. This function takes the order submission block number in order to derive distinct order ID, as well as reward asset and max reward. It also emits a `Claimed` event, which includes the order ID, the user's address, and the max reward amount with reward asset. This event is used to track the refund.

- supports batching - multiple orders can be refunded in a single transaction, this is especially useful for Ethereum, where the gas costs are high (`function claimRefundBatch(RefundAfterTimeout[] memory payouts) public payable `)

```solidity
function claimRefund(uint32 orderSubmissionBlockNumber, address rewardAsset, uint256 maxReward) public payable {
    // Check if Refund timeout is satisfied
    require(block.number >= orderSubmissionBlockNumber + 128, "Order has not timed out");
    // Derive order ID from the order submission block number and the user's address
    bytes32 sfxId = generateId(msg.sender, orderSubmissionBlockNumber);
    // Withdraw the reward as refund
    bytes32 paymentPayloadHash = keccak256(abi.encode(rewardAsset, maxReward));
    bytes32 paymentHash = escrowGMP.getRemotePaymentPayloadHash(sfxId);
    bytes32 calculatedRefundHash = keccak256(abi.encode(paymentPayloadHash, address(0)));
    require(paymentHash == calculatedRefundHash, "Payload for refund not matching");
    escrowGMP.nullifyPayloadHash(sfxId);
    if (rewardAsset == address(0)) {
        payable(msg.sender).transfer(maxReward);
    } else {
        IERC20(rewardAsset).safeTransfer(msg.sender, maxReward);
    }
    emit Claimed(sfxId, msg.sender, maxReward, rewardAsset);
}
```

```typescript
// Example - call remoteOrder::claimRefund as a user
const txPromise: Promise<TxResponse> = contract.claimRefund(
  params.orderSubmissionBlockNumber,
  params.rewardAsset,
  params.maxReward,
  {
    gasPrice: txExecParams.gasPrice,
    nonce: txExecParams.nonce,
    gasLimit: txExecParams.gasLimit,
  }
);
```
