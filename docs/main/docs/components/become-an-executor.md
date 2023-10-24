---
sidebar_position: 2
---

# How to Become an Executor

This documentation describes how to become an executor. It assumes that you have accounts with the correct funds for the desired blockchains, and that you also have substrate installed (needed for generating keys with subkey) - we’ll share the resources to these steps.

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

## Step 3 - Configure your Executor

Configure your executor by editing `.envrc-example` and changing the values to your liking, renaming it to `.envrc` and using with [direnv](https://github.com/direnv/direnv#how-it-works).

<p align="center">
    <img height="150" src="/img/envrc.png?raw=true"/>
</p>

## Step 4 - Run executor

If you’ve configured your `.envrc` file, then simply run `pnpm start`.

<p align="center">
    <img height="150" src="/img/run-executor.png?raw=true"/>
</p>

You can also run the command below and add your keys as in the example, if you don’t want to store them unencrypted on the server
Run `CIRCUIT_SIGNER_KEY=abc RELAYCHAIN_SIGNER_KEY=xyz pnpm start`.

<p align="center">
    <img height="150" src="/img/run-executor-w-keys.png?raw=true"/>
</p>

## Troubleshooting

#### Not enough funds:

If this error **‘RpcError: 1010: Invalid Transaction: Inability to pay some fees , e.g. account balance too low’** is shown it means you need to increase your balance in used accounts, for the specific networks.

#### Generate keys with subkey:

[Install subkey](https://docs.substrate.io/reference/command-line-tools/subkey/) to generate keys. Send the correct tokens to the account(s) depending on the networks they will operate on.
