# @t3rn/cli

A CLI tool for interacting with the t3rn circuit.

## Installation

The CLI is yet to be published to NPM, so you must manually install the dependencies and build the CLI from the source. To do so, run the command below to install the CLI dependencies:

```
pnpm i
```

## Setup

To start using the cli, you need to generate a `.t3rnrc.json` config file.

```
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

### submit

The `submit` command is used to submit an extrinsic to the t3rn circuit.

Usage:

```bash
pnpm cli submit [options]
```

Options:

- **-e, --extrinsic \<file-path\>**: Path to the extrinsic JSON file
- **-h, --headers \<gateway_id\>**: Submit the latest headers of a gateway to portal. All available finalized headers will be added.

### set-operational

The `set-operational` command is used to set a gateway as operational.

Usage:

```bash
pnpm cli set-operational [gateway_id] [operational]
```

Arguments:

- **gateway_id \<string\>**: The gateway ID
- **operational \<bool\>**: The operational status

### bid

The `bid` command is used to bid on an execution as an executor.

```bash
pnpm cli [sfxId] [amount]
```

Arguments:

- **sfxId \<string\>**: sfxId of the side effect to bid on
- **amount \<float\>**: bid amount

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

# Submit the transfer extrinsic
pnpm cli submit -t transfer.json
```
