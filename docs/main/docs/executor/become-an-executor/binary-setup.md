# Binary Setup

Welcome to the t3rn Executor Setup! This guided process will help you configure your executor with ease, providing step-by-step instructions to ensure a smooth start. Let's get you set up and ready to operate efficiently across multiple blockchain networks.

## Executor Binary

### Download Executor Binary

**1.** Download the executable (`tar.gz`) Executor binary file according to your OS from here: https://github.com/t3rn/executor-release/releases/

:::info Optional: Verify the download by comparing the SHA256 checksum with the provided sha256sum file to ensure file integrity

https://github.com/t3rn/executor-release/releases/
:::

**2.** After unzip, in terminal, navigate to the folder that includes the executable file named `executor`.

### Configure Settings and Environment Required Variables

To set the environment variables, copy and paste each command into your terminal. These commands will configure the necessary settings for your Executor to run properly. Make sure you adjust the variable values to your own.

#### GENERAL SETTINGS

**1.** Set your preferred Node Environment. Example:

```bash
export NODE_ENV=testnet
```

**2.** Set your log settings:

```bash
export LOG_LEVEL=debug
export LOG_PRETTY=false
```

**3.** Process orders and claims

```
export EXECUTOR_PROCESS_ORDERS=true
export EXECUTOR_PROCESS_CLAIMS=true
```

Set both to `true` if you want your Executor to process orders and claims.

You can set them to false at any point. I.e. `export EXECUTOR_PROCESS_ORDERS=false` will stop your Executor from processing new orders.

#### PRIVATE KEYS

**1.** Set the `PRIVATE_KEY_LOCAL` variable of your Executor, which is the private key of the wallet you will use. The example below is a fake generated key that should/cannot not be used:

```bash
export PRIVATE_KEY_LOCAL=dead93c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56dbeef
```

:::tip Executor Privacy
Read more about [Executor Privacy and Security](../../resources/executor-privacy.md)
:::

#### NETWORKS & RPC

**1.** Add your preferred networks to operate on. Example:

```bash
export ENABLED_NETWORKS='arbitrum-sepolia,base-sepolia,optimism-sepolia,l1rn'
```

:::info Available networks: `arbitrum-sepolia,base-sepolia,blast-sepolia,optimism-sepolia,l1rn`

If your wallet balance falls below the threshold on one of your enabled networks, that specific network will be removed.
:::

:::info Optional
You can add your custom RPC URLs or skip this step to automatically use the default RPC URLs.

`export RPC_ENDPOINTS_${NETWORK_NAME}='https://url1.io,https://url2.io'`

Example for Arbitrum Sepolia: `export RPC_ENDPOINTS_ARBT='https://url1.io,https://url2.io'`

Supported network names: `arbt, bssp, blss, opsp`.
:::

**2.** Enable orders processing via RPC

Set `export EXECUTOR_PROCESS_PENDING_ORDERS_FROM_API=false` if you want to process orders via RPC. Set to true to process via our API.

The benefit of setting to false and using your own RPC URLs is to avoid issues caused by overloaded public RPC servers.

The default value to `EXECUTOR_PROCESS_PENDING_ORDERS_FROM_API` is true.

### Start

To start the Executor, run the following command:

```bash
./executor
```

:::info Faucet
In order to bid on transaction orders on testnet, you need to have our BRN token. You can find the [faucet link here](../../resources/faucet)
:::
