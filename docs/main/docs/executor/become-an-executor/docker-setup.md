# Docker Setup

Welcome to the t3rn Executor Setup! This guided process will help you configure your executor with ease, providing step-by-step instructions to ensure a smooth start. Let's get you set up and ready to operate efficiently across multiple blockchain networks.

## Executor Docker Compose

### Clone repo

**1.** Clone the `executor-release` repo from here: https://github.com/t3rn/executor-release

:::info We recommend running Executor in Docker container for ease of use and management
:::

### Configure ENVs

**1.** Any available Executor settings will be found in the `.envrc` file.

**2.** Set the `Executor private key` variable to the private key of the wallet you are going to use.

This is the most important and sensitive information in Executor. Please keep it safe and never share it with anyone.

Replace the `PRIVATE_KEY_LOCAL` in the `.envrc` file with your own.

### Configure Supported Networks & Assets

#### Pre-defined configurations

**Native:** For native assets on t3rn and other networks (e.g. ETH, BSC, etc.) run `docker compose -f docker-compose.mainnet.native.yml up`

**Tokens:** For ERC20 tokens on t3rn and other networks (e.g. USDC, DAI, t3\* tokens, etc) run `docker compose -f docker-compose.mainnet.tokens.yml up`

**TRN:** For t3rn native assets and tokens on t3rn run `docker compose -f docker-compose.mainnet.trn.yml up`

#### Custom setup

**1.** Add the networks you want to operate on in the `EXECUTOR_ENABLED_NETWORKS` variable. Example:

```bash
export EXECUTOR_ENABLED_NETWORKS='arbitrum,base,binance,ethereum,linea,,optimism,t3rn'
```

:::info Available networks

If your wallet balance falls below the threshold on one of your enabled networks, that specific network will be removed.

Here's where you can find all of our supported networks: **[Supported network names](../../resources/supported-chains.md).**
:::

**2.** Add support for your preferred assets under the `EXECUTOR_ENABLED_ASSETS` variable. Example:

```bash
export EXECUTOR_ENABLED_ASSETS="eth,t3eth,t3mon,t3sei,mon,sei"
```

### Run Executor with Docker Compose

Save all files and run Executor with `docker compose up`.
