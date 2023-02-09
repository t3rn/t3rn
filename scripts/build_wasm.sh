#!/bin/bash -e

if [[ -z "$1" ]]; then
  echo "Need to pass a parachain name as argument"
  exit 1
fi

parachain_name=$1

# TODO: subwasm always missing on epyc even though it's installed
cargo install --locked --git https://github.com/chevdor/subwasm --tag v0.16.1

echo "🏭 building runtime wasm..."

cargo build \
  --locked \
  --profile release \
  --package ${parachain_name}-parachain-runtime \
  --target-dir target/ \
  -Z unstable-options

used_wasm=target/release/wbuild/${parachain_name}-parachain-runtime/${parachain_name}_parachain_runtime.compact.compressed.wasm

echo "🔢 hashing ${parachain_name}_parachain_runtime.compact.compressed.wasm..."
wasm_info=$(subwasm info --json $used_wasm)
echo $wasm_info | jq .
echo $wasm_info | jq -r .blake2_256 > $used_wasm.blake2_256
