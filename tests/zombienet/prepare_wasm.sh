#!/usr/bin/env bash
bin_dir=../../bin

echo "Prepare WASM for $1"

cp ../../target/release/wbuild/${1}-parachain-runtime/${1}_parachain_runtime.compact.compressed.wasm ${bin_dir}/parachain_runtime.compact.compressed.wasm