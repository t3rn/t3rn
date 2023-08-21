#!/bin/bash

POLKADOT_CLI_VERSION="@polkadot/api-cli@0.55.3"

if [[ -z "$1" || -z $2 || -z $3 ]]; then
  echo "usage 'sudo secret' \$tag \$parachain_name [--dryrun]"
  # fx: $0 'sudo secret' v0.0.0-up t0rn --dryrun
  exit 1
fi

get_finalized_head(){
  block_hash="$( \
    curl \
      -sSfH "content-type: application/json" \
      -d '{"id":1,"jsonrpc":"2.0","method":"chain_getFinalizedHead","params":[]}' \
      $rpc_endpoint \
      | \
      jq -r .result \
  )"
  block_number="$( \
    curl \
      -sSfH "content-type: application/json" \
      -d "{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"chain_getBlock\",\"params\":[\"$block_hash\"]}" \
      $rpc_endpoint \
      | \
      jq -r .result.block.header.number \
  )"
  printf $(( block_number ))
}

sudo_secret="$1"
tag=$2
parachain_name=$3
rpc_endpoint="wss://rpc.${parachain_name}.io"
wasm_binary=./${parachain_name}-parachain-runtime-${tag}.compact.compressed.wasm
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
  echo "🚨 $wasm_binary does not exist!"
  exit 1
fi

if ! git tag --list | grep -Fq $tag; then
  echo -e "$tag is not a git tag\ntag and push the runtime for the upgrade"
  exit 1
fi

set -Ee

echo "🐙 checking out $tag..."
git checkout $tag &>/dev/null
echo "✅ tag checked out"
echo

echo "🔎 making sure runtime version got updated..."

runtime_version="$( \
  npm exec -- $POLKADOT_CLI_VERSION \
    --ws $rpc_endpoint \
    consts.system.version \
    2>/dev/null )"

old_spec_version=$(jq -r .version.specVersion <<<"$runtime_version")
old_impl_version=$(jq -r .version.implVersion <<<"$runtime_version")
old_tx_version=$(jq -r .version.transactionVersion <<<"$runtime_version")
old_author_version=$(jq -r .version.authoringVersion <<<"$runtime_version")

new_spec_version=$(cat $root_dir/runtime/${parachain_name}-parachain/src/lib.rs | grep -o 'spec_version: [0-9]*' | tail -1 | grep -o '[0-9]*')
new_impl_version=$(cat $root_dir/runtime/${parachain_name}-parachain/src/lib.rs | grep -o 'impl_version: [0-9]*' | tail -1 | grep -o '[0-9]*')
new_tx_version=$(cat $root_dir/runtime/${parachain_name}-parachain/src/lib.rs | grep -o 'transaction_version: [0-9]*' | tail -1 | grep -o '[0-9]*')
new_author_version=$(cat $root_dir/runtime/${parachain_name}-parachain/src/lib.rs | grep -o 'authoring_version: [0-9]*' | tail -1 | grep -o '[0-9]*')

if [[ $new_spec_version -le $old_spec_version ]]; then
  echo "🔴 runtime spec version not incremented"
  exit 1
fi

if [[ $new_impl_version -le $old_impl_version ]]; then
  echo "🔴 runtime impl version not incremented"
  exit 1
fi

if [[ $new_tx_version -le $old_tx_version ]]; then
  echo "🔴 runtime transaction version not incremented"
  exit 1
fi

if [[ $new_author_version -le $old_author_version ]]; then
  echo "🔴 runtime authoring version not incremented"
  exit 1
fi
echo "✅ runtime versions updated"

echo
echo "🫧 check WASM artifact..."
wasm_hash_calculated=$(subwasm info --json $wasm_binary | jq -r .blake2_256)
wasm_hash_fetched="$(cat ${wasm_binary}.blake2_256)"
echo "🔢 WASM blake2_256 hash: $wasm_hash_calculated"
echo "🔢 WASM blake2_256 hash fetched from release: $wasm_hash_fetched"

if [[ "$wasm_hash_calculated" != "$wasm_hash_fetched" ]]; then
  echo "🔴 WASM blake2_256 hash is not matching"
  exit 1
else
  echo "✅ WASM blake2_256 hash is matching"
fi

echo "⚙️ set_code runtime upgrade... $dryrun"

# Skip converting wasm to hex when run with dryrun flag
if [[ -z $dryrun ]]; then
  node <<<"
    var fs = require('fs')
    fs.writeFileSync(
      '$wasm_binary',
      '0x' + fs.readFileSync('$wasm_binary').toString('hex')
    )
  "
  echo "✅ Converted WASM to hex"
fi

# Execute runtime upgrade if dryrun flag is not present
if [[ -z $dryrun ]]; then
  npm exec -- $POLKADOT_CLI_VERSION \
    --ws $rpc_endpoint \
    --sudo \
    --seed "$sudo_secret" \
    --params $wasm_binary \
    tx.system.setCode
fi
echo "✅ runtime upgrade executed... $dryrun"
