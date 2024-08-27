# Troubleshooting

## Error Codes

### biddingBook.sol

- **BD#0:** Ensures that only the owner can execute functions protected by this modifier.
- **BD#1:** Ensures that only current committee members can execute functions protected by this modifier.
- **BD#2:** Ensures that only the `orderBook` contract can call functions protected by this modifier.
- **BD#3:** Ensures that the specified order exists before proceeding with the function execution.
- **BD#4:** Ensures that a bid exists for the specified order before proceeding with the function execution.
- **BD#5:** Ensures that the quorum for the order has been reached before allowing the bid.
- **BD#6:** Ensures that the proposed reward is greater than zero.
- **BD#7:** Ensures that the proposed reward does not exceed the maximum reward specified in the order.
- **BD#8:** Ensures that underbidding is allowed before allowing a new bid to be placed.
- **BD#9:** Ensures that the new bid's proposed reward is lower than the current best bid when underbidding is enabled.
- **BD#10:** Ensures that the bidding period has not elapsed before allowing a new bid.
- **BD#11:** Ensures that the insurance amount provided with the bid matches the required amount when the previous best bidder's insurance needs to be refunded.
- **BD#12:** Ensures that the bid amount provided matches the required insurance amount when no previous bid exists.
- **BD#13:** Ensures that the proposed reward matches the order's maximum reward if underbidding is disabled.
- **BD#14:** Ensures that the `claimer` is the current best bidder for the specified order.
- **BD#15:** Ensures that the insurance for the specified order has not already been claimed.
- **BD#16:** Ensures that the order has been properly attested and meets the quorum requirements before allowing the insurance claim.
- **BD#17:** Ensures that the refund of the insurance amount to the `claimer` is successful.
- **BD#18:** Ensures that the new bid's decrement is greater than or equal to the required minimum bid decrement.

### escrowOrder.sol

- **EO#0:** Ensures that only the owner of the contract can execute functions protected by this modifier.
- **EO#1:** Requires that the owner address provided during contract initialization is not the zero address.
- **EO#2:** Protects functions from being executed while the contract is paused.
- **EO#3:** Requires that the caller is the designated attesters address.
- **EO#4:** Ensures that each escrow order in a batch operation is correctly paired with corresponding parameters.
- **EO#5:** Verifies that the provided value equals the total required sum for native currency withdrawals
- **EO#6:** Verifies that the native currency provided equals the reward amount when the reward asset is the native currency.
- **EO#7:** Prevents unsupported assets from being used in escrow orders
- **EO#8:** Ensures that the reward amount is within the defined limits for the asset
- **EO#9:** Ensures that the main asset amount adheres to pre-defined limits
- **EO#10:** Ensures that the `escrowGMP.storeEscrowOrderPayload` function successfully stores the payload associated with the creation of an escrow order or its corresponding lock.
- **EO#11:** Ensures that the correct amount of native currency is locked in escrow.
- **EO#12:** Ensures that the `escrowGMP.storeEscrowOrderPayload` function successfully stores the payload specifically associated with the locking of funds in escrow.
- **EO#13:** Ensures that the escrow lock is claimable.
- **EO#14:** Ensures that the escrow can be refunded.
- **EO#15:** Ensures that both the payout and the associated fees are successfully settled.

### remoteOrder.sol

- **RO#0:** Is related to access control and ensuring the integrity of critical contract operations, such as preventing unauthorised access to functions and the assignment of roles to an invalid address.
- **RO#2:** Ensures that functions are only executed when the contract is active and not in halted state.
- **RO#3:** This error enforces role-based access control within the contract, preventing unauthorized parties from executing sensitive functions.
- **RO#4:** Prevents users from attempting to create orders with unsupported assets.
- **RO#5:** Ensures that `maxReward` is within the accepted range for the specific `rewardAsset`.
- **RO#7:** Prevents mismatches between the expected reward and the actual funds provided.
- **RO#10:** Prevents issues where the order payload might not be correctly recorded, which could lead to disputes or untraceable orders.
- **RO#11:** Ensures that the claim for a refund is only made after the order has timed out.
- **RO#12:** Ensures that the payment hash matches the expected hash before processing a refund.
- **RO#13:** Ensures that the payment hash matches the expected hash before processing a payout.
- **RO#14:** Ensures that only the `attesters` address can settle surplus rewards.
- **RO#15 in `confirmBatchOrder`:** Ensures that the total amount of native currency expected by the contract matches the amount actually sent with the transaction - `require(totalNativeAmount == msg.value, "RO#15");`
- **RO#15 in `claimPayoutBatch`:** Enforces checks related to the validity of the payout claim process.
- **RO#16:** Ensures that the process of settling payouts with associated fees succeeds. This prevents issues during the payout process, such as incorrect fee deductions or failed transfers
