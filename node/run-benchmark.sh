#!/usr/bin/env bash

# Usage: sh run-benchmark.sh pallet_name location_to_generate_weights_file
# Example for Execution-Delivery: sh run-benchmark.sh pallet_circuit_execution_delivery ./src/execution-delivery/src/weights.rs

set -eux

pallet=$1
output=$2

echo "Benchmark: ${pallet}"
cargo +nightly run --release --features runtime-benchmarks -- benchmark \
  --chain=dev \
  --steps=50 \
  --repeat=100 \
  --pallet="${pallet}" \
  --extrinsic=* \
  --execution=wasm \
  --wasm-execution=compiled \
  --heap-pages=4096 \
  --output="${output}" \
  --template=../benchmarking/frame-weight-template.hbs

# Weights file may not be formatted 
cargo fmt