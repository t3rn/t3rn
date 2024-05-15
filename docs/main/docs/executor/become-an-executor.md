---
sidebar_position: 2
---

# How to Become an Executor

This documentation covers how to become an Executor on the t3rn network. It assumes that you have accounts with the correct funds for the desired blockchains, and that you have Substrate installed. We're using subkey to generate keys but feel free to skip this part if you already have your keys or using another tool to generate some.

## Step 1 - Clone repo

Clone the [t3rn repo](https://github.com/t3rn/t3rn/tree/development/client/packages/executor): `git clone git@github.com:t3rn/t3rn.git`.

<p align="center">
    <img height="150" src="/img/clone-executor-repo.png?raw=true"/>
</p>

## Step 2 - Install dependencies

`pnpm install` in the above directory (t3rn/client/packages/executor/).

<p align="center">
    <img height="150" src="/img/install-executor-dep.png?raw=true"/>
</p>

## Step 3 - Run Executor

Run the command below and add your keys as in the example.
Run `LOG_PRETTY=true CIRCUIT_WS_ENDPOINT=wss://rpc.t0rn.io CIRCUIT_SIGNER_KEY=0x1234 RELAYCHAIN_SIGNER_KEY=0x1234 pnpm start`.

<p align="center">
    <img height="150" src="/img/run-executor-w-keys.png?raw=true"/>
</p>

## Troubleshooting

#### Not enough funds:

If this error **‘RpcError: 1010: Invalid Transaction: Inability to pay some fees , e.g. account balance too low’** is shown it means you need to increase your balance in used accounts, for the specific networks.

Visit [https://faucet.t0rn.io](https://faucet.t0rn.io/) to get some T0RN.

#### Generate keys with subkey:

[Install Subkey](https://docs.substrate.io/reference/command-line-tools/subkey/) and run `subkey generate` to generate keys. Send the correct tokens to the account(s) depending on the networks they will operate on.
