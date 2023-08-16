#!/usr/bin/env bash
version=$(($2+1))
bin_dir=../../bin

echo "=========== ${1} ==========="
version=$(git tag --list --sort=-version:refname "v[0-9]*.[0-9]*.[0-9]*-rc.[0-9]*" | head -n 1)
echo Previous version: "$version"

./download_previous_collator.sh "${1}"
./prepare_wasm.sh "$version"