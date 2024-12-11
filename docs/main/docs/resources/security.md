# Security

At t3rn, security is paramount in enabling seamless, trustless execution of smart contracts across multiple blockchains. This document highlights the key security considerations of the t3rn protocol, focusing on its smart contracts, associated functions, and the mechanics of secure order flow.

t3rn’s smart contracts have been audited by Halborn, Quantstamp, and SR Labs, reflecting our commitment to robust security.

The **core mission of t3rn** is to empower developers and users with a reliable platform for interoperable decentralized applications. Cross-chain interoperability presents unique challenges, especially in ensuring transactions are executed securely without exposing vulnerabilities. To address these, t3rn combines fault tolerance, validation mechanisms, and a focus on trustless, decentralized architecture.

### **Scope of This Documentation**

This document is intended for developers, auditors, and users seeking a deep understanding of t3rn's secure and reliable cross-chain operations. Topics include:

- A detailed examination of the **order flow**, focusing on smart contracts and Executors.
- Validation and fault tolerance mechanisms.
- Code snippets demonstrating critical security processes like reentrancy protection and transaction lifecycle management.
- Insights into economic incentives and penalties ensuring system reliability.

### **Objectives**

By the end of this documentation, you’ll understand:

- The security principles underpinning t3rn’s design.
- The smart contract mechanisms securing the protocol’s order flow.
- How t3rn ensures integrity and resilience in cross-chain operations.

## Order Flow Overview

### **Flow Diagram (Optional for Designer)**

While graphics aren’t required now, consider a simple linear flow:

1. **Order Creation** →
2. **Bidding** →
3. **Execution** →
4. **Validation** →
5. **Settlement & Rewards**

