#!/bin/bash

set -Ee

if [[ -z $1 || -z $2 ]]; then
  echo "usage: $0 't0rn collator one adrs' 't0rn collator two adrs'"
  # fx: ./get-rococo-artifacts.sh 'adrs one' 'adrs two'
  exit 1
fi

t0rn_collator_one_adrs=$1
t0rn_collator_two_adrs=$2
root_dir=$(git rev-parse --show-toplevel)
collator_binary=$root_dir/target/release/circuit-collator
output_dir=$root_dir/specs

# build the collator
cargo build \
  --locked \
  --release \
  --manifest-path $root_dir/node/parachain/Cargo.toml

# gen the collator chain spec blueprint
$collator_binary build-spec \
  --chain rococo \
  --disable-default-bootnode \
> $output_dir/t0rn.json

# reset the collators node addresses - replacing alice and bob
sed "s/5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY/$t0rn_collator_one_adrs/g" \
  -i $output_dir/t0rn.json
sed "s/5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty/$t0rn_collator_two_adrs/g" \
  -i $output_dir/t0rn.json

# gen the raw collator chain spec
$collator_binary \
  build-spec \
  --chain $output_dir/t0rn.json \
  --disable-default-bootnode \
  --raw \
> $output_dir/t0rn.raw.json

# gen the collator genesis state
$collator_binary \
  export-genesis-state \
  --chain $output_dir/t0rn.raw.json \
> $output_dir/t0rn.genesis

# gen the collator genesis wasm
$collator_binary export-genesis-wasm > $output_dir/t0rn.wasm

# fetch rococo chain spec
curl -fsSL \
  -o $output_dir/rococo.raw.json \
  https://paritytech.github.io/chainspecs/rococo/relaychain/chainspec.json

# reporting
echo "📦 all artifacts stored at $output_dir..."
ls $output_dir

echo "commit, tag, and push these artifacts, wait for the pipeline to release the prebuilt collator, then ssh alibaba@000.00.00.00 'bash -s' < ./run-rococo-collators.sh v0.0.0-roco \"'secret one'\" \"'secret two'\""