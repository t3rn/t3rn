#!/bin/bash
bin_dir=../../bin

echo "Downloading collator for parachain $1"

url=$(./releases.sh "$1" --upgrade | awk '{print $2}' | head -n 1)

echo Url: $url

[ "$url" ] || exit 1
curl -L -o - "$url" | gunzip -c > ${bin_dir}/collator-old && chmod +x ${bin_dir}/collator-old
