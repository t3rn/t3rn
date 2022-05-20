#!/bin/bash

EXECUTOR_KEY="${EXECUTOR_KEY:-$1}"
if [[ -z "$EXECUTOR_KEY" ]]; then
  echo "missing env var EXECUTOR_KEY" >&2
  exit 1
fi

if [ "$(uname)" == "Darwin" ]; then
    TERM_NAME=iTerm
else
    TERM_NAME=gnome-terminal
fi

set -xEeu

## build the custom justification decoder and standalone circuit
cargo build \
  --manifest-path ./justification-decoder/Cargo.toml
cargo build \
  --manifest-path ../../node/standalone/Cargo.toml

## pull all node modules
[ ! -O /usr/local/bin ] && SUDO_MAYBE=sudo
$SUDO_MAYBE npm i -g ttab
npm i @polkadot/api @polkadot/types
npm install --prefix ../grandpa-ranger
npm install --prefix ../executor

## killall leftover circuits and nodes - if any
set +e
killall circuit-standalone node
set -e

## run standalone circuit
ttab -w -a $TERM_NAME exec cargo run \
  --manifest-path ../../node/standalone/Cargo.toml \
  -- \
  --dev \
  --ws-port 9944 \
  --unsafe-ws-external \
  --rpc-cors all \
> /tmp/xtx-circuit.log

## register rococo gateway on circuit
node ./register_rococo_gateway.js

## run grandpa-ranger
ttab -w -a $TERM_NAME exec npm start --prefix ../grandpa-ranger

## run executor
SIGNER_KEY=$EXECUTOR_KEY npm start --prefix ../executor
