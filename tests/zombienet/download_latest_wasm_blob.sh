#!/bin/bash
new_version=$(($2+1))
bin_dir=../../bin

case "$1" in
  t0rn*)
    export tag_version=$(git tag --list --sort=-version:refname "v[0-9]*.[0-9]*.[0-9]*-rc.[0-9]*" | head -n 1)
    echo Version: "$tag_version"
    export tag_version=test

    export url=$(curl -s https://api.github.com/repos/t3rn/t3rn/releases/tags/"$tag_version" | jq -r '.assets[] | select(.name == "t0rn-parachain-runtime.compact.compressed.wasm").browser_download_url')
    echo Url: $url
    
    [ "$url" ] || exit 1
    curl -L -o - "$url" > ${bin_dir}/parachain_runtime.compact.compressed.wasm
    ;;
  t3rn*)
    export tag_version=$(version=git tag --list --sort=-version:refname "v[0-9]*.[0-9].[0-9]" | head -n 1)
    echo Version: "$tag_version"

    export url=$(curl -s https://api.github.com/repos/t3rn/t3rn/releases/tags/"$tag_version" | jq -r '.assets[] | select(.name == "t0rn-parachain-runtime.compact.compressed.wasm").browser_download_url')
    echo Url: $url
    
    [ "$url" ] || exit 1
    curl -L -o - "$url" > ${bin_dir}/parachain_runtime.compact.compressed.wasm
    ;;
  *)        exit 1;;
esac
