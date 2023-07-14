# @t3rn/cli

A CLI tool for interacting with the t3rn circuit.

## Installation

The CLI is yet to be published to NPM, so you must manually install the dependencies and build the CLI from the source. To do so, run the command below to install the CLI dependencies:

```bash
npm i -g yarn
yarn i
```

## Setup

To start using the cli, you need to generate a `.t3rnrc.json` config file.

```bash
yarn cli init -c
```

By using this command, you will create a configuration file that is already set up for you in your current working directory.

Refer to the [configuration docs](./CONFIG.md) to learn how to configure a `.t3rnrc.json` file.

## Commands

### init

The `init` command is used to generate a config or transfer template.

Usage:

```bash
yarn cli init [options]
```

Options:

- **-c, --config [file-path]**: Generate a config template
- **-t, --transfer [file-path]**: Generate a transfer template

### register

The `register` command is used to register a gateway with the t3rn circuit.

Usage:

```bash
yarn cli register [options]
```

Options:

- **-g, --gateway \<id\>**: ID of the gateway to register
- **-x, --export**: Export extrinsic data to a file

### submit

The `submit` command is used to submit an extrinsic to the t3rn circuit.

Usage:

```bash
yarn cli submit [options]
```

Options:

- **-s, --sfx \<file-path\>**: Path to the sfx JSON file
- **-h, --headers \<gateway_id\>**: Submit the latest headers of a gateway to portal. All available finalized headers will be added.
- **-x, --export**: Export extrinsic data to a file

### bid

The `bid` command is used to bid on an execution as an executor.

```bash
yarn cli bid [sfxId] [amount]
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
yarn cli dgf [options]
```

Options:

- **-s, --sfx \<file-path\>**: Path to the sfx JSON file
- **-t, --timeout \<timeout\>**: Timeout in seconds for waiting for events from the chain. Default timeout is 30 seconds
- **-x, --export**: Export extrinsic data to a file


### estimate

The `estimate` command is used to estimate the max reward with a given target amount.

Usage:

```bash
yarn cli estimate [options]
```

Options:

- **--action \<action\>**: The execution action i.e tass, tran, swap
- **--base-asset \<symbol\>**: The base asset symbol. i.e dot, t3rn
- **--target \<name\>**: The target on which is execution will be executed. i.e 'eth', 'roco'
- **--target-asset \<symbol\>**: The target asset symbol. i.e usdt, dot 
- **--target-amount \<amount\>**: The target amount

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

### Export

Each command that interacts with the node incorporates the `-x, --export` option. This feature facilitates the export of extrinsic data to a file. By default, this data is directed to the `/exports` directory in your current working environment. To alter this default path, you need to adjust the `EXPORT_PATH` variable. Here's an example:

```bash
EXPORT_PATH="path/to/folder" yarn cli submit -h roco -x
```

## Examples

### Submit transfer

Use the following commands to submit a sample transfer:

```bash
# Generate config file
yarn cli init -c

# Generate a sample transfer file
yarn cli init -t

# Register the roco gateway
yarn cli register -g roco

# Submit a transfer
yarn cli submit -s transfer.json
```
