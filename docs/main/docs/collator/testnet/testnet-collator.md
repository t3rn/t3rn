---
sidebar_position: 1
---

# Run a t0rn Testnet Collator

This guide outlines the essential minimum of steps required to run a collator for t0rn - a release candidate of t3rn on the Rococo testnet. This guide uses latest `v1.70.2-rc.0` release.

Make sure to have your machine setup for [Rust and Substrate development](https://docs.substrate.io/v3/getting-started/installation/).

:::note
This guide assumes that everything will be run and stored in current directory.
:::

## Boot Node for t0rn

:::caution
t0rn currently has single bootnode but it might change in the future
:::

The `t0rn` boot node with static DNS address:

```sh
/dns/bootnode.t0rn.io/tcp/33333/p2p/12D3KooWEepV69XCJB4Zi193cZcm5W22ZR62DEP84iLFTUKVPtwp
```

## Prerequistes

### Setup Directories

Create the collator node's data and a specs directory:

```sh
mkdir -p {data,keystore,specs}
```

### Install subkey

Install the `subkey` tool:

```sh
cargo install subkey --git https://github.com/paritytech/substrate
```

### Fetch Chain Specs

To associate your node to the correct network we need to provide the t0rn chain spec as well as the Rococo chain specification. We need the latter as every collator runs an embedded relay chain node.

```sh
curl -sSfL \
  -o specs/rococo.raw.json \
  https://raw.githubusercontent.com/t3rn/t3rn/v1.70.2-rc.0/specs/rococo.raw.json

curl -sSfL \
  -o specs/t0rn.raw.json \
  https://raw.githubusercontent.com/t3rn/t3rn/v1.70.2-rc.0/specs/t0rn.raw.json
```

## Running Collator

We support running collator with our binary attached to Github Releases or with a Docker Image.

:::danger
When running the collator the first time, add the `--rpc-methods=unsafe` argument to be able to call rotateKeys later.  
For security reasons please restart your node after the registering the candidate process without this flag.
:::

### Optional Collator Node Key

:::tip
This is optional, only if the collator should have the same node ID after restart
:::

To generate a new Substrate node key just run:

```sh
subkey generate-node-key
```

Save the entire output to a proper secret vault or at least keep note of the secret phrase.

Add a flag for running collator
```
--node-key=<generated node key by subkey>
```

### Option 1 - Binary

#### Fetch Collator from a Github Release

We maintain collator binaries which we release alongside every runtime release. Our sole target platform is glibc based Linux. Download and extract the prebuild:

```sh
curl -sSfL \
  https://github.com/t3rn/t3rn/releases/download/v1.70.2-rc.0/t0rn-collator-v1.70.2-rc.0-x86_64-unknown-linux-gnu.gz \
| gunzip > t0rn-collator
```

Don't forget to make it executable:

```sh
chmod +x t0rn-collator
```

#### Start the Collator

```sh
./t0rn-collator \
  --collator \
  --name=<collator-name> \
  --base-path=data \
  --keystore-path=keystore \
  --chain=specs/t0rn.raw.json \
  --bootnodes="/dns/bootnode.t0rn.io/tcp/33333/p2p/12D3KooWEepV69XCJB4Zi193cZcm5W22ZR62DEP84iLFTUKVPtwp" \
  --rpc-port=9944 \
  --port=33333 \
  --prometheus-port=7003 \
  --telemetry-url='wss://telemetry.polkadot.io/submit 1' \
  --pruning=archive \
  -- \
  --chain=specs/rococo.raw.json \
  --rpc-port=9933 \
  --port=33334 \
  --prometheus-port=7004 \
  --prometheus-external \
  --sync=fast
```

### Option 2 - Docker

#### Pull latest Docker image

```sh
docker pull ghcr.io/t3rn/t0rn-collator:v1.70.2-rc.0
```

#### Start the Collator

```sh
docker run -p 33333:33333 -p 33334:33334 -p 9944:9944 -p 9933:9933 -p 7003:7003 \
  -v /node ghcr.io/t3rn/t0rn-collator:v1.70.2-rc.0 \
  --collator \
  --name=<collator-name> \
  --base-path=/node/data \
  --keystore-path=/node/keystore \
  --chain=/node/specs/t0rn.raw.json \
  --bootnodes="/dns/bootnode.t0rn.io/tcp/33333/p2p/12D3KooWEepV69XCJB4Zi193cZcm5W22ZR62DEP84iLFTUKVPtwp" \
  --rpc-port=9944 \
  --port=33333 \
  --prometheus-port=7003 \
  --telemetry-url='wss://telemetry.polkadot.io/submit 1' \
  --pruning=archive \
  -- \
  --chain=/node/specs/rococo.raw.json \
  --rpc-port=9933 \
  --port=33334 \
  --prometheus-port=7004 \
  --prometheus-external \
  --sync=fast
```

## Set Your Collator's Aura Key

Your collator needs an [Aura](https://docs.substrate.io/v3/advanced/consensus/#aura) identity in order to produce blocks.

## Aura Keypair Generation

To generate a new generic Substrate keypair just run:

```sh
subkey generate
```

Save the entire output to a proper secret vault or at least keep note of the secret phrase.

## Aura Keypair Insert

The Aura key must be inserted into the keystore *after* startup.

For the Binary Collator:
```sh
./t0rn-collator \
  key \
  insert \
  --base-path data \
  --chain specs/t0rn.raw.json \
  --scheme Sr25519 \
  --suri "<your collator's secret phrase generated in >" \
  --key-type aura
```

## Produce Blocks on t0rn Chain

### Get Some T0RN Balance

Your Collator needs some funds to register on testnet.

Go to the [t0rn testnet faucet](https://faucet.t0rn.io), insert your substrate address and get some T0RN to cover transaction costs.

### Register as Candidate for Block Production 

1. Go to the [polkadot.js app](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.t0rn.io#/accounts) and connect your collator account by clicking "Add account", then inserting your previously generated secret phrase aka mnemonic.

2. Generate a new session key pair and obtain the corresponding public key:

```
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' http://localhost:9944
```

Your output should look similar to:

```
{"jsonrpc":"2.0","result":"0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef","id":1}
```


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
You can check [here](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.t0rn.io#/collators) if your collator has registered successfully.