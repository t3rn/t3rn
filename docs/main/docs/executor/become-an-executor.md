---
sidebar_position: 2
---

# How to Become an Executor

This documentation covers how to become an Executor on the t3rn network. It assumes that you have accounts with the correct funds for the desired blockchains, and that you have Substrate installed. We're using subkey to generate keys but feel free to skip this part if you already have your keys or using another tool to generate some.

## Step 1 - Clone Repository

Clone the [Executor repository](https://github.com/t3rn/executor) by running `git clone git@github.com:t3rn/executor.git`.

## Step 2 - Installation Instructions

Follow the installation instructions in the readme: https://github.com/t3rn/executor/blob/main/README.md.

## Step 3 - Configure Settings

Either in `.env` or `.envrc`:

1. Add your `PRIVATE_KEY_EXECUTOR`
2. Add your preferred networks under `ENABLED_NETWORKS`
3. Add your preferred environment under `NODE_ENV`
4. Create your arbitrage strategies in `executor-arbitrage-strategies.ts`

## Step 3 - Run Executor

Start Executor by running `pnpm start:executor` in the terminal
