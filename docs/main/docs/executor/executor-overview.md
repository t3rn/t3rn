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

### [How to become an Executor](become-an-executor)

## t3rn protocol

<img height="50%" src="/img/t3rn-protocol-anatomy.jpg" />

The concept can be compared with intents, or auctions.

### Evaluation formula

Executor fills the orders that undergo t3rn protocol:
submit order -> bid -> execute -> submit proof -> get reward
order evaluation undergoes the following evaluation:

`max reward * reward asset price > gas cost on target + target asset price * target asset amount`

## Architecture

cross-chain orders can be submitted to:

- a) RemoteOrder.sol - smart contracts on Ethereum & EVM compatible chains (e.g. BSC, Polygon, etc.)
- b) Circuit - Substrate's pallet deployed to t3rn on Substrate based chains (e.g. Polkadot, Kusama, Rococo)

### Account Abstraction Layer

** To deem the overall execution architecture easier to process, we're implementing the full EVM-support for all t3rn's Parachains. That means that effectively all orders coming via t3rn's Circuit Pallet can be listened to exactly as if they were coming from Ethereum. **

We'll therefore focus on describing the orders handling process from EVMs (Ethereum Virtual Machines) perspective.

### [How to integrate t3rn into a custom AMM?](/integrations/custom-amm)

### [Gas Costs Breakdown](/resources/gas-cost-breakdown)

### How to run t3rn's default Executor with Arbitrage strategy?

1. Clone the executors [t3rn repo](https://github.com/t3rn/guardian):
2. Install pnpm (if not installed already): `npm install -g pnpm`
3. Install dependencies: `pnpm install`
4. Set your keys for dedicated networks in `.env` file (see `.env.example` for reference)

```
export PRIVATE_KEY_EXECUTOR=PRIVATE-KEY-HERE

export ENABLED_NETWORKS='base-sepolia,optimism-sepolia,binance-testnet,scroll-sepolia,arbitrum-sepolia'
export RPC_HEALTH_CHECK_INTERVAL_SEC=3
export PRICER_CLEANUP_INTERVAL_SEC=60
export EXECUTOR_MIN_BALANCE_THRESHOLD_ETH='0.1'

export LOG_LEVEL=debug
export LOG_PRETTY=true

# Prometheus
export PROMETHEUS_PORT_EXECUTOR=9334

source_env .envrc.local
```

5. Run Executor: `source .envrc && pnpm start:ranger`
6. Re-define your arbitrage strategy in `src/config/executor-arbitrage-strategies.ts` file. Set supported assets on supported chains and your arbitrage strategy for each of them, by modifying the `defaultSettings` object:

```typescript
const defaultSettings: OrderArbitrageStrategy = {
  minProfitPerOrder: BigNumber.from('1'),
  minProfitRate: 0.1,
  maxAmountPerOrder: BigNumber.from('1000'),
  minAmountPerOrder: BigNumber.from('5'),
  maxShareOfMyBalancePerOrder: 25,
}

export default {
  eth: {
    eth: {
      ...defaultSettings,
    },
  },
  bsct: {
    eth: {
      ...defaultSettings,
    },
  },
```

Revisit [arbitrage strategy section](#how-to-run-t3rns-default-executor-with-arbitrage-strategy) for more details on the arbitrage strategy configuration.

<img height="32%" src="/img/executor-running.png"/>

### [Supported Chains](/resources/supported-chains)

### Troubleshooting

Feel free to reach out to us on [Discord](https://discord.com/invite/T3sJYkEvPY) if you have any questions or issues.
With obvious errors, please open an issue on [t3rn's Github](https://github.com/t3rn/t3rn/issues/new/choose)
