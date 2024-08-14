# GUI Setup

Welcome to the t3rn Executor Setup! This guided process will help you configure your executor with ease, providing step-by-step instructions to ensure a smooth start. Let's get you set up and ready to operate efficiently across multiple blockchain networks.

## Executor GUI

### Download Executor GUI

**1.** Download and run the Executor GUI file according to your OS from here:

- Apple (Intel): Coming soon
- Apple Silicon: Coming soon
- Windows: Coming soon

### Configure Settings

**1.** Go to Settings and set your preferred Network Environment - Testnet.

**2.** Enable your preferred networks to operate on by checking your selections.
:::info If your wallet balance falls below the threshold on one of your enabled networks, that specific network will be removed.
:::

### Optional: Configure Your Network RPC URLs

Click configure your network RPC URLs and add RPC URLs for each enabled network, or skip this step and automatically use the default RPC URLs.

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

**6.** Save strategy

#### PRIVATE KEYS

**1.** Set the `Executor private key` variable to the private key of the wallet you are going to use. Example (this is a fake generated key that should/cannot not be used): 0xdead93c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56dbeef

:::tip Executor Privacy
Read more about [Executor Privacy and Security](../../resources/executor-privacy.md)
:::

### Log Level

Set your preferred log level - Debug or Info.

**Debug:** Contains messages primarily useful for developers to debug issues. These messages provide detailed information about the internal state and operations of the system.

**Info:** Provides general information about the processes running in the system. These messages offer insights into the regular operation and workflow, without the detailed level needed for debugging.

### Start

Save your settings and click the Play button to start your Executor.

:::info Faucet
In order to bid on transaction orders on testnet, you need to have our BRN token. You can find the [faucet link here](../../resources/faucet)
:::
