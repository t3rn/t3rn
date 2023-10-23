#!/usr/bin/env bash
bin_dir=../../bin

case "$1" in
    t0rn*)
        echo "=========== T0RN ==========="
        new_version=$(git tag --list --sort=-version:refname "v[0-9]*.[0-9]*.[0-9]*-rc.[0-9]*" | head -n 1)
        echo New version: "$new_version"
        
        ./download_previous_collator.sh "t0rn"
        ../../scripts/build_wasm.sh "t0rn"
        ../../scripts/update_specs.sh "t0rn"
        cp ../../target/release/wbuild/t0rn-parachain-runtime/${1}_parachain_runtime.compact.compressed.wasm  ${bin_dir}/parachain_runtime.compact.compressed.wasm
    ;;
    t3rn*)
        echo "=========== T3RN ==========="
        new_version=$(git tag --list --sort=-version:refname "v[0-9]*.[0-9].[0-9]" | head -n 1)
        echo Tags: "$tags_list"
        echo New version: "$new_version"
        
        ./download_previous_collator.sh "t3rn"
        ../../scripts/build_wasm.sh "t3rn"
        ../../scripts/update_specs.sh "t3rn"
        cp ../../target/release/wbuild/t0rn-parachain-runtime/${1}_parachain_runtime.compact.compressed.wasm  ${bin_dir}/parachain_runtime.compact.compressed.wasm
    ;;
    *)
    exit 1;;
esac
