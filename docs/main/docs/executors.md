# Executor

## What are Executors

Executors are off-chain agents acting as market makers that fulfil crosschain orders, earning fees and rewards.

## How to become an Executor?

We provide three ways to become an Executor:

- [Docker Setup](executor/become-an-executor/docker-setup)
- [Binary Setup](executor/become-an-executor/binary-setup)
- [AIxecutor Setup](executor/become-an-executor/aixecutor-setup)

## Understanding Executors

When a crosschain transaction is triggered on the t3rn protocol, Executors start bidding on it for the winner to execute the transaction. Once the transaction is finalized an inclusion proof is generated, which is then submitted to the t3rn protocol, proving that the transaction was executed correctly. This unlocks the reward to the Executor, on the t3rn blockchain.

As Executors operate on multiple chains, they periodically need to move funds across chains. Whether they do this through a centralized exchange, OTC, or any other means, is entirely up to them.

Since Executors are in competition with one another, operating in a free-market environment, a fair amount of risk analysis must be done to be efficient and competitive. Users set a maximum reward they are looking to pay, which triggers a reverse bidding where Executors can undercut each other (meaning the bidding starts at the maximum reward and moving downwards), bringing the fees down to a value that makes economic sense for them to carry out the order for.

### Evaluation formula

Executor fills the orders that undergo t3rn protocol:
submit order → bid → execute → submit proof → get reward

Order evaluation undergoes the following evaluation:

`Max Reward + Initial Asset Value > Gas Fees + Target Asset Value`

## Troubleshooting

Feel free to reach out to us on [Discord](https://discord.gg/nFEq2fRpdn) if you have any questions or issues.

With obvious errors, please open an issue on [t3rn's Github](https://github.com/t3rn/t3rn/issues/new/choose)

Also, check our [Troubleshooting page](resources/troubleshooting) for more information.