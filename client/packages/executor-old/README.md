# t3rn Executor:
An essential actor of the t3rn ecosystem is the executor. Executor are off-chain participants that can generate yield by executing transactions that where triggered by an on-chain transaction on the t3rn circuit. Once the transaction is finalized an inclusion proof is generated, which is then submitted to the t3rn blockchain, prooving that the transaction was executed correctly. This in turn unlocks the reward to the executor. 

Since executors are in competition with each other, and in a free-market environment, a fair amount of risk analysis must be done to be efficient and competitive. This is the first iteration of the executors, covering the main aspects that are important for executors. 

## Components:
The Executors can roughly be seperated into six main components. 

### Circuit Relayer
The executor must be able to communicate with the circuit. For one, the executor must be able to submit transaction. This is done via the `CircuitRealyer` class, which contains all relevant functions. The executor must also subscript to state updates. In t3rn, these are emitted via events. The `CircuitListener` class is responsible for this, forwarding all relevant incoming events.  

### Execution Manager:
The execution manager is the class connecting all functionality together. It tracks all incoming XTX throughout the TX lifecycle, processing incoming events and reacting accordingly. At its core, the execution manager tracks the state transitions emitted as an event by the circuit, reacting accordingly.

For example, if a new bid for a SFX we have bid on is detected, the execution manager evaluates the SFX, deciding if a counter bid should be submitted. 

### Pricing Engine:
To be able to evaluate the profitability of a SFX, the executor needs to have access to a constant stream of asset prices. The `PriceEngine` takes care of this, tracking prices for assets that should be tracked. These are then returned as `Observable` which enables changing prices to automatically be pushed to all instances that are tracking a certain asset.

Currently, the `PriceEngine` is very basic, only querying prices from coingecko. It is built with extensibility in mind, allowing new price sources to be added without much pain.

### Strategy Engine:
The strategy engine is a component used for specifying execution strategies. It allows executors to define constraints that are then evaluated for incoming XTXs. A strategy is always defined on a per-target basis. The strategy engine has two types of checks it performs:

#### XTX Strategy:
When a new XTX is added, the XTX strategy is evaluated. This is done by iterating through the XTXs SFX and checking that it passes the XTX strategy for its target chain. If any of these evaluations fails, the XTX is ignored, and not further considered for execution.

#### SFX Strategy:
The SFX strategy is used to decided if a certain SFX is deemed profitable. Any SFX that pass the evluation are deemed favorable, triggering a bid to be generated and submitted. This stage is only reached if the XTX strategy passed beforehand.

### Bidding Engine:
The bidding engine is responsible for generating the bidding amount for a specific SFX. It only receives SFXs that have been deemed profitable by the strategy engine. In its current form, the bidding engine uses `[Strategy.minProfitUsd; sfx.profitUsd]` as a range and then calculates the bid by returning the amount at a configured quartile.   

### Gateway Relayers
To execute transactions on other blockchains, a custom relayer class is needed. These classes need to be able to execute a certain set of transactions and generate an inclusion proof for said transaction. Currently, a substrate based version is implmented.

## Getting Started:
The first thing to setup is the config file.

- in packages directory, run `make` to build the local dependencies (types & sdk)
- run `yarn`
- run `yarn start`