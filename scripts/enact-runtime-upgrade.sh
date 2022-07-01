#!/bin/bash

set -x

trap 'cleanup' EXIT

cleanup() {
  rm -rf node_modules
  rm -f package.json package-lock.json
}

if [[ -z "$1" || -z $2 || -z $3 ]]; then
  echo "usage: $0 'collator sudo secret' \$provider \$wasm"
  # fx: $0 'collator sudo secret' wss://dev.net.t3rn.io /tmp/wasm
  exit 1
fi

sudo_secret="$1"
provider=$2
wasm=$3
root_dir=$(git rev-parse --show-toplevel)
dryrun=$(echo "$@" | grep -o dry)

echo "enacting runtime upgrade... $dryrun"

if [[ -z $dryrun ]]; then
  # PROVIDER=$provider SUDO=$sudo_secret WASM=$wasm WHEN=$when \
  #   node $root_dir/scripts/schedule-runtime-upgrade.js

  node <<<"
    var fs = require('fs')
    var buf = fs.readFileSync('$wasm')
    buf = buf.toString('hex')
    buf = '0x' + buf
    fs.writeFileSync('$wasm', buf)
  "

  npx --yes @polkadot/api-cli@0.51.7 \
    --ws $provider \
    --sudo \
    --seed "$sudo_secret" \
    --params $wasm \
    tx.parachainSystem.enactAuthorizedUpgrade
else
  # echo "
  #   PROVIDER=$provider SUDO=$sudo_secret WASM=$wasm WHEN=$when \\
  #     node $root_dir/scripts/schedule-runtime-upgrade.js
  # "
  # cat $root_dir/scripts/schedule-runtime-upgrade.js
  echo "
  npx --yes @polkadot/api-cli@0.51.7 
    --ws $provider 
    --sudo 
    --seed "$sudo_secret" 
    --params $wasm 
    tx.parachainSystem.enactAuthorizedUpgrade
  "
fi