![Screenshot 2024-12-09 at 17.48.06.png](https://prod-files-secure.s3.us-west-2.amazonaws.com/e19bbbe0-c917-4eb1-b936-1515ae9aa86b/8eb32f29-8535-4e77-a966-3784975e2b3c/Screenshot_2024-12-09_at_17.48.06.png)

### **1. Order Lifecycle**

1. **Order Creation**:
   - Orders are initialized in the `OrderBook` contract, containing details like destination chain, asset type, and transaction amount. Orders are identified by unique `orderId` hashes.
   - **Validation**: Orders must meet quorum requirements, enforced by committee-based attestation.

```
function addOrder(
    Order calldata order,
    bytes32 orderId,
    bytes32 orderTxHash
    ) public onlyCurrentCommitteeMember {
    addOrderInternal(order, orderId, orderTxHash, order.orderTimestamp);
}
```

1. **Bid Submission**:

- Executors bid on orders via the `BiddingBook`, proposing execution rewards and locking insurance. Only the best bid is retained, ensuring economic efficiency.
- **Security**: Reentrant bids and excessive reward proposals are rejected.

```
function bidOrder(bytes32 orderId, uint256 newProposedReward, address bidder) public payable orderMustExist(orderId) nonReentrant {
        Bid memory currentBestBid = bestOrderBid[orderId];
        (OrderBook.Order memory order, bytes32 _coordinates, uint256 orderTimestamp) = orderBook.getOrderWithTimestamp(
            orderId
        );
    // Further validations...
}
```

1. **Execution and Escrow**:

   - Accepted bids lock transaction funds in `EscrowOrder`. Assets are securely held until execution confirmation or refund.
   - Executors fulfill the order, and funds are released upon successful validation.

   ```
   function escrowLock(
       bytes32 remoteOrderId,
       bytes4 chainId,
       uint32 assetId,
       address assetAddress,
       uint256 amount,
       bytes32 targetAccount
   ) public payable isOn returns (bytes32) {
    ensureAssetIsSupported(assetAddress);
       ...
   }
   ```

2. **Order Finalization**:
   - Successful transactions trigger payout via `settleNativeOrToken`, while failed or timed-out orders invoke the refund process.

### **Key Actors and Roles**

- **Order Creators**:
  Initiate cross-chain transactions by submitting orders with specified parameters.
- **Executors**:
  Decentralized actors responsible for bid placement and transaction execution. Incentivized via rewards locked in the protocol.

### **Security Considerations in the Flow**

- **Event-Driven Transparency**:
  Key events like `RemoteOrderCreated`, `BidReceived`, and `EscrowFundsLocked` ensure all stages are traceable on-chain.
- **Quorum Enforcement**:
  The `checkOrderHasReachedQuorum` function ensures a minimum percentage of committee attesters validate orders before they proceed.
- **Timeout Mechanisms**:
  Embedded in contracts like `EscrowOrder`, these safeguards prevent funds from being locked indefinitely.

## Smart Contract Functionality in Order Flow

t3rn utilizes a suite of interconnected contracts to manage secure and efficient cross-chain operations.

### **Core Contracts in the Order Flow**

1. **OrderBook**

   The central repository for all cross-chain orders. It handles:

   - **Order Creation**: Tracks and verifies new orders.
   - **Order Attestation**: Ensures orders meet quorum through committee-based validation.
   - **Event Emissions**: Emits critical events such as `RemoteOrderCreated` for tracking order lifecycle.

   **Key Functions**:

   - **`addOrder`**: Adds new orders after validating quorum.
   - **`checkOrderHasReachedQuorum`**: Ensures committee approval by verifying attestation counts.

2. **BiddingBook**

   Manages bids placed by Executors for specific orders. It ensures:

   - Fairness in bid selection with rules.
   - Economic security by locking insurance funds.
   - Distributes inflation and rewards to honest Executors.

   **Key Functions**:

   - **`bidOrder`**: Facilitates bidding while ensuring insurance and reward rules are followed.
   - **`claimInsuranceBack`**: Handles insurance refunds for Executors whose bids are not successful.

3. **EscrowOrder**

   Facilitates fund management by locking assets in escrow until successful completion or refund of the order.

   - **Order Validation**: Ensures only supported assets are locked.
   - **Fund Security**: Implements timeout and refund mechanisms for stuck transactions.
   - **Dynamic Payouts**: Settles payouts with fees and refunds if necessary.

   **Key Functions**:

   - **`escrowLock`**: Locks funds for a specific cross-chain transaction.
   - **`refundEscrowToSender`**: Releases locked funds back to the sender if conditions are met.

### **2. Security Mechanisms Embedded in Smart Contracts**

t3rn’s smart contracts include several critical features to ensure transaction integrity and safeguard against vulnerabilities:

- **Reentrancy Guard**:
  Prevents reentrancy attacks across sensitive functions like `claimInsuranceBack` and `settleNativeOrToken`.

## **Core Security Features**

To ensure safe, reliable interactions across its ecosystem, t3rn integrates several critical security features into its protocol design. These measures are aimed at safeguarding user assets, maintaining operational integrity, and protecting against known attack vectors.

### **Reentrancy Guards**

Reentrancy attacks, where malicious actors manipulate external calls to interfere with the flow of a smart contract, are mitigated through strict reentrancy protections. t3rn leverages OpenZeppelin’s `nonReentrant` modifier to secure critical functions.

This ensures that once a function is called, no re-entrant calls can manipulate its state mid-execution.

### **Access Control**

Role-based permissions are enforced across critical smart contract functions using OpenZeppelin’s `AccessControl` module. This ensures only authorized entities can interact with sensitive features.

- **Roles in t3rn**:
  - **Administrator**: Manages protocol parameters and upgrades.
  - **Executor**: Facilitates cross-chain order execution.

This restricts potentially destructive operations, such as modifying reward parameters, to trusted parties.

- **Validation Layers**:
  Functions like `checkOrderHasReachedQuorum` and `ensureAmountWithinAcceptedRange` ensure data integrity and prevent exploits.
- **Time-Based Safeguards**:
  Timeout mechanisms in contracts like `EscrowOrder` ensure transactions that exceed predefined limits are reverted, avoiding stale locks.
- **Event-Driven Transparency**:
  Events like `RemoteOrderCreated`, `EscrowFundsLocked`, and `BidReceived` provide a clear, traceable transaction history.

## Security and Resilience in t3rn Protocol

### **1. Event-Driven Architecture**

Events provide an immutable log for transaction states, ensuring transparency and aiding in real-time monitoring.

### **2. Secure Cross-Chain Messaging**

t3rn prevents tampering and replay attacks through cryptographic attestations and unique identifiers.

### **3. Vulnerabilities and Mitigation Strategies**

- **Reentrancy Risks**:
  - Contracts like `EscrowOrder` and `BiddingBook` use the `nonReentrant` modifier to prevent malicious recursion.
- **Sybil and Quorum Attacks**:
  - Validators undergo strict committee selection processes, reducing the likelihood of malicious actors influencing quorum-based decisions.
- **Timeout Failures**:
  - Timeout mechanisms ensure stuck transactions are refunded or reverted in a timely manner.

### **4. Testing and Auditing Practices**

t3rn’s smart contracts have undergone rigorous testing and third-party audits to ensure security and reliability.

- **Auditing Partners**:
  - Audited by industry leaders such as Halborn, Quantstamp, and SR Labs.
- **Testing Strategy**:
  - Unit and integration tests simulate real-world scenarios, including edge cases for high traffic and complex cross-chain interactions.
