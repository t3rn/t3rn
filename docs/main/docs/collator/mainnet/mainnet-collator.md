# Run a t3rn mainnet Collator

This guide outlines the essential minimum of steps required to run a collator for t3rn - a release candidate of t3rn on the Polkadot mainnet. This guide uses the `v1.0.0` release, however always use the latest available version.

:::warning
The mainnet collator is currently **not** available and will be rolled out once we are live on Polkadot.
:::

Make sure to have your machine setup for [Rust and Substrate development](https://docs.substrate.io/install/).

## Generate a Collator Keypair

Install the `subkey` tool:

```sh
cargo install subkey --git https://github.com/paritytech/polkadot-sdk
```

To generate a new generic Substrate keypair just run:

```sh
subkey generate
```

Save the entire output to a proper secret vault or at least keep note of the secret phrase.

## Setup Directories

Create the collator node's data and a specs directory:

```sh
mkdir -p ~/t3rn/{data,specs}
```

## Option 1: Install a Prebuilt Collator

We maintain collator binaries which we release alongside every runtime release. Our sole target platform is glibc based Linux. Download and extract the prebuild:

```sh
curl -sSfL \
  https://github.com/t3rn/t3rn/releases/download/v1.0.0/t3rn-collator-v1.0.0-x86_64-unknown-linux-gnu.gz \
| gunzip > ~/t3rn/circuit-collator
```

Don't forget to make it executable:

```sh
chmod +x ~/t3rn/circuit-collator
```

## Fetch Chain Specs (Binary)

To associate your node to the correct network we need to provide the t3rn chain spec as well as the Polkadot chain specification. We need the latter as every collator runs an embedded relay chain node.

```sh
curl -sSfL \
  -o ~/t3rn/specs/rococo.raw.json \
  https://raw.githubusercontent.com/t3rn/t3rn/v1.0.0/specs/polkadot.raw.json

curl -sSfL \
  -o ~/t3rn/specs/t3rn.raw.json \
  https://raw.githubusercontent.com/t3rn/t3rn/v1.0.0/specs/t3rn.raw.json
```

## Option 2: Pull latest Docker image

```sh
docker pull ghcr.io/t3rn/t3rn-collator:v1.0.0
```



## Select Boot Nodes
We publish these chain specs alongside our runtime releases.

Also, select a `Polkadot` boot node:

```sh
polkadot_boot_node="$(jq -r .bootNodes[0] ~/t3rn/specs/polkadot.raw.json)"
```
or:
```sh
polkadot_boot_node=/ip4/34.90.151.124/tcp/30333/p2p/12D3KooWF7BUbG5ErMZ47ZdarRwtpZamgcZqxwpnFzkhjc1spHnP
```



The `t3rn` boot node reads:

:::warning
The mainnet bootnode is not running yet.
:::

```sh
t3rn_boot_node=/dns/bootnode.t3rn.io/tcp/33333/p2p/12D3KooWBqic8h4nQS2KK751rdkqYPFTWxSo1keuvenBdDKzdTCf
```

## Option 1: Start the Collator (Binary)

```sh
~/t3rn/circuit-collator \
  --collator \
  --name my-collator \
  --base-path ~/t3rn/data \
  --chain ~/t3rn/specs/t3rn.raw.json \
  --bootnodes "$t3rn_boot_node" \
  --port 33333 \
  --rpc-port 8833 \
  --prometheus-port 7001 \
  --telemetry-url 'wss://telemetry.polkadot.io/submit 1' \
  --ws-port 9933 \
  --execution Wasm \
  --pruning=archive \
  --state-cache-size 0 \
  -- \
  --chain ~/t3rn/specs/polkadot.raw.json \
  --bootnodes "$polkadot_boot_node" \
  --port 10001 \
  --rpc-port 8001 \
  --ws-port 9001 \
  --execution Wasm
```

## Option 2: Start the Collator (Docker image)

```sh
docker run -p 33333:33333 -p 8833:8833 -p 9933:9933 \
  -v /node ghcr.io/t3rn/t3rn-collator:v1.0.0 \
  --collator \
  --name genius \
  --base-path /node \
  --chain /node/specs/t3rn.raw.json \
  --bootnodes "$t3rn_boot_node" \
  --execution Wasm \
  --pruning=archive \
  --state-cache-size 0 \
  -- \
  --chain /node/specs/polkadot.raw.json \
  --bootnodes "$polkadot_boot_node" \
  --port 10001 \
  --rpc-port 8001 \
  --prometheus-port 7001 \
  --telemetry-url 'wss://telemetry.polkadot.io/submit 1' \
  --ws-port 9001 \
  --execution Wasm
```

When running the collator the first time, add the `--rpc-methods=unsafe` argument to be able to call rotateKeys later.
Please restart your node after the registration process without the argument.


## Set Your Collator's Aura Key

Your collator needs an [Aura](https://docs.substrate.io/v3/advanced/consensus/#aura) identity in order to produce blocks.

The Aura key must be inserted into the keystore *after* startup.

For the Binary Collator:
```sh
~/t3rn/circuit-collator \
  key \
  insert \
  --base-path ~/t3rn/data \
  --chain ~/t3rn/specs/t3rn.raw.json \
  --scheme Sr25519 \
  --suri "your collator's secret phrase ..." \
  --key-type aura
```

For the Docker Collator:
`cd /usr/local/bin/`
```sh
circuit-collator \
  key \
  insert \
  --base-path ~/t3rn/data \
  --chain ~/t3rn/specs/t3rn.raw.json \
  --scheme Sr25519 \
  --suri "your collator's secret phrase ..." \
  --key-type aura
```

## Get Some t3rn Balance

Your Collator needs 10.000 TRN funds to register on mainnet.

## Register as a candidate

1. Go to the [polkadot.js app](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fws.t0rn.io#/accounts) and connect your collator account by clicking "Add account", then inserting your previously generated secret phrase aka mnemonic.

2. Generate a new session key pair and obtain the corresponding public key:

```
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' http://localhost:8833
```

Your output should look similar to:

``{"jsonrpc":"2.0","result":"0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef","id":1}``


The `result` key is your public key of the newly created session key pair. Copy it as it is needed in the next step.

3. Set the session key for your collator under:

```
Developer --> Extrinsics --> session -> setKeys(sr25519_pubkey, 0x00)
```

4. Now finally register your collator as candidate under:

```
Developer --> Extrinsics --> collatorSelection -> registerAsCandidate()
```

After some time your collator should be included and producing blocks!
You can check [here](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fws.t0rn.io#/collators) if your collator has registered successfully.