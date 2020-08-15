#!/bin/bash

cd node-full/substrate || exit

echo "[run-node-full.sh] Building node-full with 'cargo build'..."
cargo build || exit

echo "[run-node-full.sh] Cleaning the node-full dev database with './target/debug/substrate purge-chain --dev'..."
yes | ./target/debug/substrate purge-chain --dev

echo "[run-node-full.sh] Running node-full with './target/debug/substrate --dev -lruntime=debug'..."
./target/debug/substrate --dev -lruntime=debug
