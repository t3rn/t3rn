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
- **BD#10:** Either orders were already bid by another executor or failed spam prevention filter.
- **BD#11:** Ensures that the insurance amount provided with the bid matches the required amount when the previous best bidder's insurance needs to be refunded.
- **BD#12:** Ensures that the bid amount provided matches the required insurance amount when no previous bid exists.
- **BD#14:** Ensures that the `claimer` is the current best bidder for the specified order.
- **BD#15:** Ensures that the insurance for the specified order has not already been claimed.
- **BD#16:** Ensures that the order has been properly attested and meets the quorum requirements before allowing the insurance claim.
- **BD#17:** Ensures that the refund of the insurance amount to the `claimer` is successful.
- **BD#18:** Ensures that the new bid's decrement is greater than or equal to the required minimum bid decrement.

### escrowOrder.sol

- **ZERO_ADDRESS_NOT_ALLOWED:** This error indicates that a zero address was provided where a valid address is required.
- **ONLY_OWNER:** This error ensures that only the contract owner can execute a specific function.
- **ONLY_ATTESTERS:** This error ensures that only authorized attesters can execute a specific function.
- **IS_HALTED:** This error indicates that the contract is currently halted and cannot perform the requested operation.
- **INVALID_ORDER_TYPE:** This error means an unsupported or invalid order type was specified.
- **INVALID_ASSET:** This error occurs when a specified asset is not supported by the contract.
- **INVALID_AMOUNT:** This error indicates that the provided amount does not match the expected value for the operation.
- **AMOUNT_OUT_OF_RANGE:** This error signifies that the provided amount is outside the minimum or maximum allowed range for the asset.
- **INVALID_COUNT:** This error occurs when an invalid count (i.e zero) is provided for a batch operation.
- **ESCROW_ORDER_ALREADY_EXISTS:** This error indicates that an escrow order with the given ID already exists.
- **NOT_CLAIMABLE:** This error signifies that the order is not in a state where it can be claimed.
- **NOT_REFUNDABLE:** This error means the order is not eligible for a refund at this time.
- **SETTLE_PAYOUT_WITH_FEES_FAILED:** This error indicates that the process of settling a payout, including deducting fees, failed.

### remoteOrder.sol

- **RO#0:** Ensures that a valid address or range of values was provided during initialization.
- **RO#1:** This error indicates that the specified destination asset or network is not supported, or the order is not claimable or refundable.
- **RO#2:** Ensures that requested cross-chain route is enabled (both source & destination chains and assets are enabled).
- **RO#7:** Ensures orders aren't accepted after network's order limit has been reached.
