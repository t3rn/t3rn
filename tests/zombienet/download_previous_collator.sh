#!/bin/bash
bin_dir=../../bin

echo "Downloading collator for parachain $1"

url=$(./releases.sh "$1" --latest | awk '{print $2}')

echo Url: $url

[ "$url" ] || exit 1
curl -L -o - "$url" | gunzip -c > ${bin_dir}/collator-old && chmod +x ${bin_dir}/collator-old
