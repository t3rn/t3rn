#!/usr/bin/env bash
bin_dir=../../bin

case "$1" in
  t0rn*)
    echo "=========== T0RN ==========="
    new_version=$(git tag --list --sort=-version:refname "v[0-9]*.[0-9]*.[0-9]*-rc.[0-9]*" | head -n 1)
    echo New version: "$new_version"

    ./download_previous_collator.sh "t0rn"
    ./download_latest_wasm_blob.sh "$new_version"
    ;;
  t3rn*)
    echo "=========== T3RN ==========="
    new_version=$(git tag --list --sort=-version:refname "v[0-9]*.[0-9].[0-9]" | head -n 1)
    echo Tags: "$tags_list"
    echo New version: "$new_version"

    ./download_previous_collator.sh "t3rn"
    ./download_latest_wasm_blob.sh "$new_version"
    ;;
  *)        
  exit 1;;
esac
