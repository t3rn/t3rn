---
sidebar_position: 1
---

# Executor

This documentation will cover an important actor in the t3rn ecosystem, Executors.

Executors are off-chain participants that can generate yield by executing cross chain transactions that were triggered by an on-chain transaction on the t3rn Circuit. Once the transaction is finalized an inclusion proof is generated, which is then submitted to the t3rn blockchain, proving that the transaction was executed correctly. This unlocks the reward to the Executor, on the t3rn blockchain. Since Executors operate on multiple chains, they periodically need to move funds across chains. Whether they do this through a centralized exchange, OTC, or any other means, is entirely up to them.

Since Executors are in competition with one another, operating in a free-market environment, a fair amount of risk analysis must be done to be efficient and competitive. Users set a maximum reward they are looking to pay, which triggers a reverse bidding where Executors can undercut each other (meaning the bidding starts at the maximum reward and moving downwards), bringing the fees down to a value that makes economic sense for them to carry out the order for.

To further incentivise execution with t3rn, we have a series of three token incentive programs taking place.

**Airdrop 500k TRN tokens to users who submitted Remote Orders from Ethereum.**

**Airdrop of 5k TRN to the top 100 Executors.**

**Airdrop of 1M TRN tokens to the top 10 Executors.**

### [How to become an Executor](https://docs.t3rn.io/components/become-an-executor)

## Executor components

### Circuit Relayer:

An Executor must be able to communicate with the t3rn Circuit. The Executor must be able to submit transactions, this is done via the Circuit Realyer class, which contains all relevant functions. The Executor must also subscribe to state updates. In t3rn, these are emitted via events. The CircuitListener class is responsible for this, forwarding all relevant incoming events.

### Execution Manager:

The execution manager is the class connecting all functionality together. It tracks all incoming Cross Chain Transactions (XTX) throughout the TX lifecycle, processing incoming events and reacting accordingly. At its core, the execution manager tracks the state transitions emitted as an event by the t3rn Circuit, and reacts accordingly.
For example, if a new bid for a Side Effects (SFX) we have bid on is detected, the execution manager evaluates the SFX, deciding if a counter bid should be submitted.

### Pricing Engine:

To be able to evaluate the profitability of a SFX, the Executor needs to have access to a constant stream of asset prices. The PriceEngine takes care of this, tracking prices for assets that need to be tracked. This enables changing prices to automatically be pushed to all instances that are tracking a certain asset.
Currently, the PriceEngine is very basic, solely querying prices from CoinGecko. However, it is built with extensibility in mind, allowing new price sources to be added without much configuration.

### Strategy Engine:

The strategy engine is a component used for specifying execution strategies. It allows Executors to define constraints that are then evaluated for incoming XTXs. A strategy is always defined on a per-target basis. The strategy engine has two types of checks it performs:

#### XTX Strategy:

When a new XTX is added, the XTX strategy is evaluated. This is done by iterating through the XTXs SFX and checking that it passes the XTX strategy for its target chain. If any of these evaluations fails, the XTX is ignored and no longer considered for execution.

#### SFX Strategy:

The SFX strategy is used to decide if a certain SFX is deemed profitable. Any SFX that pass the evaluation are deemed favorable, triggering a bid to be generated and submitted. This stage is only reached if the XTX strategy passes beforehand.

### Bidding Engine:

The bidding engine is responsible for generating the bidding amount for a specific SFX. It only receives SFXs that have been deemed profitable by the strategy engine. In its current form, the bidding engine uses [Strategy.minProfitUsd; sfx.profitUsd] as a range and then calculates the bid by returning the amount at a configured quartile.

### Gateway Relayers:

To execute transactions on other blockchains, a custom relayer class is needed. These classes need to be able to execute a certain set of transactions and generate an inclusion proof for said transaction. Currently, a Substrate based version is implemented.
