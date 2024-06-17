---
sidebar_position: 1
---

# Binary Setup

Welcome to the t3rn Executor Setup! This guided process will help you configure your executor with ease, providing step-by-step instructions to ensure a smooth start. Let's get you set up and ready to operate efficiently across multiple blockchain networks.

## Executor Binary - macOS v.0.8.5

### Download Executor Binary

**1.** Download the Executor binary zip file according to your OS from here: https://github.com/t3rn/executor/releases.

**2.** After unzip, in terminal, navigate to the folder that includes the executable file named `executor`.

### Configure Settings and Environment Required Variables

To set the environment variables, copy and paste each command into your terminal. These commands will configure the necessary settings for your Executor to run properly. Make sure you adjust the variable values to your own.

#### GENERAL SETTINGS

**1.** Set your preferred Node Environment. Example:

```bash
export NODE_ENV=testnet
```

:::info Devnet Example

```bash
export NODE_ENV=devnet
```

:::

**2.** Set your log settings:

```bash
export LOG_LEVEL=debug
export LOG_PRETTY=true
```

#### PRIVATE KEYS

**1.** Set the `PRIVATE_KEY_LOCAL` variable of your Executor, which is the private key of the wallet you will use. Example:

```bash
export PRIVATE_KEY_LOCAL=0xdead93c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56dbeef
```

#### EXECUTOR SETTINGS

```bash
export EXECUTOR_MIN_BALANCE_THRESHOLD_ETH='0.1'
export SUPPORT_TOKENS=true
export EXCLUDED_LIFECYCLES_FROM_BATCHING='Bid,Attest'
```

#### NETWORKS & RPC

**1.** Add your preferred networks to operate on. Example:

```bash
export ENABLED_NETWORKS='arbitrum-sepolia,base-sepolia,optimism-sepolia,l1rn'
```

:::info For devnet, change 1 to 0, and for testnet, use 1.

Devnet Example

```bash
export ENABLED_NETWORKS='arbitrum-sepolia,base-sepolia,optimism-sepolia,l0rn'
```

:::

**2.** Set RPC check interval.

```bash
export RPC_HEALTH_CHECK_INTERVAL_SEC=3
```

#### PRICER

**1.** Set the `PRICER_URL` and `PRICER_CORS_ORIGINS` variables. Example:

```bash
export PRICER_URL='https://pricer.t1rn.io'
export PRICER_CORS_ORIGINS='https://bridge.t1rn.io'
```

#### BATCH

```bash
export BATCH_SIZE=5
export BATCH_CREATION_TIMEOUT_SEC=30
```

#### BATCH

```bash
export PROMETHEUS_PORT_GUARDIAN=9333
export PROMETHEUS_PORT_EXECUTOR=9334
export PROMETHEUS_PORT_FASTWRITER=9335
```

### Start

To start the Executor, run:

```bash
./executor
```
