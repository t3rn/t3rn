#!/bin/bash
bin_dir=../../bin

echo "Downloading wasm for tag $1"

export url=$(curl -s https://api.github.com/repos/t3rn/t3rn/releases/tags/"$1" | jq -r '.assets[] | select(.name | endswith ("compact.compressed.wasm")).browser_download_url')
echo Url: $url

[ "$url" ] || exit 1
curl -L -o - "$url" > ${bin_dir}/parachain_runtime.compact.compressed.wasm