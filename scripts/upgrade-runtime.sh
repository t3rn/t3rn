#!/bin/bash

set -xEe

if [[ -z "$1" || -z $2 || -z $3 ]]; then
  echo "usage: $0 'sudo secret' \$provider \$wasm"
  # fx: ./upgrade-runtime.sh 'sudo secret' wss://dev.net.t3rn.io ../specs/t0rn.wasm
  exit 1
fi

sudo_secret="$1"
provider=$2
wasm=$3

tmp_dir=$(mktemp -d)
wd=$(pwd)
cd $tmp_dir
npm i blake2b
hash=0x$( \
  node --print " \
    const blake2b = require('blake2b'); \
    const magicBytes = Buffer.from([1, 3]); \
    const wasmHex = fs.readFileSync('$wasm', 'utf8');
    const wasmBuf = Buffer.from(wasmHex.slice(2), 'hex'); \
    const digest = blake2b(32).update(wasmBuf).digest(); \
    const prefixed = Buffer.concat([magicBytes, digest]); \
    const hash = blake2b(32).update(prefixed).digest('hex'); \
    hash \
    " \
)
cd $wd

read -n 1 -p "runtime tested and incremented spec version? (y/n) " answer

echo

if [[ "${answer,,}" != "y" ]]; then exit 1; fi

npx --yes @polkadot/api-cli@beta \
  --ws $provider \
  --sudo \
  --seed "$sudo_secret" \
  tx.parachainSystem.authorizeUpgrade \
  $hash

npx @polkadot/api-cli@beta \
  --ws $provider \
  --sudo \
  --seed "$sudo_secret" \
  --params $wasm \
  tx.parachainSystem.enactAuthorizedUpgrade