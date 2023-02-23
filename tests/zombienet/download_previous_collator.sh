#!/bin/bash
bin_dir=../../bin

echo "Downloading collator for tag $1"

export url=$(curl -s https://api.github.com/repos/t3rn/t3rn/releases/tags/"$1" | jq -r '.assets[] | select(.name | endswith ("unknown-linux-gnu.gz")).browser_download_url')
echo Url: $url

[ "$url" ] || exit 1
curl -L -o - "$url" | gunzip -c > ${bin_dir}/collator-old && chmod +x ${bin_dir}/collator-old
