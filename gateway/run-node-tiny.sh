#!/bin/bash

cd demo-runtime || exit

echo "[run-demo-runtime.sh] Building demo-runtime with 'cargo build'..."
cargo build || exit

echo "[run-demo-runtime.sh] Cleaning the demo-runtime dev database with './target/debug/demo-runtime purge-chain --dev'..."
yes | ./target/debug/demo-runtime purge-chain --dev

echo "[run-demo-runtime.sh] Running demo-runtime with './target/debug/demo-runtime --dev -lruntime=debug'..."
./target/debug/demo-runtime --dev -lruntime=debug
