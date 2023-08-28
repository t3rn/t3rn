# @t3rn/cli

A CLI tool for interacting with the t3rn circuit.

## Installation

The CLI is yet to be published to NPM, so you must manually install the dependencies and build the CLI from the source. To do so, run the command below to install the CLI dependencies:

```bash
npm i -g pnpm
pnpm i
```

## Setup

To start using the cli, you need to generate a `.t3rnrc.json` config file.

```bash
pnpm cli init -c
```

By using this command, you will create a configuration file that is already set up for you in your current working directory.

Refer to the [configuration docs](./CONFIG.md) to learn how to configure a `.t3rnrc.json` file.

## Commands

### init

The `init` command is used to generate a config or transfer template.

Usage:

```bash
pnpm cli init [options]
```

Options:

- **-c, --config [file-path]**: Generate a config template
- **-t, --transfer [file-path]**: Generate a transfer template

### register

The `register` command is used to register a gateway with the t3rn circuit.

Usage:

```bash
pnpm cli register [options]
```

Options:

- **-g, --gateway \<id\>**: ID of the gateway to register
- **-x, --export**: Export extrinsic data to a file

### submit

The `submit` command is used to submit an extrinsic to the t3rn circuit.

Usage:

```bash
pnpm cli submit [options]
```

Options:

- **-s, --sfx \<file-path\>**: Path to the sfx JSON file
- **-h, --headers \<gateway_id\>**: Submit the latest headers of a gateway to portal. All available finalized headers will be added.
- **-x, --export**: Export extrinsic data to a file

### bid

The `bid` command is used to bid on an execution as an executor.

```bash
pnpm cli bid [sfxId] [amount]
```

Arguments:

- **sfxId \<string\>**: sfxId of the side effect to bid on
- **amount \<float\>**: bid amount

Options:

- **-x, --export**: Export extrinsic data to a file

### dgf

The `dgf` command is used to generate side effects data with specific
error modes for testing purposes on the chain.

It allows users to simulate different failure scenarios for a specified SFX file.
These failure scenarios involve external actors, mainly executors,
and aim to ensure that transactions fail where they should.

Usage:

```bash
pnpm cli dgf [options]
```

Options:

- **-s, --sfx \<file-path\>**: Path to the sfx JSON file
- **-t, --timeout \<timeout\>**: Timeout in seconds for waiting for events from the chain. Default timeout is 30 seconds
- **-x, --export**: Export extrinsic data to a file

### estimate-gas-fee

The `estimate-gas-fee` command is used to estimate the gas fee required for an execution

Usage:

```bash
pnpm cli estimate-gas-fee [options]
```

Options:

- -t, **--target \<name\>**: The target on which is execution will be executed. i.e 'eth', 'roco'
- -a, **--action \<action\>**: The execution action i.e tass, tran, swap
- -o, **--args \<speed mode, eth estimation param or SFX JSON string\>**: The execution arguments. It's value can be a speed mode, a EVM call estimation or a side-effect JSON string
- -s, **--sfx \<file-path\>**: The SFX file path
- **--signer \<address\>**: The signer's address

Examples:
```
# Estimate the gas fee for an asset transfer on ETH target
pnpm cli estimate-gas-fee -t eth -a tass -o 'fast'

# Estimate the gas fee an EVM call on ETH target
pnpm cli estimate-gas-fee -t eth -a cevm -o '{"fromAddress":"0x1234567890AbCdEfFeDcBa09876eFfEDCBA54321","toAddress":"0x9876543210FeDcBaABcDEfFeDCbA98765EDCBA12","data":"0x0000","speedMode":"fast"}'

# Estimate the gas fee required for executing the side effect
# --signer is optional, if not provided, Alice address is used
pnpm cli estimate-gas-fee -t roco -a tass --sfx ./transfer.json --signer "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
```

### estimate-bid-amount

The `estimate-bid-amount` command is used to estimate the bid amount with a specified profit margin for an execution

Usage:

```bash
pnpm cli estimate-bid-amount [options]
```

Options:

- -t, **--target \<name\>**: The target on which is execution will be executed. i.e 'eth', 'roco'
- -a, **--action \<action\>**: The execution action i.e tass, tran, swap
- -p, **--profit-margin \<profit-margin\>**: The profit margin (%)
- -o, **--args \<speed mode, eth estimation param or SFX JSON string\>**: The execution arguments. It's value can be a speed mode, a EVM call estimation or a side-effect JSON string
- -s, **--sfx \<file-path\>**: The SFX file path
- **--signer \<address\>**: The signer's address

