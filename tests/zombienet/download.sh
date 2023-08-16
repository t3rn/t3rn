#!/usr/bin/env bash
bin_dir=../../bin

echo "=========== ${1} ==========="
version=$(git tag --list --sort=-version:refname "v[0-9]*.[0-9]*.[0-9]*-rc.[0-9]*" | head -n 1)
echo Previous version: "$version"

# Prepare collator binary
./download_collator.sh "${1}"
# Prepare WASM binary
cp ../../target/release/wbuild/${1}-parachain-runtime/${1}_parachain_runtime.compact.compressed.wasm ${bin_dir}/parachain_runtime.compact.compressed.wasm