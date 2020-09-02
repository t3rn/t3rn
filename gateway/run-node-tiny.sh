#!/bin/bash

cd node-tiny || exit

echo "[run-node-tiny.sh] Building node-tiny with 'cargo build'..."
cargo build || exit

echo "[run-node-tiny.sh] Cleaning the node-tiny dev database with './target/debug/node-tiny purge-chain --dev'..."
yes | ./target/debug/node-tiny purge-chain --dev

echo "[run-node-tiny.sh] Running node-tiny with './target/debug/node-tiny --dev -lruntime=debug'..."
./target/debug/node-tiny --dev -lruntime=debug