Examples:
```
# Estimate the bid amount for an asset transfer on ETH target with a given profit margin
pnpm cli estimate-bid-amount -t eth -a tass --profit-margin 0.1 -o 'fast'

# Estimate the bid amount for an EVM call on ETH target with a given profit margin
pnpm cli estimate-bid-amount -t eth -a cevm --profit-margin 0.1 -o '{"fromAddress":"0x1234567890AbCdEfFeDcBa09876eFfEDCBA54321","toAddress":"0x9876543210FeDcBaABcDEfFeDCbA98765EDCBA12","data":"0x0000","speedMode":"fast"}'

# Estimate the bid amount for executing the side effect with a given profit margin
# --signer is optional, if not provided, Alice address is used
pnpm cli estimate-bid-amount -t roco -a tass --profit-margin 0.1 --sfx ./transfer.json --signer "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
```

### estimate-max-reward

The `estimate-max-reward` command is used to estimate the max reward with a given target amount.

Usage:

```bash
pnpm cli estimate-max-reward [options]
```

Options:

- **--action \<action\>**: The execution action i.e tass, tran, swap
- **--base-asset \<symbol\>**: The base asset symbol. i.e dot, t3rn
- **--target \<name\>**: The target on which is execution will be executed. i.e 'eth', 'roco'
- **--target-asset \<symbol\>**: The target asset symbol. i.e usdt, dot 
- **--target-amount \<amount\>**: The target amount
- **--over-spend \<amount\>**: The percentage of the target amount to be used as a profit margin
- -o, **--args \<speed mode, eth estimation param or SFX JSON string\>**: The execution arguments. It's value can be a speed mode, a EVM call estimation or a side-effect JSON string
- -s, **--sfx \<file-path\>**: The SFX file path
- **--signer \<address\>**: The signer's address

A sample estimate will look like this:
| Index               | 0                                       | 1                                        | Value                  | Symbol |
|---------------------|-----------------------------------------|------------------------------------------|------------------------|--------|
| gasFee              | { value: 0.000634745609043, symbol: 'eth' } | { value: 0.2247957921881875, symbol: 'dot' } |                        |        |
| executorFeeEstimate |                                         |                                          | 0.00017758193000000003 | 'dot'  |
| maxReward           |                                         |                                          | 0.5801372341181875     | 'dot'  |
| estimatedValue      |                                         |                                          | 0.35516386             | 'dot'  |


Below is a detailed explanation of the estimation results for a blockchain transaction. Each field provides crucial information about the costs, rewards, and values associated with the transaction:

- **gasFee**: This field represents the cost of computational resources required to execute a transaction on the target blockchain network. It is calculated in the native asset of the target network. For debugging purposes, we also provide the gas fee converted into the base asset.

- **executorFeeEstimate**: This field provides an estimated fee that will be paid to the executor of a transaction. It is calculated as an overspent percentage over the target amount and then converted into the base asset. The executor is the entity that processes and validates the transaction on the blockchain.

- **maxReward**: This field represents the maximum reward for executing the transaction. It is calculated as the sum of the gas fee estimate, the executor fee estimate, and the target amount involved in the transaction. The max reward provides an upper limit on the total cost of the transaction, including all fees and the transaction amount itself. It is estimated in the base asset.

- **estimatedValue**: This field represents the estimated value of the target amount in the base asset. It is included primarily for debugging purposes and provides a way to understand the value of the transaction in terms of the base asset.

Please note that these estimations are subject to change based on the state of the blockchain network at the time of the transaction, and they serve as a guide to understanding the potential costs and rewards associated with a transaction.

Examples:
```
# Estimate the max reward of an asset transfer DOT -> ETH on Seploia target with a given over spend percent
pnpm cli estimate-max-reward --action tass --base-asset dot --target sepl --target-asset eth --target-amount 10 --over-spend 0.1 -o 'fast'

# Estimate the max reward of an asset transfer DOT -> ACA  on Rococo target with a given over spend percent
# --signer is optional, if not provided, Alice address is used
pnpm cli estimate-max-reward --action tass --base-asset DOT --target roco --target-asset aca --target-amount 10 --over-spend 0.1 -s ./transfer.json --signer "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
```

### Export

Each command that interacts with the node incorporates the `-x, --export` option. This feature facilitates the export of extrinsic data to a file. By default, this data is directed to the `/exports` directory in your current working environment. To alter this default path, you need to adjust the `EXPORT_PATH` variable. Here's an example:

```bash
EXPORT_PATH="path/to/folder" pnpm cli submit -h roco -x
```

## Examples

### Submit transfer

Use the following commands to submit a sample transfer:

```bash
# Generate config file
pnpm cli init -c

# Generate a sample transfer file
pnpm cli init -t

# Register the roco gateway
pnpm cli register -g roco

# Submit a transfer
pnpm cli submit -s transfer.json
```
