# How 2 Run a t0rn Collator

This guide outlines the essential minimum of steps required to run a collator in the t0rn testnet. We will be working on top our `v1.0.0-rc.2` release.

Make sure to have your machine setup for [Rust and Substrate development](https://docs.substrate.io/v3/getting-started/installation/).

## Generate a Collator Keypair

Install the `subkey` tool:

```sh
cargo install subkey --version 2.0.1 --git https://github.com/paritytech/substrate
```

To generate a new generic Substrate keypair just run:

```sh
subkey generate
```

Save the entire output to a proper secret vault or at least keep note of the secret phrase.

## Setup Directories

Create the collator node's data and a specs directory:

```sh
mkdir -p ~/t0rn/{data,specs}
```

## Install a Prebuilt Collator

We maintain collator binaries which we release alongside every runtime release. Our sole target platform is glibc based Linux. Download and extract the prebuild:

```sh
curl -sSfL \
  https://github.com/t3rn/t3rn/releases/download/v1.0.0-rc.2/t0rn-circuit-collator-v1.0.0-rc.2-x86_64-unknown-linux-gnu.gz \
| gunzip > ~/t0rn/circuit-collator
```

Don't forget to make it executable:

```sh
chmod +x ~/t0rn/circuit-collator
```

## Fetch Chain Specs

To associate your node to the correct network we need to provide the t0rn chain spec as well as the Rococo chain specification. We need the latter as every collator runs an embedded relay chain node.

```sh
curl -sSfL \
  -o ~/t0rn/specs/rococo.raw.json \
  https://raw.githubusercontent.com/t3rn/t3rn/v1.0.0-rc.2/specs/rococo.raw.json

curl -sSfL \
  -o ~/t0rn/specs/t0rn.raw.json \
  https://raw.githubusercontent.com/t3rn/t3rn/v1.0.0-rc.2/specs/t0rn.raw.json
```

We publish these chain specs alongside our runtime releases.

Also, select a Rococo boot node:

```sh
rococo_boot_node="$(jq -r .bootNodes[0] ~/t0rn/specs/rococo.raw.json)"
```

The `t0rn` boot node reads:

```sh
t0rn_boot_node=/ip4/159.69.77.34/tcp/33333/p2p/12D3KooWBqic8h4nQS2KK751rdkqYPFTWxSo1keuvenBdDKzdTCf
```

## Start the Collator

```sh
~/t0rn/circuit-collator \
  --collator \
  --name my-collator \
  --base-path ~/t0rn/data \
  --chain ~/t0rn/specs/t0rn.raw.json \
  --bootnodes "$t0rn_boot_node" \
  --port 33333 \
  --rpc-port 8833 \
  --ws-port 9933 \
  --execution Wasm \
  --pruning=archive \
  -- \
  --chain ~/t0rn/specs/rococo.raw.json \
  --bootnodes "$rococo_boot_node" \
  --port 10001 \
  --rpc-port 8001 \
  --ws-port 9001 \
  --execution Wasm
```

## Set Your Collator's Aura Key

Your collator needs an [Aura](https://docs.substrate.io/v3/advanced/consensus/#aura) identity in order to produce blocks.

The Aura key must be inserted into the keystore *after* startup:

```sh
~/t0rn/circuit-collator \
  key \
  insert \
  --base-path ~/t0rn/data \
  --chain ~/t0rn/specs/t0rn.raw.json \
  --scheme Sr25519 \
  --suri "your collator's secret phrase ..." \
  --key-type aura
```

Your collator should be running and also produce blocks eventually!

## Troubleshooting

+ Our testnet got a temporary Rococo slot, meaning `t0rn` and other parachains will be on- and offboarded to Rococo in a round-robin fashion. When offboarded collators do not produce blocks.