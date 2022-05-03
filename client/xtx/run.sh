#!/bin/bash
# extravaganza script

set -x

## build the custom justification decoder
cargo build --release --manifest-path ./justification-decoder/Cargo.toml

## run standalone circuit
SKIP_WASM_BUILD=1 cargo run \
  --manifest-path ../../node/standalone/Cargo.toml \
  -- \
  --dev \
  --ws-port 9944 \
  -lmulti-finality-verifier=debug,circuit-portal=debug \
  &

## register rococo gateway
wd=$(pwd)
tmp_dir=$(mktemp -d)
cp $wd/register_rococo_gateway.js $tmp_dir/register_rococo_gateway.js
cd $tmp_dir
npm i @polkadot/api @polkadot/types
node $tmp_dir/register_rococo_gateway.js
cd $wd

## run grandpa-ranger


## run executor