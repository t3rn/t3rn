# Docker Setup

Welcome to the t3rn Executor Setup! This guided process will help you configure your executor with ease, providing step-by-step instructions to ensure a smooth start. Let's get you set up and ready to operate efficiently across multiple blockchain networks.

Executor is a core component of the t3rn ecosystem, responsible for executing cross-chain transactions and bridging assets across different networks. It is designed to be modular and extensible, allowing it to support various assets and networks seamlessly.

## Prerequisites

### Docker and Docker Compose Installation

#### For Ubuntu

```bash
# Update package index
sudo apt-get update

# Install Docker
sudo apt-get install docker.io

# Install Docker Compose
sudo apt-get install docker-compose

# Add your user to the docker group (optional, to run without sudo)
sudo usermod -aG docker $USER
```

#### For macOS

```bash
# Install Docker Desktop (includes Docker Compose)
brew install --cask docker

# Start Docker Desktop from Applications
```

### Verification (Optional)

```bash
# Verify Docker installation
docker --version

# Verify Docker Compose installation
docker-compose --version
```

## Configure Executor

### Private Key

The most important and sensitive information in `executor` is the `PRIVATE_KEY`. Make sure to keep it safe.

Depending on which executor flavour you want to run we recommend adding file which will keep it. Using environment variables is less safe.

For **mainnet native** it's `./secrets/private-key-mainnet-native`
For **mainnet tokens** it's `./secrets/private-key-mainnet-tokens`
For **mainnet trn** it's `./secrets/private-key-mainnet-trn`

:::tip Executor Privacy
Read more about [Executor Privacy and Security](../../resources/executor-privacy.md)
:::

### Funds

Make sure to have sufficient funds in your wallet for each network you want to bridge assets on.
Each executor is obligated to participate in Bid Auction and needs to have sufficient balance to cover the gas fees for the transactions in TRN on t3rn network.

### Environment Variables

Please refer to the [documentation](https://docs.t3rn.io) and `.envrc` to see available `executor` settings.
Each setting can be overridden in the `docker compose`, which takes precedence over the `.envrc` file.

## Running with Docker Compose

### Pre-defined Configurations

For mainnet there are three different executor configurations available:

**Native**: For native assets on t3rn and other networks (e.g. ETH, BSC, etc.)
```bash
docker compose -f docker-compose.mainnet.native.yml up
```

**Tokens**: For ERC20 tokens on t3rn and other networks (e.g. USDC, DAI, t3* tokens, etc.)
```bash
docker compose -f docker-compose.mainnet.tokens.yml up
```

**TRN**: For t3rn native assets and tokens on T3rn
```bash
docker compose -f docker-compose.mainnet.trn.yml up
```

Running the latest executor for testnet:
```bash
docker compose up
```

:::warning
It is not advised to use the `latest` image as it can contain breaking changes. We strongly recommend pinning the executor version in your Docker Compose file.
:::

:::info
Updating executor image can be done by running `docker compose pull`
:::

### Custom Setup

**1.** Add the networks you want to operate on in the `EXECUTOR_ENABLED_NETWORKS` variable. Example:

```bash
export EXECUTOR_ENABLED_NETWORKS='arbitrum,base,binance,ethereum,linea,optimism,t3rn'
```

:::info Available networks
If your wallet balance falls below the threshold on one of your enabled networks, that specific network will be removed.

Here's where you can find all of our supported networks: **[Supported network names](../../resources/supported-chains.md).**
:::

**2.** Add support for your preferred assets under the `EXECUTOR_ENABLED_ASSETS` variable. Example:

```bash
export EXECUTOR_ENABLED_ASSETS="eth,t3eth,t3mon,t3sei,mon,sei"
```

**3.** Configure RPC URL's

You can add your custom RPC URLs or skip this step to automatically use the default RPC URLs.

Example:
```bash
export RPC_ENDPOINTS='{
    "l2rn": ["https://t3rn-b2n.blockpi.network/v1/rpc/public", "https://b2n.rpc.caldera.xyz/http"],
    "arbt": ["https://arbitrum-sepolia.drpc.org", "https://sepolia-rollup.arbitrum.io/rpc"],
    "bast": ["https://base-sepolia-rpc.publicnode.com", "https://base-sepolia.drpc.org"],
    "blst": ["https://sepolia.blast.io", "https://blast-sepolia.drpc.org"],
    "mont": ["https://testnet-rpc.monad.xyz"],
    "opst": ["https://sepolia.optimism.io", "https://optimism-sepolia.drpc.org"],
    "unit": ["https://unichain-sepolia.drpc.org", "https://sepolia.unichain.org"]
}'
```

You can add multiple RPCs for each network by separating with comma, here's an example:
`"arbt": ["https://arbitrum-sepolia.drpc.org", "https://sepolia-rollup.arbitrum.io/rpc"]`

A good resource for finding RPC URLs for EVM networks is [ChainList](https://chainlist.org/).

**4.** Enable orders processing via API

The default value to `EXECUTOR_PROCESS_PENDING_ORDERS_FROM_API` is true.

The benefit of having this set to true and using the API to process orders is higher reliability in executions and that all events are included.

Set `export EXECUTOR_PROCESS_PENDING_ORDERS_FROM_API=false` if you want to process orders via RPC. Set to true to process via our API.

## Running Executor in Background

### Option 1: Using Screen (Recommended for Beginners)

Screen allows you to run the executor in the background and detach/reattach to the session:

```bash
# Install screen (Ubuntu)
sudo apt-get install screen

# Create and start a new screen session
screen -S t3rn-executor

# Start the executor in the screen session
docker compose -f docker-compose.mainnet.native.yml up

# To detach: Press Ctrl + A, then D
# To reattach: screen -r t3rn-executor
```

### Option 2: Using tmux (Modern Alternative)

tmux is a modern terminal multiplexer with similar functionality to screen:

```bash
# Install tmux (Ubuntu)
sudo apt-get install tmux

# Create and start a new session
tmux new -s t3rn-executor

# Start the executor in the tmux session
docker compose -f docker-compose.mainnet.native.yml up

# To detach: Press Ctrl + B, then D
# To reattach: tmux attach -t t3rn-executor
```

### Option 3: Using systemd (Ubuntu Only, For Advanced Users)

For a permanent solution that starts automatically on boot, you can create a systemd service. Contact your system administrator or refer to systemd documentation for setup.

### Option 4: Docker Compose Detached Mode

You can also run Docker Compose in detached mode:

```bash
docker compose -f docker-compose.mainnet.native.yml up -d
```

To view logs:
```bash
docker compose -f docker-compose.mainnet.native.yml logs -f
```

To stop:
```bash
docker compose -f docker-compose.mainnet.native.yml down
```

## Troubleshooting

If you encounter issues:
- Verify all environment variables are set correctly.
- Ensure your private key is valid (**always keep your private key secret and never reveal it to anyone**)
- Check network connectivity to enabled networks.
- Verify sufficient balance in your wallet for each network.

For further assistance, join our [Discord community](https://discord.com/invite/S5kHFQTtp6) or watch the comprehensive [YouTube setup guide](https://youtu.be/KYFWwV6ZkLY).
