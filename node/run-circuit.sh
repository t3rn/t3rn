#!/bin/bash
set -xeu

cargo clean -p circuit

echo "[run-circuit.sh] Building circuit with 'cargo build'..."
cargo build || exit

echo "[run-circuit.sh] Cleaning the circuit dev database with './target/debug/circuit purge-chain --dev'..."
yes | ./target/debug/circuit purge-chain --dev

echo "[run-circuit.sh] Running circuit with './target/debug/circuit --dev -lruntime=debug'..."
./target/debug/circuit --dev -lruntime=debug
