#!/usr/bin/env bash
set -e
DIR=$(git rev-parse --show-toplevel)
BIN_DIR=$DIR/bin

case "$1" in
    t0rn|t1rn*)
        new_version=$(git tag --list --sort=-version:refname "v[0-9]*.[0-9]*.[0-9]*-rc.[0-9]*" | head -n 1)
    ;;
    t3rn*)
        new_version=$(git tag --list --sort=-version:refname "v[0-9]*.[0-9].[0-9]" | head -n 1)
    ;;
    *)
esac
echo New version: "$new_version"

./download_collator.sh $1
$DIR/scripts/update_parachain_versions.sh $1
$DIR/scripts/build_wasm.sh $1
cp $DIR/target/release/wbuild/${1}-parachain-runtime/${1}_parachain_runtime.compact.compressed.wasm  ${BIN_DIR}/parachain_runtime.compact.compressed.wasm
