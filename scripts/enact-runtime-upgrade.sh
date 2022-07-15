#!/bin/bash

# NOTE: Usually this script should not be necessary for a runtime upgrade
# as ./upgrade-runtime.sh covers the entire authorize-enact upgrade flow.
# Nonetheless, in case the twofold upgrade process gets interrupted somehow
# this script can complete the enactment of a previously authorized upgrade.

set -x

if [[ -z "$1" || -z $2 || -z $3 ]]; then
  echo "usage: $0 'collator sudo secret' \$ws_provider \$wasm"
  # fx: $0 'collator sudo secret' wss://dev.net.t3rn.io /tmp/wasm
  exit 1
fi

sudo_secret="$1"
ws_provider=$2
wasm=$3
used_wasm=$HOME/.runtime-upgrade.wasm
root_dir=$(git rev-parse --show-toplevel)
dryrun=$(echo "$@" | grep -o dry)

cp $wasm $used_wasm

echo "⚙️ enacting runtime upgrade... $dryrun"

if [[ -z $dryrun ]]; then
  node <<<"
    var fs = require('fs')
    fs.writeFileSync(
      '$used_wasm',
      '0x' + fs.readFileSync('$used_wasm').toString('hex')
    )
  "
  npx --yes @polkadot/api-cli@0.51.7 \
    --ws $ws_provider \
    --sudo \
    --seed "$sudo_secret" \
    --params $used_wasm \
    tx.parachainSystem.enactAuthorizedUpgrade
else
  echo "
  node <<<\"
    var fs = require('fs')
    fs.writeFileSync(
      '$used_wasm',
      '0x' + fs.readFileSync('$used_wasm').toString('hex')
    )
  \"
  npx --yes @polkadot/api-cli@0.51.7 
    --ws $ws_provider 
    --sudo 
    --seed "$sudo_secret" 
    --params $used_wasm 
    tx.parachainSystem.enactAuthorizedUpgrade
  "
fi