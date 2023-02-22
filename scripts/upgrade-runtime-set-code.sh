#!/bin/bash

POLKADOT_CLI_VERSION="@polkadot/api-cli@0.52.27"

if [[ -z "$1" || -z $2 || -z $3 ]]; then
  echo "usage 'sudo secret' \$tag \$parachain_name [--dryrun]"
  # fx: $0 'sudo secret' v0.0.0-up t0rn --dryrun
  exit 1
fi

trap 'cleanup' EXIT

cleanup() {
  rm -rf $root_dir/scripts/node_modules
  rm -f \
    $root_dir/scripts/package.json \
    $root_dir/scripts/package-lock.json \
    $wasm_binary
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
tag=$2
parachain_name=$3
ws_provider="wss://ws.${parachain_name}.io"
http_provider="https://rpc.${parachain_name}.io"
wasm_binary=./target/release/${parachain_name}_parachain_runtime.compact.compressed.wasm
root_dir=$(git rev-parse --show-toplevel)
dryrun=$(echo "$@" | grep -o dry) || true

echo "💈 Script started at $(get_finalized_head) block in ${parachain_name} chain"

if [[ ! -z $dryrun ]]; then
  echo
  echo "🐡 Running with dryrun flag!"
  echo
fi

# Check if wasm exists
if [[ ! -f $wasm_binary ]]; then
  echo "🚨 $wasm_binary does not exist!" >&2
  exit 1
fi

if ! git tag --list | grep -Fq $tag; then
  echo -e "$tag is not a git tag\ntag and push the runtime for the upgrade" >&2
  exit 1
fi

set -Ee

echo "🐙 checking out $tag..."

git checkout $tag &>/dev/null

echo "🔎 making sure runtime version got updated..."

runtime_version="$( \
  npx --yes $POLKADOT_CLI_VERSION \
    --ws $ws_provider \
    consts.system.version \
    2>/dev/null )"

old_spec_version=$(jq -r .version.specVersion <<<"$runtime_version")
old_impl_version=$(jq -r .version.implVersion <<<"$runtime_version")
old_tx_version=$(jq -r .version.transactionVersion <<<"$runtime_version")
old_author_version=$(jq -r .version.authoringVersion <<<"$runtime_version")

new_spec_version=$(cat $root_dir/runtime/${parachain_name}-parachain/src/lib.rs | grep -o 'spec_version: [0-9]*' | tail -1 | grep -o '[0-9]')
new_impl_version=$(cat $root_dir/runtime/${parachain_name}-parachain/src/lib.rs | grep -o 'impl_version: [0-9]*' | tail -1 | grep -o '[0-9]')
new_tx_version=$(cat $root_dir/runtime/${parachain_name}-parachain/src/lib.rs | grep -o 'transaction_version: [0-9]*' | tail -1 | grep -o '[0-9]')
new_author_version=$(cat $root_dir/runtime/${parachain_name}-parachain/src/lib.rs | grep -o 'authoring_version: [0-9]*' | tail -1 | grep -o '[0-9]')

# Skip version tests when run with dryrun flag
if [[ -z $dryrun ]]; then
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
fi

echo "🫧 Check WASM artifact..."
wasm_hash_calculated=$(subwasm info --json $wasm_binary | jq -r .blake2_256)
wasm_hash_fetched="$(cat ${wasm_binary}.blake2_256)"
echo "🔢 calculated WASM blake2_256 hash is $wasm_hash_calculated"
echo "🔢 fetched WASM blake2_256 hash from release is $wasm_hash_fetched"

if [[ "$wasm_hash_calculated" -ne "$wasm_hash_fetched" ]]; then
  echo "🔴 WASM artifact blake2_256 hash is not matching"
  exit 1
else
  echo "✅ WASM artifact blake2_256 hash is matching"
fi

echo "⚙️ set_code runtime upgrade... $dryrun"

# Convert wasm to hex and write to file
node <<<"
  var fs = require('fs')
  fs.writeFileSync(
    '$wasm_binary',
    '0x' + fs.readFileSync('$wasm_binary').toString('hex')
  )
"

if [[ -z $dryrun ]]; then
  npx --yes $POLKADOT_CLI_VERSION \
    --ws $ws_provider \
    --sudoUncheckedWeight "100000" \
    --seed "$sudo_secret" \
    --params $wasm_binary \
    tx.system.setCode
else
  echo "
  npx --yes $POLKADOT_CLI_VERSION 
    --ws $ws_provider
    --sudoUncheckedWeight "100000"
    --seed "$sudo_secret"
    --params $wasm_binary
    tx.system.setCode
  "
fi