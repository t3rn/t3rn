# Acala Escrow contract:


## To run:
- run local acala node
- for tests: `yarn build && yarn test`
- for deployment: `yarn deploy`

## Functions Implemented:
quick explaination of the different functions that are implemented. 

### **execute(bytes32 xtxId, address to, address token, uint amount):** 
This function can be used to execute erc20 transfers, swaps receiving erc20 or providing liquidity. The parameters for these operations are the same, so it can be reused for these operations.
on success: `emit ExecuteToken(xtxId, msg.sender, to, token, amount)`

### **settle(CircuitEvent memory evnt, address to, address token, uint amount):**
used to settle a transaction. Currently, the finality proofs are not checked, and not passed as parameters.

**On Commitment:**  `emit Commit(xtxId)`

**On Revert:**  `emit Revert(xtxId)`

### executeRemoveLiquidity(bytes32 xtxId, address to, address tokenA, address tokenB, uint amountA, uint amountB):
Function for removing liquidity from a liq pool. As the escrow contract only has to deal with the outputs of any transaction, we need a custom function, allowing the execution and settling of transactions with two erc20 outputs. One thing to consider: Uniswap pools use WETH instead of ETH. We need to decide if we want a seperate function that deals with unwrapping.

**On Success:** `emit ExecuteRemoveLiquidity(xtxId, msg.sender, to, tokenA, tokenB, amountA, amountB)`

### settleRemoveLiquidity(CircuitEvent memory evnt, address to, address tokenA, address tokenB, uint amountA, uint amountB):
used to settle a transaction. Currently, the finality proofs are not checked, and not passed as parameters.

**On Commitment:**  `emit Commit(xtxId)`

**On Revert:**  `emit Revert(xtxId)`