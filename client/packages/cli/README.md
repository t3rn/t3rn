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
- **-t, --timeout \<timeout\>**: Timeout in seconds for wating for events from the chain
- **-x, --export**: Export extrinsic data to a file

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
