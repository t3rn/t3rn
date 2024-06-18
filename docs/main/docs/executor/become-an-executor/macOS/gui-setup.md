---
sidebar_position: 2
---

# GUI Setup

Welcome to the t3rn Executor Setup! This guided process will help you configure your executor with ease, providing step-by-step instructions to ensure a smooth start. Let's get you set up and ready to operate efficiently across multiple blockchain networks.

## Executor GUI

### Download Executor GUI

**1.** Download and install the Executor GUI file from here:

- Intel: https://s3.eu-west-1.amazonaws.com/release.t3rn.io/t3rn-releases/x86_64-apple-darwin/1.0.15/Executor.dmg
- Apple Silicon: https://s3.eu-west-1.amazonaws.com/release.t3rn.io/t3rn-releases/aarch64-apple-darwin/1.0.15/Executor.dmg

### Configure Settings

**1.** Go to Settings and set your preferred Network Environment - Testnet.

**2.** Enable your preferred networks to operate on by checking your selections.
:::info If your wallet balance falls below the threshold on one of your enabled networks, that specific network will be removed.
:::

### Configure Your Arbitrage Strategies

Click configure your arbitrage strategies, and expand for each network and asset to add your own strategies.

There are 5 fields for each asset.

**1.** Minimum Profit per Order

- Specify the lowest acceptable profit that your executor should aim for when bidding on orders.

**2.** Minimum Profit Rate

- Determine the lowest acceptable profit rate, as a percentage, for your executor when bidding on orders. This rate helps assess the profitability of an order relative to its size.

**3.** Maximum Amount per Order

- Indicate the highest amount your executor is allowed to bid for a single order.

**4.** Minimum Amount per Order

- Establish the smallest amount your executor should consider when bidding on orders.

**5.** Maximum Share of my Balance per Order

- Specify the highest percentage of your total balance that your executor can allocate to a single order.

Save strategy

#### PRIVATE KEYS

**1.** Set the `Executor private key` variable to the private key of the wallet you are going to use. Example (this is a fake generated key that should/cannot not be used): 0xdead93c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56dbeef

### Log Level

Set your preferred log level - Debug, Warn, Info, or Trace.

### Start

Save your settings and click the Play button to start your Executor.
