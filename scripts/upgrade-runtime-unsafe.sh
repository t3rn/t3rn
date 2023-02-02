#!/bin/bash

set -x

if [[ -z "$1" || -z $2 || -z $3 || -z $4 || -z $5 ]]; then
  echo "usage 'collator sudo secret' \$ws_provider \$http_provider \$tag \$parachain_name [--dryrun]"
  # fx: $0 'collator sudo secret' ws://localhost:1933 http://localhost:1833 v0.0.0-up 33 t0rn --dryrun
  exit 1
fi

trap 'cleanup' EXIT

cleanup() {
  rm -rf $root_dir/scripts/node_modules
  rm -f \
    $root_dir/scripts/package.json \
    $root_dir/scripts/package-lock.json \
    $used_wasm
}

get_finalized_head(){
  block_hash="$( \
    curl \
      -sSfH "content-type: application/json" \
      -d '{"id":1,"jsonrpc":"2.0","method":"chain_getFinalizedHead","params":[]}' \
      $http_provider \
      | \
      jq -r .result \
  )"
  block_number="$( \
    curl \
      -sSfH "content-type: application/json" \
      -d "{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"chain_getBlock\",\"params\":[\"$block_hash\"]}" \
      $http_provider \
      | \
      jq -r .result.block.header.number \
  )"
  printf $(( block_number ))
}

sudo_secret="$1"
ws_provider=$2
http_provider=$3
tag=$4
parachain_name=$5
used_wasm=$HOME/.runtime-upgrade.wasm
root_dir=$(git rev-parse --show-toplevel)
dryrun=$(echo "$@" | grep -o dry)

if ! git tag --list | grep -Fq $tag; then
  echo -e "$tag is not a git tag\ntag and push the runtime for the upgrade" >&2
  exit 1
fi

echo "🏭 installing chevdor/subwasm v0.16.1..."
cargo install --locked --git https://github.com/chevdor/subwasm --tag v0.16.1

set -Ee

echo "🐙 checking out $tag..."

git checkout $tag

echo "🔎 making sure runtime version got updated..."

runtime_version="$( \
  npx --yes @polkadot/api-cli@0.51.7 \
    --ws $ws_provider \
    consts.system.version \
)"

old_spec_version=$(jq -r .version.specVersion <<<"$runtime_version")
old_impl_version=$(jq -r .version.implVersion <<<"$runtime_version")
old_tx_version=$(jq -r .version.transactionVersion <<<"$runtime_version")
old_author_version=$(jq -r .version.authoringVersion <<<"$runtime_version")

new_spec_version=$(cat $root_dir/runtime/${parachain_name}-parachain/src/lib.rs | grep -o 'spec_version: [0-9]*' | tail -1 | grep -o '[0-9]')
new_impl_version=$(cat $root_dir/runtime/${parachain_name}-parachain/src/lib.rs | grep -o 'impl_version: [0-9]*' | tail -1 | grep -o '[0-9]')
new_tx_version=$(cat $root_dir/runtime/${parachain_name}-parachain/src/lib.rs | grep -o 'transaction_version: [0-9]*' | tail -1 | grep -o '[0-9]')
new_author_version=$(cat $root_dir/runtime/${parachain_name}-parachain/src/lib.rs | grep -o 'authoring_version: [0-9]*' | tail -1 | grep -o '[0-9]')

if [[ $new_spec_version != $((old_spec_version + 1)) ]]; then
  echo "runtime spec version not incremented" >&2
  exit 1
fi

if [[ $new_impl_version != $((old_impl_version + 1)) ]]; then
  echo "runtime impl version not incremented" >&2
  exit 1
fi

if [[ $new_tx_version != $((old_tx_version + 1)) ]]; then
  echo "runtime transaction version not incremented" >&2
  exit 1
fi

if [[ $new_author_version != $((old_author_version + 1)) ]]; then
  echo "runtime authoring version not incremented" >&2
  exit 1
fi

echo "🏭 building runtime wasm..."

cargo build \
  --locked \
  --profile release \
  --package ${parachain_name}-parachain-runtime \
  --target-dir $root_dir/target/ \
  -Z unstable-options

used_wasm=$root_dir/target/release/wbuild/${parachain_name}-parachain-runtime/${parachain_name}_parachain_runtime.compact.compressed.wasm

echo "🔢 hashing ${parachain_name}_parachain_runtime.compact.compressed.wasm..."

hash=$(subwasm info --json $used_wasm | jq -r .blake2_256)

echo "🎱 authorizing runtime upgrade... $dryrun"

npm i @polkadot/api@8.6.2

if [[ -z $dryrun ]]; then
  PROVIDER=$ws_provider SUDO=$sudo_secret HASH=$hash WHEN=$after \
    node $root_dir/scripts/schedule-authorize-runtime-upgrade.js

  echo "scheduled runtime upgrade authorization at block $after"
else
  echo "
    PROVIDER=$ws_provider SUDO=$sudo_secret HASH=$hash AFTER=$after \
      node $root_dir/scripts/schedule-authorize-runtime-upgrade.js
  "
  cat $root_dir/scripts/schedule-authorize-runtime-upgrade.js
fi

echo "🛂 awaiting runtime upgrade authorization..."

head=$(get_finalized_head)
when=$(( head + after ))

while [[ $head -ne $when ]]; do
  sleep 12s
  head=$(get_finalized_head)
done

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