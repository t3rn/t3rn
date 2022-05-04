#!/bin/bash

set -xEeu

## build the custom justification decoder and standalone circuit
cargo build \
  --manifest-path ./justification-decoder/Cargo.toml \
  --release
cargo build \
  --manifest-path ../../node/standalone/Cargo.toml \
  --release

## killall any leftover circuits
killall circuit-standalone

## run standalone circuit
cargo run \
  --manifest-path ../../node/standalone/Cargo.toml \
  --release \
  -- \
  --dev \
  --ws-port 9944 \
> /tmp/xtx-circuit.log 2>&1 &

# await circuit ws rpc available
tail -f /tmp/xtx-circuit.log | sed '/Listening for new connections on 127.0.0.1:9944/ q'

## register rococo gateway on circuit
npm i @polkadot/api @polkadot/types
node ./register_rococo_gateway.js

## run grandpa-ranger
npm start --prefix ../grandpa-ranger &

## run executor
npm start --prefix ../executor &