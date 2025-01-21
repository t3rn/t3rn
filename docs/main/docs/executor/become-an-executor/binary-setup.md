# Binary Setup

Welcome to the t3rn Executor Setup! This guided process will help you configure your executor with ease, providing step-by-step instructions to ensure a smooth start. Let's get you set up and ready to operate efficiently across multiple blockchain networks.

## Executor Binary

### Download Executor Binary

**1.** Download the executable (`tar.gz`) Executor binary file according to your OS from here: https://github.com/t3rn/executor-release/releases/

:::info Optional: Verify the download by comparing the SHA256 checksum with the provided sha256sum file to ensure file integrity

https://github.com/t3rn/executor-release/releases/
:::

### Installation Steps

#### For Ubuntu

```bash
# Create and navigate to t3rn directory
mkdir t3rn
cd t3rn

# Download latest release
curl -s https://api.github.com/repos/t3rn/executor-release/releases/latest | \
grep -Po '"tag_name": "\K.*?(?=")' | \
xargs -I {} wget https://github.com/t3rn/executor-release/releases/download/{}/executor-linux-{}.tar.gz

# Extract the archive
tar -xzf executor-linux-*.tar.gz

# Navigate to the executor binary location
cd executor/executor/bin
```

#### For macOS

```bash
# Create and navigate to t3rn directory
mkdir t3rn
cd t3rn

# Download latest release
curl -s https://api.github.com/repos/t3rn/executor-release/releases/latest | \
grep -o '"tag_name": "[^"]*' | \
cut -d'"' -f4 | \
xargs -I {} curl -LO https://github.com/t3rn/executor-release/releases/download/{}/executor-macos-{}.tar.gz

# Extract the archive
tar -xzf executor-macos-*.tar.gz

# Navigate to the executor binary location
cd executor/executor/bin
```

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

```bash
export EXECUTOR_PROCESS_ORDERS_ENABLED=true
export EXECUTOR_PROCESS_CLAIMS_ENABLED=true
```

Set both to `true` if you want your Executor to process orders and claims.

You can set them to false at any point. I.e. `export EXECUTOR_PROCESS_ORDERS_ENABLED=false` will stop your Executor from processing new orders.

**4.** Specify limit on gas usage

This will stop your Executor from running if gas rises above this level. The value is in gwei.
The default is 10 gwei.

Example: `export EXECUTOR_MAX_L3_GAS_PRICE=100`

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
export RPC_ENDPOINTS_L1RN='https://brn.rpc.caldera.xyz/'
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

**2.** Enable orders processing via API

The default value to `EXECUTOR_PROCESS_ORDERS_API_ENABLED` is true.

The benefit of having this set to true and using the API to process orders is higher reliability in executions and that all events are included.

Set `export EXECUTOR_PROCESS_ORDERS_API_ENABLED=false` if you want to process orders via RPC. Set to true to process via our API.

### Running the Executor

#### Start

To start the Executor, run the following command:

```bash
./executor
```

#### Running in Background

##### Option 1: Using Screen (Recommended for Beginners)

```bash
# Install screen (Ubuntu)
sudo apt-get install screen

# Create and start a new screen session
screen -S t3rn-executor

# Start the executor in the screen session
./executor

# To detach: Press Ctrl + A, then D
# To reattach: screen -r t3rn-executor
```

##### Option 2: Using tmux (Modern Alternative)

```bash
# Install tmux (Ubuntu)
sudo apt-get install tmux

# Create and start new session
tmux new -s t3rn-executor

# Start the executor in the tmux session
./executor

# To detach: Press Ctrl + B, then D
# To reattach: tmux attach -t t3rn-executor
```

##### Option 3: Using systemd (Ubuntu Only)

For a permanent solution that starts automatically on boot, you can create a systemd service. Contact your system administrator or refer to systemd documentation for setup.

:::info Faucet
In order to bid on transaction orders on testnet, you need to have our BRN token. You can find the [faucet link here](../../resources/faucet)
:::

### Verification & Troubleshooting

To verify the executor is running correctly:

1. Check the terminal output for any error messages
2. Monitor the logs using the configured log level
3. Verify network connections to enabled networks

If you encounter issues:

- Verify all environment variables are set correctly
- Ensure your private key is valid
- Check network connectivity to enabled networks
- Verify sufficient balance in your wallet for each network

For further assistance, join our [Discord community](https://discord.com/invite/S5kHFQTtp6) or watch the comprehensive setup guide below.

## Executor Binary Walkthrough

<iframe width="560" height="315" src="https://www.youtube.com/embed/KYFWwV6ZkLY?si=OQ3JVyh45XmdCygI" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
