#!/bin/bash

set -x

if [[ -z "$1" || -z $2 || -z $3 || -z $4 || -z $5 ]]; then
  echo "usage: $0 'collator sudo secret' \$ws_provider \$http_provider \$tag \$when [--dryrun]"
  # fx: $0 'collator sudo secret' ws://localhost:1933 http://localhost:1833 v0.0.0-up 33 --dryrun
  exit 1
fi

trap 'cleanup' EXIT

cleanup() {
  rm -rf node_modules
  rm -f package.json package-lock.json
  rm $used_wasm
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
when=$5
used_wasm=$HOME/.runtime-upgrade.wasm
root_dir=$(git rev-parse --show-toplevel)
dryrun=$(echo "$@" | grep -o dry)

if ! git tag --list | grep -Fq $tag; then
  echo -e "$tag is not a git tag\ntag and push the runtime for the upgrade" >&2
  exit 1
fi

if ! cargo install --list | grep -Fq 'srtool-cli v0.8.0'; then
  echo "installing srtool-cli..."
  cargo install \
    --git https://github.com/chevdor/srtool-cli \
    --tag v0.8.0
fi

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

new_spec_version=$(grep -Pom1 'spec_version: *\K[0-9]+' $root_dir/runtime/parachain/src/lib.rs)
new_impl_version=$(grep -Pom1 'impl_version: *\K[0-9]+' $root_dir/runtime/parachain/src/lib.rs)
new_tx_version=$(grep -Pom1 'transaction_version: *\K[0-9]+' $root_dir/runtime/parachain/src/lib.rs)
new_author_version=$(grep -Pom1 'authoring_version: *\K[0-9]+' $root_dir/runtime/parachain/src/lib.rs)

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

echo "🐳 monkey patching srtool-cli..."

docker pull paritytech/srtool:1.57.0

image_id=$( \
  docker image ls \
  | \
  grep -P 'paritytech/srtool\s+1\.57\.0' \
  | \
  awk '{ print $3 }' \
)

srtool_latest=$( \
  curl -sSfL \
  https://hub.docker.com/v2/repositories/paritytech/srtool/tags/?page_size=1000 \
  | \
  jq -r '.results | .[] | .name' \
  | head -n1 \
)

docker tag $image_id t3rn/srtool:$srtool_latest

echo "🏭 compiling runtime wasm..."

report="$( \
  srtool build \
    --image t3rn/srtool \
    --profile release \
    --runtime-dir runtime/parachain \
    --package circuit-parachain-runtime \
    --json \
    $root_dir \
)"

report="{${report#*\{}" # left trimming nonjson
wasm="$root_dir/$(jq -r .runtimes.compressed.wasm <<<"$report")"
hash=$( \
  jq -r .runtimes.compressed.blake2_256 <<<"$report" \
)

cp $wasm $used_wasm

read -n 1 -p "e2e-tested on rococo-local?
runtime upgrade tested on rococo-local?
runtime benchmarked?
storage migrated?
(y/n) " answer

echo

if [[ "${answer,,}" != "y" ]]; then exit 1; fi

head=$(get_finalized_head)

if [[ $head -gt $(( when - 5 )) ]]; then
  echo "reschedule at a later block" >&2
  exit 1
fi

echo "🎱 authorizing runtime upgrade... $dryrun"

npm i @polkadot/api@8.6.2

if [[ -z $dryrun ]]; then
  PROVIDER=$ws_provider SUDO=$sudo_secret HASH=$hash WHEN=$when \
    node $root_dir/scripts/schedule-authorize-runtime-upgrade.js

  echo "scheduled runtime upgrade authorization at block $when"
else
  echo "
    PROVIDER=$ws_provider SUDO=$sudo_secret HASH=$hash WHEN=$when \
      node $root_dir/scripts/schedule-authorize-runtime-upgrade.js
  "
  cat $root_dir/scripts/schedule-authorize-runtime-upgrade.js
fi

echo "🛂 awaiting runtime upgrade authorization..."

head=$(get_finalized_head)

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