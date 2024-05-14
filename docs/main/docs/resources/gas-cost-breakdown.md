---
sidebar_position: 4
---

# Gas Costs Breakdown

| Action           | Payer    | Cost (GAS)                                                                                                                                                                 | Remarks                                                                                                                                                    |
| ---------------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Order**        | User     | Avg. 68,000                                                                                                                                                                |                                                                                                                                                            |
| **Bid**          | Executor | Avg. 54,000                                                                                                                                                                | After t3rn is deployed on Polkadot, bids on active + valid orders will cost 0 GAS, as the bidding process will be integrated into the t3rn Circuit pallet. |
| **Execute**      | Executor | Native: 21,000 <br/> ERC-20: Avg. 50,000 <br/> Using `remoteOrder::confirmOrder`: Avg. 41,000 <br/> Using `remoteOrder::confirmBatchOrder`: Avg. 41,000 GAS / X in a batch |                                                                                                                                                            |
| **Claim Reward** | Executor | Batch: Cost of single claim / X in a batch <br/> ERC-20: Avg. 60,453 <br/> Native: Avg. 40,521                                                                             |                                                                                                                                                            |
