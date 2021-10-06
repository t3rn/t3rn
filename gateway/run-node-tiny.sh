#!/bin/bash

echo "[demo-gateway-runtime.sh] Building demo-gateway-runtime with 'cargo build'..."
cargo build || exit

echo "[gateway-runtime.sh] Cleaning the gateway-runtime dev database with './target/debug/gateway-runtime purge-chain --dev'..."
yes | ./target/debug/gateway-runtime purge-chain --dev

echo "[gateway-runtime.sh] Running demo-runtime with './target/debug/gateway-runtime --dev -lruntime=debug'..."
./target/debug/gateway-runtime --dev -lruntime=debug
