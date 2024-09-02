# Order Flow

This order flow covers the main stages of the lifecycle of an order on the t3rn platform, providing insights into when funds are accessible to users or Executors and what happens if something goes wrong.

### Order Flow Stages and Statuses

<!-- <img src="/img/order-cycle.png"/> -->

1. **Order Created**
   - **Status:** `OrderStatus.Open`
   - **Description:** The order is created and recorded in the order book. The order is now available for Executors to bid on.
2. **Placed**
   - **Status:** `OrderStatus.Placed`
   - **Description:** The order is placed on the source chain but hasn't been processed on our blockchain, yet.
3. **Bidding Stage**
   - **Status:** `OrderStatus.BidAccepted`
   - **Description:** The fastest Executor that bids on an order, wins this phase and gets to execute the order. In future release, Executors bid on the order.
4. **Order Execution**
   - **Status:** `OrderStatus.Executed`
   - **Description:** The winning Executor carries out the transaction on the destination network. The transaction's execution is recorded, and confirmation is awaited.
5. **Attestation on t3rn Settlement Layer**
   - **Status:** `OrderStatus.AttestedL3`
   - **Description:** After execution, the transaction must be attested on the t3rn Settlement Layer. This involves verification steps to ensure the transaction was performed correctly.
6. **Attestation on the Source Network**
   - **Status:** `OrderStatus.AttestedL2`
   - **Description:** The transaction is further attested on the source network network. This step ensures that the transaction's details are correct and meet the protocol's requirements.
7. **GMP Mismatch/Failure**
   - **Status:** `OrderStatus.GMPMismatch`
   - **Description:** A GMP (General Message Passing) mismatch or failure indicates that there was an issue with the transaction data, causing the order to be unclaimable. This status is critical and may lead to a refund process.
8. **PendingRefund**
   - **Status:** `OrderStatus.PendingRefund`
   - If the order encounters an issue such as a failure in execution or a GMP mismatch, the order enters the `PendingRefund` state, where the system begins the process of refunding the user.
9. **Order Refunded**
   - **Status:** `OrderStatus.Refunded`
   - **Description:** If the order is not executed correctly, or if the Executor fails to complete the transaction within the given time frame, the order may be refunded. This status indicates that the order was not completed as expected, and funds are returned to the user.
10. **Order Claimed**
    - **Status:** `OrderStatus.Claimed`
    - **Description:** Once the transaction is successfully attested, the Executor can claim their reward. The order status is updated to claimed.

### Refund and Claim Details

- **Refund Eligibility:** Refunds are possible if the order is not executed within the required time frame or if there is a GMP mismatch or faulty execution.
- **Claim Eligibility:** The Executor can claim their reward after the transaction is successfully attested on both the t3rn Settlement Layer and the Source Network.
